use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

//This function is used to deploy a container on a node
pub fn deploy(args: Option<&clap::ArgMatches>){
    // Connect to the remote SSH server
    let tcp = TcpStream::connect("172.16.194.128:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent("user").unwrap();
    let mut channel = sess.channel_session().unwrap();

    let mut options = String::new();
    if args.unwrap().is_present("options"){
        options = match args.unwrap().value_of("options"){
            Some(options) => options.to_string(),
            None => ("").to_string(),
        };
    }

    let command = match args.unwrap().value_of("command"){
        Some(command) => command.to_string(),
        None => ("").to_string(),
    };

    let cmd = format!("docker run --name container {options} {image} {command} && docker container ls -a",
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