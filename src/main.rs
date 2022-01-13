#[macro_use]
//We import the different modules that contain the subcommands
mod save;
mod deploy;
mod list;
mod destroy;
mod utils;
mod args;
extern crate dotenv;
use std::path::Path;


use crate::args::{EntryArgs, Action};
use clap::{Parser};


fn main() {
    //Loading the configuration file.
    //Keep in mind that these variables can be overwritten as they are environment variables.
    let p = Path::new("/etc/r2dock/r2dock.conf");
    match dotenv::from_path(p){
        Ok(_) => (),
        Err(e) => panic!("Couldn't access config file at {0}, caused error : {1}", p.display(), e)
    }

    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let args = EntryArgs::parse();  

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match &args.action {
        Action::Deploy { image, options, nodes, bootstrap, command } => deploy::entry(image, options, nodes, bootstrap, command),
        Action::Destroy { yes, nodes } => destroy::entry(yes, nodes),
        Action::List { details } => list::entry(details),
        Action::Save { name, node } => save::entry(name, node)
    }
}