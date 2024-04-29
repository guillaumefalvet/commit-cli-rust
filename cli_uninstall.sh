#!/bin/bash

# Check if vcli exists in the bin directory
if [ ! -e "/usr/local/bin/com" ]; then
    echo "commit-cli is not installed in /usr/local/bin."
    exit 0
fi

# Remove the file from the bin directory
if [ -w "/usr/local/bin/com" ]; then
    rm /usr/local/bin/com
    echo "commit-cli has been uninstalled successfully from /usr/local/bin."
else
    echo "Error: Permission denied. You may need to run this script with sudo."
    exit 1
fi

echo "commit-cli uninstallation complete."
