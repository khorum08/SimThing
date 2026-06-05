# FIELD_POLICY tensor/stencil WGSL prototype notes

**Probe date:** 2026-05-19  
**Purpose:** Prototype-only structured 2D field stencil kernel to test whether dense local propagation should be a generic `StructuredFieldStencilOp` rather than per-edge `AccumulatorOp` registrations.

## Kernel design

Single WGSL entry point `stencil_step` in `field_policy_tensor_stencil_prototype.wgsl`.

**Inputs (uniform + buffers):**
- `input_values` / `output_values` — flat row-major `slot × n_dims`
- `width`, `height`, `n_dims`
- `source_col`, `target_col`
- `alpha_self_decay`, `gamma_neighbor`, optional `cap`
- `boundary_mode` — zero (0) or clamp (1)
- `variant` — raw / normalized / directed / clamped / decayed_normalized
- optional `active_mask` — skip inactive cells (prototype only)

**Per-cell update (NSEW unless directed):**
```text
next = alpha * center + gamma * neighbor_sum            (raw)
next = alpha * center + gamma * (neighbor_sum / count) (normalized, decayed_normalized)
next = alpha * center + gamma * (south + east)          (directed)
next = min(cap, max(0, next))                           (clamped)
```

No map, faction, RegionCell, AI, or Resource Flow semantics in WGSL.

## Variant parameter presets (same shader, different uniforms)

| Preserved alias file | variant id | alpha | gamma | notes |
|---|---|---|---|---|
| `field_policy_tensor_stencil_raw_additive_prototype.wgsl` | 0 | 1.0 | 0.8 | blowup baseline |
| `field_policy_tensor_stencil_normalized_prototype.wgsl` | 1 | 1.0 | 0.8 | **recommended** |
| `field_policy_tensor_stencil_decayed_prototype.wgsl` | 4 | 0.8 | 0.2 | stable but weak horizon |
| `field_policy_tensor_stencil_clamped_prototype.wgsl` | 3 | 1.0 | 0.8 | cap=10000, saturates |

Directed variant (2) requires directed setup seeding; not equivalent to NSEW one-hop setup used in horizon tests.

## Harness

Rust prototype: `crates/simthing-gpu/src/field_policy_tensor_stencil_prototype.rs`  
Integration test: `crates/simthing-driver/tests/field_policy_tensor_stencil_wgsl_sandbox.rs`

Not wired into `Pipelines` or production tick path.

## Key findings (summary)

- **Generality:** PASS — flat-buffer structured field math only.
- **Cost:** PASS — projected 30k ~285 ms (normalized, 10×10 measure) vs AccumulatorOp 3236.6 ms dirty-adjusted (~11×); scales to 80–1200× on larger grids (rough projection).
- **Correctness:** PASS on 3×3 CPU oracle (gpu_cpu_max_error=0).
- **Horizon:** PARTIAL — `normalized_stencil` reaches [4][4] with correct gradient at H=8; raw blows up; decayed_normalized too weak at H≤16; directed fails with NSEW setup.
- **Hybrid:** PARTIAL — stencil + SlotRange Sum works; urgency EML needs parent personality columns populated separately.

## Revert

All production paths under `crates/simthing-gpu/src/shaders/field_policy_tensor_stencil_prototype.wgsl`, `field_policy_tensor_stencil_prototype.rs`, and driver test are removed on revert. Preserved copies remain under `docs/workshop/`.
