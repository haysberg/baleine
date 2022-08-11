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
use tracing::{warn};
use tracing_subscriber;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    // We initialize the logs formatting
    let format = fmt::format()
    .with_level(true)
    .with_timer(())
    .with_thread_ids(false)
    .with_source_location(false)
    .with_target(false)
    .with_line_number(false)
    .compact();

    //Initializing the log display
    tracing_subscriber::fmt()
    .event_format(format)
    .init();

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
        Action::Deploy { image, options, nodes, bootstrap, command } => deploy::entry(image, options, nodes, bootstrap, command).await,
        Action::Destroy { yes, nodes } => destroy::entry(yes, nodes).await,
        Action::List { details } => list::entry(details).await,
        Action::Save { name, node } => save::entry(name, node).await,
        Action::Build { file, tags, url } => build::entry(file, url, tags).await
    }
}