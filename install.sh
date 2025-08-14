#!/bin/bash
cargo install --path .
cd example/
ranni -t image -i example/sakura.jpg -s 8

echo "installed successfully"
