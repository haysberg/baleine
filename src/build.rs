use tracing::{error, instrument};

use crate::utils::{env_var, local_command};

extern crate dotenv;

/// This function takes a container running on a node and saves it to the remote registry configured in config.toml
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved
#[instrument]
pub fn build(file: &Option<String>, url: &Option<String>, tags: &Vec<String>) {
    let port = env_var("SAVE_PORT").unwrap_or("80".to_string());
    let primary_tag = tags.get(0).unwrap();
    let repo_url = env_var("SAVE_URL").unwrap_or("faraday.repo".to_string());
    let tag_args : String = tags.iter().map(|x| format!(" -t localhost:{port}/{x} -t {repo_url}/{x}")).collect();

    let cmd : String = match url{
        Some(url) => format!("docker build{tag_args} {url} && docker push --all-tags localhost:{port}/{primary_tag}"),
        None => format!("docker build {tag_args} -f {path} . && docker push --all-tags localhost:{port}/{primary_tag}", path = file.clone().unwrap())
    };

    let push_args : String = tags.iter().map(|x| format!(" && docker push localhost:{port}/{x} && docker push {repo_url}/{x}")).collect();

    match local_command(format!("{cmd}{push_args}")){
        Ok(_) => (),
        Err(_) => error!("Error while running command {}{}", cmd, push_args)
    }
}

/// The entry() function works as an entrypoint that does a bit of parsing as well as other checks depending on the function it calls later
///
/// # Arguments
///
/// * `name` - name of the image that you are creating
/// * `node` - target slave node that will be saved
#[instrument]
pub fn entry(file: &Option<String>, url: &Option<String>, tags: &Vec<String>) {
    build(file, url, tags);
}
