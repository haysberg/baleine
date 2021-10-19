use std::process::Command;
use std::io::prelude::*;
use crate::utils::ssh_command;

///This function stops and removes the container currently running on a node.
pub fn destroy(node : &str){
    match ssh_command(node.to_string(), "docker stop container && docker container prune -f".to_string()){
        Ok(_) => (),
        Err(_) => println!("{}", format!("PROBLEM DURING SSH CONNECTION TO NODE {node}", node = node))
    }
}


pub fn entry(args: &clap::ArgMatches){
    //We deal with the "yes" flag, which can be triggered with -y or --yes (cf args.yaml)
    //If the user hasn't put the flag, we ask him if he really wants to delete the containers
    let mut choice = String::new();
    if !args.subcommand_matches("destroy").unwrap().is_present("yes"){
        print!("Are you sure you want to destroy the containers ? [y/N] ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice).expect("Problem when reading the line.");
    } else {
        //If he put the "yes" flag we just change the choice string without asking.
        choice.push_str("y")
    }
    //If the user is okay with it, we proceed with the deletion
    if choice.trim() == "y" {

        let args = args.subcommand_matches("destroy").unwrap();
        
        //Setting up the nodes variable
        let nodes : String = args.values_of("nodes").unwrap().collect();

        let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
        .arg(nodes)
        .output()
        .expect("Problem while running the nodes command");

        let mut nodes = String::from_utf8(cmd.stdout).unwrap();
        nodes.pop();

        match crossbeam::scope(|scope| {
            for node in nodes.split(" ") {
                scope.spawn(move |_| {
                    destroy(&node);
                });
            }
        }) {
            Ok(_) => println!("Destruction complete !"),
            Err(_) => println!("ERROR DURING DEPLOYMENT"),
        };
    }
    //If the user changes his mind, we simply put a message to tell him not to worry.
    else {
        println!("\nAborting.")
    }
}
