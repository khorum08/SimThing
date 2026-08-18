# CONTENTION-CONSERVATION-JUDGE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 8.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `6c58b7771d63` (ingress repair #1780; prior `ba7a36cfbf18` stale)
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- ANCHOR-ACK: `orientation-harness-core@8a365d1c0864`
- ANCHOR-ACK: `scanner-selftest-delta-gate@34fb2662baae`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- ANCHOR-ACK: `core-rf-arenas@d171614211e9`
- ANCHOR-ACK: `stemthing-binding-laws@6787a118c3ca`
- Board dispatch: comment `5323932594`
- expected_route: `DA-RESERVE(gate-wiring)`

## What landed

One scenario-neutral conservation referee: `judge_conservation`. It judges declared snapshots and never allocates or picks a winner. Reconstruction uses only `reduce_owner_channel_rf` / `reconstruct_owner_channel_rf_map`. Multi-owner containers are normal.

## Table battery

| Case | Verdict | Reason |
|---|---|---|
| LawfulA (6+4 of 10) | GREEN | two lawful partitions, same supply |
| LawfulB (3+7 of 10) | GREEN | resolution-rule independence |
| MultiOwner | GREEN | distinct owners are not a defect |
| OverAccounting | RED | `SeededOverAccounting` |
| UnderAccounting | RED | `SeededUnderAccounting` |
| OwnerUniformity | RED | `OwnerUniformityRejection` |
| QuantizedConserves | GREEN | input `V = N*C + R`; output is creation |
| CrossChannelSum | RED | `CrossChannelSum` |
| SeamExact | GREEN | `child + seam + parent == admitted` |
| ChildParentOnly | RED | `ChildParentOnly` |
| StemThingExact | GREEN | `free + in_flight + occupied == capacity` |
| StemThingBroken | RED | `StemThingPartition` |
| ActionBandIncluded | GREEN | ActionBand claim is an ordinary declared claim |
| ActionBandOmitted | RED | `ActionBandOmission` |

All seven planted REDs fire on `judge_conservation` with their named reason. No `refuse_*` helper, no 8.2 executor, no ladder/orientation edit, no `OVERLAY-PEER-AUTHORITY` retirement.

## Remand 5324058663 blast radius

Mechanical transport/proof record only. Judge semantics unchanged (`owner_uniformity_required`, `fold_output_into_input`, `omit_seam` preserved).

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test contention_conservation_judge_0` | 1/1 |
| `cargo test -p simthing-driver --test owner_channel_intrinsic_reduce_up_0` | 4/4 |
| `cargo test -p simthing-spec --lib owner_channel_rf` | 4/4 (9 filtered) |
| `cargo test -p simthing-spec --test async_command_queue_0` | 3/3 |
| `cargo test -p simthing-spec --test event_generation_stamp_reduce_up_0` | 4/4 |
| `cargo test -p simthing-core --test band_quantized_draw_0` | 9/9 |
| `cargo test -p simthing-driver --test residency_tier_vocabulary_0` | 4/4 |
| `cargo test -p simthing-driver --test actionband_recursive_composition_0` | 5/5 |
| `cargo check -p simthing-driver` | ok |
| detachability / lifecycle-schema / doc-budget / inventory-drift | PASS |
