# RUNTIME-0080-0-R2 Results

Status: IMPLEMENTED / PASS — stable 100-tick GPU-forward rehearsal
Verdict: PASS
Primitive: `STABLE-100-TICK-GPU-FORWARD-REHEARSAL-0`
Rung: `RUNTIME-0080-0-R2`
Scope: stable 100-tick GPU-forward rehearsal over R1a–R1c-f
Stable report checksum: `73d818417f5b98bf`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Rehearsal runner
- runs_100_ticks: true
- tick_count: 100
- wall_time_r2_test_harness: ~2.24s (OnceLock single run; gameplay-representative)

## Resident stack consumed
| Rung | Consumed | Evidence |
|------|----------|----------|
| R1a | yes | Tier-A GPU next-tick; tick-100 matches R6C oracle trajectory |
| R1b | yes | Resident event journal per tick; full journal parity |
| R1c-a | yes | Free-list marks from rehearsal journal (11 rows) |
| R1c-b | yes | Allocation from marks + LocalBirthRequest (4 rows) |
| R1c-c | yes | Membership apply (426 delta rows) |
| R1c-d | yes | Compaction + lineage staging (16 + 26 rows) |
| R1c-e | yes | Compacted-view apply (16 remap, 17 compacted, 426 membership remap) |
| R1c-f | yes | GPU-decided ZeroCohort from resident `num_ships` (1 row) |

## ZeroCohort
- gpu_decided: true
- cpu_witness_decides_zero_cohort: false
- zero_cohort_row_count: 1
- structural_decisions_gpu_emitted_zero_cohort: true
- structural_decisions_gpu_emitted (umbrella): false

## R6C CPU oracle comparison
- r6c_checksum_expected: `1bba891c779190a4`
- r6c_checksum_observed: `1bba891c779190a4`
- r6c_checksum_matches: true
- tier_a_tick100_matches_oracle: true
- event_journal_parity: true
- explanation: tier-A tick-100 + full journal parity against R6C oracle; equivalent to pinned R6C checksum

## Remaining CPU-decided structural classes (findings, not blockers)
- DamageDelta
- MoveRequest
- LocalBirthRequest
- FusionRequest
- ShipCountDelta
- OwnerCodeFlip
- remaining_class_blocked_run: false

## Anti-ceremony
- m4a_required: false
- multi_atlas_required: false
- new_copy_substrate_added: false
- report_only_aggregation: false (real 100-tick runner executed)

## Required commands (VERIFY-0, foreground PowerShell)

| Target | Result | Wall time |
|--------|--------|-----------|
| `runtime_0080_0_r2` | 18 passed; 0 failed | 2.21s |
| `runtime_0080_0_r1c_f` | (predecessor; unchanged) | — |
| `dress_rehearsal_r6c_integrated_run` | (oracle baseline) | — |
| `cargo build --workspace` | success (exit 0) | 0.21s |
| `cargo fmt --all -- --check` | success (exit 0) | 2.47s |
| `cargo check --workspace` | success (exit 0) | 1.86s |

No scratch logs committed.
