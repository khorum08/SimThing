# MOBILITY-RUNTIME-1A-R1 — production-fixture boundary verification results

Date: 2026-06-02

## Verdict

**PASS WITH NARROWING**

RUNTIME-1A is green only as a **`simthing-spec` CPU-only default-off production-fixture model**
(`MobilityRuntime1aSimSessionSurface`). It does **not** wire `simthing-driver`, `simthing-gpu`, or any
production runtime crate. Actual production runtime crate fixture wiring is a **separate, currently-closed
gate**: **MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE**
(`mobility_runtime1a_runtime_crate_fixture_closed`). **RUNTIME-1B** GPU pass-graph registration remains
closed.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_runtime1a.rs` — boundary report fields + closed runtime-fixture gate constant
- `crates/simthing-spec/src/designer_admission/mod.rs` — export
- `crates/simthing-spec/src/lib.rs` — export
- `crates/simthing-spec/tests/mobility_runtime1_production_fixture.rs` — boundary lock tests (+2)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime1a_r1_results.md`

## Boundary Finding

| Surface | Present in tree? |
| --- | --- |
| `simthing-spec` production-fixture model (`MobilityRuntime1aSimSessionSurface`) | **Yes** — authorized RUNTIME-1A |
| Actual `simthing-driver` / production runtime crate `SimSession` wiring | **No** — closed follow-on gate |
| GPU pass-graph registration | **No** — RUNTIME-1B closed |

Repo search confirms zero references to RUNTIME-1A symbols in `simthing-driver`, `simthing-gpu`, or other runtime crates. Compatible with `invariants.md` production-wiring rails (default-off, fixture-only, separately gated).

## Exact Status Wording

**MOBILITY-RUNTIME-1A PASS** — CPU-only default-off **`simthing-spec` production-fixture model**; no real runtime crate or GPU pass graph. **MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE** — closed (actual production runtime crate wiring). **RUNTIME-1B** — closed (GPU pass-graph registration).

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo test -p simthing-spec --test mobility_scenario0_admission` | 13 passed |
| `cargo test -p simthing-spec --test mobility_audit0_owner_band_budget` | 8 passed |
| `cargo test -p simthing-spec --test mobility_alloc0_substrate` | 15 passed |
| `cargo test -p simthing-spec --test mobility_reenroll0_substrate` | 16 passed |
| `cargo test -p simthing-spec --test mobility_idroute0_substrate` | 20 passed |
| `cargo test -p simthing-spec --test mobility_econ0_substrate` | 20 passed |
| `cargo test -p simthing-spec --test mobility_owner0_substrate` | 24 passed |
| `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation` | green |
| `cargo check --workspace` | 0 errors |

New boundary tests:

- `runtime1a_declares_fixture_model_not_runtime_crate_wiring`
- `runtime1a_real_simsession_runtime_wiring_remains_absent`

## Posture Attestation

- v7.9 mobility/transfer substrate ladder complete at substrate level.
- RUNTIME-0 green (test-only, default-off composition harness).
- RUNTIME-1A green at `simthing-spec` production-fixture model layer only.
- RUNTIME-1A-RUNTIME-FIXTURE and RUNTIME-1B remain closed.
- No default schedule; no gameplay-facing integration; no semantic/raw WGSL; no CPU planner/urgency/commitment; no Hybrid-Strata/faction-index scaling; no invariant edits.
