#!/bin/bash
rustup update
cargo update
cargo build --release
sudo cp ./target/release/baleine /bin/baleine

mkdir /etc/baleine
[ $? -eq 0 ] && cp ./setup_files/gateway/baleine.example.conf /etc/baleine/baleine.conf || echo "Done."
