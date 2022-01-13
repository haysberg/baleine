use clap::{Parser, Subcommand, AppSettings};

#[derive(Parser, Debug)]
#[clap(name = "r2dock")]
#[clap(author = "Téo Haÿs <teo.hays@inria.fr>")]
#[clap(version = "0.3")]
#[clap(about = "Deploys Docker containers using Rhubarbe. \nPlease create issues and read the wiki here : \nhttps://github.com/haysberg/r2dock")]
//#[clap(usage = "r2dock deploy [FLAGS] [OPTIONS] [--nodes \"<nodes>\"] --image <image> [--command <command>]")]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
#[clap(setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::PropagateVersion))]
pub struct EntryArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
#[clap(setting(AppSettings::TrailingVarArg))]
#[clap(setting(AppSettings::DontDelimitTrailingValues))]
pub enum Action {
    #[clap(about = "deploys the selected container on nodes")]
    #[clap(setting(AppSettings::AllowHyphenValues))]
    Deploy {
        #[clap(help = "the image to deploy, <repository>/image:tag format")]
        #[clap(short, long)]
        image: String,

        #[clap(help = "the options string that you want to send to the container")]
        #[clap(short, long)]
        options: Option<String>,

        #[clap(help = "nodes you want to deploy the container on, using the rhubarbe format")]
        #[clap(short, long)]
        nodes: Option<String>,

        #[clap(help = "deploys the latest r2dock compatible image on the machine before deploying the container")]
        #[clap(long)]
        bootstrap: bool,

        #[clap(help = "choose a .ndz image to be deployed before the container")]
        #[clap(long)]
        ndz: Option<String>,

        #[clap(help = "Use this option to choose what command to pass to the container. ALWAYS USE LAST.")]
        #[clap(short, long)]
        command: Option<String>
    },

    #[clap(about = "destroys containers PERMANENTLY")]
    Destroy {
        #[clap(help = "doesn't ask if you really want to destroy the container")]
        #[clap(short, long)]
        yes: bool,

        #[clap(help = "nodes you want to destroy in the rhubarbe format")]
        #[clap(short, long)]
        nodes: Option<String>,
    },

    #[clap(about = "lists the CUSTOM images to deploy. Images on Dockerhub are NOT listed")]
    List {
        #[clap(help = "name of the image for which you want to display all the different versions available")]
        #[clap(short, long)]
        details: Option<String>,
    },

    #[clap(about = "saves the selected container as an image for future use")]
    Save {
        #[clap(help = "name of the image you wish to create")]
        #[clap(long)]
        name: String,
        
        #[clap(help = "identifier of the node you want to save")]
        #[clap(short, long)]
        node: String,
    }
}