#!/bin/bash

cargo build --release
sudo cp ./target/release/r2dock /bin/r2dock

mkdir /etc/baleine
[ $? -eq 0 ] && cp ./baleine.example.conf /etc/baleine/baleine.conf || echo "Done."
