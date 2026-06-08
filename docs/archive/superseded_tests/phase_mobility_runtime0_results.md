# MOBILITY-RUNTIME-0 - default-off substrate-composition harness results

Date: 2026-06-01

## Verdict

**PASS / test-only composition harness.**

MOBILITY-RUNTIME-0 implements the narrowed harness authorized by MOBILITY-RUNTIME-0-OPEN-0. It composes the completed v7.9 substrate ladder in order:

```text
ALLOC -> REENROLL -> IDROUTE -> ECON -> OWNER
```

The harness lives in `simthing-spec` designer admission/test surfaces, is invoked only by explicit opt-in config, and is default-off. It calls the existing substrate planners and composes their reports/checksums; it does not duplicate substrate logic.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_runtime0.rs`
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/mobility_runtime0_composition.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime0_results.md`

## Implemented Scope

- Explicit opt-in/default-off harness config.
- Deterministic canonicalization of composed fixture inputs before substrate execution.
- Ordered composition of ALLOC, REENROLL, IDROUTE, ECON, and OWNER substrate reports.
- Composed CPU/GPU-proxy checksum surface.
- Report fields proving no `SimSession` pass-graph wiring, no production runtime integration authorization, and no GPU hook/pass graph.
- Regression coverage for movement own-column discipline, capture as owner-column flip, isolated owner-overlay delivery, ECON/OWNER separation, hard/soft band separation, DirtyOnly zero redisperse, and integrated 34k soak.

## Non-Goals Preserved

No production `SimSession` wiring, default session pass graph, GPU pass graph, GPU runtime hook, production gameplay integration, default-on behavior, semantic/raw WGSL, designer-authored shader code, CPU planner, CPU urgency, CPU commitment emission, owner-entity spatial parent, capture-as-reparenting, nested arena reparenting, default-on Resource Flow, hard-currency-through-Resource-Flow, Hybrid-Strata/faction-index scaling, atlas production runtime, E-11B-5, B-1, ClauseThing/L3, FrontierV2-5, ACT/EVENT/OBS/PIPE reopen, or invariant edit.

## Test Battery

`cargo test -p simthing-spec --test mobility_runtime0_composition`

- Substrate-integration floor: 10/10 passed.
- Guardrails: 10/10 passed.
- Performance/soak bars: 3/3 passed.
- Total: 23/23 passed.

## Commands

```bash
cargo test -p simthing-spec --test mobility_runtime0_composition
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_econ0_substrate
cargo test -p simthing-spec --test mobility_owner0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture

RUNTIME-0 is green only as a test-only, default-off substrate-composition harness. Real production `SimSession`/GPU pass-graph wiring remains a separate, currently closed later gate. Hybrid-Strata/faction-index ECON scaling remains parked.
