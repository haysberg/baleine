use crate::utils::env_var;
use futures::future::join_all;
use openssh::KnownHosts;
use openssh::Session;
use tracing::{error, info, warn};


/// This function deploys a Docker container on a node given in input.
///
/// # Arguments
///
/// * `image` - Reference to a String. Name of the Docker image you are deploying.
/// * `options` - A list of string containing the different options given by the user. This is an Option object, so if no option has been given, it is going to be a None object.
/// * `command` - A list of string containing the command and flags given by the user. This is an Option object, so if no command has been given, it is going to be a None object.
pub async fn deploy(
    image: &String,
    options: &Option<Vec<String>>,
    command: &Option<Vec<String>>,
    node: &String,
) {
    // We change the &Option<Vec<String>> object into a String using this method.
    let (command, options) = crate::utils::parse_cmd_opt(command, options);

    let deploy_cmd = format!("run --name container {options} -v /home/container/container_fs:/var -v /lib/modules:/lib/modules -v /var/run/dbus:/var/run/dbus -v /sys/fs/cgroup:/sys/fs/cgroup --network=host --privileged --cap-add=ALL --dns {dns} {image} {command} && docker container ls -a",
        options = match options {
            None => format!(""),
            Some(content) => content
        },
        image = image,
        //If the command is empty, we don't add double quotes.
        command = match command {
            None => format!(""),
            Some(content) => content
        },
        dns = env_var("DNS_ADDR").unwrap_or({
            warn!("DNS_ADDR not set in config file, using 192.168.3.100 by default.");
            "192.168.3.100".to_string()
        })
    );

    let session = Session::connect(format!("ssh://root@{node}:22"), KnownHosts::Accept)
    .await
    .expect(&format!("Could not establish session to host {}", node).as_str());

    //We stop the previous container if there is one
    let output = session.command("docker").raw_arg("stop container").output().await.unwrap();
    match output.status.success() {
        true => info!("Stopped container on {}", node),
        false => ()
    }

    //We delete the previous container if there is one
    let output = session.command("docker").raw_arg("rm container").output().await.unwrap();
    match output.status.success() {
        true => info!("Deleted container on {}", node),
        false => ()
    }

    //We pull the docker image on the server
    let output = session.command("docker").raw_arg(format!("pull {image}")).output().await.unwrap();
    match output.status.success() {
        true => info!("Pulled image {} on {}", image, node),
        false => error!("Could not pull image {} on {}, output : {:?}", image, node, output.stdout)
    }

    //Then we deploy the new container
    let output = session.command("docker").raw_arg(deploy_cmd).output().await.unwrap();
    match output.status.success() {
        true => info!("Successfully deployed image {} on {}", image, node),
        false => error!("Could not deploy image {} on {}, output : {:?}", image, node, output.stdout)
    }
    
    session.close().await.unwrap();
}

/// This is the entry function for the Deploy function.
/// This function is used to do all the necessary steps before we send the SSH command to deploy the Docker container.
///
/// # Arguments
///
/// * `image` - Reference to a String. Name of the Docker image you are deploying.
/// * `options` - A list of string containing the different options given by the user. This is an Option object, so if no option has been given, it is going to be a None object.
/// * `command` - A list of string containing the command and flags given by the user. This is an Option object, so if no command has been given, it is going to be a None object.
/// * `bootstrap` - The name of the disk image we will deploy before deploying the Docker container. Option object.
/// * `command` - The command that we will pass to the Docker container, overriding the possible entrypoint. Optional Vector of String, that might contain the name of the command and all the arguments given to it.
/// For example : ["ls", "--all", "-t"]
pub async fn entry(
    image: &String,
    options: &Option<Vec<String>>,
    nodes: &Option<Vec<String>>,
    bootstrap: &Option<String>,
    command: &Option<Vec<String>>,
) {
    //We call this function so that rhubarbe-nodes can parse our list of nodes provided by the user.
    let mut nodes = crate::utils::list_of_nodes(nodes);
    
    //We deploy the specified image if the --bootstrap option is used
    match bootstrap {
        Some(ndz) => {
            match crate::utils::bootstrap(ndz, &nodes).await {
                Ok(_) => info!("Deployed {ndz} on hosts : {hosts}", ndz = ndz, hosts = nodes.clone().join(" ")),
                Err(e) => error!("Could not deploy the chosen disk image, error : {}", e)
            }

            match crate::utils::rwait().await {
                Ok(_) => (),
                Err(e) => error!("Could not execute rwait, error : {}", e)
            }
        }
        None => (),
    }
    
    /*
     * We deploy the first node before all the others, to ensure that the docker image
     * will be pulled through the proxy for the rest of the nodes.
     * We use swap_remove as it always has a O(1) complexity.
     */
    let first_node = nodes.swap_remove(0);
    info!("Deploying first node : {}", first_node);
    deploy(image, options, command, &first_node).await;

    if !nodes.is_empty() {
        let mut tasks = Vec::new();

        //we create threads and destroy the nodes
        for node in nodes.iter(){
            tasks.push(deploy(image, options, command, &node));
        }
        join_all(tasks).await;
    }
}
