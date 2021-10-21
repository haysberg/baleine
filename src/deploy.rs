use crate::utils::ssh_command;
use clap;
use crossbeam;
use std::process::{Command};
extern crate json;
extern crate dotenv;
use dotenv_codegen::dotenv;

/**
 * This function is used to deploy a container on a node
 */
pub fn deploy(args: &clap::ArgMatches, node: &str) {
    //We parse the Docker options that the user might have supplied
    let mut options: String = match args.value_of("options") {
        Some(_) => args.values_of("options").unwrap().collect(),
        //If there is no options provided we just return an empty string
        None => ("").to_string(),
    };

    //We add a space before each options passed on to Docker.
    //Without doing this they are glued to each other, causing the deployment to fail.
    options = str::replace(&options, "-", " -");

    //We do exactly the same for the command
    let command = match args.value_of("command") {
        Some(command) => command.to_string(),
        None => ("").to_string(),
    };

    //We then create the command before sending it to the ssh_command() function
    let cmd = format!("docker run --privileged --cap-add=ALL --name container {options} {image} {command} && docker container ls -a", options = options, image = args.value_of("image").unwrap(), command = command);
    
    //We run the SSH command
    match ssh_command(node.to_string(), cmd) {
        Ok(_) => (),
        Err(_) => println!(
            "{}",
            format!("Could not connect using SSH to {node}, is it on ?", node = node)
        ),
    }
}

/**
 * This function acts as an entry point for the deploy function. It does some parsing
 * And then creates threads to deploy the containers
 */
pub fn entry(args: &clap::ArgMatches) {
    //Parsing of the arguments so that they are in the scope of the function and not in main() anymore
    let args = args.subcommand_matches("deploy").unwrap();
    //Setting up the nodes variable provided by the user
    let nodes: String = args.values_of("nodes").unwrap().collect();

    //We run the "rhubarbe nodes" command to get a list of nodes
    //Basically we don't do the automatic parsing here.
    let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
        .arg(nodes)
        .output()
        .expect("Problem while running the nodes command");

    //We then take the list of nodes provided by rhubarbe, and trim the little \n at the end
    let mut nodes = String::from_utf8(cmd.stdout).unwrap();
    nodes.pop();

    //We deploy the latest r2dock compatible image if the bootstrap option is used
    if args.is_present("bootstrap") {
        println!("Deploying the latest r2dock image...");
        crate::utils::bootstrap(dotenv!("DEFAULT_BOOTSTRAP_IMAGE"), &nodes);
        println!("Waiting for the nodes to be available...");
        crate::utils::rwait();
    }

    //We then create a thread for each node, running the deploy command through SSH
    match crossbeam::scope(|scope| {
        for node in nodes.split(" ") {
            scope.spawn(move |_| {
                deploy(args, &node);
            });
        }
    }) {
        //We display a message depending of the outcome of the commands
        Ok(_) => println!("Deployment complete !"),
        Err(_) => println!("ERROR DURING DEPLOYMENT"),
    };
}