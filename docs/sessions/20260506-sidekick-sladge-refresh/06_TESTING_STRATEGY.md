# Testing Strategy

## Focused Checks

- `git diff --check` passed.
- README badge presence with `rg` passed.

## Repo-Native Checks

- `cargo test --workspace --offline` is blocked before tests by missing
  workspace member `crates/sidekick-dispatch/Cargo.toml`.
- `cargo clippy --workspace --offline -- -D warnings` is blocked by the same
  missing workspace member.
- `cargo fmt --check` is blocked by the same Cargo metadata error.

Any Cargo blockers should be recorded as current checkout issues instead of
repaired in this README/session-doc governance change.
