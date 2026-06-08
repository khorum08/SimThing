# MOBILITY-IDROUTE-0-R1 - identity-routing substrate hardening results

Date: 2026-06-02

## Verdict

**PASS**

MOBILITY-IDROUTE-0-R1 hardens the landed local D=2 identity-routing substrate before any ECON opening review. The change stays inside `simthing-spec` substrate/admission modeling and tests; it opens no runtime or downstream ladder.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_idroute0.rs`
- `crates/simthing-spec/tests/mobility_idroute0_substrate.rs`
- `docs/tests/phase_mobility_idroute0_r1_results.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`

## Test-Battery Mapping

All authorized checks are explicit tests: 7 substrate floor + 10 guardrails + 3 performance bars = 20 tests.

| Track | Tests | Result |
| --- | --- | --- |
| Substrate floor | `idroute_masked_sum_correct`, `idroute_multi_term_sum_determinism`, `idroute_argmax_packed_key_unique`, `idroute_directed_disburse_correct`, `idroute_directed_disburse_atomic_or_immutable`, `idroute_identity_column_not_tree_structure`, `idroute_cpu_gpu_parity_layout` | PASS |
| Guardrails | `idroute_rejects_global_faction_vector`, `idroute_accepts_many_cells_with_local_k_bound`, `idroute_rejects_one_cell_exceeding_max_factions_per_cell`, `idroute_rejects_owner_as_spatial_parent`, `idroute_rejects_capture_as_reparenting`, `idroute_rejects_econ_owner_runtime`, `idroute_keeps_econ_owner_parked`, `idroute_does_not_authorize_production_simsession_wiring`, `idroute_does_not_enable_default_on_behavior`, `idroute_rejects_semantic_or_raw_wgsl` | PASS |
| Performance bars | `idroute_d2_masked_dispatch_scale`, `idroute_concentration_one_cell`, `idroute_scale_soak_34k` | PASS |

## Admission Hardening

- Local admission is explicit: records are grouped by cell, and each cell must fit `max_factions_per_cell`.
- Many cells with local `k <= 4` lanes are accepted without treating repeated lane ids as a global vector.
- Any one cell exceeding `k` is rejected with `exceeding_max_factions_per_cell`.
- Explicit global-vector requests are rejected with `global_faction_vector`; no broad cross-cell distinct-identity heuristic is used.
- Identity remains a local bounded column, not tree structure or owner-parent semantics.

## Directed Disburse

Directed disburse remains immutable-by-construction in this substrate model: the planner returns a complete report or rejects with no mutable target-state commit path. `idroute_directed_disburse_atomic_or_immutable` covers both the accepted immutable report and the rejected no-partial-output case.

## Commands

```bash
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture Attestation

MOBILITY-IDROUTE-0-R1 is remedial hardening only. ECON and OWNER remain proposed/parked. No production runtime integration, production `SimSession` wiring, GPU kernels, semantic/raw WGSL, default-on behavior, global faction vector, CPU planner, CPU urgency computation, CPU commitment emission, invariant edit, or downstream ladder implementation is introduced.
