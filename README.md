# Baleine 🐋
Orchestrate Docker containers over the R2Lab platform from a single CLI.

### How to install the CLI on the master
```
git clone https://github.com/haysberg/baleine
cd baleine
cargo build --release
./target/release/baleine --help
```

### How to setup a slave node
```
sudo sh -c "$(wget https://raw.githubusercontent.com/haysberg/baleine/main/setup_files/nodes/setup_node.sh -O -)"
```
### Need help ?

Please refer to the [docs](https://github.com/haysberg/baleine/wiki) to get an exhaustive list of example commands.

📨 Please [open an issue](https://github.com/haysberg/baleine/issues/new) or send me an email in case of bugs : [teo.hays@inria.fr]((mailto:teo.hays@inria.fr))
