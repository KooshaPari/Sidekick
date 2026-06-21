# Sidekick Getting Started

## Why Sidekick?

Sidekick solves the foundational challenge of agent infrastructure: routing dispatch requests across heterogeneous LLM providers, tracking user presence and status, and managing budget-conscious inference. Whether you're building a multi-agent system that needs to balance cost with latency, or you need to track agent user-status (online, away, focus modes) across distributed systems, Sidekick provides the primitives.

**Key problems Sidekick solves:**

- **Multi-provider dispatch** — Route a single task to Forge, Codex, Copilot, Gemini, Cursor, or local models, with transparent failover and cost-aware selection
- **User presence tracking** — Maintain real-time agent status (online, away, focus, inactive) synced across distributed presence services
- **Budget LLM routing** — Use Minimax, Kimi, or Fireworks for tasks that don't need Opus/Sonnet, cutting inference costs 3-10x
- **Cross-collection coordination** — Emit dispatch events to typed dispatch events so downstream collections (Eidolon, Observably) can react

## Install

Add the crates you need:

```bash
cargo add sidekick-dispatch
cargo add sidekick-presence
cargo add sidekick-cheap-llm
```

Or in your `Cargo.toml`:

```toml
[dependencies]
sidekick-dispatch = { path = "../../sidekick/crates/sidekick-dispatch" }
sidekick-presence = { path = "../../sidekick/crates/sidekick-presence" }
sidekick-cheap-llm = { path = "../../sidekick/crates/sidekick-cheap-llm" }

tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Quickstart (20 lines)

```rust
use sidekick_presence::{PresenceManager, UserStatus};
use sidekick_dispatch::{Dispatcher, DispatchRequest};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Track agent user status
    let presence = Arc::new(PresenceManager::new());
    presence.set_status("agent-001", UserStatus::Online).await?;

    // Dispatch a task to the cheapest available provider
    let dispatcher = Dispatcher::new();
    let request = DispatchRequest::new("agent-001", "summarize: ...");
    let response = dispatcher.dispatch(request).await?;

    println!("Dispatch result: {:?}", response);

    // Query presence
    let status = presence.get_status("agent-001").await?;
    println!("Agent status: {:?}", status);

    Ok(())
}
```

## Common Patterns

### Pattern 1: Presence Tracking with Status Transitions

Track agent lifecycle: online → away (timeout) → inactive (logout).

```rust
use sidekick_presence::{PresenceManager, UserStatus};
use std::time::Duration;

let presence = PresenceManager::new();

// Agent comes online
presence.set_status("agent-001", UserStatus::Online).await?;

// After 15 minutes of inactivity, mark away
tokio::spawn({
    let presence = presence.clone();
    async move {
        tokio::time::sleep(Duration::from_secs(900)).await;
        presence.set_status("agent-001", UserStatus::Away).await.ok();
    }
});
```

### Pattern 2: Cost-Aware Dispatch Routing

Route tasks based on budget constraints:

```rust
use sidekick_dispatch::{Dispatcher, DispatchRequest, Provider};

let dispatcher = Dispatcher::new();

// For quick summaries, use Minimax (cheap-llm)
let cheap_request = DispatchRequest::new("agent", "summarize...")
    .with_provider_hint(Provider::Minimax);
let result = dispatcher.dispatch(cheap_request).await?;

// For complex reasoning, route to Opus (full-cost)
let expensive_request = DispatchRequest::new("agent", "design architecture...")
    .with_provider_hint(Provider::OpusAsNeeded);
let result = dispatcher.dispatch(expensive_request).await?;
```

### Pattern 3: Cross-Collection Events via typed dispatch events

Emit dispatch lifecycle events so other collections (Eidolon, Observably) can respond.

```rust
use phenotype_bus::{Bus, Event};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DispatchStarted {
    pub dispatch_id: String,
    pub agent_id: String,
    pub provider: String,
}

impl Event for DispatchStarted {
    fn event_name(&self) -> &'static str {
        "DispatchStarted"
    }
}

// Emit event
let bus = Bus::new(100);
let event = DispatchStarted {
    dispatch_id: "disp-001".into(),
    agent_id: "agent-001".into(),
    provider: "forge".into(),
};
bus.publish(event).await?;

// Eidolon or Observably can subscribe
let mut rx = bus.subscribe();
while let Ok(event) = rx.recv().await {
    println!("Dispatch started: {}", event.dispatch_id);
}
```

## Cross-Collection Integration

Sidekick integrates with the Phenotype ecosystem via **typed dispatch events**:

- **Emits**: `DispatchStarted`, `DispatchCompleted`, `StatusChanged` events
- **Receives**: Status checks from Eidolon (automation triggers dispatch), Observably (traces dispatch decisions)

See [typed dispatch events](../../typed dispatch events/README.md) for event bus patterns. Sidekick works
seamlessly with [Eidolon](../../Eidolon/README.md) (device automation triggered by dispatch),
[Observably](../../PhenoObservability/README.md) (traces all dispatch routing), and
[Stashly](../../Stashly/README.md) (caches dispatch results).

## Next Steps

- Review the routing and presence examples in this guide.
- Read the [dispatch-worker routing pattern](../../docs/governance/dispatch_routing_pattern_2026_04_27.md).
- See [cheap-llm MCP patterns](../crates/sidekick-cheap-llm/README.md).
- Try the [cross-collection demo](../crates/sidekick-dispatch/examples/cross_collection_demo.rs).
- Check [docs/FUNCTIONAL_REQUIREMENTS.md](FUNCTIONAL_REQUIREMENTS.md) for requirement coverage.
