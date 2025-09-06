#!/bin/bash

# Remove compiled Rust executables based on .rs file names

echo "Removing compiled executables..."

for rs_file in *.rs; do
    if [ -f "$rs_file" ]; then
        # Get the executable name by removing the .rs extension
        executable="${rs_file%.rs}"
        
        if [ -f "$executable" ]; then
            rm "$executable"
        fi
    fi
done

echo "Cleanup complete!"
