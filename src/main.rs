#[macro_use]
//We import the different modules that contain the subcommands
mod save;
mod deploy;
mod list;
mod destroy;
mod utils;

extern crate dotenv;
use crate::utils::list_of_images;
use dotenv::dotenv;

extern crate serde_derive;
extern crate serde_json;

fn main() {
    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = clap::load_yaml!("../args.yaml");
    let matches = clap::App::from_yaml(app_yaml).get_matches();

    list_of_images();

    dotenv().ok();

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => deploy::entry(&matches),
        Some("destroy") => destroy::entry(&matches),
        Some("list") => list::list(matches.subcommand_matches("list")),
        Some("save") => save::entry(matches.subcommand_matches("save")),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),
    }
}