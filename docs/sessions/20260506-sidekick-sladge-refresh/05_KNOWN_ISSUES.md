# Known Issues

- The old `Sidekick-wtrees/sidekick-sladge-current` worktree is stale relative
  to the current `ci/pin-trufflehog` branch.
- Cargo validation may fail before tests if workspace members or sibling path
  dependencies are unavailable in the current checkout shape.
- Current Cargo validation stops before tests because the workspace manifest
  references missing member `crates/sidekick-dispatch`.
