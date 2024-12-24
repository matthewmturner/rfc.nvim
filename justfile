install-targets:
    rustup target add aarch64-unknown-linux-gnu

install-dependencies:
    cargo install cross --git https://github.com/cross-rs/cross

install: install-targets install-dependencies
    echo "Finished installation"

build-artifacts:
    cross build --release --target aarch64-unknown-linux-gnu --manifest-path crates/ffi/Cargo.toml
