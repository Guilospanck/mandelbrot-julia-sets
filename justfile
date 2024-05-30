default:
  just --list

clippy:
  cargo clippy --all-targets -- -D warnings

check:
  cargo check --all-targets

test:
  cargo test --tests