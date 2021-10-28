use std::process::Command;
use crate::utils::ssh_command;

extern crate dotenv;
use dotenv_codegen::dotenv;

/**
 * This function takes a container running on a node and saves it to the
 * remote registry configured in config.toml
 */
pub fn save (args: &clap::ArgMatches, node: &str){    
    //We create the string for the command that we are going to execute remotely.
    //Here, we create a new image from the running container on the node, and push it to the
    //remote registry.
    let cmd = format!("docker commit container {repository}/{image_name} && docker push {repository}/{image_name}",
    repository = dotenv!("REGISTRY_URL"),
    image_name = args.value_of("name").unwrap());

    //We run the docker commit container command on the node. If the ssh_command() function doesn't work
    //We display an error message
    match ssh_command(node.to_string(), cmd){
        Ok(_) => (),
        Err(_) => println!("{}", format!("Could not connect to {node}, is it on ?", node = node))
    }   
}

/**
 * The entry() function works as an entrypoint that does a bit of parsing
 * as well as other checks depending on the function it calls later
 */
pub fn entry(args: &clap::ArgMatches){
    //Parsing of the arguments so that they are in the scope of the function and not in main() anymore
    let args = args.subcommand_matches("save").unwrap();

    //We parse the list of nodes given by the user
    let nodes_arg : String = args.values_of("node").unwrap().collect();

    //We then run rhubarbe nodes with the nodes.
    //This is a prerequisite to save the nodes
    let output = Command::new("rhubarbe")
    .arg("nodes")
    .arg(nodes_arg)
    .output()
    .unwrap();

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.truncate(stdout.len() - 1);
    
    //Once the nodes command has run, we get the list of parsed nodes
    //by getting the list of nodes in the NODES environment variable
    let nodes : Vec<&str> = stdout.split(" ").collect();

    //for each of the nodes, we run the save() function
    for node in nodes {
	    save(args, node);
    }
}
