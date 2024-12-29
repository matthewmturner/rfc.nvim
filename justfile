install-targets:
    rustup target add aarch64-unknown-linux-gnu

install-dependencies:
    cargo install cross --git https://github.com/cross-rs/cross

install: install-targets install-dependencies
    echo "Finished installation"

build-artifacts:
    cargo build --release --manifest-path crates/ffi/Cargo.toml

[macos]
copy-artifacts:
    cp target/release/libffi.dylib artifacts/

[linux]
copy-artifacts:
    cp target/release/libffi.so artifacts/

generate-artifacts: build-artifacts copy-artifacts

publish-dry-run:
    cargo publish -n --manifest-path crates/tf_idf/Cargo.toml
    cargo publish -n --manifest-path crates/cli/Cargo.toml

generate-test-index:
    cargo r --manifest-path tests/generate-data/Cargo.toml
