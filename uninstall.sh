#!/bin/bash

if [ -d "$HOME/.bin/hiw" ]; then
    rm -rf "$HOME/.bin/hiw"
    echo "Folder ~/.bin/hiw deleted."
else
    echo "Folder ~/.bin/hiw not found."
fi

remove_line() {
    local file=$1
    local pattern=$2
    if [ -f "$file" ]; then
        sed -i.bak "/$pattern/d" "$file"
        echo "Line '$pattern' removed from $file."
    else
        echo "File $file not found."
    fi
}

remove_line "$HOME/.bashrc" 'export PATH="$HOME/.bin/hiw:$PATH"'
remove_line "$HOME/.zshrc" 'export PATH="$HOME/.bin/hiw:$PATH"'
