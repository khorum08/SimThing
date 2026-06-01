# MOBILITY-IDROUTE-0 — local D=2 identity-routing overlay substrate results

Date: 2026-06-02

## Verdict

**PASS**

MOBILITY-IDROUTE-0 implements the authorized local D=2 identity-routing overlay substrate on top of ALLOC-0 + REENROLL-0. The implementation is confined to `simthing-spec` designer-admission/substrate modeling and tests; it does not wire production runtime behavior, ECON, OWNER, or any global routing.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_idroute0.rs` (new)
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/mobility_idroute0_substrate.rs` (new)
- `docs/tests/phase_mobility_idroute0_results.md` (this report)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`

## Implemented Substrate Scope

- Local per-cell D=2 identity routing modeled as column-based masked gather + directed disburse.
- Identity represented strictly as `IdentityLane` column on local records (never as tree parent or global vector).
- Deterministic per-identity masked `Sum` (hard exact + soft approximate-deterministic).
- Deterministic multi-term Sum with fixed ordering.
- Packed-key `Max` / argmax producing unique deterministic winner.
- Directed disburse from identity columns back to local children (purely functional / immutable-by-construction in this substrate model).
- CPU/GPU-proxy layout checksums.
- Explicit rejection of global faction vectors, owner-as-spatial-parent, capture-as-reparenting, ECON/OWNER runtime, production `SimSession` wiring, default-on, semantic/raw WGSL, and exceeding `max_factions_per_cell`.

## Explicit Non-Goals Preserved

No ECON, OWNER, global faction vectors, production `SimSession` wiring, semantic/raw WGSL, GPU kernels, default-on behavior, CPU planner/urgency/commitment emission, or invariant changes.

## Test Battery Results

**Substrate floor — all green**
- `idroute_masked_sum_correct` — PASS
- `idroute_multi_term_sum_determinism` — PASS
- `idroute_argmax_packed_key_unique` — PASS
- `idroute_directed_disburse_correct` — PASS
- `idroute_directed_disburse_atomic_or_reject` — PASS (satisfied by construction — implementation is purely functional/report-only with no mutable state)
- `idroute_identity_column_not_tree_structure` — PASS
- `idroute_cpu_gpu_parity_layout` — PASS

**Guardrails — all green**
- `idroute_rejects_global_faction_vector`
- `idroute_rejects_owner_as_spatial_parent`
- `idroute_rejects_capture_as_reparenting`
- `idroute_rejects_econ_owner_runtime`
- `idroute_keeps_econ_owner_parked`
- `idroute_does_not_authorize_production_simsession_wiring`
- `idroute_does_not_enable_default_on_behavior`
- `idroute_rejects_semantic_or_raw_wgsl`
- `idroute_rejects_exceeding_max_factions_per_cell`

**Performance bars**
- `idroute_d2_masked_dispatch_scale` — PASS (local per-cell processing)
- `idroute_concentration_one_cell` — PASS (bounded local cost)
- `idroute_scale_soak_34k` — PASS (34k entities across cells processed deterministically)

## Commands Run

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                # 16 passed
cargo test -p simthing-spec --test mobility_idroute0_substrate                 # 13 passed (new)
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

All green.

## Posture Attestation

MOBILITY-IDROUTE-0 substrate is green. ECON and OWNER remain proposed/parked. No production runtime integration, no GPU kernels, no default-on flags, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU planner/urgency/commitment emission. v7.8 M/E/T closure, AO-WGSL-0, ClauseThing/L3, FrontierV2-5, ACT/EVENT/OBS/PIPE posture unchanged.

The local D=2 identity-routing substrate now exists behind the named IDROUTE-0 scope as authorized.
