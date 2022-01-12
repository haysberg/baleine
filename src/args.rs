use clap::{Parser, Subcommand, AppSettings};

#[derive(Parser, Debug)]
#[clap(name = "r2dock")]
#[clap(author = "Téo Haÿs <teo.hays@inria.fr>")]
#[clap(version = "1.0.1")]
#[clap(about = "Deploys Docker containers using Rhubarbe. \nPlease create issues and read the wiki here : \nhttps://github.com/haysberg/r2dock")]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
#[clap(setting(AppSettings::InferSubcommands))]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
#[clap(setting(AppSettings::DontCollapseArgsInUsage))]
pub struct EntryArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
#[clap(setting(AppSettings::AllowHyphenValues))]
#[clap(setting(AppSettings::TrailingVarArg))]
pub enum Action {
    Deploy {
        #[clap(short, long)]
        image: String,
        #[clap(short, long)]
        options: Option<String>,
        #[clap(short, long)]
        nodes: Option<String>,
        #[clap(long)]
        bootstrap: bool,
        #[clap(long)]
        ndz: Option<String>,
        #[clap(short, long)]
        command: Option<String>
    },

    Destroy {
        #[clap(short, long)]
        yes: bool,
        #[clap(short, long)]
        nodes: Option<String>,
    },

    List {
        #[clap(short, long)]
        details: Option<String>,
    },

    Save {
        #[clap(long)]
        name: String,
        #[clap(short, long)]
        node: String,
    }
}