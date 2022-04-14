use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

/// Runs a command on a specified host.
/// Please note that it doesn't use the SSH2 crate, but instead the included ssh binary on the master machine.
/// 
/// The output is printed in real time and is piped to the current terminal stdout.
///
/// # Arguments
///
/// * `host` - name of the SSH host the command will be executed on
/// * `command` - command to be executed on the remote host
pub fn ssh_command(host: String, command: String) -> Result<(), Error> {
    let stdout = Command::new("ssh")
        .arg(format!("root@{host}", host = host))
        .arg("-t")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("\r{host} : {line}", host = host, line = line));

    println!("\r");
    Ok(())
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
pub fn bootstrap(image: &String, nodes: &String) {
    //Run the imaging through rhubarbe
    let stdout = Command::new("/usr/local/bin/rhubarbe-load")
        .arg("-i")
        .arg(image)
        .arg(nodes)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout;

    let reader = BufReader::new(stdout.unwrap());

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{line}", line = line));

    println!("\r");
}

/// This function runs the rhubarbe-wait command.
/// This is important because if we don't do this, we send SSH commands to a machine that is not ready yet.
/// 
/// So if we don't, the program fails and crashes.
pub fn rwait() {
    //rwait
    let stdout = Command::new("/usr/local/bin/rhubarbe-wait")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout;

    let reader = BufReader::new(stdout.unwrap());

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{line}", line = line));

    println!("\r");
}

/// This function returns the value of a provided environment variable
///
/// # Arguments
///
/// * `key` - The environment variable we are looking for
pub fn env_var(key: &str) -> String {
    match env::var(key) {
        Ok(_) => (),
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    };

    return env::var(key).unwrap();
}

/// Checks if a container is currently deployed on a host
///
/// # Arguments
///
/// * `host` - the slave node we want to check
pub fn container_deployed(host: &str) -> bool {
    let output = Command::new("ssh")
        .arg(format!("root@{host}", host = host))
        .arg("-t")
        .arg("docker container ls -a | wc -l")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();
    return !stdout.contains("1\n");
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
pub fn list_of_nodes(nodes: &Option<Vec<String>>) -> String {
    return match nodes {
        Some(nodes) => {
            let nodes_arg : Vec<_> = nodes.iter().map(|r| format!("{} ", r)).collect();
            //We run the "rhubarbe nodes" command to get a list of nodes
            //Basically we don't do the automatic parsing here.
            let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
                .args(nodes_arg)
                .output()
                .expect("Problem while running the nodes command");
    
            //We then take the list of nodes provided by rhubarbe, and trim the \n at the end
            let mut nodes = String::from_utf8(cmd.stdout).unwrap();
            println!("{}", nodes);
            nodes.pop();
    
            nodes
        }
        None => {
            match env::var("NODES") {
                Ok(value) => {
                    if value != "" { value }
                    else { panic!("$NODES is not set, and you didn't provide a list of nodes. Please use the -n option.") }
                }
                Err(_) => panic!("$NODES is not set, and you didn't provide a list of nodes. Please use the -n option.")
            } 
        }
    }
}

/// This command sanitizes the terminal.
/// We had weird terminal issues when deploying a large number of machines before.
/// Using this function at the end of your code should help.
pub fn stty_sane() {
    Command::new("/usr/bin/stty").arg("sane").output().expect("");
    Command::new("/usr/bin/echo").arg("").output().expect("");
}


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

    println!("cmd : {:?}, opt : {:?}", parsed_command, parsed_options);
    (parsed_command, parsed_options)
}