#[macro_use]
//We import the different modules that contain the subcommands
mod save;
mod deploy;
mod list;
mod destroy;
mod utils;
use std::thread;
extern crate dotenv;
use crate::utils::*;
use dotenv::dotenv;

extern crate serde_derive;
extern crate serde_json;

fn main() {
    dotenv().ok();

    // thread::spawn(|| {
    //     loop {
    //         update_nodes_state();
    //     }
    // });

    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = clap::load_yaml!("../args.yaml");
    let matches = clap::App::from_yaml(app_yaml).get_matches();    

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => deploy::entry(&matches),
        Some("destroy") => destroy::entry(&matches),
        Some("list") => list::entry(&matches),
        Some("save") => save::entry(matches.subcommand_matches("save")),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),
    }
}