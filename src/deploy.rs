use crate::utils::parse_options_cmd;
use crate::utils::ssh_command;
use crate::utils::stty_sane;
use clap;
use crossbeam;

extern crate dotenv;
extern crate json;

/**
 * This function is used to deploy a container on a node
 */
pub fn deploy(args: &clap::ArgMatches, node: &str) {
    let (command, options) = parse_options_cmd(args);

    //We then create the command before sending it to the ssh_command() function
    let cmd = format!("docker run --name container -v /home/container/container_fs:/var --privileged --cap-add=ALL {options} {image} {command} && docker container ls -a",
    options = options,
    image = args.value_of("image").unwrap(),
    command = command);

    //We run the SSH command
    match ssh_command(node.to_string(), cmd) {
        Ok(_) => stty_sane(),
        Err(_) => println!(
            "{}",
            format!(
                "Could not connect using SSH to {node}, is it on ?",
                node = node
            )
        ),
    }
}

/**
 * This function acts as an entry point for the deploy function. It does some parsing
 * And then creates threads to deploy the containers
 */
pub fn entry(args: &clap::ArgMatches) {
    //Parsing of the arguments so that they are in the scope of the function and not in main() anymore
    let args = args.subcommand_matches("deploy").unwrap();
    let nodes = crate::utils::list_of_nodes(&args);

    //We deploy the latest r2dock compatible image if the bootstrap option is used
    if args.is_present("bootstrap") {
        crate::utils::bootstrap("r2dock", &nodes);
        crate::utils::rwait();
    }
    //We deploy the specified image if the --ndz option is used
    else if args.is_present("ndz") {
        crate::utils::bootstrap(args.value_of("ndz").unwrap(), &nodes);
        crate::utils::rwait();
    }

    //We destroy the containers running before on the host
    match crossbeam::scope(|scope| {
        for node in nodes.split(" ") {
            scope.spawn(move |_| {
                crate::destroy::destroy_if_container(&node);
            });
        }
    }) {
        Ok(_) => (),
        Err(_) => panic!("We could not destroy the running containers for an unknown reason."),
    };

    let cmd = format!("docker run --name container -v /home/container/container_fs:/var --privileged --cap-add=ALL {options} {image} {command} && docker container ls -a", options = parse_options_cmd(args).1, image = args.value_of("image").unwrap(), command = parse_options_cmd(args).0);
    println!("Mapping : {}", cmd);

    let mut nodes : Vec<_> = nodes.split(" ").collect();

    /**
     * We deploy the first node before all the others, to ensure that the docker image
     * will be pulled through the proxy for the rest of the nodes
    */
    deploy(args, nodes.swap_remove(0));

    //We then create a thread for each node, running the deploy command through SSH
    match crossbeam::scope(|scope| {
        for node in nodes {
            scope.spawn(move |_| {
                deploy(args, &node);
            });
        }
    }) {
        //We display a message depending of the outcome of the commands
        Ok(_) => println!("Deployment complete !"),
        Err(_) => println!("ERROR DURING DEPLOYMENT"),
    };

    stty_sane();
}
