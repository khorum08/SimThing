# HD-DOCS-CASCADE-0 — results

**Status:** PROBATION (implementer). DA authors the graduation stamp at merge (HD ruling 6).
**PR / branch / merge:** branch `cursor/hd-docs-cascade-0`; PR [#1340](https://github.com/khorum08/SimThing/pull/1340); merge <pending, DA>.
**HD-RECEIPT:** ed12c8f71f66 · **ORIENT-RECEIPT:** ada87881548c (role coding)
**tested_code_sha:** `1bacc5c74015a65796734f19de8548558d2a75ae` (battery-validated repaired tree; head advances by this sha-binding commit)

## What changed (docs-only; no crates/**, no script-logic)
- `agent_onboarding.md`: each tier section (coding / orchestrator / DA) now **leads** — first line under
  the heading — with its HD ingress paragraph ("handoffs arrive as HD projections — render yours;
  'approved, implement'"); the pre-landed HD Board per-tier prompt-protocol bullets stay **folded** into
  those tiers (one operator-protocol home; the removed duplicate three-bullet block is not restored).
- `agents.md`: router gains a **Handoffs (HD)** entry (render-per-role + `HD-RECEIPT`; pointers to schema/board).
- `ci_screening_surface.md`: §2 engines table gains `handoff_dispatch.sh` + `librarian.sh` HD rows
  (handoff lint · `HD-RECEIPT` drift · sticky ingress/board sync · stewardship), within the 525 cap.
- `handoff_template.md`: compressed **360 → 112** to schema + authoring rules; required-reading blocks and
  restated-doctrine essays deleted (anchors carry doctrine); anti-reaccretion header prepended; frontmatter
  schema matches `handoff_dispatch.sh` required keys; `§10b/§10c/§11/§H` labels preserved for external pointers.
- `doc_budget_baseline.tsv`: `handoff_template.md` cap `364 → 112` (hard anti-reaccretion tripwire).

## Prose-delta (git numstat vs master; all nine changed files at the repaired head)

| file | added | deleted | net |
|---|---|---|---|
| docs/handoff_template.md | 77 | 325 | −248 |
| docs/agent_onboarding.md | 8 | 6 | +2 |
| docs/agents.md | 3 | 0 | +3 |
| docs/ci_screening_surface.md | 2 | 0 | +2 |
| docs/design_0_0_8_4_8_4_hd_board.md | 1 | 1 | 0 |
| docs/orchestrator_orientation.md | 2 | 2 | 0 |
| docs/tests/current_evidence_index.md | 1 | 0 | +1 |
| scripts/ci/doc_budget_baseline.tsv | 1 | 1 | 0 |
| docs/tests/hd_docs_cascade_0_results.md | 69 | 0 | +69 |
| **total (all nine files)** | **164** | **335** | **−171** |

**Guidance-corpus subtotal** (the four onboarding/router/screening/template docs only): 90 added /
331 deleted / **−241 subtotal** — this is a subtotal, not the rung's total proof. The full nine-file
total above is also net-negative; HD-CLOSEOUT-0's net-decrease binding is satisfied on this rung alone.

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

## Cloud env note (reported here, not AGENTS.md — envelope discipline)
The `agent_scan.sh` allowlist scans require `python` on `PATH`; the cloud VM ships only `python3`. Installing
`python-is-python3` fixes it (persists in the VM snapshot). Not appended to AGENTS.md: this rung's declared
`surfaces` do not include `AGENTS.md`, and adding it would widen the otherwise-narrow docs-cascade changed-file
envelope — so the durable note lives on the setup branch's `## Cursor Cloud specific instructions` instead.

## Escalation — ingress sticky render blocked by handoff-object length (orchestrator/DA)
Adding `Rung: HD-DOCS-CASCADE-0` (remand defect 1) makes the resolver **resolve** the handoff
(`handoff_dispatch.sh --resolve-handoff` → `handoffs/HD-DOCS-CASCADE-0.hd.md`, exit 0). But the sticky
**render** then fails `HD-LINT-VERDICT: FAIL(ingress-line-cap)`: the dispatched `.hd` renders a **60-line**
coding projection and `command_render_ingress` prepends 3 header lines (63 > the 60-line sticky cap). Both
levers are **outside this rung's envelope** — `handoffs/**` is not a declared `surface` (editing it re-hashes
`HD-RECEIPT: ed12c8f71f66`) and `scripts/ci/*.sh` logic is a forbidden surface. Escalated to orchestrator/DA:
trim the dispatched `.hd` body/`owner_notes` (re-dispatch → new receipt) or adjust the sticky renderer/cap so
an 80-line-body handoff fits the 60-line sticky. Not fixable by the coding rung without breaching the fence.

## Known gaps / next
DA deep pass → graduation stamp at merge; then HD-C closeout measures median ingress and confirms net decrease.
