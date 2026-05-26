# S-2 Legacy Intensity Sunset Inventory

**Status:** **Complete** — legacy `intensity_update.wgsl` and Pass 2 pipeline deleted.  
**Prerequisite:** C-8 complete (C-8a–C-8d + completion gate).

---

## S-2 landed

- `crates/simthing-gpu/src/shaders/intensity_update.wgsl` **deleted**.
- Legacy intensity pipeline, bind group layout, and dispatch branches **removed** from `passes.rs`.
- `IntensityParams` buffer, `build_intensity_params`, `n_intensity_params` **removed** from `WorldGpuState`.
- `IntensityBehavior` → `compile_intensity_behavior_to_eml` → persistent EML table → EvalEML AccumulatorOp is the **only** production intensity path.
- `use_accumulator_intensity` and `use_accumulator_eml` default **true**; disabling intensity with `IntensityBehavior` registered **panics** at boundary validation.
- C-8b parity tests rewritten to CPU/EML golden oracle (`intensity_eml_direct_cpu`); no legacy WGSL oracle.
- C-8 full pipeline integration remains green.

---

## Production replacement (current)

| Deleted (S-2) | Replacement |
|---------------|-------------|
| `intensity_update.wgsl` Pass 2 | C-8b AccumulatorOp `EvalEML` intensity session |
| `Pipelines::run_intensity_update` | `Pipelines::run_accumulator_intensity_eml` / `encode_intensity_eml_into` in tick pipeline |
| `intensity_params` GPU buffer | EML program table + intensity EvalEML ops at boundary sync |
| `IntensityParams` / property-level params buffer | `IntensityBehavior` → `compile_intensity_behavior_to_eml` at boundary |

---

## Files deleted in S-2

| Path | Notes |
|------|-------|
| `crates/simthing-gpu/src/shaders/intensity_update.wgsl` | Legacy Pass 2 kernel |
| `crates/simthing-gpu/src/passes.rs` — intensity block | `intensity_layout`, `intensity_pipeline`, `run_intensity_update`, legacy branches |
| `WorldGpuState::intensity_params` buffer | Removed with `IntensityParams` / `build_intensity_params` |

**Kept:** `IntensityBehavior`, `compile_intensity_behavior_to_eml`, `intensity_accumulator.rs`, EML registry intensity consumer.

---

## Tests

| Test file | S-2 outcome |
|-----------|-------------|
| `c8b_intensity_eml_parity.rs` | CPU/EML golden oracle only |
| `passes.rs` unit tests | Rewritten to `run_accumulator_intensity_eml` |
| `s2_legacy_intensity_sunset.rs` | Default-path, validation panic, shader absent, GPU EvalEML |
| `c8_full_pipeline_integration.rs` | Structural shader-deleted guard (no legacy dispatch counter) |

---

## Follow-ups (non-blocking)

- Update `docs/agents.md` Pass 2 / `IntensityParams` references when agents doc is next revised.
- **S-3** overlay sunset · **S-6** threshold sunset remain next gates.
