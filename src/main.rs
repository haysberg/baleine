#[macro_use]
extern crate clap;
use clap::{App};
use std::process::Command;
use std::io::{self, Write};

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
        Some("save") => println!("save"),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),

    }

    fn list (args: Option<&clap::ArgMatches>, config: &config::Config){
        let filter = match config.get::<std::string::String>("repository_url") {
            Ok(value) => format!("\"{}/*\"", value),
            Err(e) => format!("{}", e),
        };
        

        if !args.unwrap().is_present("all"){
            println!("docker images {}", filter);
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
}
