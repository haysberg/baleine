use crate::utils::{env_var, ssh_command};
use std::process::Command;
use tracing::error;

extern crate dotenv;

/// This function takes a container running on a node and saves it to the remote registry configured in config.toml
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved
pub fn save(name: &String, node: &str) {
    //We create the string for the command that we are going to execute remotely.
    //Here, we create a new image from the running container on the node, and push it to the
    //remote registry.
    let cmd = format!("docker commit container {image_name} && docker image tag {image_name} {repository}/{image_name} && docker push --all-tags {repository}/{image_name}",
    repository = env_var("SAVE_URL").unwrap_or("faraday.repo".to_string()),
    image_name = name);

    //We run the docker commit container command on the node. If the ssh_command() function doesn't work
    //We display an error message
    match ssh_command(node.to_string(), cmd) {
        Ok(_) => (),
        Err(_) => error!("Could not connect to {node}, is it on ?", node = node),
    }
}

/// The entry() function works as an entrypoint that does a bit of parsing as well as other checks depending on the function it calls later
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved
pub fn entry(name: &String, node: &String) {
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
        save(name, node);
    }
}
