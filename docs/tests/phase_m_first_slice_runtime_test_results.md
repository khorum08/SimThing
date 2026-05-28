# Phase M-first-slice — Runtime Wiring + Boundary/Budget Probe — Test Results

**Date:** 2026-05-19  
**Final commit SHA:** `447adc1`

## Decision gate summary

| Test | Area | PASS / PARTIAL / FAIL | Key result |
|---|---|---|---|
| 0 | Guardrail sanity | **PASS** | Default Disabled; no atlas/active-mask in hot path; simthing-sim map-free |
| 1 | RON → runtime | **PASS** | Compile preview + budget estimate; atlas request still rejects |
| 2 | GPU stencil execution | **PASS** | 10×10 SourceCappedNormalized H=8; finite interior gradient |
| 3 | Edge boundary nullification | **PASS** | Corner/edge/center seeds; H=1,4,8; GPU/CPU parity ≤0.0001; no wraparound |
| 4 | FieldScheduler | **PASS** | Dirty dispatches; clean skips; false_skips=0 |
| 5 | Layer 2 Sum | **PASS** | Parent threat column matches manual SlotRange Sum oracle |
| 6 | Layer 3 EvalEML | **PASS** | field_urgency finite; higher weight → higher urgency |
| 7 | No-readback hot path | **PASS** | readback_values=false; values=None in report |
| 8 | Default-off | **PASS** | Disabled profile no dispatch; SparseRegionFieldV1 dispatches |
| 9 | Determinism | **PASS** | Identical reduction + EML across replay sessions |
| 10 | Budget estimator | **PASS** | SingleGrid 1.0×; PhysicalGutter N=10/H=8 ≈6.76×; over-budget rejects |

## Algebraic implication (Test 3)

The first-slice edge-boundary test is **not** atlas masking, but it confirms the same generic boundary principle at the single-grid edge: invalid neighbor contributions are nullified by generic `BoundaryMode::Zero` boundary math, not semantic map logic.

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/region_field_budget.rs` | **New** — VRAM budget estimator |
| `crates/simthing-spec/src/compile/region_field_admission.rs` | Budget validation on compile preview |
| `crates/simthing-spec/src/spec/region_field.rs` | Optional `max_region_field_vram_bytes` |
| `crates/simthing-driver/src/first_slice_mapping_runtime.rs` | **New** — opt-in first-slice runtime |
| `crates/simthing-driver/src/field_scheduler.rs` | `regions_mut()` helper |
| `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` | **New** — 11 integration tests |
| Docs (production plan, guidance, workshop state, todo, worklog, ADR landing note) | Status updates |

## Docs-only confirmation

**No** — production Rust landed (runtime module + budget estimator). **No** WGSL changes. **No** atlas. **No** M-4A atlas masking. **No** session.rs default wiring.

## Commands run

```text
git rev-parse HEAD
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture
cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture
cargo check --workspace
cargo test --workspace
```

## Final verdict

**PASS — First-slice mapping runtime landed behind explicit opt-in; single-grid algebraic boundary parity and designer-facing RegionField budget preview are green; no atlas, active mask, perception, or production M-4A implementation landed.**
