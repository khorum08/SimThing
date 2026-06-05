# Phase M EML-GADGET-1 R2 — Per-Gadget Node Cap Hygiene — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `4a04a1d7e04343c1bfb6c0aaf4b2979af672a845`
- Branch: `phase-m-eml-gadget-tier1-r2`
- Final commit SHA: recorded at merge

## Files Changed

- `crates/simthing-spec/src/compile/eml_gadget.rs` — `apply_stack_node_cap_policy`; per-gadget cap only
- `crates/simthing-spec/tests/eml_gadget_tier1.rs` — R2 node-cap tests (+2)
- Docs: production plan, gadget design note, mapping guidance, workshop state, todo, worklog; R1 report note

## Node-Cap Semantics Fix

| Level | Before (R1) | After (R2) |
|---|---|---|
| Per-gadget template | Reject if `nodes.len() > 32` | Unchanged |
| Single-gadget inline tree | Reject if over cap | Unchanged |
| Multi-gadget stack total | **Reject** if `total_node_count > 32` | **Admit** as `PerGadgetOnly` + `stack_total_exceeds_inline_cap` diagnostic |

The EvalEML node cap (`MAX_EML_TREE_NODES = 32`) applies to each executable gadget/tree, not to the informational total of a `PerGadgetOnly` multi-gadget stack.

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `4a04a1d7e04343c1bfb6c0aaf4b2979af672a845` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-spec --test eml_gadget_tier1` | PASS; 14/14 |
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
| R2 — multi-gadget total over cap admits as PerGadgetOnly | PASS |
| R2 — single gadget over cap rejects | PASS |
| R1 — single-gadget flatten preview executable | PASS |
| R1 — multi-gadget chain PerGadgetOnly + parity | PASS |
| R1 — no runtime flatten consumption | PASS |
| Tier-1 registry + oracle parity (all 3 gadgets) | PASS |
| RON stack admits | PASS |
| Unknown/deferred/invalid reject | PASS |
| Posture preservation | PASS |
| Regressions | PASS |

## Tier-1 Parity Summary

FieldSampler, WeightedAccumulator, and SoftStep retain CPU-oracle parity against compiled postfix node templates.

## Stack Composition / Node-Cap Summary

- Three independent SoftStep gadgets (17 nodes each, 51 total) admit with `PerGadgetOnly` and `stack_total_exceeds_inline_cap`.
- WeightedAccumulator with 9 input pairs (36 nodes) rejects at per-gadget admission.
- Normal chained stack (FieldSampler → SoftStep → WeightedAccumulator) remains `PerGadgetOnly` with manual orchestration parity.

## Posture Summary

Unchanged: no runtime gadget execution, no OrderBand scheduling, no temporal memory, no new opcode/WGSL, defaults unchanged, `simthing-sim` map-free.

## Deferred Items

- EML-GADGET-2 temporal-memory slice
- Chained OrderBand runtime scheduling
- True inline flatten with proven intermediate wiring

## Final Verdict

**PASS — Phase M EML-GADGET-1 R2 landed; node-cap semantics now align with the R1 composition model by enforcing the EvalEML cap on executable per-gadget/single-tree templates while allowing multi-gadget PerGadgetOnly stacks to exceed the single-tree total with diagnostics, preserving CPU-oracle parity, no new WGSL, no new EML opcode, no runtime gadget execution, no temporal memory, no production economy→mapping bridge, no DailyResolutionBoundary, no semantic simthing-sim changes, no Resource Flow default, no atlas/default mapping wiring, and simthing-sim map-freedom.**
