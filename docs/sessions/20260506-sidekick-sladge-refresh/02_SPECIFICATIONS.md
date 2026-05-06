# Specifications

## Acceptance Criteria

- Add the Sladge badge near the existing README badge block.
- Do not alter source, workflow, dependency, or governance files outside this
  session evidence.
- Keep the canonical checkout untouched unless it is clean and integration is
  explicitly safe.
- Record validation results and blockers for the projects-landing governance
  ledger.

## Assumptions, Risks, Uncertainties

- Assumption: README badge disclosure is sufficient for this governance item.
- Risk: Cargo workspace validation may still be blocked by existing workspace
  membership or path dependency drift.
- Mitigation: record command-backed blockers without broad source repairs in a
  README/session-doc badge change.
