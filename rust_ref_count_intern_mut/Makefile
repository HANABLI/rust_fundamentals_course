# This Makefile is used to build the Rust project and run tests.
rust-version: 
			@echo "Checking Rust version..."
			rustc --version
			cargo --version
			rustfmt --version
			rustup --version
			clippy-driver --version

format:
			cargo fmt --quiet

lint: 
			cargo clippy --quiet

test: 
			cargo test --quiet

build-debug:
			cargo build --debug --quiet

build-release:
			cargo build --release --quiet
run:
			cargo run --quiet
clean:
			cargo clean --quiet

