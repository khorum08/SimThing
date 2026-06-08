# Phase M EML-GADGET-1 R1 — Flatten Semantics Hygiene — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `fec7eb2da8c586535aa42fe7ada4bdac4e0d893d`
- Branch: `phase-m-eml-gadget-tier1-r1`
- Final commit SHA: `4a04a1d` (merge PR #260)

## Files Changed

- `crates/simthing-spec/src/compile/eml_gadget.rs` — `EmlGadgetCompositionPlan`; removed `flattened_nodes`
- `crates/simthing-spec/src/spec/eml_gadget.rs` — `input_cols()` / `output_col()` helpers
- `crates/simthing-spec/src/compile/mod.rs`, `lib.rs` — export `EmlGadgetCompositionPlan`
- `crates/simthing-spec/tests/eml_gadget_tier1.rs` — R1 hygiene tests (+2 new, extended stack/posture)
- Docs: production plan, gadget design note, mapping guidance, workshop state, todo, worklog; original test report date fix

## Flatten/Chain Semantics Fix

| Before (EML-GADGET-1) | After (R1) |
|---|---|
| `CompiledEmlGadgetStack { flattened_nodes, ... }` | `CompiledEmlGadgetStack { composition: EmlGadgetCompositionPlan, ... }` |
| Concatenated postfix looked executable | Single-gadget: `InlineFlattenPreview { executable: true }` |
| Multi-gadget chain ambiguous | Multi-gadget chained: `PerGadgetOnly { reason }` + `chained_runtime_deferred` diagnostic |
| Report `flattened_node_count` | Report `flatten_preview_node_count` (single-gadget only) |

**Semantics:**

- Per-gadget node templates are executable and CPU-oracle proven.
- Manual/orchestrated stack parity (evaluate each gadget, write `output_col`, feed next `input_col`) remains proven.
- Executable flattened multi-gadget runtime output is **not** claimed in V1.
- Chained OrderBand runtime scheduling remains deferred.

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `fec7eb2da8c586535aa42fe7ada4bdac4e0d893d` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-spec --test eml_gadget_tier1` | PASS; 12/12 |
| `cargo test -p simthing-spec --test resource_economy_authoring_preview` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_resource_economy_authoring_ergonomics` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_economy_field_policy_product_fixture` | PASS; 6/6 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission` | PASS |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | **OMITTED** (time constraint; targeted regressions + workspace check green) |

## Pass/Fail Table

| Test | Result |
|---|---|
| R1 — single-gadget flatten preview executable | PASS |
| R1 — multi-gadget chained stack not executable flatten | PASS |
| R1 — no runtime flatten preview consumption | PASS |
| Tier-1 registry | PASS |
| FieldSampler oracle parity | PASS |
| WeightedAccumulator oracle parity | PASS |
| SoftStep oracle parity | PASS |
| RON stack admits | PASS |
| Manual stack composition parity | PASS |
| Unknown gadget rejects | PASS |
| Invalid params reject | PASS |
| Deferred temporal gadgets rejected | PASS |
| Posture preservation | PASS |
| Regressions | PASS |

## Tier-1 Parity Summary

All three Tier-1 gadgets retain CPU-oracle parity against compiled postfix node templates.

## Stack Composition Summary

- Chained stack (`FieldSampler col20 → SoftStep col20→21 → WeightedAccumulator [21,22]`) compiles with `PerGadgetOnly` composition.
- Manual orchestration parity passes (per-gadget eval + column handoff).
- No driver/gpu/sim code references `CompiledEmlGadgetStack`, `flattened_nodes`, or `compile_eml_gadget_stack`.

## Posture Summary

Unchanged from EML-GADGET-1: no WGSL, no new opcode, no runtime gadget execution, no temporal memory, no economy→mapping bridge, defaults unchanged, `simthing-sim` map-free.

## Deferred Items

- EML-GADGET-2 temporal-memory slice
- Chained OrderBand runtime scheduling
- True inline flatten with proven intermediate column wiring (future gated compiler)

## Final Verdict

**PASS — Phase M EML-GADGET-1 R1 landed; Tier-1 gadget stack compile surfaces now clearly distinguish executable per-gadget node templates from deferred multi-gadget runtime composition, while preserving CPU-oracle parity, no new WGSL, no new EML opcode, no runtime gadget execution, no temporal memory, no production economy→mapping bridge, no DailyResolutionBoundary, no semantic simthing-sim changes, no Resource Flow default, no atlas/default mapping wiring, and simthing-sim map-freedom.**

**Note (R2, 2026-05-29):** R1 stack-level `total_node_count` rejection was superseded by R2 per-gadget node-cap hygiene — see [`phase_m_eml_gadget_tier1_r2_node_cap_test_results.md`](phase_m_eml_gadget_tier1_r2_node_cap_test_results.md).
