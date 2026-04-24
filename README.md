# Sidekick — Agent Utility Collection

A named Rust workspace consolidating core agent infrastructure utilities for the Phenotype ecosystem.

## Components

- **sidekick-presence** — Agent presence and user-status tracking (online/away/focus modes)
- **sidekick-dispatch** — Multi-provider dispatch routing for heterogeneous agent fleets
- **sidekick-cheap-llm** — Budget-conscious LLM routing (Minimax, Kimi, Fireworks) via FastMCP
- **sidekick-messaging** — Forthcoming: multi-provider messaging integration (iMessage, SMS, RCS)

## Quick Start

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/Sidekick
cargo build --release
cargo test --workspace
```

## Architecture

Sidekick is a polyglot workspace:
- **Rust crates** (`crates/sidekick-*`): Compiled binaries and libraries
- **Python sub-package** (`crates/sidekick-cheap-llm`): FastMCP wrapper, imported as Python module

Each sub-crate is independently versioned and consumable; consumers import only what they need.

## Publishing

Crates published to crates.io under `sidekick-*` prefix.

## License

Apache 2.0
