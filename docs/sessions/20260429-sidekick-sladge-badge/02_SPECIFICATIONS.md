# Specifications

## Scope

- Add the sladge badge to `README.md`.
- Do not change Rust code, Python package contents, assets, submodules, release
  registry, or generated artifacts.
- Preserve unrelated canonical checkout changes.

## Acceptance Criteria

- README contains `https://sladge.net/badge.svg`.
- Badge appears with the existing README badge block.
- Session docs explain why the repo is in scope.

## Assumptions, Risks, Uncertainties

- Assumption: Sidekick is materially AI-related because it collects agent
  dispatch, MCP presence, and budget LLM routing utilities.
- Risk: Canonical merge may need to account for unrelated brand/readme/cutover
  and submodule changes.
- Mitigation: Record the prepared commit and worktree in projects-landing.
