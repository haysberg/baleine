use crate::utils::{env_var};
use std::process::Command;
use openssh::{Session, KnownHosts};
use tracing::{error, info, warn};

extern crate dotenv;

/// This function takes a container running on a node and saves it to the remote registry configured in /etc/baleine/baleine.conf
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved
pub async fn save(name: &String, node: &str) {

    // We parse the SAVE_URL parameter    
    let repo = env_var("SAVE_URL").unwrap_or({
        warn!("SAVE_URL not set in config file, using faraday.repo by default.");
        "faraday.repo".to_string()
    });

    //We create the SSH session
    let session = Session::connect(format!("ssh://root@{node}:22"), KnownHosts::Accept)
    .await
    .expect(&format!("Could not establish session to host {}", node).as_str());

    //We save the running container as an image
    let output = session.command("docker").raw_arg(format!("commit container {name}")).output().await.unwrap();
    match output.status.success() {
        true => info!("Successfully committed container {}", name),
        false => error!("Could not commit container on {}, is there one present ?", node)
    }

    //We tag the created image
    let output = session.command("docker").raw_arg(format!("image tag {name} {repo}/{name}")).output().await.unwrap();
    match output.status.success() {
        true => info!("Successfully tagged container as {repo}/{name} on {}", node, repo = repo, name = name),
        false => error!("Could not tag image on {}", node)
    }

    //We push the image to the R2Lab Docker repository
    let output = session.command("docker").raw_arg(format!("push --all-tags {repo}/{name}")).output().await.unwrap();
    match output.status.success() {
        true => info!("Successfully pushed {repo}/{name}", repo = repo, name = name),
        false => error!("Failed to push {repo}/{name}, is the repository reachable ?", repo = repo, name = name)
    }

    session.close().await.unwrap();
}

/// The entry() function works as an entrypoint that does a bit of parsing as well as other checks depending on the function it calls later
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved

pub async fn entry(name: &String, node: &String) {
    //We then run rhubarbe nodes with the nodes.
    //This is a prerequisite to save the nodes
    let output = Command::new("rhubarbe")
        .arg("nodes")
        .arg(node)
        .output()
        .unwrap();

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.truncate(stdout.len() - 1);
    //Once the nodes command has run, we get the list of parsed nodes
    //by getting the list of nodes in the NODES environment variable
    let nodes: Vec<&str> = stdout.split(" ").collect();

    //for each of the nodes, we run the save() function
    for node in nodes {
        save(name, node).await;
    }
}
