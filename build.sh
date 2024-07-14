#!/bin/bash

# Installing dependency
sudo apt-get install mingw-w64

# First building for Linux and Windows
echo "Building binaries..."
cargo build --release
cargo build --target x86_64-pc-windows-gnu --release

# Creating release folder
mkdir hiw


# Next copying binaries to temporary folder
cp target/release/hiw hiw/hiw
cp target/x86_64-pc-windows-gnu/release/hiw.exe hiw/hiw.exe

# Copying modules and dependencies
echo "Adding modules and dependencies"
cp modules/* hiw
cp src/vm.rs hiw

# Finally packing release

echo "Packing release..."

zip hiw-release.zip hiw/*

# Removing temp files

rm hiw -d -r
