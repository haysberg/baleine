use std::env::{self, VarError};
use std::io::{Error};
use std::process::{Command};
use tracing::{info, debug};

/// Runs a command on a specified host.
/// Please note that it uses the bash binary on the machine
/// 
/// The output is printed in real time and is piped to the current terminal stdout.
pub async fn local_command(command: String) -> Result<(), Error> {
    info!("command : {:?}", command);

    match Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
}

/// This function deploys the given disk image (.ndz) on the slave node.
/// Use it when bootstraping a new disk image on a server.
/// 
/// By doing this we can be sure that the server receiving the container is configured correctly.
///
/// # Arguments
///
/// * `image` - the .ndz image to deploy
/// * `nodes` - list of slave nodes affected
pub async fn bootstrap(image: &String, nodes: &Vec<String>) -> Result<(), Error> {
    let tmp_nodes : String = nodes.iter().map(|x| format!("{} ", x)).collect();
    //Run the imaging through rhubarbe
    match Command::new("/usr/local/bin/rhubarbe-load")
        .arg("-i")
        .arg(image)
        .arg(tmp_nodes)
        .spawn(){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
}

/// This function runs the rhubarbe-wait command.
/// This is important because if we don't do this, we send SSH commands to a machine that is not ready yet.
/// 
/// So if we don't, the program fails and crashes.
pub async fn rwait() -> Result<(), Error> {
    //rwait
    match Command::new("/usr/local/bin/rhubarbe-wait").spawn(){
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/// This function returns the value of a provided environment variable
///
/// # Arguments
///
/// * `key` - The environment variable we are looking for
pub fn env_var(key: &str) -> Result<String, VarError> {
    match env::var(key) {
        Ok(_) => Ok(env::var(key).unwrap()),
        Err(e) => Err(e)
    }
}

/// Takes in a list of strings sent from the CLI
/// 
/// Parses it and sends it to rhubarbe-nodes to get a String in the form of :
/// "fit01 fit02 fit03"
/// 
/// Please note that if no nodes is provided, the function will check if $NODES is set and
/// will stop the program if no value was there as well.
///
/// # Arguments
///
/// * `nodes` - the list of nodes we are sending
pub fn list_of_nodes(nodes: &Option<Vec<String>>) -> Vec<String> {
    return match nodes {
        Some(nodes) => {
            let nodes_arg : Vec<_> = nodes.clone().iter().map(|r| format!("{} ", r)).collect();
            //We run the "rhubarbe nodes" command to get a list of nodes
            //Basically we don't do the automatic parsing here.
            let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
                .args(nodes_arg)
                .output()
                .expect("Problem while running the nodes command");
    
            //We then take the list of nodes provided by rhubarbe, and trim the \n at the end
            let mut nodes = String::from_utf8(cmd.stdout).unwrap();
            info!("List of nodes : {}", nodes);
            nodes.pop();

            nodes.split(" ").map(|x| x.to_string()).collect()
        }
        None => {
            match env::var("NODES") {
                Ok(value) => {
                    if value != "" { vec!(value.split(" ").map(|x| x.to_string()).collect()) }
                    else { panic!("$NODES is not set, and you didn't provide a list of nodes. Please use the -n option.") }
                }
                Err(_) => panic!("$NODES is not set, and you didn't provide a list of nodes. Please use the -n option.")
            } 
        }
    }
}

/// This command parses the command argument and the options argument.
/// When the CLI loads, if both are used at the same time,
/// the first one takes precedence and gets all the arguments.
/// We have to split them up using this function.
pub fn parse_cmd_opt(command: &Option<Vec<String>>, options: &Option<Vec<String>>) -> (Option<String>, Option<String>) {
    let mut parsed_options : Option<String> = None;
    let mut parsed = false;
    
    let mut parsed_command = match command {
        //In case --command is used BEFORE --options
        Some(vector) => {
            parsed = true;
            if vector.contains(&"--options".to_string()){
                let index = vector.iter().position(|x| x == &"--options".to_string()).unwrap();
                parsed_options = Some(vector[(index+1)..].iter().map(|x| format!("{} ", x)).collect());
                Some(vector[..(index)].iter().map(|x| format!("{} ", x)).collect())
            }else{
                Some(vector.iter().map(|x| format!("{} ", x)).collect())
            }
        },
        None => None
    };

    if !parsed {
        parsed_options  = match options {
            Some(vector) => {
                if vector.contains(&"--command".to_string()){
                    let index = vector.iter().position(|x| x == &"--command".to_string()).unwrap();
                    parsed_command = Some(vector[(index+1)..].iter().map(|x| format!("{} ", x)).collect());
                    Some(vector[..(index)].iter().map(|x| format!("{} ", x)).collect())
                }else{
                    Some(vector.iter().map(|x| format!("{} ", x)).collect())
                }
            },
            None => None
        };
    }

    debug!("cmd : {:?}, opt : {:?}", parsed_command, parsed_options);
    (parsed_command, parsed_options)
}