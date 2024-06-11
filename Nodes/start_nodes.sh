#!/bin/bash

# Directory to run cargo in
node_dir="full_node"

# Change to the specified directory
cd "$node_dir"

# Open new terminals and run cargo with different ports
gnome-terminal -- bash -c "cargo run -- --address localhost --port 3334; exec bash"
gnome-terminal -- bash -c "cargo run -- --address localhost --port 3335; exec bash"
gnome-terminal -- bash -c "cargo run -- --address localhost --port 3336; exec bash"

# Wait for all cargo run commands to finish
wait
