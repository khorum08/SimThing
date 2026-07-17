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
