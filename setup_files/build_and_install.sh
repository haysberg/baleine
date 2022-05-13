#!/bin/bash

sudo dnf install rust cargo

cargo build --release
sudo cp ./target/release/baleine /bin/baleine

mkdir /etc/baleine
[ $? -eq 0 ] && cp ./baleine.example.conf /etc/baleine/baleine.conf || echo "Done."
