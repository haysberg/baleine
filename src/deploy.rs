use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use clap;
use crossbeam;
use std::process::{Command};

///This function is used to deploy a container on a node
pub fn deploy(args: &clap::ArgMatches, node : &str){
    // Connect to the remote SSH server;
    let tcp = TcpStream::connect(format!("{}:22",node)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("user", "").unwrap();
    let mut channel = sess.channel_session().unwrap();

    //We parse the Docker options that the user might have supplied
    let mut options : String = match args.value_of("options"){
            Some(_) => args.values_of("options").unwrap().collect(),
            //If there is no options provided we just return an empty string
            None => ("").to_string(),
    };

    //We add a space before each options passed on to Docker.
    //Without doing this they are glued to each other, causing the deployment to fail.
    options = str::replace(&options, "-", " -");

    //We do exactly the same for the command 
    let command = match args.value_of("command"){
        Some(command) => command.to_string(),
        None => ("").to_string(),
    };

    //We assemble all the arguments to create the command that will be run through SSH on the node
    let cmd = format!("docker run --privileged --cap-add=ALL --name container {options} {image} {command} && docker container ls -a",
        options = options,
        image = args.value_of("image").unwrap(),
        command = command);

    //We execute the command. Only one command can run in this SSH session.
    channel.exec(&cmd).unwrap();

    //We read the response from the session then print it in the terminal.
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    for line in s.split("\n") {
        println!("{}:  {}", node, line);
    }
         
    //We then close the SSH session.
    match channel.wait_close(){
        Ok(_) => (),
        Err(_) => println!("Problem during closure of the SSH connection !")
    }        
}

pub fn entry(args: &clap::ArgMatches){

    //Parsing of the arguments so that they are in the scope of the function and not in main() anymore
    let args = args.subcommand_matches("deploy").unwrap();
    
    //Setting up the nodes variable provided by the user
    let nodes : String = args.values_of("nodes").unwrap().collect();

    //We run the "rhubarbe nodes" command to get a list of nodes
    //Basically we don't do the automatic parsing here.
    let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
    .arg(nodes)
    .output()
    .expect("Problem while running the nodes command");

    //We then take the list of nodes provided by rhubarbe, and trim the little \n at the end
    let mut nodes = String::from_utf8(cmd.stdout).unwrap();
    nodes.pop();

    //We then create a thread for each node, running the deploy command through SSH
    match crossbeam::scope(|scope| {
        for node in nodes.split(" ") {
            scope.spawn(move |_| {
                deploy(args, &node);
            });
        }
    }) { //We display a message depending of the outcome of the commands
        Ok(_) => println!("Deployment complete !"),
        Err(_) => println!("ERROR DURING DEPLOYMENT"),
    };
}
