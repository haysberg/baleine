use clap::{Parser, Subcommand, AppSettings, ArgSettings, crate_version};

#[derive(Parser, Debug)]
#[clap(name = "r2dock")]
#[clap(author = "Téo Haÿs <teo.hays@inria.fr>")]
#[clap(version = crate_version!())]
#[clap(about = "Deploys Docker containers using Rhubarbe. \nPlease create issues and read the wiki here : \nhttps://github.com/haysberg/r2dock")]
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
#[clap(setting(AppSettings::AllowHyphenValues))]
pub enum Action {
    #[clap(about = "deploys the selected container on nodes")]
    #[clap(setting(AppSettings::AllowHyphenValues))]
    Deploy {
        #[clap(help = "the image to deploy, <repository>/image:tag format")]
        #[clap(required = true, short, long)]
        image: String,

        #[clap(help = "the options string that you want to send to the container")]
        #[clap(setting(ArgSettings::MultipleValues))]
        #[clap(short, long)]
        options: Option<Vec<String>>,

        #[clap(help = "nodes you want to deploy the container on, using the rhubarbe format")]
        #[clap(setting(ArgSettings::MultipleValues))]
        #[clap(short, long)]
        nodes: Option<Vec<String>>,

        #[clap(help = "allows you to choose what ndz image to install on a node before deploying a container")]
        #[clap(long, value_name = "NDZ_IMAGE", default_missing_value="r2dock")]
        bootstrap: Option<String>,

        #[clap(help = "Use this option to choose what command to pass to the container. ALWAYS USE LAST.")]
        #[clap(setting(ArgSettings::MultipleValues))]
        #[clap(short, long)]
        command: Option<Vec<String>>
    },

    #[clap(about = "destroys containers PERMANENTLY")]
    Destroy {
        #[clap(help = "doesn't ask if you really want to destroy the container")]
        #[clap(short, long)]
        yes: bool,

        #[clap(help = "nodes you want to destroy in the rhubarbe format")]
        #[clap(setting(ArgSettings::MultipleValues))]
        #[clap(short, long)]
        nodes: Option<Vec<String>>,
    },

    #[clap(about = "lists the CUSTOM images to deploy. Images on Dockerhub are NOT listed")]
    List {
        #[clap(help = "name of the image for which you want to display all the different versions available")]
        #[clap(short, long, value_name = "DOCKER_IMAGE")]
        details: Option<String>,
    },

    #[clap(about = "saves the selected container as an image for future use")]
    Save {
        #[clap(help = "name of the image you wish to create")]
        #[clap(required = true, long)]
        name: String,
        
        #[clap(help = "identifier of the node you want to save")]
        #[clap(required = true, short, long)]
        node: String,
    }
}