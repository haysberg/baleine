#[macro_use]
mod save;
mod deploy;
mod list;
mod destroy;

extern crate dotenv;
use dotenv::dotenv;

fn main() {
    //We get the arguments provided by the user, and match them with the ones listed in args.yaml
    let app_yaml = clap::load_yaml!("../args.yaml");
    let matches = clap::App::from_yaml(app_yaml).get_matches();

    dotenv().ok();

    //Depending on what subcommand the user has put in the CLI, we call the related function.
    match matches.subcommand_name() {
        Some("deploy") => deploy::deploy(matches.subcommand_matches("deploy")),
        Some("destroy") => destroy::destroy(matches.subcommand_matches("destroy")),
        Some("list") => list::list(),
        Some("save") => save::save(matches.subcommand_matches("save")),
        None => println!("You need to put a subcommand for r2dock to work"),
        _ => unreachable!(),
    }
}