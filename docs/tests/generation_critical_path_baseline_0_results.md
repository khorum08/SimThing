# GENERATION-CRITICAL-PATH-BASELINE-0 Results

> **Status: PROBATION / proof-present / DA-review-pending.** Coding lane
> only; no merge, graduation, pointer movement, closeout apply, or 14.2+ work.
> Comparator evidence only: wall-clock values are dated facts about one
> reproducibility envelope, never a go/no-go gate or portable CI threshold.

**Date:** 2026-08-31
**Dispatch:** Board `5472690735`
**DA remand:** Board `5473368436` (ruling `5473356953`)
**Prior relay:** `5473146768`
**DA authority:** `5472516590`
**HD-RECEIPT:** `f27b4dbaf650`
**ORIENT-RECEIPT:** `55747d120d2b` (`orientation_rule_stamp=6e41e33143db01ef`)
**Dispatch master / measured production door:** `625290768f266276c68240dc88ffbb1db6de35bb`
**Old instrument head (remand):** `3d55aba823933808b2c837306c47f0bf8c26d3f9`
**tested_code_sha:** see PR head after this remand commit
**ANCHOR-ACK:** `orientation-harness-core@8a365d1c0864`, `scanner-selftest-delta-gate@34fb2662baae`

Lossless artifact: `crates/simthing-workshop/tests/generation_critical_path_baseline_reports.txt`

## Envelope

| Field | Value |
|---|---|
| UTC | `2026-08-31T03:50:28Z` |
| CPU | Intel64 Family 6 Model 183 Stepping 1, GenuineIntel |
| GPU | NVIDIA GeForce RTX 4080 Laptop GPU vendor=0x10de device=0x27a0 type=DiscreteGpu |
| Adapter / backend | Vulkan |
| Driver | NVIDIA 595.79 |
| OS | windows-x86_64 |
| Toolchain | rustc 1.95.0 (59807616e 2026-04-14) |
| Profile | cargo-test (optimized+debuginfo) |
| Seed | `0x00014001` (81921) |
| Authored program | `TransformOp::set(1.0)` (one equal score band; proportional remainder) |
| Command | `cargo test -p simthing-workshop --test generation_critical_path_baseline_0 -- --nocapture` |

GPU adapter was queried for the envelope only. The ordinary generation
critical path is the CPU-host door `clear_constrained_claims_at_generation`.
GPU-to-host readback and host-to-GPU upload are **door-absent**: 0 bytes / 0 ns
(no `map_async`, no `write_buffer`, no queue submit, no GPU clearing kernel).

## Remand D1 / D2 / D3

### D1 — signed remainder

`grant_result_construction` is now `i64`. The prior `.max(0)` clamp is gone.

| | Workload | Value |
|---|---|---|
| Before (PR #1907 @ `3d55aba8`) | `scale_1000` `grant_result_construction` | `min_ns=0` with an explicit `0` sample after clamp (samples included `..., 252100, 0, 210000, ...`) |
| After (this packet) | `scale_10000` `grant_result_construction` | `min_ns=-4397100` (signed remainder retained) |
| After | `divergent_generation_trees` `grant_result_construction` | `min_ns=-4474400` |

Serialization boundary: `LegSamples.sample_ns`, `median_ns`, `p95_ns`, `min_ns`, `max_ns` are `i64`.

### D2 — instrument work is named (shape 2)

Chosen shape: **`shape-2-instrument-legs-inside-e2e`**. The nested restatement and the second uninstrumented clear stay inside the end-to-end envelope and are published as `instrument_restatement` and `neutrality_reclear` with raw samples. Residual = end-to-end minus those named instrument legs plus the production accounted legs. grouping/scoring/sorting/apportionment are components of `instrument_restatement`, not extra e2e work. Arithmetic is not forced-closed.

### D3 — N+1 delay vs next-generation re-clear

| Quantity | Boundary | 1M median ns |
|---|---|---:|
| enclosing host clear | `clear_constrained_claims_at_generation` at current generation | 1,968,425,200 |
| `n_plus_one_launch_delay` | grants-available → construct generation+1 `ClearingRemainderAuthority`; GPU launch 0 | 100 |
| `next_generation_host_reclear` | full ordinary-door re-clear at generation+1 (comparator) | 1,801,596,900 |
| `cpu_schedule_replay_recording` | `record_cleared_grant` after grants; **not** inside N+1 delay (no overlap; next host clear does not require a schedule append) | 274,468,600 |
| residual | e2e − accounted named legs | 1,701,426,700 |

## Host clearing-door census

Mechanical census of every public / re-exported host clearing entry on dispatch
base `62529076`. 14.6 dispositions are recorded, not performed.

| Symbol | Path | Generation authority | Posture | Callers | 14.6 |
|---|---|---|---|---|---|
| `clear_constrained_claims_at_generation` | `crates/simthing-spec/src/spec/constrained_clearing.rs:254` | caller `ClearingRemainderAuthority` | ordinary production CPU-host door | production `growth_entitlement.rs`; wrapper `clear_reduced_owner_channels_at_generation`; tests listed in the census artifact; re-export spec + embedder `run.rs` | narrow behind `CpuVendorizedOracle` |
| `clear_reduced_owner_channels` | `crates/simthing-spec/src/spec/constrained_clearing.rs:420` | generationless: granter raw 0, generation 0 | compatibility shim / test oracle | test `contention_arena_executed_0.rs`; re-export spec lib.rs + spec/mod.rs | **DELETE** |
| `clear_reduced_owner_channels_at_generation` | `crates/simthing-spec/src/spec/constrained_clearing.rs:437` | caller authority; converts reduce-up via `from_runtime_demand` then the ordinary door | conversion wrapper | generationless shim + `clear_stamped_owner_channels` | narrow behind `CpuVendorizedOracle` (or delete once callers are gone) |
| `clear_stamped_owner_channels` | `crates/simthing-spec/src/spec/constrained_clearing.rs:488` | generation from `StampedReduceUpProduct` | canonical stamped-RF market binding | germ `stemthing_b_flow_market_germ_0.rs`; re-export spec + embedder `run.rs` | narrow behind `CpuVendorizedOracle` |

No additional public `clear_*` host-clearing entry exists on this base.

## Path and leg definitions

```
RuntimeOwnerSiloDemandBucket
  -> ConstrainedClaim::from_runtime_demand     [claim_production_completion]
  -> (no GPU map/readback)                     [gpu_to_host = 0 bytes / 0 ns]
  -> group by OwnerChannelScopeKey             [host_conversion_grouping]
  -> TransformOp::apply_with_params            [eml_scoring]
  -> sort score-bits then id; equal-bit bands  [score_sorting_banding]
  -> largest remainder + generation-rotated ties [integer_apportionment]
  -> ConstrainedGrant::from_clearance + result [grant_result_construction]
enclosing production door: clear_constrained_claims_at_generation
  -> (no GPU upload)                           [host_to_gpu = 0 bytes / 0 ns]
  -> grants-available → gen+1 authority        [n_plus_one_launch_delay]
  -> full gen+1 host clear (comparator)        [next_generation_host_reclear]
  -> record_cleared_grant -> IntegrationSchedule [cpu_schedule_replay_recording; not in N+1 delay]
  -> fund_unresolved_persistence               [lawful_structural_consequence]
instrument (D2 shape 2, inside e2e):
  nested grouping/scoring/sorting/apportionment [instrument_restatement]
  second uninstrumented production clear       [neutrality_reclear]
```

Integer apportionment is a workshop-local restatement of the live
largest-remainder + generation-rotated-tie loop on the same scored/sorted/banded
input. Production remains the authority; isolation is proven by matching
`grant.granted`. Grant/result construction is the signed remainder of the
enclosing door after those nested sequential components (not forced-closed).

## Scale (median ns)

| Workload | claims | warm / N | enclosing | apportionment | setup |
|---|---:|---|---:|---:|---:|
| `scale_1000` | 1,000 | 3 / 11 | 857,100 | 403,400 | 343,500 |
| `scale_10000` | 10,000 | 2 / 7 | 9,985,100 | 5,392,100 | 819,500 |
| `scale_100000` | 100,000 | 1 / 5 | 159,656,500 | 77,644,800 | 8,785,400 |
| `scale_1000000` | 1,000,000 | 1 / 3 | 1,968,425,200 | 1,248,710,300 | 139,536,700 |

## 1,000,000-claim decomposition (median ns)

| Leg | median_ns | p95_ns | bytes in / out | isolation |
|---|---:|---:|---|---|
| claim_production_completion | 118,803,100 | 122,344,400 | 0 / 0 | `from_runtime_demand` |
| gpu_to_host_synchronization_readback | 0 | 0 | 0 / 0 | door-absent |
| host_conversion_grouping | 150,623,600 | 159,062,800 | 0 / 0 | BTreeMap by scope |
| eml_scoring | 128,578,700 | 129,090,100 | 0 / 0 | `apply_with_params` |
| score_sorting_banding | 5,490,500 | 5,709,600 | 0 / 0 | one equal-score band |
| integer_apportionment | 1,248,710,300 | (see packet) | 0 / 0 | live remainder restatement |
| grant_result_construction | 399,008,100 | (see packet) | 0 / 0 | signed enclosing remainder |
| host_to_gpu_upload | 0 | 0 | 0 / 0 | door-absent |
| n_plus_one_launch_delay | 100 | 100 | 0 / 0 | grants-available → N+1 authority; GPU 0 |
| next_generation_host_reclear | 1,801,596,900 | (see packet) | 0 / 0 | full gen+1 host clear comparator |
| instrument_restatement | 2,475,323,800 | (see packet) | 0 / 0 | D2 named nested pass |
| neutrality_reclear | 1,850,550,600 | (see packet) | 0 / 0 | D2 named second clear |
| cpu_schedule_replay_recording | 274,468,600 | (see packet) | 0 / 0 | `record_cleared_grant`; not in N+1 delay |
| lawful_structural_consequence | (see packet) | (see packet) | 0 / 0 | `fund_unresolved_persistence` |
| enclosing `clear_constrained_claims_at_generation` | 1,968,425,200 | (see packet) | 0 / 0 | production door |
| end-to-end | 10,429,480,700 | (see packet) | 0 / 0 | includes named instrument legs |

Likely dominant host-clearing cost under this equal-band program: **integer
apportionment**. End-to-end includes named D2 instrument legs
(`instrument_restatement`, `neutrality_reclear`) plus the full next-generation
re-clear comparator. Residual is observation overhead after those named legs;
it is not forced to zero.

## Multi-tree (median enclosing ns)

| Shape | trees | claims | enclosing | coupling |
|---|---:|---:|---:|---|
| one large tree | 1 | 100,000 | 132,101,900 | same single-tree CPU-host door |
| many independent small trees | 100 | 100 each | 7,081,500 | sequential independent clears; no host-wide lock |
| divergent generations | 4 | 1,000 each at gens 1, 10, 100, 1000 | 3,922,900 | per-call `ClearingRemainderAuthority`; no shared generation |
| overlapping local raw ids | 2 | 1,000 each, raws `1..=N` including 7 | 1,354,000 | same raw interpreted only under distinct `owner_ref` / `scope_id` / granter |

Overlapping construction door: `ConstrainedClaim::from_runtime_demand` →
`SimThingId::from_session_raw` (ordinary production admission used by
`constrained_clearing` itself). No `SimThingId::new()`, no test-only identity
wrapper, no raw-id fiction.

Process-global observations (not used by the overlapping fixture's claim ids):

- `SimThingId::new` holds process-global `AtomicU32 NEXT_SIMTHING_ID`
- `OverlayId::new` holds a process-global `AtomicU32` (structural overlay mint)
- no host-wide clearing lock in `constrained_clearing.rs`
- `IntegrationSchedule` is caller-owned per tree
- generation is per-authority, not a host barrier

## Neutrality

Instrumented production-door outputs equal uninstrumented
`clear_constrained_claims_at_generation` outputs on every workload, including
1,000,000 claims. Existing seal `contention_arena_executed_0` remains green.
Zero production crate files changed.

## Ledger

`scripts/ci/closeout_artifacts.tsv` row for
`crates/simthing-spec/src/spec/constrained_clearing.rs` converted from
`owed-measurement:2026-08-30` to
`measured:2026-08-31@625290768f266276c68240dc88ffbb1db6de35bb`. New instrument /
report / test / results artifacts leased to PERFORMANCE-TRACK. Unrelated rows
unchanged, including Gu-Yang A1 and other performance debts.

## Containment

Zero production source change in core/kernel/spec/gpu/driver/sim/feeder/
clausething/mapeditor. Zero Cargo.toml / lockfile / gate-code / pointer /
closeout-apply / 14.2+ identity or resident-germ work.

## Certificate

`cargo test -p simthing-workshop --test generation_critical_path_baseline_0`:
**4 passed / 0 failed**. `cargo test -p simthing-driver --test contention_arena_executed_0`:
**1 passed**. `cargo check -p simthing-workshop`: green.
