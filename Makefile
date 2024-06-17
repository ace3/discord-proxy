# Install the toolchain to build Linux x86_64 binaries
# rustup target add x86_64-unknown-linux-gnu
build-linux:
	cargo build --release --target=x86_64-unknown-linux-gnu
watch:
	cargo watch -c -w src -x run