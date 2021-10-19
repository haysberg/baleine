extern crate json;
extern crate dotenv;
use dotenv_codegen::dotenv;

///This function allows us to list the images available on the registry configured in config.toml.
///We call the Docker API available on the registry image then format it to make it readable for the user.
pub fn list (args: &clap::ArgMatches){

    //We generate the URL used to call the API
    let url = match args.value_of("details"){
        Some(image_name) => format!("{protocol}{address}/v2/{image_name}/tags/list",
        protocol = dotenv!("REGISTRY_PROTOCOL"),
        address = dotenv!("REGISTRY_URL"),
        image_name = image_name),
        
        None => format!("{protocol}{address}/v2/_catalog",
        protocol = dotenv!("REGISTRY_PROTOCOL"),
        address = dotenv!("REGISTRY_URL")),
    };
    
    
    //We call the API in question...
    let result = match reqwest::blocking::get(url) {
        Ok(value) => value.text().unwrap(),
        Err(_) => "ERROR".to_string(),
    };

    //Then we parse the JSON result.
    let parsed = json::parse(&result);
    
    match args.value_of("details"){
        Some(image_name) => println!("List of tags for the {} image :", image_name),
        None => println!("List of Images on {protocol}{address}", protocol = dotenv!("REGISTRY_PROTOCOL"), address = dotenv!("REGISTRY_URL")) 
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

pub fn entry (args: &clap::ArgMatches){
    list(&args);
}