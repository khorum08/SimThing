# S-2 Legacy Intensity Sunset Inventory

**Status:** Preparation only — deletion not authorized in C-8 completion gate PR.  
**Prerequisite:** C-8 complete (C-8a–C-8d + remedial hardening); `use_accumulator_intensity` default-on + 7 days CI green before S-2 deletion.

---

## Production replacement

| Legacy | Replacement |
|--------|-------------|
| `intensity_update.wgsl` Pass 2 | C-8b AccumulatorOp `EvalEML` intensity session |
| `Pipelines::run_intensity_update` | `encode_intensity_eml_into` in tick pipeline |
| `intensity_params` GPU buffer | EML program table + intensity EvalEML ops at boundary sync |
| `IntensityParams` / property-level params buffer | `IntensityBehavior` → `compile_intensity_behavior_to_eml` at boundary |

When `use_accumulator_intensity = true`, legacy intensity must not dispatch (guarded by `legacy_intensity_dispatch_count()` in tests).

---

## Files to delete in S-2

| Path | Notes |
|------|-------|
| `crates/simthing-gpu/src/shaders/intensity_update.wgsl` | Legacy Pass 2 kernel |
| `crates/simthing-gpu/src/passes.rs` — intensity block | `intensity_layout`, `intensity_pipeline`, `run_intensity_update`, legacy branches in `run_tick_pipeline_with_accumulators` |
| `crates/simthing-gpu/src/passes.rs` — test module intensity tests | Oracle tests for legacy WGSL (rewrite or delete) |

### Conditionally delete (verify no other consumers)

| Path | Notes |
|------|-------|
| `WorldGpuState::intensity_params` buffer | Built by `build_intensity_params`; only used by legacy intensity bind group |
| `build_intensity_params`, `IntensityParams` in `world_state.rs` | Remove if buffer deleted |
| `n_intensity_params` field on `WorldGpuState` | Remove with buffer |

**Keep:** `IntensityBehavior`, `compile_intensity_behavior_to_eml`, `intensity_accumulator.rs`, EML registry intensity consumer — these are C-8b production path.

---

## Call sites to remove or rewrite

| Location | Action |
|----------|--------|
| `crates/simthing-gpu/src/passes.rs` | Remove legacy intensity bind group creation, pipeline dispatch branches, `run_intensity_update` |
| `crates/simthing-gpu/src/passes.rs` `[cfg(test)]` | Remove or rewrite `intensity_update_matches_cpu_oracle` and related legacy parity tests |
| `crates/simthing-sim/tests/c8b_intensity_eml_parity.rs` | Keep `run_legacy_intensity` as **oracle only** until S-2; rewrite parity tests to use CPU/golden EML oracle instead of legacy WGSL |
| `crates/simthing-sim/src/boundary.rs` | Remove flag-off path that leaves legacy intensity as production fallback; intensity requires `use_accumulator_eml` |
| `docs/agents.md`, `design_v7.md` | Remove Pass 2 legacy intensity references after deletion |

---

## Tests to rewrite (legacy → CPU/golden oracle)

| Test file | Current oracle | S-2 target |
|-----------|----------------|------------|
| `c8b_intensity_eml_parity.rs` | `run_legacy_intensity` (WGSL) | `intensity_eml_direct_cpu` / `eval_eml_cpu` only |
| `passes.rs` unit tests | `PropertyValue::update_intensity` + WGSL | EML CPU oracle + GPU EvalEML |
| `simthing-workshop/tests/eml_phase5_intensity.rs` | `intensity_update_direct_cpu` | Keep as authoritative CPU reference |

---

## Risks / blockers

1. **`use_accumulator_intensity` default false** — S-2 deletion requires flipping default-on and validating 7+ days CI green per C-8 design.
2. **Oracle coverage** — C-8b parity tests still compare against legacy WGSL; must migrate to EML CPU oracle before deleting WGSL.
3. **`intensity_params` buffer** — Confirm no debug/readback tooling depends on it outside legacy path.
4. **Workshop harness** — `simthing-workshop` references `intensity_update.wgsl` FMA order in comments; update docs only, not production.
5. **No zombie fallback** — Do not add CPU production intensity evaluator; flag-off after S-2 should panic or no-op with clear error, not silently fall back to deleted WGSL.

---

## S-2 PR checklist (future)

- [ ] `use_accumulator_intensity` default `true`
- [ ] Delete `intensity_update.wgsl` and pipeline wiring
- [ ] Remove `intensity_params` buffer if unused
- [ ] Rewrite c8b parity tests to EML CPU oracle
- [ ] Remove `run_legacy_intensity` helper
- [ ] Remove `legacy_intensity_dispatch_count` test counter (no legacy path remains)
- [ ] Update ADR, design_v7, agents.md, workshop_current_state
- [ ] Mark S-2 complete in todo.md
