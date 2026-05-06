# Research

## Current State

- Canonical checkout: `Sidekick`
- Active branch: `ci/pin-trufflehog`
- Active HEAD before this work: `b3519a5`
- Prior prepared evidence:
  `Sidekick-wtrees/sidekick-sladge-current` commit `647c27b`

The prior prepared worktree was based on the older `main` lineage and reported
`ahead 1, behind 1`, so it was treated as stale evidence rather than reused.

## Badge Applicability

The README describes Sidekick as an agent utility collection that includes MCP
presence/status tooling, cheap LLM routing, and agent messaging adapters. That
is direct LLM/agent-runtime ownership, so Sladge disclosure is appropriate.
