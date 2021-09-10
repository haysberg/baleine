#[macro_use]
extern crate clap;
use clap::{App};
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("deploy") => println!("deploy"),
        Some("destroy") => println!("destroy"),
        Some("list") => list(),
        Some("save") => println!("save"),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),

    }

    fn list (){
        println!("Docker images stored on this machine :");
        let result = Command::new("sh")
        .arg("-c")
        .arg("docker images")
        .output()
        .expect("Erreur lors de l'exécution de la commande");
        io::stdout().write_all(&result.stdout).unwrap();
    }
}
