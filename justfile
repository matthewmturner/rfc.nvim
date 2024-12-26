install-targets:
    rustup target add aarch64-unknown-linux-gnu

install-dependencies:
    cargo install cross --git https://github.com/cross-rs/cross

install: install-targets install-dependencies
    echo "Finished installation"

build-artifacts:
    cargo build --release --manifest-path crates/ffi/Cargo.toml --target aarch64-apple-darwin
    cp crates/ffi/target/release/libffi.dylib artifacts/

publish-dry-run:
    cargo publish -n --manifest-path crates/fetch/Cargo.toml
    cargo publish -n --manifest-path crates/threadpool/Cargo.toml
    cargo publish -n --manifest-path crates/tf_idf/Cargo.toml
    cargo publish -n --manifest-path crates/cli/Cargo.toml
