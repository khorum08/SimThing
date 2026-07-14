# HD-DOCS-CASCADE-0 — results

**Status:** PROBATION (implementer). DA authors the graduation stamp at merge (HD ruling 6).
**PR / branch / merge:** branch `cursor/hd-docs-cascade-0`; PR <pending>; merge <pending>.
**HD-RECEIPT:** ed12c8f71f66 · **ORIENT-RECEIPT:** ada87881548c (role coding)
**tested_code_sha:** <branch head — bound in the relay/PR body>

## What changed (docs-only; no crates/**, no script-logic)
- `agent_onboarding.md`: each tier section now LEADS with its HD ingress line ("handoffs arrive as HD
  projections — render yours; 'approved, implement'"); the pre-landed HD Board per-tier prompt-protocol
  bullets folded into those tiers — one operator-protocol home, zero duplication.
- `agents.md`: router gains a **Handoffs (HD)** entry (render-per-role + `HD-RECEIPT`; pointers to schema/board).
- `ci_screening_surface.md`: §2 engines table gains `handoff_dispatch.sh` + `librarian.sh` HD rows
  (handoff lint · `HD-RECEIPT` drift · sticky ingress/board sync · stewardship), within the 525 cap.
- `handoff_template.md`: compressed **360 → 112** to schema + authoring rules; required-reading blocks and
  restated-doctrine essays deleted (anchors carry doctrine); anti-reaccretion header prepended; frontmatter
  schema matches `handoff_dispatch.sh` required keys; `§10b/§10c/§11/§H` labels preserved for external pointers.
- `doc_budget_baseline.tsv`: `handoff_template.md` cap `364 → 112` (hard anti-reaccretion tripwire).

## Prose-delta (git numstat vs master; touched guidance docs)

| file | added | deleted | net |
|---|---|---|---|
| docs/agent_onboarding.md | 7 | 6 | +1 |
| docs/agents.md | 3 | 0 | +3 |
| docs/ci_screening_surface.md | 2 | 0 | +2 |
| docs/handoff_template.md | 77 | 325 | −248 |
| **total** | **89** | **331** | **−242** |

`doc_budget_baseline.tsv` is data (1/1, net 0). HD-CLOSEOUT-0 net-decrease binding is satisfied on this rung alone.

## Load-bearing proofs (+ what each catches)
- `doc_budget_check.sh --check` — template cap 112 live + all caps held (catches re-fattening / cap regression).
- `gen_orientation.sh --check` — generated orientation fresh after the ladder stamp (catches hand-edit drift).
- `relay_lint.sh --selftest` (36 fixtures) — relay/receipt/anchor-ack grammar intact (catches routing drift).
- `agent_scan.sh` — RELIABLE clean on the delta (catches doctrinal scan violations).
- `anchor_check.sh --check` — no touched doc is anchored; the `ORIENT-RECEIPT` rule stamp is unchanged.

## Scope Ledger
Specified = implemented. No `crates/**`, Studio/UI, script-logic, or new tables touched. `§10b/§10c/§11/§H`
external pointers preserved; older design-doc pointers to the deleted template essays resolve to the
anchored doctrine they cited.

## Known gaps / next
DA deep pass → graduation stamp at merge; then HD-C closeout measures median ingress and confirms net decrease.
