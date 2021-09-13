#[macro_use]
extern crate clap;
use clap::{App};
use std::process::Command;
use std::io::{self, Write};
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

fn main() {
    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = load_yaml!("../args.yaml");
    let matches = App::from_yaml(app_yaml).get_matches();

    //Loading the settings from config.toml
    let mut config = config::Config::default();
    config.merge(config::File::with_name("config")).unwrap();

    match matches.subcommand_name() {
        Some("deploy") => println!("deploy"),
        Some("destroy") => println!("destroy"),
        Some("list") => list(matches.subcommand_matches("list"), &config),
        Some("save") => save(),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),

    }

    fn list (args: Option<&clap::ArgMatches>, config: &config::Config){
        let filter = match config.get::<std::string::String>("repository_url") {
            Ok(value) => format!("\"{}/*\"", value),
            Err(e) => format!("{}", e),
        };
        

        if !args.unwrap().is_present("all"){
            let result = Command::new("sh")
            .arg("-c")
            .arg(format!("docker images {}", filter))
            .output()
            .expect("Error during command execution");
            io::stdout().write_all(&result.stdout).unwrap();
        }
        else{
            let result = Command::new("sh")
            .arg("-c")
            .arg("docker images")
            .output()
            .expect("Error during command execution");
            io::stdout().write_all(&result.stdout).unwrap();
        }
    }

    fn save (/*args: Option<&clap::ArgMatches>, config: &config::Config */){
        // Connect to the remote SSH server
        let tcp = TcpStream::connect("172.16.194.128:22").unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_agent("user").unwrap();

        let mut channel = sess.channel_session().unwrap();
        channel.exec("docker commit test 192.168.228.1:5000/image1").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        channel.wait_close();
        println!("{}", channel.exit_status().unwrap());
    }

    fn deploy(){

    }

    fn destroy(){

    }
}
