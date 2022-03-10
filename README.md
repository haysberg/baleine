# Baleine 🐋
Orchestrate Docker containers over the R2Lab platform from a single CLI.

Using the four subcommands of this tool, you can deploy Docker containers on top of the 37 test nodes available in R2Lab.
- `deploy` to automatically deploy containers on the specified nodes with the Docker options you want
- `destroy` to remove the containers
- `save` to save a running container as a new image on the local R2Lab repository
- `list` to get a list of all the images on the repository

Other benefits :
- By using a local repository, we can cache the image downloaded from external sources, allowing for much faster deployment.
- The code is fully multi-threaded, allowing for good scalability.
- The project is completely open-source and all the libraries are contained within the binary, which makes it extremely portable.
- Using Docker containers, we can save, deploy and destroy an instance in mere seconds.
- Thanks to the nature of Docker containers, our experiments can be easily reproduced without having to fiddle with config files.

### How to install the CLI on the master
```
git clone https://github.com/haysberg/baleine
cd baleine
cargo build --release
./target/release/baleine --help
```

### How to deploy a docker registry cache
```
wget https://raw.githubusercontent.com/haysberg/baleine/main/setup_files/gateway/docker-compose.yml
# -d detaches the output from the terminal
docker compose up -d

```

### How to setup a slave node
```
sudo sh -c "$(wget https://raw.githubusercontent.com/haysberg/baleine/main/setup_files/nodes/setup_node.sh -O -)"
```
### Need help ?

Please refer to the [docs](https://github.com/haysberg/baleine/wiki) to get an exhaustive list of example commands.

📨 Please [open an issue](https://github.com/haysberg/baleine/issues/new) or send me an email in case of bugs : [teo.hays@inria.fr]((mailto:teo.hays@inria.fr))
