# Phase M First-Slice Vertical Proof Parking — Test Results

Date: 2026-05-28

## Base

- Base HEAD: `29e95246865cea4c8672c19f2aa5c72c3b18e7b7` (FirstSliceScenarioSpec-R1 merge)
- Final commit SHA: not known at report authoring; branch commit/merge records the final SHA.

## Files Changed

- `docs/reviews/phase_m_first_slice_vertical_proof_review_packet.md` (created)
- `docs/accumulator_op_v2_production_plan.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/tests/phase_m_first_slice_vertical_proof_parking_test_results.md`

No runtime code changes.

## Review Packet Created

[`docs/reviews/phase_m_first_slice_vertical_proof_review_packet.md`](../reviews/phase_m_first_slice_vertical_proof_review_packet.md)

Summarizes for Opus/product review:

1. What the vertical proof proves and does not prove
2. Landed chain (scenario RON → GPU-resident mapping → FIELD_POLICY commitment)
3. Evidence table (9 prior test reports + this parking pass)
4. Active code surfaces (production vs acceptance-test support)
5. Default-off / opt-in posture and M-4 atlas boundary
6. Known queue-write scale caveat
7. Recommended next options (A first; then C or D; not E yet)

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS; pre-existing workshop report dirt only |
| `git rev-parse HEAD` | PASS; `29e95246865cea4c8672c19f2aa5c72c3b18e7b7` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 2/2 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (~280s) |

No Rust test assertion failures. No GPU/device loss or cargo artifact format errors observed.

## Posture Summary

Phase M first-slice vertical proof parked for Opus/product review.
The landed chain now covers scenario-level RON authoring with explicit MappingExecutionProfile, RegionFieldSpec, CommitmentSpec, GPU-resident field propagation, parent reduction, field_urgency EvalEML, and Threshold + EmitEvent commitment.
No additional runtime behavior landed in this parking pass.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Known Caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

## Final Verdict

PASS — Phase M first-slice vertical proof parking completed; review packet created for Opus/product, existing GPU-resident scenario/commitment path remains green, and no atlas, semantic WGSL, source_mask, perception, map residency, default SimSession wiring, or CPU-side AI planning was introduced.
