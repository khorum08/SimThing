# AGENT-COMPLETION-DISCIPLINE-0 — prevent cargo-output hangs and require durable final reporting

> **Lifecycle: OPERATIONAL** — agent handoff discipline; not a product feature rung. Not DA-promoted.

## Status

LANDED — operational discipline documented and wired into `docs/agents.md`.

## PR / branch / merge

| Item | Value |
|------|-------|
| Branch | `agent-completion-discipline-0` |
| PR | #793 — AGENT-COMPLETION-DISCIPLINE-0 |
| Merge | `dbf33b640091052f377c95db1a2b5392459b5c1a` |

Post-merge metadata finalized by AGENT-COMPLETION-DISCIPLINE-0R.

## Current defect or mission

Recent agent turns completed and merged PRs but ended inside cargo/test terminal output without a durable natural-language summation. Owner/orchestrator could not tell whether work was complete, merged, validated, or hung.

## Implemented changes

- Canonical discipline: `docs/tests/agent_completion_discipline_0.md`
- Router integration: `docs/agents.md` § Agent completion discipline (mandatory for implementation rungs)
- Evidence index row for operational discipline

## Boundary / constitution checks

| Check | Status |
|-------|--------|
| No product/runtime code changes | PASS (by design) |
| No GPU/WGSL changes | PASS (by design) |
| No sim runtime changes | PASS (by design) |
| No Terran Pirate fixture edits | PASS (by design) |
| Live ledger preserved | PASS (by design) |

## Validation commands

| Command | Status |
|---------|--------|
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (2 doc files) |
| `cargo test` (all packages) | SKIP — documentation/evidence metadata only; no Rust code touched |

## Files changed

- `docs/tests/agent_completion_discipline_0.md` (new)
- `docs/tests/agent_completion_discipline_0_results.md` (new)
- `docs/agents.md`
- `docs/tests/current_evidence_index.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/agent_completion_discipline_0.md` | OPERATIONAL — prepend to implementation handoffs |
| `docs/tests/agent_completion_discipline_0_results.md` | OPERATIONAL landing report |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER update |

## Known gaps

None for this operational rung.

## Deferred next rung

Apply this discipline on every subsequent SimThing implementation/remediation handoff.

## DA status

**N/A** — operational discipline only; not a product evidence rung; not DA-promoted.