# Phase M EML-GADGET-1 — Tier-1 Stateless Gadget Library — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `106537b2678259e17ae1f8db7aa91e82f501efe9`
- Branch: `phase-m-eml-gadget-tier1`
- Final commit SHA: `fec7eb2` (merge PR #259)

## Files Changed

- `crates/simthing-spec/src/spec/eml_gadget.rs` — RON `EmlGadgetStackSpec` / tagged `EmlGadgetInstanceSpec`
- `crates/simthing-spec/src/compile/eml_gadget.rs` — registry, compiler, CPU oracles, postfix eval, preview report
- `crates/simthing-spec/src/error.rs` — `SpecError::EmlGadgetAdmission`
- `crates/simthing-spec/src/ron.rs` — `deserialize_eml_gadget_stack_ron`
- `crates/simthing-spec/src/lib.rs`, `compile/mod.rs`, `spec/mod.rs` — exports
- `crates/simthing-spec/tests/eml_gadget_tier1.rs` — 10-test Tier-1 suite
- Docs: production plan, gadget design note, mapping guidance, workshop state, todo, worklog

## Implemented Gadget Surfaces

| Surface | Detail |
|---|---|
| Registry | `EmlGadgetRegistry`, `EmlGadgetKind` — FieldSampler, WeightedAccumulator, SoftStep |
| RON authoring | Tagged `kind` enum; `deserialize_eml_gadget_stack_ron` |
| Compiler | `compile_eml_gadget_stack` → per-gadget nodes + explicit `EmlGadgetCompositionPlan` |
| CPU oracles | `oracle_field_sampler`, `oracle_weighted_accumulator`, `oracle_soft_step` |
| Parity eval | `eval_eml_postfix` (spec-test postfix interpreter for Tier-1 opcodes) |
| Preview | `EmlGadgetPreviewReport` (ids, kinds, node counts, diagnostics) |
| Admission | Unknown/deferred kinds, invalid params, duplicate ids, column bounds, node cap |

## Registry Summary

- **FieldSampler** — `clamp(input/cap, 0, 1)`; `ExactDeterministic`; 5 nodes
- **WeightedAccumulator** — `sum(input_i * weight_i)`; `ExactDeterministic`; 2-input example 8 nodes
- **SoftStep** — `0.5 + 0.5 * u / (1 + abs(u))`, `u = steepness * (x - center)`; `ExactDeterministic`; 17 nodes
- Deferred (rejected): VelocityMonitor, EMA, Acceleration, Hysteresis, Decay

## RON Authoring Summary

Minimal stack RON with tagged `kind` per instance (`FieldSampler`, `SoftStep`, `WeightedAccumulator`), column bindings, and params. Inline fixture in `eml_gadget_tier1.rs` (`TIER1_STACK_RON`).

## Compiler Summary

- Per-gadget postfix `EmlNode` templates using existing opcodes only (`SLOT_VALUE`, `LITERAL_F32`, `ADD`, `SUB`, `MUL`, `DIV` safe, `ABS`, `CLAMP_BOUNDED`, `RETURN_TOP`)
- Stack admission enforces duplicate-id rejection, column bounds, finite params, non-empty WeightedAccumulator inputs, matching input/weight counts, and total stack node count ≤ `MAX_EML_TREE_NODES` (32)
- Inline flatten preview strips intermediate `RETURN_TOP` nodes; chained OrderBand runtime scheduling **deferred**
- **R1 (2026-05-29):** `flattened_nodes` replaced by `EmlGadgetCompositionPlan`; multi-gadget stacks use `PerGadgetOnly`; see [`phase_m_eml_gadget_tier1_r1_hygiene_test_results.md`](phase_m_eml_gadget_tier1_r1_hygiene_test_results.md)

## CPU Oracle Parity Summary

All three Tier-1 gadgets: compiled postfix evaluation matches CPU oracle within `1e-6` tolerance (exact for FieldSampler boundary cases). Stack composition (FieldSampler → SoftStep → WeightedAccumulator) matches manual oracle chaining.

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS; `106537b2678259e17ae1f8db7aa91e82f501efe9` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture` | PASS; 10/10 |
| `cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_resource_economy_authoring_ergonomics -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_economy_field_policy_product_fixture -- --nocapture` | PASS; 6/6 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | **OMITTED** (time constraint; targeted regressions + workspace check green) |

## Pass/Fail Table

| Test | Result |
|---|---|
| 1 — registry contains Tier-1 gadgets | PASS |
| 2 — FieldSampler oracle parity | PASS |
| 3 — WeightedAccumulator oracle parity | PASS |
| 4 — SoftStep oracle parity | PASS |
| 5 — RON gadget stack admits | PASS |
| 6 — stack composition parity | PASS |
| 7 — unknown gadget rejects | PASS |
| 8 — invalid params reject | PASS |
| 9 — deferred temporal gadgets rejected | PASS |
| 10 — posture preservation | PASS |
| Regressions | PASS |

## Posture Summary

- No new WGSL files for gadgets
- No per-gadget GPU kernel
- No new EML opcode
- No exp/logistic/transcendental opcode
- `simthing-gpu` unchanged (generic interpreter)
- `simthing-sim` has no Gadget/Personality types
- `MappingExecutionProfile::default()` remains `Disabled`
- Resource Flow E-11 remains default-off
- Economy→FIELD_POLICY remains fixture-only
- No production economy→mapping bridge
- No `DailyResolutionBoundary`
- No default SimSession mapping wiring
- No atlas/M-4A work
- `day_index` / `ticks_per_day` unchanged

## Deferred Items

- EML-GADGET-2 temporal-memory slice (VelocityMonitor, EMA, hysteresis, bounded feedback)
- Chained multi-gadget OrderBand runtime scheduling (V1: compile-plan preview + manual oracle chain only)
- Full designer RON surface integration into GameModeSpec / scenario wiring
- Runtime gadget stack execution path (compile-to-nodes only in this slice)

## Final Verdict

**PASS — Phase M EML-GADGET-1 landed; Tier-1 stateless EML gadgets now compile in simthing-spec to existing EvalEML node templates with mandatory CPU-oracle parity, while preserving no new WGSL, no new EML opcode, no runtime economy behavior changes, no production economy→mapping bridge, no DailyResolutionBoundary, no semantic simthing-sim changes, no Resource Flow default, no atlas/default mapping wiring, and simthing-sim map-freedom.**
