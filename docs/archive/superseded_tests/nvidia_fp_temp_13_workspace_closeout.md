# NVIDIA FP temp — Battery 13: workspace closeout (RTX 4080)

**Date:** 2026-06-04 · **By:** Opus (design authority) · **Track:** `docs/nvidia_fp_determinism_test.md`
**Status:** **CLOSED / COMPLETE — full workspace green on the discrete RTX 4080.**

## What Battery 13 resolved

The only blocker after Battery 12 was the full-workspace failure:

```
simthing-spec --test jit_kernel_cohort_preview :: jit_cohort0_distinct_graphs_split
left: ["variant"]  right: ["base"]   (positional assertion)
```

**Design-authority ruling.** `preview_kernel_graph_cohorts` groups via a `BTreeMap` keyed on
`stable_key` and sorts `request_ids` within each cohort — so cross-cohort order is **canonical by
stable_key (deterministic, input-order-independent)**, proven by the sibling test
`jit_cohort0_output_stable_under_request_order_variation`. The base-vs-variant *position* is therefore
**graph-hash-determined, not request/insertion order** — not a contract. The test's hardcoded
`cohorts[0] == ["base"]` was a **stale positional assumption**.

**Fix:** test-only. `jit_cohort0_distinct_graphs_split` now asserts the two distinct graphs split into
separate single-request cohorts **order-insensitively** (sorted membership `{["base"], ["variant"]}`).
The cohort-preview implementation is **unchanged** — its deterministic stable_key ordering is the
intended behavior and stays. No GPU/shader/math/tolerance change; no production behavior change.

## Evidence

```
cargo test -p simthing-spec --test jit_kernel_cohort_preview
  → 7 passed; 0 failed; 0 ignored

cargo test --workspace
  → 60 test binaries; ALL "test result: ok"; 0 failed (longest GPU binaries 286s / 401s = real RTX work)
```

GPU adapter (via the `GpuContext` always-discrete fix): **NVIDIA GeForce RTX 4080 Laptop GPU**.

## Net status of the NVIDIA validation ladder

- Batteries 01–11: priority f32 `GpuVerified` + runtime/exact surfaces — PASS on RTX.
- Battery 07 (stale doc guard; admission ordering), 08 (missing doc include), 11 (descriptor compile
  skew) — **Resolved by Battery 12.**
- Battery 12 last blocker (cohort ordering) — **Resolved by Battery 13 (this file).**
- **Full `cargo test --workspace` green on the discrete RTX 4080.** Adapter-scope caveat **lifted**.
- **Ladder COMPLETE.** No false bit-exact claims (f32 paths remain `GpuVerified`; integer/exact paths
  remain exact); no production default wiring, no gameplay/resource-flow/CPU-planner changes.

> Performance note (unchanged): this ladder validates **correctness on the RTX**, not throughput. Real
> GPU performance still needs a separate timestamp-query-backed benchmark track (resident buffers,
> batched multi-location/owner workloads) — wall-clock cargo runtime is not a GPU perf metric.
