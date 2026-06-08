# MOBILITY-GPU-KERNEL-11 Results

Date: 2026-06-02

Verdict: **PASS / deterministic budget-envelope assertions over KERNEL-10 stream accounting**

## Scope

MOBILITY-GPU-KERNEL-11 evaluates integer-only budget envelopes over the accepted KERNEL-10 stream
accounting summary. Active-stream and zero-cost envelopes use exact integer comparisons; negative
tests use local fake over-budget accounting inputs with deterministic diagnostics.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel11_budget_envelope_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel11_budget_envelope_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel11_results.md`

## Budget Envelope Constants

| Envelope field | Active stream | Zero-cost (disabled/registration-only) |
| --- | --- | --- |
| Frames | 4 | 0 |
| Variants per frame | 4 | 0 |
| Replays per variant | 2 | 0 |
| Rows per variant | 34,000 | 0 |
| Variant dispatch attempts | 16 | 0 |
| Replay dispatch attempts | 32 | 0 |
| Total rows processed | 1,088,000 | 0 |
| CPU oracle rows | 1,088,000 | 0 |
| GPU rows | 1,088,000 (ExactParity) or none (GpuUnavailable) | none |

## Over-Budget Fake-Input Diagnostics

| Fake input | Deterministic diagnostics |
| --- | --- |
| `fake_over_budget_rows_accounting` | `kernel11_budget_rows_over_envelope`, `kernel11_budget_cpu_oracle_rows_over_envelope` |
| `fake_over_budget_dispatches_accounting` | `kernel11_budget_variant_dispatch_over_envelope`, `kernel11_budget_replay_dispatch_over_envelope` |

## GPU Execution

Local GPU execution ran and classified as **ExactParity**; budget envelope satisfied with full GPU
row count. Hosts without a wgpu adapter classify **GpuUnavailable** honestly with zero GPU rows.

## Tests

```bash
cargo test -p simthing-driver --test mobility_gpu_kernel11_budget_envelope_fixture  # 32 passed
cargo test -p simthing-driver --test mobility_gpu_kernel10_stream_accounting_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel9_frame_stream_fixture
# ... full KERNEL-0..8 + runtime regression suite
cargo check --workspace
```

## Posture Attestation

- No wall-clock benchmark, floating-point timing, or host-dependent timing thresholds.
- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No new shader text; KERNEL-11 reuses KERNEL-10 accounting over KERNEL-9 stream unchanged.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner/urgency/commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Budget evaluation does not mutate KERNEL-10 accounting; KERNEL-10 checksums preserved.
- Fast-lane under "semantic-free + default-off + parity-backed = ship it."
