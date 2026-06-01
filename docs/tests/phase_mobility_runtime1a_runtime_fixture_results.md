# MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE — driver test/support CPU fixture results

Date: 2026-06-02

## Verdict

**PASS**

Implemented the authorized Tier-1 fast-lane `simthing-driver` test/support CPU-only default-off
fixture delegating to the green `simthing-spec` RUNTIME-1A model. Confined to `tests/support`; not
wired into default lib/session `SimSession` path. **RUNTIME-1B** GPU pass-graph, default schedule,
gameplay integration, and non-test-support production path remain closed.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_runtime1a_fixture.rs`
- `crates/simthing-driver/tests/mobility_runtime1a_runtime_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime1a_runtime_fixture_results.md`

## Implemented Scope

- `simthing-driver` test/support fixture with explicit opt-in/default-off named gate.
- CPU-only bridge delegating to `run_mobility_runtime1a_production_fixture` (no duplicated substrate logic).
- Default `SimSession` lib path unchanged; no GPU pass-graph, default schedule, or gameplay path.
- Disabled path: deterministic no-op with zero composition invocations.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo test -p simthing-spec --test mobility_owner0_substrate` | 24 passed |
| `cargo test -p simthing-spec --test mobility_econ0_substrate` | 20 passed |
| `cargo test -p simthing-spec --test mobility_idroute0_substrate` | 20 passed |
| `cargo test -p simthing-spec --test mobility_reenroll0_substrate` | 16 passed |
| `cargo test -p simthing-spec --test mobility_alloc0_substrate` | 15 passed |
| `cargo test -p simthing-spec --test mobility_scenario0_admission` | 13 passed |
| `cargo test -p simthing-spec --test mobility_audit0_owner_band_budget` | 8 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- RUNTIME-1A spec fixture model remains green.
- RUNTIME-1A-RUNTIME-FIXTURE green at driver test/support layer only.
- Non-test-support default `SimSession` path, RUNTIME-1B GPU pass-graph, default schedule, and gameplay integration remain closed.
- No semantic/raw WGSL; no CPU planner/urgency/commitment; no Hybrid-Strata/faction-index scaling; no invariant edits.
