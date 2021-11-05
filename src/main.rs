#[macro_use]
//We import the different modules that contain the subcommands
mod save;
mod deploy;
mod list;
mod destroy;
mod utils;
extern crate dotenv;
use std::path::Path;

fn main() {
    //Loading the configuration file.
    //Keep in mind that these variables can be overwritten as they are environment variables.
    let p = Path::new("/etc/r2dock/r2dock.conf");
    match dotenv::from_path(p){
        Ok(_) => (),
        Err(e) => panic!("Couldn't access config file at {0}, caused error : {1}", p.display(), e)
    }

    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = clap::load_yaml!("../args.yaml");
    let matches = clap::App::from_yaml(app_yaml).get_matches();    

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => deploy::entry(&matches),
        Some("destroy") => destroy::entry(&matches),
        Some("list") => list::entry(&matches),
        Some("save") => save::entry(&matches),
        None => println!("No subcommand provided. Use --help for more info"),
        _ => unreachable!(),
    }
}