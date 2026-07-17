---
rung: RF-CONSERVATION-ORACLE-0
kind: rung
track: 0.0.8.6
base_sha: 6500c0cdbc764382d83d6af74bdc1120122297ab
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "RF-INTEGRATION sub-track rung RF-1 (Owner ruling: recursive Arena RF becomes the DEFAULT EXECUTED tick source, legacy retired, a FRESH oracle built — once recursive RF executes it can't be its own oracle). RF-1 builds ONLY that oracle, ORACLE-FIRST, before the RF-2 execution flip. Coder=Grok CLI. Orchestrator owns CI/clearance/tree-review/remands + delegated merge; escalations→orchestrator not DA. Engine-math tier: if the derivation needs a kernel/spec primitive that doesn't exist, STOP+DA-route."
surfaces: ["crates/simthing-spec", "crates/simthing-driver", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["flipping execution default / retiring legacy / touching resource_flow_execution_profile (that is RF-2)", "building the oracle on the source-under-test (owner_silo_recursive_rf_source / recursive runtime_rf_tick_source) — circular", "new grammar/kernel/WGSL/GPU primitive; TP-specific golden in a sealed crate"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening-into-execution-flip", "oracle-needs-nonexistent-primitive", "oracle-would-depend-on-source-under-test"]
---

## Status

**PROBATION candidate — implementation and falsification accepted; merge pending exact-head admission.**

`tested_code_sha: 675e1544ecb9746602339629426ec7b13feb10f8`

`coverage_basis: PASS` — independent conservation oracle, actual governed GPU Balance readout, non-zero residual runtime-path falsifier, oracle-derived orphan detection, authored-pack-bound sibling-aware TP golden, and fail-closed adapter proof.

## PR / branch / merge

- PR: `#1408`
- Branch: `coder/rf-conservation-oracle-0`
- Base: `master`
- Merge authority: orchestrator delegated by Owner.
- Merge condition: exact-head Clearance fresh, Doctrine green, Relay Lint PASS, Handoff Ingress PASS, and no semantic remand open.

## What changed

- Added `simthing-driver::rf_conservation_oracle`, an independent closed-form checker for the ADR recipe, allocator, and arena conservation invariants.
- Added actual executed FlatStarOptIn validation against GPU-read allocations and governed Balance deltas.
- Added a paired runtime falsifier that removes only the root governed Balance dispatch while preserving bit-identical allocations and residual arithmetic.
- Added oracle-derived orphan discovery from structural evidence rather than caller verdicts.
- Added the canonical Terran/Pirate sibling-aware Owner aggregate golden in `simthing-workshop`, hydrated from the authored ClauseScript pack.
- Did not flip recursive execution, retire legacy RF, or touch `resource_flow_execution_profile`.

## Load-bearing proofs

- Conservative recipe/allocator/arena observations pass; deliberately non-conservative observations fail.
- Seven equal child weights produce a deterministic non-zero f32 residual within `O(epsilon*n)`.
- The connected GPU run writes a non-zero governed root Balance delta matching that residual within the declared bound.
- The paired disconnected run preserves the same allocations and residual rate, leaves root Balance unchanged, and fails specifically as `ResidualNotIntegrated`.
- Omitting a participant from structural lineage makes the oracle discover the orphan id.
- Altering the canonical TP authored upkeep changes the selected child, sibling sum, and Owner aggregate and breaks the fixed golden.
- Adapter absence fails closed.

## Scope Ledger

- Classification: sealed generic oracle in `simthing-driver`; scenario-flavored golden in `simthing-workshop`.
- Lifecycle: **PROBATION** pending orchestrator merge.
- ORACLE-ONLY fence: held.
- Independence fence: held; `rf_conservation_oracle.rs` imports only `std::collections` and does not import or call the recursive RF source under future test.
- Kernel/WGSL/grammar surface: unchanged.
- Runtime execution default and legacy retirement: unchanged and reserved for RF-2.

## Conformance

- ANCHOR-ACK: founding-ontology-invariants@b960ed2d493d
- ANCHOR-ACK: field-policy-time-decisions@ae2d4c2c0c7d
- ANCHOR-ACK: stead-spatial-contract-core@b4a112cd02e8
- ANCHOR-ACK: property-value-rf-overlays@084ee935326b
- ANCHOR-ACK: one-tree-owners-never-spatial@c88002b72898
- ANCHOR-ACK: structural-execution-convergence@17fa0732f44d
- ANCHOR-ACK: workshop-candidate-homing@3e584f0ad175
- HD-RECEIPT: 9772abd8fcac
- ORIENT-RECEIPT: 46d89a04fc85
- WORKSHOP-HOMING-DETECTION: PASS 0.

## Known gaps

- RF-1 does not execute recursive Arena RF by default; that is RF-2.
- RF-1 does not retire the legacy RF path; that is RF-2/RF-3.
- RF-1 does not close Studio Owner OVL; RF-4 resumes 12.9 after recursive execution exists.
- The TP golden is an independent RF-4 target, not evidence that RF-2 already executes.

## Graduation routing

- CI verdict: PASS at the tested implementation/evidence boundary; exact-head admission must remain green through merge.
- Triage entries: none required; Doctrine delta reported zero hard failures and zero INSPECT.
- Risk class: DA-RESERVE(gate-wiring), deep-tree verification completed by orchestrator.
- Falsification check: PASS — non-zero governed Balance disconnect, non-conservative allocator, orphan omission, authored-pack drift, and NoAdapter all bite.
- Recommended posture: ORCHESTRATOR-GRADUATED on successful exact-head merge; then dispatch RF-2.

## BUILD

- READ FIRST: `docs/adr/resource_flow_substrate.md` §"Conservation policy" + the founding-ontology /
  field-policy / stead anchors you must ACK.
- Build an INDEPENDENT RF **conservation oracle** — a closed-form checker enforcing the ADR's three
  conservation invariants on any arena tree: per-recipe EXACT (`Σ ΔNeed + emit×Σc = 0`); per-allocator
  `|Σ disbursed(I→Cᵢ) − budget(I)| ≤ O(ε·n)` with the residual integrating into the parent Balance via
  existing `governed_by`; per-arena structural (intrinsic + coupling = leaf allocations + Balance +
  emission consumption; no orphan participants). Derive it from the ADR — NOT from the recursive source
  RF-2 will execute.
- VALIDATE it live against the CURRENT executed flat path (`open_from_spec` + `FlatStarOptIn`; ct_2a_intrinsic_flow /
  ct_2c_category_economy): it must agree with admitted RF, zero false positives.
- Author the canonical TP child→ancestor **reduce-up golden** (expected ancestor/Owner aggregate — the RF-4 OVL
  target) analytically; scenario-flavored → `crates/simthing-workshop`.

## FENCES

- ORACLE-ONLY: do not flip execution, retire legacy, or touch `resource_flow_execution_profile` (that's RF-2).
- INDEPENDENCE is load-bearing (the anti-cosplay of this track): the oracle derives the invariants itself and
  does NOT import `owner_silo_recursive_rf_source` or the recursive branch of `runtime_rf_tick_source`. Grep-prove it.
- No new grammar/kernel/WGSL/GPU primitive (RF rides existing AccumulatorOp/`governed_by`); if it can't →
  STOP+DA-route. §12: generic oracle = sealed engine; TP golden = workshop; WORKSHOP-HOMING-DETECTION PASS 0.

## EXIT-PROOF

- The oracle enforces all three invariants and BITES: a non-conservative fixture (a disburse breaking the O(ε·n)
  bound, or an orphan participant) → FAIL; a conservative one → PASS. Paste both. "Any tolerance" passing = fail.
- Validated live vs current flat-opt-in RF (ct_2a/ct_2c): agrees, zero false positives. Independence grep pasted.
- Canonical TP reduce-up golden lands in workshop as the RF-4 target.
- cargo/doctrine-scan(PR-delta)/orient/doc-budget green; tests ledgered (birth_track 0.0.8.6-studio-live-ops) with
  inspect_justifications + SHA-bound triage if TEST-BUDGET INSPECTs. PROBATION LEADS the RF-1 cell + Active-open-rung
  row updated; orientation regenerated.
