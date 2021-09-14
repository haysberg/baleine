// use std::io::prelude::*;
// use std::net::{TcpStream};
// use ssh2::Session;
extern crate json;

//This function allows us to list the images available on the registry configured in config.toml
//We call the Docker API available on the registry image then format it to make it readable for the user.
pub fn list (config: &config::Config){
    //We generate the URL used to call the API
    let url = match config.get::<std::string::String>("registry_protocol") {
        Ok(protocol) => format!("{protocol}{address}/v2/_catalog",
        protocol = protocol,
        address = config.get::<std::string::String>("repository_url").unwrap()),
        Err(e) => format!("{}",e),
    };
    
    //We call the API in question...
    let result = reqwest::blocking::get(url).unwrap().text().unwrap();

    //Then we parse the JSON result.
    let parsed = json::parse(&result);

    //Nice message before the list of images, which allows the user to see the address
    //of the repo. Just in case.
    println!("List of Images on {protocol}{address}", 
        protocol = config.get::<std::string::String>("registry_protocol").unwrap(),
        address = config.get::<std::string::String>("repository_url").unwrap()); 

    //We print the list of images before exiting the function.
    for repo in parsed.unwrap()["repositories"].members() {
        println!("{}", repo)
    }
}