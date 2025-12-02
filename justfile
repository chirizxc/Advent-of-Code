set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

[private]
@default:
    just --list

fmt:
  cargo fmt-nightly

lint:
  cargo fmt-nightly --check
  cargo clippy --all-targets

run DAY:
  cargo run --release --package aoc2025 --bin {{ DAY }}
