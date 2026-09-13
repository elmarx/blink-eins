ci:
    cargo check --locked
    cargo check -F commands --locked
    cargo nextest run --no-tests=warn
    cargo fmt -- --check
    cargo clippy --all-features --all-targets -- -D warnings
    cargo clippy --all-features --all-targets -- -W clippy::pedantic
