# GENERATION-CRITICAL-PATH-BASELINE-0 Results

> **Status: PROBATION / proof-present / DA-review-pending.** Coding lane
> only; no merge, graduation, pointer movement, closeout apply, or 14.2+ work.
> Comparator evidence only: wall-clock values are dated facts about one
> reproducibility envelope, never a go/no-go gate or portable CI threshold.

**Date:** 2026-08-31
**Dispatch:** Board `5472690735`
**DA authority:** `5472516590`
**HD-RECEIPT:** `f27b4dbaf650`
**ORIENT-RECEIPT:** `55747d120d2b` (`orientation_rule_stamp=6e41e33143db01ef`)
**Dispatch master / measured production door:** `625290768f266276c68240dc88ffbb1db6de35bb`
**tested_code_sha:** `8d36d0f4c4312ee54b7557f10de92be733d2b431`
**ANCHOR-ACK:** `orientation-harness-core@8a365d1c0864`, `scanner-selftest-delta-gate@34fb2662baae`

Lossless artifact: `crates/simthing-workshop/tests/generation_critical_path_baseline_reports.txt`

## Envelope

| Field | Value |
|---|---|
| UTC | `2026-08-31T02:39:44Z` |
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
  -> generation+1 re-clear                     [n_plus_one_launch_delay]
  -> record_cleared_grant -> IntegrationSchedule [cpu_schedule_replay_recording]
  -> fund_unresolved_persistence               [lawful_structural_consequence]
```

Integer apportionment is a workshop-local restatement of the live
largest-remainder + generation-rotated-tie loop on the same scored/sorted/banded
input. Production remains the authority; isolation is proven by matching
`grant.granted`. Grant/result construction is the signed remainder of the
enclosing door after those nested sequential components (not forced-closed).

## Scale (median ns)

| Workload | claims | warm / N | enclosing | apportionment | setup |
|---|---:|---|---:|---:|---:|
| `scale_1000` | 1,000 | 3 / 11 | 801,200 | 407,600 | 106,200 |
| `scale_10000` | 10,000 | 2 / 7 | 9,598,000 | 5,354,100 | 812,400 |
| `scale_100000` | 100,000 | 1 / 5 | 115,091,600 | 72,631,400 | 8,808,700 |
| `scale_1000000` | 1,000,000 | 1 / 3 | 1,698,676,500 | 1,128,329,300 | 100,974,300 |

## 1,000,000-claim decomposition (median ns)

| Leg | median_ns | p95_ns | bytes in / out | isolation |
|---|---:|---:|---|---|
| claim_production_completion | 118,803,100 | 122,344,400 | 0 / 0 | `from_runtime_demand` |
| gpu_to_host_synchronization_readback | 0 | 0 | 0 / 0 | door-absent |
| host_conversion_grouping | 150,623,600 | 159,062,800 | 0 / 0 | BTreeMap by scope |
| eml_scoring | 128,578,700 | 129,090,100 | 0 / 0 | `apply_with_params` |
| score_sorting_banding | 5,490,500 | 5,709,600 | 0 / 0 | one equal-score band |
| integer_apportionment | 1,128,329,300 | 1,131,850,700 | 0 / 0 | live remainder restatement |
| grant_result_construction | 294,342,700 | 340,436,700 | 0 / 0 | enclosing remainder |
| host_to_gpu_upload | 0 | 0 | 0 / 0 | door-absent |
| n_plus_one_launch_delay | 1,669,563,100 | 1,683,677,100 | 0 / 0 | host re-clear gen+1; GPU launch 0 |
| cpu_schedule_replay_recording | 310,300,500 | 336,342,400 | 0 / 0 | `record_cleared_grant` |
| lawful_structural_consequence | 213,800,700 | 244,373,300 | 0 / 0 | `fund_unresolved_persistence` |
| enclosing `clear_constrained_claims_at_generation` | 1,698,676,500 | 1,754,189,300 | 0 / 0 | production door |
| end-to-end (named sequence) | 9,561,292,100 | 9,651,716,800 | 0 / 0 | includes inner+enclosing overlap |

Likely dominant host-clearing cost under this equal-band program: **integer
apportionment** (largest-remainder + remainder-order sort of the full 1M-claim
band). End-to-end exceeds the enclosing door because named inner legs are also
counted inside enclosing plus N+1 / schedule / structural / claim production.
The signed unattributed remainder in the lossless artifact records that overlap
and observation overhead; it is not forced to zero.

## Multi-tree (median enclosing ns)

| Shape | trees | claims | enclosing | coupling |
|---|---:|---:|---:|---|
| one large tree | 1 | 100,000 | 141,790,900 | same single-tree CPU-host door |
| many independent small trees | 100 | 100 each | 6,869,400 | sequential independent clears; no host-wide lock |
| divergent generations | 4 | 1,000 each at gens 1, 10, 100, 1000 | 3,695,900 | per-call `ClearingRemainderAuthority`; no shared generation |
| overlapping local raw ids | 2 | 1,000 each, raws `1..=N` including 7 | 1,669,700 | same raw interpreted only under distinct `owner_ref` / `scope_id` / granter |

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
