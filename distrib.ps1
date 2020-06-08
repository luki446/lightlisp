# Get version number from Cargo.toml
$version = (Get-Content .\Cargo.toml | Select-String version).Line.Split("""")[1]

# Build and pack windows binary
cargo build --release
Compress-Archive -Path .\target\release\llisp.exe -DestinationPath ".\llisp-$version-windows.zip" -Force
# Use WSL to compile and pack linux binary (full path to cargo because of some weird error with WSL)
bash -c " ~/.cargo/bin/cargo build --release && tar -C ./target/release/ -czf llisp-$version-linux.tar.gz llisp"