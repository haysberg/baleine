use clap::{Parser, Subcommand, crate_version};

#[derive(Parser, Debug)]
#[clap(name = "baleine")]
#[clap(author = "Téo Haÿs <teo.hays@inria.fr>")]
#[clap(version = crate_version!())]
#[clap(about = "Deploys Docker containers using Rhubarbe.
Report a bug : https://github.com/haysberg/baleine/issues
Wiki : https://github.com/haysberg/baleine/wiki")]
#[clap(subcommand_required = true)]
#[clap(dont_collapse_args_in_usage = true)]
#[clap(propagate_version = true)]
pub struct EntryArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
#[clap(trailing_var_arg = true)]
#[clap(dont_delimit_trailing_values = true)]
#[clap(allow_hyphen_values = true)]
pub enum Action {
    #[clap(about = "deploys the selected container on nodes")]
    #[clap(allow_hyphen_values = true)]
    Deploy {
        #[clap(help = "the image to deploy, <repository>/image:tag format")]
        #[clap(required = true, short, long)]
        image: String,

        #[clap(help = "the options string that you want to send to the container")]
        #[clap(allow_hyphen_values = true)]
        #[clap(multiple_values = true)]
        #[clap(long)]
        options: Option<Vec<String>>,

        #[clap(help = "nodes you want to deploy the container on, using the rhubarbe format")]
        #[clap(long)]
        #[clap(multiple_values = true)]
        nodes: Option<Vec<String>>,

        #[clap(help = "allows you to choose what ndz image to install on a node before deploying a container")]
        #[clap(long, value_name = "NDZ_IMAGE", default_missing_value="baleine")]
        bootstrap: Option<String>,

        #[clap(help = "Use this option to choose what command to pass to the container. ALWAYS USE LAST.")]
        #[clap(allow_hyphen_values = true)]
        #[clap(multiple_values = true)]
        #[clap(long)]
        command: Option<Vec<String>>
    },

    #[clap(about = "destroys containers PERMANENTLY")]
    Destroy {
        #[clap(help = "doesn't ask if you really want to destroy the container")]
        #[clap(short, long)]
        #[clap(short_alias = 'f')]
        #[clap(alias = "force")]
        yes: bool,

        #[clap(help = "nodes you want to destroy in the rhubarbe format")]
        #[clap(long)]
        nodes: Option<Vec<String>>,
    },

    #[clap(about = "builds and uploads a Dockerfile to the repo")]
    Build {
        #[clap(help = "path to the Dockerfile")]
        #[clap(required_unless_present("url"), short, long)]
        file: Option<String>,

        #[clap(help = "path to the Dockerfile")]
        #[clap(required_unless_present("file"), short, long)]
        url: Option<String>,

        #[clap(help = "tags to give to the image created")]
        #[clap(required = true, long)]
        #[clap(multiple_values = true)]
        tags: Vec<String>,
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
        #[clap(required = true, long)]
        node: String,
    }
}