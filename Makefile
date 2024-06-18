watch:
	cargo watch -c -w src -x run
# Cross Install
# cargo install cross --git https://github.com/cross-rs/cross
build-x86:
	cross build --target x86_64-unknown-linux-musl --release
build-aarch64:
	cross build --target aarch64-unknown-linux-gnu --release
