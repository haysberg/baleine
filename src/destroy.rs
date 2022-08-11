use std::io::prelude::*;
use futures::future::join_all;
use openssh::{Session, KnownHosts};
use tracing::{error, info, };

/// This function stops and removes the container currently running on a node even if there is none.
///
/// # Arguments
///
/// * `node` - The node you wish to remove the Docker container currently running on
pub async fn destroy(node : &str){

    //We create the SSH session
    let session = Session::connect(format!("ssh://root@{node}:22"), KnownHosts::Accept)
    .await
    .expect(&format!("Could not establish session to host {}", node).as_str());

    //We stop the previous container if there is one
    let output = session.command("docker").raw_arg("stop container").output().await.unwrap();
    match output.status.success() {
        true => info!("Stopped container on {}", node),
        false => error!("Could not stop container on {}, is there one present ?", node)
    }

    //We delete the previous container if there is one
    let output = session.command("docker").raw_arg("container prune -f").output().await.unwrap();
    match output.status.success() {
        true => info!("Deleted container on {}", node),
        false => error!("Could not delete container on {}, is there one present ?", node)
    }

    session.close().await.unwrap();
}

/// Entry point for the destroy feature. 
/// Does parsing and asks for user input for confirmation
///
/// # Arguments
///
/// * `nodes` - The list of nodes you wish to remove the Docker containers currently running on
/// * `yes` - Does not ask for user input to delete the container

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

        if !nodes.is_empty() {
            let mut tasks = Vec::new();
    
            //we create threads and destroy the nodes
            for node in nodes.iter(){
                tasks.push(destroy(&node));
            }
            join_all(tasks).await;
        }
    }
    //If the user changes his mind, we simply put a message to tell him not to worry.
    else {
        info!("\nAborting.")
    }
}
