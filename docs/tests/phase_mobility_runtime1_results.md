# MOBILITY-RUNTIME-1A — CPU-only production fixture wiring results

Date: 2026-06-02

## Verdict

**PASS**

MOBILITY-RUNTIME-1A implements the narrowed CPU-only, default-off production fixture authorized by
MOBILITY-RUNTIME-1-OPEN-0. The fixture wires the green RUNTIME-0 composition harness into a
`MobilityRuntime1aSimSessionSurface` **model in `simthing-spec`** behind an explicit named gate — not
into production runtime crates. No GPU pass-graph registration, default schedule, or gameplay integration.
Boundary clarification: [`phase_mobility_runtime1a_r1_results.md`](phase_mobility_runtime1a_r1_results.md).

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_runtime1a.rs`
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/mobility_runtime1_production_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime1_results.md`

## Implemented RUNTIME-1A Scope

- Explicit named gate (`mobility_runtime1a_explicit_opt_in_gate`); default-off.
- `MobilityRuntime1aSimSessionSurface` production fixture surface model; default `SimSession` unchanged when gate off.
- Disabled path: deterministic no-op with zero composition invocations.
- Enabled path: delegates to `compose_mobility_runtime0` preserving ALLOC→REENROLL→IDROUTE→ECON→OWNER order.
- CPU-only; no GPU pass-graph registration, no default schedule, no gameplay integration.
- Preserves deterministic replay, CPU/GPU parity proxy, isolated owner overlay delivery, ECON/OWNER separation, hard/soft separation.

## Explicit Non-Goals Preserved

No GPU pass-graph wiring (RUNTIME-1B closed); no default runtime schedule; no gameplay-facing integration; no semantic/raw WGSL; no CPU planner/urgency/commitment; no owner-as-spatial-parent; no capture-as-reparenting; no nested arena reparenting; no default-on Resource Flow; no hard-currency through Resource Flow; no Hybrid-Strata/faction-index scaling; no atlas production runtime; no E-11B-5; no B-1; no ClauseThing/L3; no FrontierV2-5 or ACT/EVENT/OBS/PIPE reopen; no invariant edits.

## Test Battery

| Test | Result |
| --- | --- |
| `runtime1_explicit_opt_in_only` | PASS |
| `runtime1_default_simsession_behavior_unchanged` | PASS |
| `runtime1_registers_named_mobility_composition_fixture` | PASS |
| `runtime1_no_default_passgraph_schedule` | PASS |
| `runtime1_cpu_only_no_gpu_passgraph` | PASS |
| `runtime1_preserves_runtime0_composition_order` | PASS |
| `runtime1_preserves_deterministic_replay` | PASS |
| `runtime1_preserves_cpu_gpu_parity_proxy` | PASS |
| `runtime1_preserves_owner_overlay_isolated_unit` | PASS |
| `runtime1_preserves_econ_owner_separation` | PASS |
| `runtime1_no_hard_soft_silent_mix` | PASS |
| `runtime1_rejects_default_on_behavior` | PASS |
| `runtime1_rejects_semantic_or_raw_wgsl` | PASS |
| `runtime1_rejects_cpu_planner_urgency_commitment` | PASS |
| `runtime1_rejects_owner_as_spatial_parent` | PASS |
| `runtime1_rejects_capture_as_reparenting` | PASS |
| `runtime1_rejects_nested_arena_reparenting` | PASS |
| `runtime1_rejects_default_on_resource_flow` | PASS |
| `runtime1_rejects_hard_currency_through_resource_flow` | PASS |
| `runtime1_rejects_hybrid_strata_or_faction_index_scaling` | PASS |
| `runtime1_rejects_closed_ladder_reopen` | PASS |
| `runtime1_rejects_unscoped_gpu_passgraph_wiring` | PASS |
| `runtime1_34k_production_fixture_soak` | PASS |
| `runtime1_dirty_owner_modifier_steady_state_zero_redisperse` | PASS |
| `runtime1_mobility_churn_with_owner_overlay_and_econ_clearinghouse` | PASS |
| `runtime1_no_default_runtime_cost_when_disabled` | PASS |

## Commands

```bash
cargo test -p simthing-spec --test mobility_runtime1_production_fixture  # 26 passed
cargo test -p simthing-spec --test mobility_runtime0_composition           # 23 passed
cargo test -p simthing-spec --test mobility_scenario0_admission           # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget        # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate             # 16 passed
cargo test -p simthing-spec --test mobility_idroute0_substrate              # 20 passed
cargo test -p simthing-spec --test mobility_econ0_substrate                 # 20 passed
cargo test -p simthing-spec --test mobility_owner0_substrate                # 24 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                     # Finished — ok
```

## Posture Attestation

MOBILITY-RUNTIME-1A is green only for CPU-only, default-off production fixture wiring. RUNTIME-1B
(GPU pass-graph registration) remains a separate, currently-closed later gate. Default runtime
scheduling and gameplay integration remain unopened. Hybrid-Strata/faction-index ECON scaling remains
parked. v7.8 M/E/T closure, AO-WGSL-0 default-off, atlas runtime / E-11B-5 / B-1 / ClauseThing-L3 /
FrontierV2-5 / ACT-EVENT-OBS-PIPE all parked/closed.
