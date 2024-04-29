#!/bin/bash

# Determine the directory of the vcli script
CLI_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if vcli already exists in the bin directory
if [ -e "/usr/local/bin/com" ]; then
    echo "commit-cli is already installed in /usr/local/bin."
    exit 0
fi

# copy the vcli script to the bin directory
if [ -w "/usr/local/bin" ]; then
    ln -s "$CLI_DIR/target/release/commit-cli-rust" /usr/local/bin/com
    echo "commit-cli has been installed successfully in /usr/local/bin."
else
    echo "Error: Permission denied. You may need to run this script with sudo."
    exit 1
fi

# make the script executable
chmod +x /usr/local/bin/com

echo "Executable permissions have been set for commit-cli"
