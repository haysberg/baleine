# r2dock
### This software allows you to deploy Docker containers on top of the rhubarbe platform.

# How to install the CLI on the master
```
git clone https://github.com/haysberg/r2dock
cd r2dock
cargo build --release
./target/release/r2dock --help
```

# How to setup a slave node
```
sudo sh -c "$(wget https://raw.github.com/haysberg/r2dock/master/setup_files/setup_node.sh -O -)"
```
# Examples
## Deploy a container on a node
```
r2dock deploy --name ubuntu --nodes 1 --options "-t -d"
```

## Save a running container on the configured Docker registry
```
r2dock save --name custom_image1 --node 1
```

## Destroy a container
```
r2dock destroy --nodes 1
```
Please note that you can bypass the confirmation to delete the container(s) by adding the --yes option :
```
r2dock destroy --nodes 1 --yes
```

## List the images currently available on the configured registry
```
r2dock list
```

## List the available tags / versions for a specific image
```
r2dock list --details custom_image1
```
## 📨 Please open an issue or send me an email in case of bugs : teo.hays@inria.fr