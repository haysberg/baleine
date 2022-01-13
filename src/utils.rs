use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

/**
 * Alows us to run a command on a specified host.
 * Please note that it doesn't use the SSH2 crate, but instead
 * the included ssh binary on the master machine.
 *
 * The output is printed in real time and is piped to the current terminal stdout.
 */
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

/**
 * This function deploys the latest r2dock image available.
 *
 * By doing this we can be sure that the server receiving the container
 * is configured correctly.
 */
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

/**
 * This function runs the rhubarbe-wait command.
 * This is important because if we don't do this, we send SSH
 * commands to a machine that is not ready yet.
 *
 * So if we don't, the program fails and crashes.
 */
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

pub fn env_var(key: &str) -> String {
    match env::var(key) {
        Ok(_) => (),
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    };

    return env::var(key).unwrap();
}

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

pub fn list_of_nodes(nodes: &Option<String>) -> String {
    return match nodes {
        Some(nodes) => {    
            //We run the "rhubarbe nodes" command to get a list of nodes
            //Basically we don't do the automatic parsing here.
            let cmd = Command::new("/usr/local/bin/rhubarbe-nodes")
                .args(nodes.split(" "))
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

pub fn parse_options_cmd(command: &Option<Vec<String>>, options: &Option<Vec<String>>) -> (String, String) {
    return (
        command.as_ref().unwrap_or(vec!["".to_string()].as_ref()).iter().map(|x| format!("{} ", x)).collect(),
        options.as_ref().unwrap_or(vec!["".to_string()].as_ref()).iter().map(|x| format!("{} ", x)).collect()
    );
}

/**
 * This command sanitizes the terminal.
 * We had weird terminal issues when deploying a large number of machines before.
 * Using this function at the end of your code should help.
 */
pub fn stty_sane() {
    Command::new("/usr/bin/stty").arg("sane").output().expect("");

    Command::new("/usr/bin/echo").arg("sane").output().expect("");
}
