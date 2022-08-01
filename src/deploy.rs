use crate::utils::ssh_command;
use crate::utils::stty_sane;
use crate::utils::env_var;
use crossbeam;
use tracing::info;
use tracing::{trace, error};


/// This function deploys a Docker container on a node given in input.
///
/// # Arguments
///
/// * `image` - Reference to a String. Name of the Docker image you are deploying.
/// * `options` - A list of string containing the different options given by the user. This is an Option object, so if no option has been given, it is going to be a None object.
/// * `command` - A list of string containing the command and flags given by the user. This is an Option object, so if no command has been given, it is going to be a None object.
pub fn deploy(
    image: &String,
    options: &Option<Vec<String>>,
    command: &Option<Vec<String>>,
    node: &str,
) {
    // We change the &Option<Vec<String>> object into a String using this method.
    let (command, options) = crate::utils::parse_cmd_opt(command, options);

    //We then create the command before sending it to the ssh_command() function
    let cmd = format!("docker pull {image} && docker run --name container {options} -v /home/container/container_fs:/var -v /lib/modules:/lib/modules -v /var/run/dbus:/var/run/dbus -v /sys/fs/cgroup:/sys/fs/cgroup --network=host --privileged --cap-add=ALL --dns {dns} {image} {command} && docker container ls -a",
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
        dns = env_var("DNS_ADDR").unwrap_or("192.168.3.100".to_string())
    );

    //Priting it just for debugging purposes
    trace!("Mapping : {}", cmd);

    //We run the SSH command
    match ssh_command(node.to_string(), cmd) {
        Ok(_) => info!("Succesfully deployed node {node}", node = node.to_string()),
        Err(_) => error!("Could not connect using SSH to {node}, is it on ?", node = node),
    }
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
pub fn entry(
    image: &String,
    options: &Option<Vec<String>>,
    nodes: &Option<Vec<String>>,
    bootstrap: &Option<String>,
    command: &Option<Vec<String>>,
) {
    //We call this function so that rhubarbe-nodes can parse our list of nodes provided by the user.
    let nodes = crate::utils::list_of_nodes(nodes);

    //We deploy the specified image if the --bootstrap option is used
    match bootstrap {
        Some(ndz) => {
            crate::utils::bootstrap(ndz, &nodes);
            crate::utils::rwait();
        }
        None => (),
    }

    //We destroy the containers running before on the host
    match crossbeam::scope(|scope| {
        for node in nodes.split(" ") {
            scope.spawn(move |_| {
                crate::destroy::destroy_if_container(&node);
            });
        }
    }) {
        Ok(_) => (),
        Err(_) => panic!("We could not destroy the running containers for an unknown reason."),
    };

    //We split our string from rhubarbe-nodes ("fit 01 fit02 fit03") into an array that we can iterate on (["fit01", "fit02", "fit03"])
    let mut nodes: Vec<_> = nodes.split(" ").collect();

    /*
     * We deploy the first node before all the others, to ensure that the docker image
     * will be pulled through the proxy for the rest of the nodes.
     * We use swap_remove as it always has a O(1) complexity.
     */
    deploy(image, options, command, nodes.swap_remove(0));

    if !nodes.is_empty() {
        //We then create a thread for each node, running the deploy command through SSH
        match crossbeam::scope(|scope| {
            for node in nodes {
                scope.spawn(move |_| {
                    deploy(image, options, command, &node);
                });
            }
        }) {
            //We display a message depending of the outcome of the commands
            Ok(_) => info!("Deployment complete !"),
            Err(_) => error!("ERROR DURING DEPLOYMENT"),
        };
    }

    //Cleaning up the terminal output in case the terminal is botched.
    stty_sane();
}
