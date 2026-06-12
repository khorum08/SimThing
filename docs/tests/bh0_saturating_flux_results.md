# BH-0 — GPU-resident SaturatingFlux operator — Test Results

**Branch:** `codex/bh-0-saturating-flux`  
**Scope:** First BH production rung — conservative saturating-flux stencil with transient register-local C.

## Implemented

| Surface | Location |
|---|---|
| `StructuredFieldStencilOperator::SaturatingFlux { u_sat, chi }` | `simthing-gpu` |
| WGSL variant 7 (`saturating_flux`) | `structured_field_stencil.wgsl` |
| CPU oracle (`cpu_stencil_step`, `cpu_compute_c_at`) | `structured_field_stencil.rs` |
| `RegionFieldOperatorSpec::SaturatingFlux { u_sat, chi }` | `simthing-spec` |
| Admission + compile bridge | `region_field_admission.rs`, `first_slice_mapping_runtime.rs` |

## Production path

- GPU-resident ping-pong dispatch via `StructuredFieldStencilOp`
- Transient C computed in shader registers (13-point 2-hop gather for `C_j`)
- Symmetric flux `0.5 * (C_i + C_j) * (u_j - u_i)` in N,S,E,W order
- Zero-flux boundary (OOB neighbors contribute no flux; factor 1.0 in C products)
- No stored C buffer/column; no second pass
- Opt-in/default-off via explicit `RegionFieldOperatorSpec::SaturatingFlux` admission

## Tests

| Test | Result |
|---|---|
| `bh0_saturating_flux_cpu_oracle_conserves_mass` | PASS |
| `bh0_saturating_flux_gpu_matches_cpu_oracle` | PASS |
| `bh0_zero_flux_boundary_does_not_drain_mass` | PASS |
| `bh0_cj_dependency_uses_two_hop_gather` | PASS |
| `bh0_uniform_field_is_fixed_point` | PASS |
| `bh0_crowding_chokes_flux` | PASS |
| `bh0_invalid_cfl_rejected` | PASS |
| `saturating_flux_clear_field_reduces_to_symmetric_diffusion` | PASS |
| `saturating_flux_admission_*` (spec) | PASS |
| Existing `structured_field_stencil` suite (unchanged) | PASS (30/30) |

## Targeted gates

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test bh0_saturating_flux
cargo test -p simthing-gpu --test structured_field_stencil
cargo test -p simthing-spec --test bh0_saturating_flux_admission
```

`cargo test --workspace` **not run**.

## Boundaries preserved

- No border service, pathfinding engine, movement policy, or semantic WGSL
- No PALMA / ClauseThing / simthing-sim changes
- CPU oracle/diagnostic only — no CPU production boundary checking
- No paper-fidelity claims

## Blockers

None. BH-1 (choke readout column) opens on separate rung.
