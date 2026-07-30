---
rung: THRESHOLD-EVENT-REGRESSION-REPAIR-0
kind: rung
track: 0.0.8.7
base_sha: 7278bea541e897a51244c975f54ff62ff2f05258
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "DA opened 5.3c at Board 5125362601. Std Grok lane; orchestrator self-routes under 5107557316 and 5125465810. Repair cause, never golden; STOP if 5.2 semantics are implicated."
surfaces: ["crates/simthing-sim/src", "crates/simthing-kernel/src/accumulator_op/session.rs", "crates/simthing-kernel/src/passes.rs", "crates/simthing-kernel/src/world_state.rs", "crates/simthing-kernel/src/sealed/anchor_table.rs", "crates/simthing-kernel/src/shaders/anchor_table_maintain.wgsl", "crates/simthing-driver/src/hosted_property_observation.rs", "crates/simthing-driver/src/session.rs", "crates/simthing-driver/tests", "scripts/ci/test_inventory.tsv", "scripts/ci/triage_log.tsv", "scripts/ci/track_closeout.sh", "scripts/ci/authorized_renames.tsv", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md", "docs/tests"]
forbidden: ["edit the CPU golden or expected event count/payload", "change 5.2 BandCrossingDelta, exact-remap, direction, or fused write-door law", "synthetic event injection, capacity masking, alternate readback, or second authority", "weaken 5.3 sole anchor-table observation or 5.3b admission/totality", "5.4-5.8, Phase 6 transport, unrelated EML/WGSL, or unlisted production scope"]
required_checks: ["workspace build; core and kernel", "adapter-pinned sim, driver, and mapeditor full batteries", "focused s6_threshold_events_match_cpu_golden", "unmodified 5.2 write-door and 5.3 anchor-table referees", "5.3b totality, RF-1, and replay determinism", "inventory and deletion guards; orientation/doc-budget/anchors", "bypass census; agent/doctrine scans; ingress; exact-head clearance"]
stop_conditions: ["stale-orient-receipt or scope-widening", "diagnosis implicates 5.2 graduated band-crossing semantics", "repair requires golden change, synthetic/second event path, or alternate authority", "full crate matrix exposes an unrelated semantic regression"]
---
## BUILD
- Reproduce `s6_threshold_events_match_cpu_golden` on the pinned adapter: GPU emits zero events while the unchanged CPU golden expects one. Confirm DA's boundary: PASS `d41a079bc680daca696137c9ab5329b961d42330`, FAIL `d9544c52`.
- Trace direct-drive preconditions and production ordering from previous/current values through the 5.2 fused threshold scan and 5.3 anchor-table maintenance/readback. Classify the cause as production regression or stale fixture precondition.
- Repair the cause, never the golden: make the smallest production ordering/buffer/dispatch correction, or correct only stale fixture setup so it exercises the unchanged contract.
- Preserve 5.2/5.3/5.3b. RENAME AUTHORITY (DA-prescribed; the free-form note bypass is REJECTED — a successor row must never self-authorize): REVERT the note-matching change to `track_closeout.sh` and instead add `scripts/ci/authorized_renames.tsv` with header `old_identity	new_identity	file	authorizing_ruling	rung` and exactly ONE row: `canonical_tp_gpu_table_matches_25_anchored_0_unobserved` -> `canonical_tp_gpu_table_matches_admission_totality`, file `crates/simthing-driver/tests/anchor_table_surface_0.rs`, ruling `5126261563-DA`, rung 5.3c. The deletion guard treats a removal as non-deletion ONLY when the ledger binds that exact old identity to a new identity that is PRESENT in the diff and rowed in `test_inventory.tsv`; unmatched old name, absent new name, or missing ledger row all still FAIL. Retain the corrected body; never reintroduce the retracted 25/0 law. The referee failure `hardening_clause_load_requests_bridge_reset` is DA-repaired on master (#1508) — the full mapeditor matrix must be green at your head with no carve-out.
- Land one signal-only results doc; stamp 5.3c PROBATION, regenerate orientation, and keep the pointer on 5.3c.
## FENCES
- 5.2 BandCrossingDelta/exact-remap semantics are graduated law; any required change is an immediate DA STOP.
- The event must arise from the ordinary production threshold path—no synthetic append, capacity masking, alternate readback, or second authority.
- Preserve 5.3 sole anchor-table observation and 5.3b admission totality. Full-crate batteries authorize no unrelated edits.
- Keep 5.4-5.8 and Phase 6 fenced; no unrelated EML, shader-family, wire, or replay-format change.
## EXIT-PROOF
- The focused test is green with golden unedited: GPU/CPU each emit the exact expected single event. Results record the bisect boundary, causal defect, and repair class.
- Unmodified 5.2/5.3 referees, 5.3b totality, RF-1, and replay remain exact.
- Full pinned matrix is green: core, kernel, sim, driver, mapeditor; report commands, adapter/backend, harness and pass/fail/ignored counts.
- Build, guards, orientation/docs/anchors, scans, ingress, SHA set, and fresh clearance are green. PR stays draft/PROBATION for DA implementation review; no pointer move or 5.4 dispatch.
