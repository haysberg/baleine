#[macro_use]
extern crate clap;
use clap::{App};
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
extern crate json;

fn main() {
    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = load_yaml!("../args.yaml");
    let matches = App::from_yaml(app_yaml).get_matches();

    //Loading the settings from config.toml
    let mut config = config::Config::default();
    config.merge(config::File::with_name("config")).unwrap();

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => println!("deploy"),
        Some("destroy") => destroy(matches.subcommand_matches("destroy")),
        Some("list") => list(&config),
        Some("save") => save(matches.subcommand_matches("save"), &config),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),

    }

    //This function allows us to list the images available on the registry configured in config.toml
    //We call the Docker API available on the registry image then format it to make it readable for the user.
    fn list (config: &config::Config){
        //We generate the URL used to call the API
        let url = match config.get::<std::string::String>("registry_protocol") {
            Ok(protocol) => format!("{protocol}{address}/v2/_catalog",
            protocol = protocol,
            address = config.get::<std::string::String>("repository_url").unwrap()),
            Err(e) => format!("{}",e),
        };
        
        //We call the API in question...
        let result = reqwest::blocking::get(url).unwrap().text().unwrap();

        //Then we parse the JSON result.
        let parsed = json::parse(&result);

        //Nice message before the list of images, which allows the user to see the address
        //of the repo. Just in case.
        println!("List of Images on {protocol}{address}", 
            protocol = config.get::<std::string::String>("registry_protocol").unwrap(),
            address = config.get::<std::string::String>("repository_url").unwrap()); 

        //We print the list of images before exiting the function.
        for repo in parsed.unwrap()["repositories"].members() {
            println!("{}", repo)
        }
    }

    //This function takes a container running on a node and saves it to the
    //remote registry configured in config.toml
    fn save (args: Option<&clap::ArgMatches>, config: &config::Config){
        // Connect to the remote SSH server
        let tcp = TcpStream::connect("172.16.194.128:22").unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_agent("user").unwrap();
        let mut channel = sess.channel_session().unwrap();

        //We create the string for the command that we are going to execute remotely.
        //Here, we create a new image from the running container on the node, and push it to the
        //remote registry.
        let cmd = format!("docker commit container {repository}/{image_name} && docker push {repository}/{image_name}",
        repository = config.get::<std::string::String>("repository_url").unwrap(),
        image_name = args.unwrap().value_of("name").unwrap());
        
        //We execute the command. Only one command can run in this SSH session.
        //That's why the command is composed of two subcommands linked with a &&.
        channel.exec(&cmd).unwrap();

        //We read the response from the session then print it in the terminal.
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        
        //We then close the SSH channel and handle if there is an issue at some point.
        match channel.wait_close(){
            Ok(_) => println!("Container saved and uploaded to the repository !"),
            Err(_) => println!("Problem during closure of the SSH connection !")
        }
    }

    // fn deploy(args: Option<&clap::ArgMatches>, config: &config::Config){

    // }

    //This function stops and removes the container currently running on a node.
    fn destroy(args: Option<&clap::ArgMatches>){
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
            let tcp = TcpStream::connect("172.16.194.128:22").unwrap();
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();
            sess.userauth_agent("user").unwrap();
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
}
