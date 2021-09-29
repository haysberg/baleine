use std::process::Command;
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

extern crate dotenv;
use dotenv_codegen::dotenv;

///This function is used to deploy a container on a node
pub fn deploy(args: Option<&clap::ArgMatches>, node : &str){
    // Connect to the remote SSH server
    let tcp = TcpStream::connect(format!("{}:22",node)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("user", "password").unwrap();
    let mut channel = sess.channel_session().unwrap();

    //We parse the Docker options that the user might have supplied
    let mut options : String = match args.unwrap().value_of("options"){
            Some(_) => args.unwrap().values_of("options").unwrap().collect(),
            //If there is no options provided we just return an empty string
            None => ("").to_string(),
    };

    //We add a space before each options passed on to Docker.
    //Without doing this they are glued to each other, causing the deployment to fail.
    options = str::replace(&options, "-", " -");

    //We do exactly the same for the command 
    let command = match args.unwrap().value_of("command"){
        Some(command) => command.to_string(),
        None => ("").to_string(),
    };

    //We assemble all the arguments to create the command that will be run through SSH on the node
    let cmd = format!("docker run --privileged --cap-add=ALL --name container {options} {image} {command} && docker container ls -a",
        options = options,
        image = args.unwrap().value_of("image").unwrap(),
        command = command);

    //We execute the command. Only one command can run in this SSH session.
    println!("{}", &cmd);
    channel.exec(&cmd).unwrap();

    //We read the response from the session then print it in the terminal.
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

pub fn deploy_entry(args: Option<&clap::ArgMatches>){

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
	deploy(args, node);
    }
}
