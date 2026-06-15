# BH-1 choke readout — test results

**Rung:** BH-1 GPU-resident choke readout (consumption deferred to BH-1R)  
**Date:** 2026-06-11  
**PR:** BH-1: expose GPU-resident choke readout

## Candidate-F sqrt audit (BH-0 + BH-1 hot path)

Searched `structured_field_stencil.wgsl` variant 7 branch and BH saturating-flux CPU oracle (`sigma_u` … `cpu_saturating_flux_step`) for: `sqrt`, `length`, `distance`, `normalize`, `hypot`, `magnitude`, `norm(`.

**Result:** no forbidden tokens in authoritative BH production paths. Choke readout uses `1 − C/χ` (linear ratio, manual clamp). No `m_jit_sqrt_f_exact` routing required.

Guard test: `bh1_no_native_sqrt_in_hot_path` (`crates/simthing-gpu/tests/bh1_choke_readout.rs`).

## Implementation summary

| Surface | Change |
|---|---|
| `RegionFieldOperatorSpec::SaturatingFlux` | optional `choke_output_col: Option<u32>` |
| WGSL variant 7 | when enabled, writes `1 − c_i/χ` to `target_col_y` |
| CPU oracle | mirrors choke write (test-only parity) |
| Admission | choke col `< n_dims`, `≠ source_col`, strict same-frame sink |

Default-off: `choke_output_col: None` preserves BH-0 behavior.

**Consumption:** not implemented in BH-1. The landed `bh1_consumption_proof_*` test was CPU-side sum/threshold only. GPU consumption is **BH-1R** (`SaturatingFluxChokeThresholdOp`).

## Targeted gates

```text
cargo fmt --all -- --check
cargo test -p simthing-gpu --test bh0_saturating_flux
cargo test -p simthing-gpu --test bh1_choke_readout
cargo test -p simthing-spec --test bh0_saturating_flux_admission
cargo test -p simthing-spec --test bh1_choke_readout_admission
```

## Test matrix

| Test | Result |
|---|---|
| `bh1_no_native_sqrt_in_hot_path` | PASS |
| `bh1_choke_readout_gpu_matches_cpu_oracle` | PASS |
| `bh1_readout_stays_gpu_resident` | PASS |
| `bh1_crowding_increases_choke_readout` | PASS |
| `bh1_uniform_field_has_neutral_choke` | PASS |
| `bh1_zero_flux_boundary_does_not_create_false_drain` | PASS |
| `bh1_crowded_fixture_choke_oracle_only` | PASS |
| `bh1_admission_rejects_invalid_output_shape` | PASS |
| BH-0 regression (`bh0_saturating_flux`, `bh0_saturating_flux_admission`) | PASS |

## Boundaries preserved

- No border service, pathfinding, movement policy, PALMA coupling, or stored C field.
- CPU oracle and compact readback are test/diagnostic only.
- `cargo test --workspace` not run.
