#!/bin/bash

cargo build --release
sudo cp ./target/release/r2dock /bin/r2dock