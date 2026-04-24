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

## Release Registry

See `release-registry.toml` for version metadata, stability information, and sub-crate status. The master index of all Phenotype collections is at `../phenotype-collections.toml`.

Schema documentation: `docs/governance/release_registry_schema.md`

## Cross-Collection Integration

Sidekick is part of the **Phenotype named collections**:

- **Sidekick** (this) — Agent dispatch & presence
- **Eidolon** — Device automation (desktop, mobile, sandbox)
- **Observably** — Distributed tracing & observability
- **Stashly** — State, events, caching, migrations
- **Paginary** — Knowledge collection (specs, tutorials, handbooks)

### Event Bus

Sidekick uses **phenotype-bus** for cross-collection communication. Collections emit domain events that other collections consume without hardcoded dependencies:

```rust
use phenotype_bus::{Bus, Event};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DispatchStarted {
    pub provider: String,
}

impl Event for DispatchStarted {
    fn event_name(&self) -> &'static str { "DispatchStarted" }
}

// Emit event
let bus = Bus::new(100);
bus.publish(DispatchStarted { provider: "forge".into() }).await?;

// Subscribe in another collection (e.g., Eidolon)
let mut rx = bus.subscribe();
while let Ok(event) = rx.recv().await {
    println!("Got dispatch event: {}", event.event_name());
}
```

See `../../phenotype-bus/README.md` and `docs/org-audit-2026-04/collection_build_matrix.md` for integration details.

## Publishing

Crates published to crates.io under `sidekick-*` prefix.

## License

Apache 2.0
