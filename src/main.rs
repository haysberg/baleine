#[macro_use]
//We import the different modules that contain the subcommands
mod save;
mod deploy;
mod list;
mod destroy;
mod utils;
mod args;
mod build;
extern crate dotenv;

use std::path::Path;
use crate::args::{EntryArgs, Action};
use clap::{Parser};
use tracing::{warn, instrument};
use tracing_subscriber;

#[instrument]
fn main() {
    //Initializing the log display
    tracing_subscriber::fmt::init();

    //Loading the configuration file.
    //Keep in mind that these variables can be overwritten as they are environment variables.
    let p = Path::new("/etc/baleine/baleine.conf");
    let relative = Path::new("./baleine.example.conf");
    match dotenv::from_path(p){
        Ok(_) => (),
        Err(_) => match dotenv::from_path(relative){
            Ok(_) => (),
            Err(_) => warn!("Couldn't access config file at {0} numerous errors could happen !", p.display())
        }
    }

    //We get the arguments provided by the user, and match them with the ones listed in args.rs
    let args = EntryArgs::parse(); 

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match &args.action {
        Action::Deploy { image, options, nodes, bootstrap, command } => deploy::entry(image, options, nodes, bootstrap, command),
        Action::Destroy { yes, nodes } => destroy::entry(yes, nodes),
        Action::List { details } => list::entry(details),
        Action::Save { name, node } => save::entry(name, node),
        Action::Build { file, tags, url } => build::entry(file, url, tags)
    }
}