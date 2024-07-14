#!/bin/bash

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Определение пакетного менеджера
if command_exists apt; then
    PKG_MANAGER="apt"
elif command_exists yum; then
    PKG_MANAGER="yum"
elif command_exists dnf; then
    PKG_MANAGER="dnf"
elif command_exists pacman; then
    PKG_MANAGER="pacman"
elif command_exists zypper; then
    PKG_MANAGER="zypper"
else
    echo "\033[0;31mSupported package manager not found. Please manually install if you don't have: what, curl, git, unzip \033[0m"
fi

case "$PKG_MANAGER" in
    apt)
        sudo apt update
        sudo apt install -y git wget curl zip unzip
        ;;
    yum)
        sudo yum install -y git wget curl zip unzip
        ;;
    dnf)
        sudo dnf install -y git wget curl zip unzip
        ;;
    pacman)
        sudo pacman -Syu --noconfirm
        sudo pacman -S --noconfirm git wget curl zip unzip
        ;;
    zypper)
        sudo zypper refresh
        sudo zypper install -y git wget curl zip unzip
        ;;
esac

echo "Installing rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.bashrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.zshrc

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

echo 'export PATH="$HOME/.bin/hiw:$PATH"' >> $HOME/.bashrc
echo 'export PATH="$HOME/.bin/hiw:$PATH"' >> $HOME/.zshrc

echo -e "\033[0;32mInstallation and download completed!\033[0m"

