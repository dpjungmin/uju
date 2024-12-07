default:
    @just dev

dev:
    @just build
    basic-http-server .

build:
    cargo build --release

list:
    @just --list

fmt:
    cargo +nightly fmt --all

lint:
    cargo +nightly fmt --all --check
    cargo +stable clippy --all-features --all-targets -- -Dwarnings
