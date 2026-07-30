---
rung: TP-PURGE-0
kind: rung
track: 0.0.8.7
base_sha: 0b1c2e12
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-directed DA draft (not a precedent: orchestration drafts Std-lane handoffs). Executes the Corpus Boundary Law, the Invariant Set, and the Detachability Law. HARD DEADLINE 2026-08-11: the 268 deferred rows reap by default that day, no extension. Stage A/B/C are separately relayable."
surfaces: ["crates/simthing-clausething/src", "crates/simthing-driver", "crates/simthing-mapeditor", "crates/simthing-workshop", "crates/simthing-kernel/tests", "crates/simthing-sim/tests", "scenarios", "scripts/ci", "docs/tests", "docs/design_0_0_8_7_rf_arena_modernization.md"]
forbidden: ["editing a corpus so engine law passes", "reclassifying a row OUT of NONE without a demonstrated planted-defect failure", "extending any expiry date", "renaming a test in place of deleting it", "new bespoke referees", "5.4-5.8 work"]
required_checks: ["detachability_check.sh (+ selftest)", "test_lifecycle_expiry_check.sh --scheduled", "test_inventory_drift_check.sh", "cargo build --workspace", "adapter-pinned driver battery", "doctrine-scan", "clearance"]
stop_conditions: ["stale-orient-receipt", "an engine proof cannot be expressed over inline-constructed input", "de-naming cannot preserve hydrated output", "a deletion would break production"]
---
## BUILD
- STAGE A - REAP. Execute the approved split (`docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv`): 296 PAIR-REAP + 357 NONE delete as PAIRS - test fn AND inventory row together, because the drift gate is bidirectional and only pairs stay green. A row leaves NONE only by DEMONSTRATED planted-defect failure, never by asserting a label. Deleting a test cannot break production; if it can, it was not a test.
- STAGE B - REPLACE. The 222 REPLACE-INLINE rows carry real invariants in the wrong FORM. Collapse them to LOW-SINGLE-DIGIT parametrized inline proofs per (invariant x mechanism) - `cpu-gpu-parity` and `determinism` are TWO LAWS, not 357 tests. Inputs constructed inline; no corpus, fixture, or generator. If your replacement count approaches the row count you have renamed, not replaced: STOP.
- STAGE C - DETACH. Remove both engine proof-coupling dev edges (`driver -> clausething`, `driver -> mapeditor`) and lower `DEV_COUPLING_CEILING` 2 -> 0 in lockstep.
- STAGE C - DE-NAME (value-preserving). `terran_weapon_damage`/`pirate_weapon_damage` (serde-bound fields, mirrored parsed struct, and authored corpus keys move together) become owner-keyed data admitting N sides; `owner == pirate` posture select becomes per-owner authored posture with a default. Hydrated output for TP stays identical.
- STAGE C - DELETE `hydrate_combat_arena.rs` outright with its corpus block: hostless RF-shaped scaffolding doing Phase 8's job early (every transfer carries `host_entity: None`); deleting it also retires 4 of the 7 `Unobserved` properties. Reap the unleased workshop survivor `crates/simthing-workshop/src/tp_rf_reduce_up_golden.rs` (zero `closeout_artifacts.tsv` rows - no clock will ever reach it) and the two TP handoff objects leased under the closed 0.0.8.6 track.
- Stamp 5.9 PROBATION; the pointer moves at the DA stamp, not here.
## FENCES
- A corpus may WITNESS engine law and may never DEFINE it. Never edit a corpus to make a proof pass.
- Deferral is not renewal. No expiry date moves. The 2026-08-11 clock is sovereign over this plan.
- The engine keeps building without the authoring layer at every stage: production coupling stays 0.
## EXIT-PROOF
- `detachability_check.sh`: `production_coupling=0 proof_coupling=0 ceiling=0`.
- Expiry sweep clean after the wave; inventory drift PASS (pairs moved together); every surviving row names an invariant AND has a planted-defect falsifier.
- Zero scenario vocabulary in engine production `src` (the 10.1 hard-fail promotion then locks it).
- THE FALSIFIABLE TEST: delete `scenarios/terran_pirate_galaxy.clause` and the engine still proves itself - no engine referee breaks. Demonstrate it, then restore the corpus.
- Workspace + adapter-pinned driver battery green; corpus green.
