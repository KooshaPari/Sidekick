# DAG WBS

## Work Breakdown

1. Verify canonical checkout status and active branch.
2. Compare old prepared badge evidence against current HEAD.
3. Create a fresh isolated worktree from current HEAD.
4. Add README badge and session evidence.
5. Run focused validation.
6. Commit the isolated badge refresh.
7. Update projects-landing governance/task ledgers.

## Dependency Notes

- Step 3 depends on confirming the old worktree is stale.
- Step 6 depends on focused validation and documented blockers.
- Step 7 depends on the downstream commit hash.
