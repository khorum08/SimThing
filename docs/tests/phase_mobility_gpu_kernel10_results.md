# MOBILITY-GPU-KERNEL-10 Results

Date: 2026-06-02

Verdict: **PASS / deterministic throughput-accounting summary over the KERNEL-9 semantic-free frame stream**

## Scope

MOBILITY-GPU-KERNEL-10 runs the accepted KERNEL-9 multi-frame projection-variant stream unchanged and
adds compact integer-only accounting counters: frame/variant/replay/dispatch/row totals, CPU oracle
rows, GPU rows when available, and aggregate stream checksums. No wall-clock timing or perf sampling.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel10_stream_accounting_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel10_stream_accounting_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel10_results.md`

## Accounting Summary

| Metric | Value |
| --- | --- |
| Frames | 4 |
| Variants per frame | 4 |
| Replays per variant | 2 |
| Rows per variant | 34,000 |
| Total variant dispatch attempts | 16 |
| Total replay dispatch attempts | 32 |
| Total rows processed | 1,088,000 |
| Total CPU oracle rows | 1,088,000 |

## GPU Execution

Local GPU execution ran and classified as **ExactParity**; aggregate GPU stream checksum matches CPU
stream shape. Hosts without a wgpu adapter classify **GpuUnavailable** honestly with zero GPU row count.

## Tests

```bash
cargo test -p simthing-driver --test mobility_gpu_kernel10_stream_accounting_fixture  # 32 passed
cargo test -p simthing-driver --test mobility_gpu_kernel9_frame_stream_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel8_variant_batch_fixture
# ... full KERNEL-0..7 + runtime regression suite
cargo check --workspace
```

## Posture Attestation

- No wall-clock benchmark, floating-point timing, or host-dependent timing thresholds.
- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No new shader text; KERNEL-10 reuses KERNEL-9 stream, KERNEL-8 variants, and KERNEL-6 chain.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner/urgency/commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Repeated accounting summaries are identical; KERNEL-9 parity/checksum results preserved.
- Disabled and registration-only paths report zero dispatch/row accounting cost.
- Fast-lane under "semantic-free + default-off + parity-backed = ship it."
