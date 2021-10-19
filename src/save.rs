use std::process::Command;
use crate::utils::ssh_command;

extern crate dotenv;
use dotenv_codegen::dotenv;

///This function takes a container running on a node and saves it to the
///remote registry configured in config.toml
pub fn save (args: Option<&clap::ArgMatches>, node: &str){    
    //We create the string for the command that we are going to execute remotely.
    //Here, we create a new image from the running container on the node, and push it to the
    //remote registry.
    let cmd = format!("docker commit container {repository}/{image_name} && docker push {repository}/{image_name}",
    repository = dotenv!("REGISTRY_URL"),
    image_name = args.unwrap().value_of("name").unwrap());
    
    match ssh_command(node.to_string(), cmd){
        Ok(_) => (),
        Err(_) => println!("{}", format!("PROBLEM DURING SSH CONNECTION TO NODE {node}", node = node))
    }   
}


pub fn entry(args: Option<&clap::ArgMatches>){

    let nodes_arg : String = args.unwrap().values_of("node").unwrap().collect();

    Command::new("sh")
    .arg("-c")
    .arg("nodes")
    .arg(nodes_arg)
    .output()
    .expect("failed to the nodes command. Are you on a machine with rhubarbe installed ?");
    
    let nodes : Vec<&str> = dotenv!("NODES").split(" ").collect();
    for node in nodes {
   	    
	    save(args, node);
    }
}
