#!/bin/bash

if command -v rustup &> /dev/null
then
    echo "rustup is already installed"
else
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' "$SHELL_CONFIG"; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$SHELL_CONFIG"
    source "$SHELL_CONFIG"
fi

source $HOME/.cargo/env

echo "Updating rustup and installing the stable version of Rust..."
rustup update stable

echo "Checking Rust installation..."
if command -v rustc &> /dev/null
then
    echo "Rust is successfully installed"
    rustc --version
else
    echo -e "\033[0;31mRust installation failed\033[0m"
    exit 1
fi

echo "Downloading the latest release from the hiw-lang repository..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/mealet/hiw-lang/releases/latest | grep "browser_download_url" | grep "hiw-release.zip" | cut -d '"' -f 4)
if [ -n "$LATEST_RELEASE_URL" ]; then
    wget $LATEST_RELEASE_URL -O hiw-release.zip
    echo "Latest release downloaded successfully as hiw-release.zip"
else
    echo -e "\033[0;31mFailed to download the latest release\033[0m"
    exit 1
fi

mkdir -p $HOME/.bin

echo "Unzipping hiw-release.zip to $HOME/.bin/..."
unzip hiw-release.zip -d $HOME/.bin/

echo "Removing the downloaded archive hiw-release.zip..."
rm hiw-release.zip

echo "export PATH="$HOME/.bin/hiw:$PATH" >> ~/.bashrc
echo "export PATH="$HOME/.bin/hiw:$PATH" >> ~/.zshrc

echo -e "\033[0;32mInstallation and download completed!\033[0m"

