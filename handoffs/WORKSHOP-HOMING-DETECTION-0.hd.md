---
rung: WORKSHOP-HOMING-DETECTION-0
kind: rung
track: 0.0.8.6
base_sha: a83f98dd5a4b9f4488b5768f1d3b30f539d6bd75
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Fully-automated: coder = Codex CLI (gpt-5.5 high, full-access). Orchestrator (webchat Codex) bears CI/clearance/doctrine-scan/tree-review + remands directly; DA rules only on the ORCHESTRATOR->DA RELAY. DA CORRECTION (supersedes the earlier RELIABLE framing): RELIABLE scans are whole-tree, and production crates already hold ~1800 game-vocabulary hits, so a RELIABLE detector is infeasible. Owner ruling on the re-scope: HEURISTIC net-new INSPECT (delta-scoped; pre-existing hits auto-suppressed; net-new routes to DA classify-before-merge). One rung at a time."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_selftest.sh", "docs/sanctioned_surface.md", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["editing any crates/** source", "RELIABLE severity (whole-tree; infeasible vs pre-existing hits)", "retiring/altering existing scans (SEMANTIC-WORDS, SPEC-LOWERER-KIND-READ)", "adding generic engine terms (fleet, cohort) to the vocabulary", "scan-runner logic changes", "kernel/WGSL/Studio/UI"]
required_checks: ["doctrine-selftest", "doctrine-scan", "gen-digest-check", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "false-positive-on-neutral-synthetic-fixture", "editing-production-crate-source", "RELIABLE-would-whole-tree-fail"]
---
## BUILD
- Add ONE **HEURISTIC** row `WORKSHOP-HOMING-DETECTION` to `scripts/ci/scans.tsv`, extending §12
  homing detection to catch NET-NEW scenario tokens in sealed crates (pairs with the live
  `workshop-candidate-homing` anchor-ack attestation):
  - severity **HEURISTIC** — the existing delta model scopes HEURISTIC to net-new hits, so the
    ~1800 pre-existing vocabulary hits are auto-suppressed; net-new hits route to **INSPECT**
    (which requires a landed `/triage` row → DA classify-before-merge). NOT RELIABLE: RELIABLE is
    whole-tree (`doctrine_scan.sh` `reliable scope: whole-tree`) and would red every PR immediately.
  - target = every production crate `src` AND `tests` (all workspace crates EXCEPT the
    `simthing-workshop` sandbox), e.g. `crates/simthing-{core,kernel,gpu,feeder,sim,driver,spec,clausething,mapgenerator,mapeditor,tools}/{src,tests}/**/*.rs`.
  - pattern = the existing `SEMANTIC-WORDS` scenario vocabulary `faction|combat|terran|pirate|diplomacy`
    (optionally `raid`). Do **NOT** add `fleet`/`cohort` — they are generic engine terms (`Cohort` is
    a `SimThingKind`). Study the `SEMANTIC-WORDS` row for the exact column/pattern/exclude form.
  - exclude = comments (`^\s*//!`,`^\s*///`,`^\s*//`), `compile_fail`, and `SimThingKind::` (generic
    kind labels are legitimate). Do **NOT** exclude `#[test]`/`assert_` — covering test code is the
    point of this rung, and the delta model (not a test-exclude) is what spares the pre-existing hits.
  - doctrine-ref cites `ci_screening_surface §12` + this rung; promotion-blocker = retire when game
    semantics are spec-boundary-typed only.
- Regenerate `docs/sanctioned_surface.md` via `bash scripts/ci/gen_digest.sh`.
- Add `scripts/ci/doctrine_selftest.sh` coverage for the four behaviors below.
## FENCES
- HEURISTIC / INSPECT only — never RELIABLE (whole-tree → infeasible here). Net-new → INSPECT, never
  a hard FAIL. Do not touch the scan runner (HEURISTIC delta scoping already exists).
- ADDITIVE: do not retire/edit `SEMANTIC-WORDS` / `SPEC-LOWERER-KIND-READ`; overlap on sim/kernel/src
  is fine. Do not touch any `crates/**` source. Keep the vocabulary narrow (no generic terms).
- Neutral synthetic fixtures (`foundry_valley`, `aqueduct_delta`) MUST NOT match. Workshop stays exempt.
## EXIT-PROOF
- Named `doctrine_selftest.sh` fixtures under the delta model, each catching a real regression:
  (a) a NET-NEW scenario-named test (`terran`/`pirate`…) in a sealed crate's `tests/**` → HEURISTIC
  **INSPECT**; (b) the same under `simthing-workshop` → exempt; (c) a neutral-synthetic-fixture generic
  test in a sealed crate → no match; (d) a pre-existing hit OUTSIDE the PR delta → suppressed.
- `doctrine-selftest` green; `gen_digest --check` green (sanctioned_surface regenerated + committed);
  the new scan does NOT fail the whole-tree self-scan (it is HEURISTIC); orientation-check / doc-budget
  green; new fixtures ledgered (birth_track 0.0.8.6-studio-live-ops, harness-fixture family).
- PROBATION LEADS the 12.7 cell + the authoritative Active-open-rung row updated; orientation
  regenerated; DA stamps graduation at merge.
