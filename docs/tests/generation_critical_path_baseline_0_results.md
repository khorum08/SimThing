# GENERATION-CRITICAL-PATH-BASELINE-0 Results

> **Status: PROBATION / proof-present / DA-review-pending.** Coding lane
> only; no merge, graduation, pointer movement, closeout apply, or 14.2+ work.
> Comparator evidence only: wall-clock values are dated facts about one
> reproducibility envelope, never a go/no-go gate or portable CI threshold.

**Date:** 2026-08-31
**Dispatch:** Board `5472690735`
**DA remand (this return):** Board `5474110915` (ruling `5474062621`)
**Prior DA remands:** `5473572716` D4/D5/D6 ACCEPTED; `5473368436` D1/D2/D3 ACCEPTED
**Prior relay:** `5473751925`
**DA authority:** `5472516590`
**HD-RECEIPT:** `f27b4dbaf650`
**ORIENT-RECEIPT:** `55747d120d2b` (`orientation_rule_stamp=6e41e33143db01ef`)
**production_door_sha:** `625290768f266276c68240dc88ffbb1db6de35bb` (dispatch master / measured live door; not the instrument SHA)
**Old evidence-tail B:** `491e82f43c75a5e9aa2d6796dd2a1a1ccfdbce4d`
**instrument_sha / tested_code_sha (commit A′):** `ea700bb6d33f15fc1a755cb6463b10214646a24a`
**Evidence-tail commit B′:** this docs bind; packet `tested_commit` names A′, not B′
**ANCHOR-ACK:** `orientation-harness-core@8a365d1c0864`, `scanner-selftest-delta-gate@34fb2662baae`

Lossless artifact: `crates/simthing-workshop/tests/generation_critical_path_baseline_reports.txt`

D7 two-commit provenance: packet generated at committed SHA A′
(`git rev-parse HEAD == ea700bb6d33f15fc1a755cb6463b10214646a24a` immediately
before the measurement command). This summary copies packet numbers; it does
not invent them.

## Envelope

| Field | Value |
|---|---|
| UTC | `2026-08-31T05:29:06Z` |
| CPU | Intel64 Family 6 Model 183 Stepping 1, GenuineIntel |
| GPU | NVIDIA GeForce RTX 4080 Laptop GPU vendor=0x10de device=0x27a0 type=DiscreteGpu |
| Adapter / backend | Vulkan |
| Driver | NVIDIA 595.79 |
| OS | windows-x86_64 |
| Toolchain | rustc 1.95.0 (59807616e 2026-04-14) |
| Profile | cargo-test (optimized+debuginfo) |
| Seed | `0x00014001` (81921) |
| Authored program | `TransformOp::set(1.0)` (one equal score band; proportional remainder) |
| Command | `cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture` |
| Packet `tested_commit` | `ea700bb6d33f15fc1a755cb6463b10214646a24a` |

D7 exact-command equality: human envelope `exact_command`, JSON
`exact_command`, and this summary Command cell are the same procedure-of-record
string.

GPU adapter was queried for the envelope only. The ordinary generation
critical path is the CPU-host door `clear_constrained_claims_at_generation`.
GPU-to-host readback and host-to-GPU upload are **door-absent**: 0 bytes / 0 ns
(no `map_async`, no `write_buffer`, no queue submit, no GPU clearing kernel).

## Remand D1 / D2 / D3 (frozen; not reopened)

### D1 — signed remainder

`grant_result_construction` remains `i64`. The prior `.max(0)` clamp stays gone.

| | Workload | Value |
|---|---|---|
| Before (PR #1907 @ `3d55aba8`) | `scale_1000` `grant_result_construction` | `min_ns=0` with an explicit `0` sample after clamp |
| After (this packet @ A′) | `scale_10000` `grant_result_construction` | `min_ns=-10842500` (signed remainder retained) |
| After | `scale_100000` `grant_result_construction` | `min_ns=-29101100` |
| After | `one_large_tree` `grant_result_construction` | `min_ns=-8268400` |

Serialization boundary: `LegSamples.sample_ns`, `median_ns`, `p95_ns`, `min_ns`, `max_ns` are `i64`.

### D2 — instrument work is named (shape 2)

Chosen shape: **`shape-2-instrument-legs-inside-e2e`**. The nested restatement and the second uninstrumented clear stay inside the end-to-end envelope and are published as `instrument_restatement` and `neutrality_reclear` with raw samples. grouping/scoring/sorting/apportionment are components of `instrument_restatement`, not extra e2e work. `grant_result_construction` is a partition of `enclosing_clear`. Arithmetic is not forced-closed.

### D3 — N+1 delay vs next-generation re-clear

| Quantity | Boundary | 1M median ns |
|---|---|---:|
| enclosing host clear | `clear_constrained_claims_at_generation` at current generation | 1,905,226,300 |
| `n_plus_one_launch_delay` | grants-available → construct generation+1 `ClearingRemainderAuthority`; GPU launch 0 | 100 |
| `next_generation_host_reclear` | full ordinary-door re-clear at generation+1 (comparator) | 1,778,242,200 |
| `cpu_schedule_replay_recording` | `record_cleared_grant` after grants; **not** inside N+1 delay | 337,641,600 |

## D6 — samplewise residual (frozen)

`observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i]` over the accepted D2 shape-2 set `E2E_ACCOUNTED_LEGS`. Signed `i64`. Not clamped. Sample count equals workload N.

`difference_of_medians_ns` is a **derived figure**, never labeled residual.

### 1M residual series

| Field | Packet / summary (identical) |
|---|---|
| name | `observation_overhead_residual` |
| samples (raw signed i64) | `[1768900000, 1755150200, 1889470700]` |
| N | 3 (equals `scale_1000000.sample_count`) |
| median_ns | 1,768,900,000 |
| p95_ns | 1,889,470,700 |
| variance_ns2 | 5461391236463333.000 |
| min_ns | 1,755,150,200 |
| max_ns | 1,889,470,700 |
| mean_ns | 1804506966.667 |
| `difference_of_medians_ns` | 1,754,509,700 (derived figure; not the residual) |

## D7 — exact invocation provenance

Procedure-of-record string (byte-identical in packet human envelope, packet JSON, and this summary):

`cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture`

A′ changed only `generation_critical_path_baseline_0.rs`: that string plus one
assertion that `packet.envelope.exact_command` equals it. No new test function.

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
D6 residual[i] = end_to_end[i] - Σ accounted_leg[i]  [observation_overhead_residual]
difference_of_medians = median(e2e) - Σ median(accounted)  [derived figure, not residual]
```

Integer apportionment is a workshop-local restatement of the live
largest-remainder + generation-rotated-tie loop on the same scored/sorted/banded
input. Production remains the authority; isolation is proven by matching
`grant.granted`. Grant/result construction is the signed remainder of the
enclosing door after those nested sequential components (not forced-closed).

## Scale (median ns)

Copied from the lossless packet at A′. No pre-remand value is carried forward.

| Workload | claims | warm / N | enclosing | apportionment | setup |
|---|---:|---|---:|---:|---:|
| `scale_1000` | 1,000 | 3 / 11 | 813,100 | 393,000 | 359,400 |
| `scale_10000` | 10,000 | 2 / 7 | 10,035,300 | 6,352,400 | 785,900 |
| `scale_100000` | 100,000 | 1 / 5 | 133,476,900 | 95,895,200 | 8,010,900 |
| `scale_1000000` | 1,000,000 | 1 / 3 | 1,905,226,300 | 1,261,617,100 | 101,909,000 |

## D5 — 1,000,000-claim field-for-field alignment

Every displayed 1M statistic below is copied from
`generation_critical_path_baseline_reports.txt` workload `scale_1000000`
(human projection and JSON agree). Packet `tested_commit` =
`ea700bb6d33f15fc1a755cb6463b10214646a24a`.

| Quantity | packet median_ns | summary median_ns | packet p95_ns | summary p95_ns | bytes in / out |
|---|---:|---:|---:|---:|---|
| claim_production_completion | 104164200 | 104,164,200 | 144077100 | 144,077,100 | 0 / 0 |
| gpu_to_host_synchronization_readback | 0 | 0 | 0 | 0 | 0 / 0 |
| host_conversion_grouping | 147280400 | 147,280,400 | 150841700 | 150,841,700 | 0 / 0 |
| eml_scoring | 158926800 | 158,926,800 | 165564400 | 165,564,400 | 0 / 0 |
| score_sorting_banding | 4605100 | 4,605,100 | 4720900 | 4,720,900 | 0 / 0 |
| integer_apportionment | 1261617100 | 1,261,617,100 | 1292103200 | 1,292,103,200 | 0 / 0 |
| grant_result_construction (signed remainder) | 416162700 | 416,162,700 | 480524900 | 480,524,900 | 0 / 0 |
| host_to_gpu_upload | 0 | 0 | 0 | 0 | 0 / 0 |
| n_plus_one_launch_delay | 100 | 100 | 200 | 200 | 0 / 0 |
| next_generation_host_reclear | 1778242200 | 1,778,242,200 | 1904447800 | 1,904,447,800 | 0 / 0 |
| instrument_restatement | 2511888000 | 2,511,888,000 | 2625782200 | 2,625,782,200 | 0 / 0 |
| neutrality_reclear | 1886698500 | 1,886,698,500 | 1957137500 | 1,957,137,500 | 0 / 0 |
| cpu_schedule_replay_recording | 337641600 | 337,641,600 | 339457300 | 339,457,300 | 0 / 0 |
| lawful_structural_consequence | 246443000 | 246,443,000 | 283845400 | 283,845,400 | 0 / 0 |
| enclosing `clear_constrained_claims_at_generation` | 1905226300 | 1,905,226,300 | 1928692300 | 1,928,692,300 | 0 / 0 |
| end_to_end | 10524813600 | 10,524,813,600 | 10728093600 | 10,728,093,600 | 0 / 0 |
| observation_overhead_residual | 1768900000 | 1,768,900,000 | 1889470700 | 1,889,470,700 | 0 / 0 |

1M residual additional packet fields: `min_ns=1755150200`, `max_ns=1889470700`,
`mean_ns=1804506966.667`, `variance_ns2=5461391236463333.000`, samples
`[1768900000, 1755150200, 1889470700]`.

1M `difference_of_medians_ns=1754509700` (derived figure; not the residual).

Likely dominant host-clearing cost under this equal-band program: **integer
apportionment**. End-to-end includes named D2 instrument legs plus the full
next-generation re-clear comparator. The residual is the D6 samplewise
observation-overhead series; it is not forced to zero and is not the
difference of medians.

## Multi-tree (median enclosing ns)

| Shape | trees | claims | enclosing | coupling |
|---|---:|---:|---:|---|
| one large tree | 1 | 100,000 | 132,743,200 | same single-tree CPU-host door |
| many independent small trees | 100 | 100 each | 8,646,400 | sequential independent clears; no host-wide lock |
| divergent generations | 4 | 1,000 each at gens 1, 10, 100, 1000 | 3,308,800 | per-call `ClearingRemainderAuthority`; no shared generation |
| overlapping local raw ids | 2 | 1,000 each, raws `1..=N` including 7 | 1,387,700 | same raw interpreted only under distinct `owner_ref` / `scope_id` / granter |

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

`scripts/ci/closeout_artifacts.tsv` keeps
`production_door_sha=625290768f266276c68240dc88ffbb1db6de35bb` distinct from
`instrument_sha=ea700bb6d33f15fc1a755cb6463b10214646a24a`. PERFORMANCE-TRACK
leases for the 14.1 instrument / report / test / packet / summary remain.
Unrelated rows unchanged. No closeout apply.

## Containment

Zero production source change in core/kernel/spec/gpu/driver/sim/feeder/
clausething/mapeditor. Zero Cargo.toml / lockfile / gate-code / pointer /
closeout-apply / 14.2+ identity or resident-germ work. A′ touched only the
focused test file. B′ is evidence/ledger only.

## Certificate

`cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture`:
**4 passed / 0 failed** at HEAD `ea700bb6d33f15fc1a755cb6463b10214646a24a` (57.25s).
`cargo test -p simthing-driver --test contention_arena_executed_0`: **1 passed**.
`cargo check -p simthing-workshop`: green.
