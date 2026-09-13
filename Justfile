ci:
    cargo check -p blink-one --locked
    cargo check -p blink-one -F commands --locked
    cargo check -p blink1-ha --locked
    cargo nextest run --all-features --no-tests=warn
    cargo fmt -- --check
    cargo clippy --workspace --all-features --all-targets -- -D warnings
    cargo clippy --workspace --all-features --all-targets -- -W clippy::pedantic
