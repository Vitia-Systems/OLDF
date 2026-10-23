setup:
    rustup show
    cargo fetch --locked

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings

test:
    cargo test --workspace --locked

ci: check test
