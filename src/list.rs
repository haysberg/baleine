extern crate json;
extern crate dotenv;
use crate::utils::env_var;

/// This function allows us to list the images available on the registry configured in config.toml.
/// We call the Docker API available on the registry image then format it to make it readable for the user.
///
/// # Arguments
///
/// * `details` - name of the image for which you want to display all the different versions available
pub fn list (details: &Option<String>) {

    //We generate the URL used to call the API
    let url = match details{
        Some(image_name) => format!("{protocol}{address}/v2/{image_name}/tags/list",
        protocol = env_var("REGISTRY_PROTOCOL").unwrap_or("http://".to_string()),
        address = env_var("REGISTRY_URL").unwrap_or("faraday".to_string()),
        image_name = image_name),
        
        None => format!("{protocol}{address}/v2/_catalog",
        protocol = env_var("REGISTRY_PROTOCOL").unwrap_or("http://".to_string()),
        address = env_var("REGISTRY_URL").unwrap_or("faraday".to_string())),
    };
    
    
    //We call the API in question...
    let result = match reqwest::blocking::get(url) {
        Ok(value) => value.text().unwrap(),
        Err(_) => "ERROR".to_string(),
    };

    //Then we parse the JSON result.
    let parsed = json::parse(&result);
    
    match details {
        Some(image_name) => println!("List of tags for the {} image :", image_name),
        None => println!("List of Images on {protocol}{address}", protocol = env_var("REGISTRY_PROTOCOL").unwrap_or("http://".to_string()), address = env_var("REGISTRY_URL").unwrap_or("faraday".to_string())) 
    }

    //We print the list of images before exiting the function.
    for member in parsed.unwrap()["repositories"].members() {
        println!("{}", member)
    }

    //Printing the list of tags as well
    let parsed = json::parse(&result);
    for member in parsed.unwrap()["tags"].members() {
        println!("{}", member)
    }
}

/// Entry function. Doesn't do anything right now.
/// Was implemented for the sake of consistency
pub fn entry (details: &Option<String>) {
    list(details);
}