---
rung: WORKSHOP-HOMING-DETECTION-0
kind: rung
track: 0.0.8.6
base_sha: 997bf0663f55169776a0072f1e171ae8ee1aecfe
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Codex CLI (gpt-5.5 high, full-access). Orchestrator (webchat Codex) bears CI/clearance/doctrine-scan/tree-review + remands directly; DA rules only on the final ORCHESTRATOR->DA RELAY. One rung at a time. Owner ruling: RELIABLE hard-FAIL."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_selftest.sh", "docs/sanctioned_surface.md", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["editing any crates/** source", "retiring/altering existing scans (SEMANTIC-WORDS, SPEC-LOWERER-KIND-READ)", "new scan-runner logic if scans.tsv schema suffices", "kernel/WGSL/Studio/UI", "excluding tests from the new scan"]
required_checks: ["doctrine-selftest", "doctrine-scan", "gen-digest-check", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "false-positive-on-neutral-synthetic-fixture", "retiring-an-existing-scan", "editing-production-crate-source"]
---
## BUILD
- Add ONE new `scripts/ci/scans.tsv` row `WORKSHOP-HOMING-DETECTION` converting §12 homing from
  attestation (the `workshop-candidate-homing` anchor-ack) into a mechanical block:
  - severity **RELIABLE** (hard FAIL on net-new delta hits; the existing delta model suppresses
    pre-existing hits outside the PR diff — no repo-wide retroactive fail).
  - target = **every production crate, `src` AND `tests`**: enumerate all workspace crates EXCEPT
    the `simthing-workshop` sandbox, e.g. `crates/simthing-{core,kernel,gpu,feeder,sim,driver,spec,clausething,mapgenerator,mapeditor,tools}/{src,tests}/**/*.rs`.
  - pattern = the game-semantic vocabulary (align with the existing `SEMANTIC-WORDS` set + §12's
    named tokens): faction / combat / terran / pirate / diplomacy / fleet / cohort / raid (word-ish
    boundaries; the existing SEMANTIC-WORDS row is the reference for form).
  - exclude = comments (`^\s*//!`, `^\s*///`, `^\s*//`), `compile_fail`, and `SimThingKind::`
    (spec-boundary role labels are legitimate). Do **NOT** exclude `#[test]` / `assert_` — including
    test code is the entire point of this rung.
  - doctrine-ref cites `ci_screening_surface §12` + this rung; promotion-blocker = retire when game
    semantics are spec-boundary-typed only.
- Regenerate `docs/sanctioned_surface.md` via `bash scripts/ci/gen_digest.sh` (scans.tsv is a digest input).
- Add `scripts/ci/doctrine_selftest.sh` coverage proving the four load-bearing behaviors below.
## FENCES
- ADDITIVE only: do not retire or edit `SEMANTIC-WORDS` / `SPEC-LOWERER-KIND-READ`; overlap on
  sim/kernel/src is acceptable (RELIABLE dominates). Do not touch any `crates/**` source.
- Prefer the existing scans.tsv schema + runner; only if it genuinely cannot express the
  crate-list `{src,tests}` target do you touch the runner — and then STOP+report first.
- Neutral synthetic fixtures (e.g. `foundry_valley`, `aqueduct_delta`) MUST NOT match — a
  false-positive there breaks generic-grammar falsifiers (12.6's pattern) and is a stop condition.
- `simthing-workshop` MUST stay exempt (candidate code is allowed there).
## EXIT-PROOF
- Named `doctrine_selftest.sh` fixtures, each catching a real regression: (a) a scenario-named test
  (`terran`/`pirate`…) in a sealed crate's `tests/**` → RELIABLE **FAIL**; (b) the same construct
  under `simthing-workshop` → exempt/pass; (c) a neutral-synthetic-fixture generic test in a sealed
  crate → pass (no false-positive); (d) a pre-existing hit outside the PR delta → suppressed.
- `doctrine-selftest` green; `gen_digest --check` green (sanctioned_surface regenerated + committed);
  doctrine-scan self-scan / orientation-check / doc-budget green; new fixtures ledgered
  (birth_track 0.0.8.6-studio-live-ops, harness-fixture family).
- PROBATION LEADS the 12.7 cell + the authoritative Active-open-rung row updated; orientation
  regenerated; DA stamps graduation at merge.
