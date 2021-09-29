use std::process::Command;
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

extern crate dotenv;
use dotenv_codegen::dotenv;

///This function stops and removes the container currently running on a node.
pub fn destroy(args: Option<&clap::ArgMatches>, node : &str){
    //We deal with the "yes" flag, which can be triggered with -y or --yes (cf args.yaml)
    //If the user hasn't put the flag, we ask him if he really wants to delete the containers
    let mut choice = String::new();
    if !args.unwrap().is_present("yes"){
        print!("Are you sure you want to destroy the containers ? [y/N] ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice).expect("Problem when reading the line.");
    } else {
        //If he put the "yes" flag we just change the choice string without asking.
        choice.push_str("y")
    }
    //If the user is okay with it, we proceed with the deletion
    if choice.trim() == "y" {
        // Connect to the remote SSH server
        let tcp = TcpStream::connect(format!("{}:22",node)).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password("user", "").unwrap();
        let mut channel = sess.channel_session().unwrap();

        //Here, we assume the container name is just "container"
        //We stop it and use docker prune to delete all non-running containers
        match channel.exec("docker stop container && docker container prune -f"){
            Ok(_) => println!("Container destroyed."),
            Err(_) => println!("Error during container destruction.")
        }

        //We display the result in the terminal
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        
        //We also display stderr just in case
        channel.stderr().read_to_string(&mut s).unwrap();
        println!("{}", s);
         
        //We then close the SSH session.
        match channel.wait_close(){
            Ok(_) => (),
            Err(_) => println!("Problem during closure of the SSH connection !")
        }
    } 
    //If the user changes his mind, we simply put a message to tell him not to worry.
    else {
        println!("\nAborting.")
    }
}


pub fn destroy_entry(args: Option<&clap::ArgMatches>){

    let nodes_arg : String = args.unwrap().values_of("nodes").unwrap().collect();
    //let mut nodes = nodes.split_whitespace().map(String::from).collect();

    Command::new("sh")
    .arg("-c")
    .arg("nodes")
    .arg(nodes_arg)
    .output()
    .expect("failed to the nodes command. Are you on a machine with rhubarbe installed ?");
    
    let nodes : Vec<&str> = dotenv!("NODES").split(" ").collect();
    for node in nodes {
   	    println!("{}", node);
	    destroy(args, node);
    }
}
