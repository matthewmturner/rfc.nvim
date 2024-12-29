install-targets:
    rustup target add aarch64-unknown-linux-gnu

install-dependencies:
    cargo install cross --git https://github.com/cross-rs/cross

install: install-targets install-dependencies
    echo "Finished installation"

[macos]
build-artifacts:
    cargo build --release --manifest-path crates/ffi/Cargo.toml --target aarch64-apple-darwin
    cp target/aarch64-apple-darwin/release/libffi.dylib artifacts/

[linux]
build-artifacts:
    cargo build --release --manifest-path crates/ffi/Cargo.toml --target aarch64-unknown-linux-gnu
    cp target/aarch64-unknown-linux-gnu/release/libffi.so artifacts/

publish-dry-run:
    cargo publish -n --manifest-path crates/tf_idf/Cargo.toml
    cargo publish -n --manifest-path crates/cli/Cargo.toml

generate-test-index:
    cargo r --manifest-path tests/generate-data/Cargo.toml
