# Step 1: Get the Rust documentation path
$rstdoc_path = rustup doc --path
$rstdoc_path = $rstdoc_path.Replace('\index.html', '')

# Step 2: Build the Rust project
$project_path = "D:\src\tav_rust\httpserver"  # Replace with your actual project path
cd $project_path
cargo build --release

# Step 3: Store the path to the executable
$exe_path = Join-Path -Path $PWD -ChildPath "target\release\httpserver.exe"

# Step 4: Change directory to the Rust documentation path
cd $rstdoc_path

# Step 5: Execute the httpserver.exe
& $exe_path
