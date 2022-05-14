set shell := ["nu", "-c"]

watch:
    watch . { cargo run } --glob=**/*.rs

run:
    cargo run

test:
    cargo test

watch-tests:
    watch . { cargo tests } --glob=**/*.rs

publish:
    cargo build --release
    @$"Build size: (ls target/release/burninator.exe | get size)"