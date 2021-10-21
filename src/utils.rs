use openssl::ssl::{SslConnector, SslMethod};
use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use tungstenite::Message;

static mut COUNT: u8 = 0;

/**
 * Unused command.
 * Calls the websocket server provided, to get the state of the nodes
 */
pub fn update_nodes_state() {
    let mut ctx = SslConnector::builder(SslMethod::tls_client()).unwrap();
    println!("A");
    ctx.set_verify(openssl::ssl::SslVerifyMode::NONE); // <- verification disabled
    println!("B");
    let connector = ctx.build();
    println!("C");
    let stream = TcpStream::connect(("r2lab.inria.fr", 999)).unwrap();
    println!("D");
    let stream = connector.connect("r2lab.inria.fr", stream).unwrap();

    let mut client = tungstenite::client_tls("ws://r2lab.inria.fr:999", stream).unwrap();

    println!("Connected to the R2Lab monitoring server. Waiting for a status message...");

    loop {
        println!("Loop");
        match client.0.write_message(Message::text(
            "{category: \"nodes\", action: \"request\", message: \"please\"}",
        )) {
            Ok(_) => match client.0.read_message().unwrap().to_text() {
                Ok(res) => set_env_json(res.to_string()),
                Err(_) => (println!("Fail")),
            },
            Err(_) => panic!("Problem while getting the list of nodes !"),
        };
    }
}

/**
 * Set the JSON response provided by the update_nodes_state function
 * as an environment variable to use later.
 * This allows us to not call it every time.
 */
pub fn set_env_json(json_str: String) {
    unsafe {
        COUNT = COUNT + 1;
        println!("{}", COUNT);
    }
    env::set_var("MONITOR_NODES", json_str);
    match env::var("MONITOR_NODES") {
        Ok(val) => println!("{}: {:?}", "MONITOR_NODES", val),
        Err(e) => println!("couldn't interpret {}: {}", "MONITOR_NODES", e),
    }
}

/**
 * Blocks the execution of code until we have infos on
 * the current nodes.
 */
pub fn lock() {
    loop {
        match env::var("MONITOR_NODES") {
            Ok(val) => {
                if val != "" {
                    break;
                }
            }
            Err(_) => (),
        }
    }
}

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
pub fn bootstrap(image: &str, nodes: &String) {
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
