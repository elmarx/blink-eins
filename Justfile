ci:
    cargo check -p blink-one --locked
    cargo check -p blink-one -F commands --locked
    cargo nextest run -p blink-one --no-tests=warn
    cargo fmt -- --check
    cargo clippy -p blink-one --all-features --all-targets -- -D warnings
    cargo clippy -p blink-one --all-features --all-targets -- -W clippy::pedantic
