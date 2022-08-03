use std::io::prelude::*;
use crate::utils::ssh_command;
use futures_lite::future;
use tracing::{error, info, debug, instrument};
use async_executor::Executor;

/// This function stops and removes the container currently running on a node even if there is none.
///
/// # Arguments
///
/// * `node` - The node you wish to remove the Docker container currently running on
#[instrument]
pub async fn destroy(node : &str){
    match ssh_command(node.to_string(), vec!["docker stop container".to_string(), "docker container prune -f".to_string()]).await{
        Ok(_) => (),
        Err(_) => error!("Error : could not connect to node {node}, are you sure it is on ?", node = node)
    }
}

/// This function stops and removes the container currently running on a node if there is one.
///
/// # Arguments
///
/// * `node` - The node you wish to remove the Docker container currently running on
#[instrument]
pub async fn destroy_if_container(node : &str){
    if crate::utils::container_deployed(node).await{
        match ssh_command(node.to_string(), vec!["docker stop container".to_string(), "docker container prune -f".to_string()]).await{
            Ok(_) => (),
            Err(_) => error!("Error : could not connect to node {node}, are you sure it is on ?", node = node)
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
#[instrument]
pub async fn entry(yes: &bool, nodes: &Option<Vec<String>>){

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
        

        debug!("Mapping : {}", "docker stop container && docker container prune -f".to_string());
        
        // Create a new executor.
        let ex = Executor::new();
        let mut tasks = Vec::new();

        //we create threads and destroy the nodes
        for node in nodes.iter(){
            tasks.push(ex.spawn(destroy(&node)));
        }
        for task in tasks {
            future::block_on(task);
        }
    }
    //If the user changes his mind, we simply put a message to tell him not to worry.
    else {
        info!("\nAborting.")
    }
}
