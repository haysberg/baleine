#!/bin/bash

cargo build --release
sudo cp ./target/release/r2dock /bin/r2dock

mkdir /etc/r2dock
[ $? -eq 0 ] && cp ./r2dock.example.conf /etc/r2dock/r2dock.conf || echo "Done."
