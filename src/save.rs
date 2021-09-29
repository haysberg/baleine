use std::process::Command;
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

extern crate dotenv;
use dotenv_codegen::dotenv;

///This function takes a container running on a node and saves it to the
///remote registry configured in config.toml
pub fn save (args: Option<&clap::ArgMatches>, node: &str){
    // Connect to the remote SSH server
    let tcp = TcpStream::connect(format!("{}:22",node)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("user", "").unwrap();
    let mut channel = sess.channel_session().unwrap();
    
    //We create the string for the command that we are going to execute remotely.
    //Here, we create a new image from the running container on the node, and push it to the
    //remote registry.
    let cmd = format!("docker commit container {repository}/{image_name} && docker push {repository}/{image_name}",
    repository = dotenv!("REGISTRY_URL"),
    image_name = args.unwrap().value_of("name").unwrap());
      
    //We execute the command. Only one command can run in this SSH session.
    //That's why the command is composed of two subcommands linked with a &&.
    channel.exec(&cmd).unwrap();

    //We read the response from the session then print it in the terminal.
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    //We also display stderr just in case
    channel.stderr().read_to_string(&mut s).unwrap();
    println!("{}", s);
       
    //We then close the SSH channel and handle if there is an issue at some point.
    match channel.wait_close(){
        Ok(_) => println!("Container saved and uploaded to the repository !"),
        Err(_) => println!("Problem during closure of the SSH connection !")
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
