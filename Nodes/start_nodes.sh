#!/bin/bash

# Array of node directories
node_dirs=("node_1" "node_2" "node_3")

# Start cargo run in each directory simultaneously in a new terminal
for dir in "${node_dirs[@]}"; do
    gnome-terminal -- bash -c "cd \"$dir\" && cargo run; exec bash"
done

# Wait for all cargo run commands to finish
wait
