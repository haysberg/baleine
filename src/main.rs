#[macro_use]
extern crate clap;

use clap::{App};
mod save;
mod deploy;
mod list;
mod destroy;

fn main() {
    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = load_yaml!("../args.yaml");
    let matches = App::from_yaml(app_yaml).get_matches();

    //Loading the settings from config.toml
    let mut config = config::Config::default();
    match config.merge(config::File::with_name("../config.toml")){
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => deploy::deploy(matches.subcommand_matches("deploy")),
        Some("destroy") => destroy::destroy(matches.subcommand_matches("destroy")),
        Some("list") => list::list(&config),
        Some("save") => save::save(matches.subcommand_matches("save"), &config),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),
    }
}