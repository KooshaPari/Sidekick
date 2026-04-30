# Implementation Strategy

## Approach

Keep the update narrowly scoped:

- Add one badge line to the existing README badge block.
- Add session docs under `docs/sessions/`.
- Avoid source, dependency, asset, submodule, and generated artifact changes.

## Git Strategy

Prepare in:

`Sidekick-wtrees/sladge-badge`

This avoids mixing the badge rollout with unrelated canonical checkout changes.
