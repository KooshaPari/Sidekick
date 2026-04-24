# AGENTS.md — Sidekick Agent Instructions

This project is part of the Phenotype ecosystem. All agent work is tracked in AgilePlus.

## Quick Reference

- **AgilePlus**: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- **Global instructions**: `~/.claude/CLAUDE.md`
- **Project instructions**: `./CLAUDE.md` (this repo)
- **Worklog aggregation**: `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/`

## Workflows

1. **Before implementing**: Check AgilePlus spec (`agileplus specify --title "..."`)
2. **During work**: Update status (`agileplus status <id> --wp <wp-id> --state <state>`)
3. **On completion**: Document in project worklog (if cross-cutting)

## Multi-Crate Builds

```bash
cargo build --release --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

Delegate file exploration and multi-file analysis to subagents. Keep parent agent context for strategic decisions.

## See Also

- `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md` — Phenotype workspace rules
- `/Users/kooshapari/CodeProjects/Phenotype/CLAUDE.md` — Phenotype org governance
