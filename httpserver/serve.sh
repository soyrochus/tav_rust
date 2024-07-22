#!/bin/bash

# Step 1: Get the Rust documentation path
rstdoc_path=$(rustup doc --path)
rstdoc_path=${rstdoc_path%/index.html}

# Step 2: Build the Rust project
project_path=${pwd}  # Or replace with your actual project path
cd "$project_path"
cargo build --release

# Step 3: Store the path to the executable
exe_path="$PWD/target/release/httpserver"

# Step 4: Change directory to the Rust documentation path
cd "$rstdoc_path"

# Step 5: Execute the httpserver executable
"$exe_path"
