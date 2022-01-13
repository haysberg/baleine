use std::io::prelude::*;
use crate::utils::ssh_command;
use crate::utils::stty_sane;

/// This function stops and removes the container currently running on a node even if there is none.
///
/// # Arguments
///
/// * `node` - The node you wish to remove the Docker container currently running on
pub fn destroy(node : &str){
    match ssh_command(node.to_string(), "docker stop container && docker container prune -f".to_string()){
        Ok(_) => (),
        Err(_) => println!("{}", format!("Error : could not connect to node {node}, are you sure it is on ?", node = node))
    }
}

/// This function stops and removes the container currently running on a node if there is one.
///
/// # Arguments
///
/// * `node` - The node you wish to remove the Docker container currently running on
pub fn destroy_if_container(node : &str){
    if crate::utils::container_deployed(node){
        match ssh_command(node.to_string(), "docker stop container && docker container prune -f".to_string()){
            Ok(_) => (),
            Err(_) => println!("{}", format!("Error : could not connect to node {node}, are you sure it is on ?", node = node))
        }
    }
}

/// Entry point for the destroy feature. 
/// Does parsing and asks for user input for confirmation
///
/// # Arguments
///
/// * `nodes` - The list of nodes you wish to remove the Docker containers currently running on
/// * `yes` - Does not ask for user input to delete the container
pub fn entry(yes: &bool, nodes: &Option<Vec<String>>){

    //We deal with the "yes" flag, which can be triggered with -y or --yes (cf args.yaml)
    //If the user hasn't put the flag, we ask him if he really wants to delete the containers
    let mut choice = String::new();
    if !yes{
        print!("Are you sure you want to destroy the containers ? [y/N] ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice).expect("Problem when reading the line.");
    } else {
        //If he put the "yes" flag we just change the choice string without asking.
        choice.push_str("y")
    }

    //If the user is okay with it, we proceed with the deletion
    if choice.trim() == "y" {        
        //Setting up the nodes variable
        let nodes = crate::utils::list_of_nodes(nodes);

        println!("Mapping : {}", "docker stop container && docker container prune -f".to_string());
        
        //we create threads and destroy the nodes
        match crossbeam::scope(|scope| {
            for node in nodes.split(" ") {
                scope.spawn(move |_| {
                    destroy(&node);
                });
            }
        }) {
            Ok(_) => println!("Destruction complete !"),
            Err(_) => println!("ERROR DURING DESTRUCTION"),
        };

        stty_sane();
    }
    //If the user changes his mind, we simply put a message to tell him not to worry.
    else {
        println!("\nAborting.")
    }
}
