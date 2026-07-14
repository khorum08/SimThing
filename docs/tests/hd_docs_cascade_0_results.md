# HD-DOCS-CASCADE-0 — results

**Status:** PROBATION (implementer). DA authors the graduation stamp at merge (HD ruling 6).
**PR / branch / merge:** branch `cursor/hd-docs-cascade-0`; PR [#1340](https://github.com/khorum08/SimThing/pull/1340); merge <pending, DA>.
**HD-RECEIPT:** eaf1e09dc42e (supersedes ed12c8f71f66 via DA remedial #1341 @ c5cdfb41) · **ORIENT-RECEIPT:** ada87881548c (coding)
**tested_code_sha:** `14fa02fa661ef7e502e6e49c520ccdfd92378276` · **coverage_basis:** PASS · **ci_green:** PASS

## What changed (docs-only; no crates/**, no script-logic)
- `agent_onboarding.md`: each tier (coding/orchestrator/DA) leads — first line under its heading — with its HD ingress paragraph; HD Board per-tier prompt bullets folded (zero duplication).
- `agents.md`: router gains a Handoffs (HD) entry. `ci_screening_surface.md`: §2 engines table gains `handoff_dispatch.sh` + `librarian.sh` HD rows (within the 525 cap).
- `handoff_template.md`: compressed 360→112 to schema + authoring rules (required-reading/restated-doctrine deleted); anti-reaccretion header; schema matches `handoff_dispatch.sh` keys; `§10b/§10c/§11/§H` labels kept.
- `doc_budget_baseline.tsv`: template cap 364→112. HD-5 ladder PROBATION-stamped in-diff; evidence-index line; orientation regenerated (generator, not hand-edited).

## Prose-delta (git numstat vs origin/master; all nine changed files, final tree)

| file | added | deleted | net |
|---|---|---|---|
| docs/handoff_template.md | 77 | 325 | −248 |
| docs/agent_onboarding.md | 8 | 6 | +2 |
| docs/agents.md | 3 | 0 | +3 |
| docs/ci_screening_surface.md | 2 | 0 | +2 |
| docs/design_0_0_8_4_8_4_hd_board.md | 1 | 1 | 0 |
| docs/orchestrator_orientation.md | 1 | 1 | 0 |
| docs/tests/current_evidence_index.md | 1 | 0 | +1 |
| scripts/ci/doc_budget_baseline.tsv | 1 | 1 | 0 |
| docs/tests/hd_docs_cascade_0_results.md | 42 | 0 | +42 |
| **total (all nine files)** | **136** | **334** | **−198** |

Guidance-corpus subtotal (four onboarding/router/screening/template docs): 90 / 331 / **−241** (subtotal, not the rung total). Net-negative; HD-CLOSEOUT-0 binding satisfied on this rung.

## Load-bearing proofs (+ what each catches)
- `doc_budget_check.sh --check` — template cap 112 live + all caps held (re-fattening / cap regression).
- `gen_orientation.sh --check` — generated orientation fresh after the ladder stamp (hand-edit drift).
- `relay_lint.sh --selftest` (36 fixtures) — relay/receipt/anchor-ack grammar intact (routing drift).
- `agent_scan.sh` — RELIABLE clean on the delta (doctrinal scan violations).
- `handoff_dispatch.sh --render-ingress` — exit 0, 50 lines, `HD-RECEIPT: eaf1e09dc42e` (ingress-line-cap resolved).

## Scope Ledger
Classification: gate-wiring. Specified = implemented. No `crates/**`, Studio/UI, `scripts/ci/*.sh` logic, or new tables touched. `§10b/§10c/§11/§H` external pointers preserved; anchor `orientation-harness-core` ACKed.

## Known gaps / next
Ingress-line-cap escalation **RESOLVED** by DA remedial #1341 (compacted `.hd`; `HD-RECEIPT: eaf1e09dc42e`; render now 50 ingress lines, exit 0). Next: DA deep pass → DA-authored graduation stamp → merge; then HD-C closeout measures median ingress and confirms net decrease.
