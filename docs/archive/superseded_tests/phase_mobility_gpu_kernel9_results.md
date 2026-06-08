# MOBILITY-GPU-KERNEL-9 Results

Date: 2026-06-02

Verdict: **PASS / deterministic multi-frame projection-variant stream soak over the KERNEL-6 chain**

## Scope

MOBILITY-GPU-KERNEL-9 sequences KERNEL-8 generic projection variants across four explicit frames through
the semantic-free KERNEL-6 chain. Each frame runs four variants with two replay dispatches each,
recording per-variant and aggregate frame CPU/GPU checksums with ExactParity or honest GpuUnavailable.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel9_frame_stream_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel9_frame_stream_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel8_variant_batch_fixture.rs` (re-export KERNEL-6 dispatch)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel9_results.md`

## Frame Stream

| Frame | Variant order | Notes |
| --- | --- | --- |
| `frame0_canonical_batch` | baseline → sparse → dense → parent-offset | Canonical KERNEL-8 order |
| `frame1_reversed_batch` | parent-offset → dense → sparse → baseline | Different order |
| `frame2_repeat_canonical` | baseline → sparse → dense → parent-offset | Identical to frame 0 |
| `frame3_alt_variant_order` | sparse → dense → parent-offset → baseline | Distinct from frames 0/1 |

Frames: **4**. Variants per frame: **4**. Replay repetitions per variant: **2**. Row count: **34,000**.

## GPU Execution

Local GPU execution ran and classified as **ExactParity** for all frames, variants, and replays.
Hosts without a wgpu adapter classify **GpuUnavailable** honestly.

## Tests

```bash
cargo test -p simthing-driver --test mobility_gpu_kernel9_frame_stream_fixture  # 28 passed
cargo test -p simthing-driver --test mobility_gpu_kernel8_variant_batch_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel7_replay_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel6_chain_fixture
# ... full KERNEL-0..4 + runtime regression suite
cargo check --workspace
```

## Posture Attestation

- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No new shader text; KERNEL-9 reuses KERNEL-8 variants and KERNEL-6 chain.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner/urgency/commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Repeated identical frames produce identical aggregate checksums; distinct frames differ where expected.
- Source baseline projection is not mutated across frame transitions.
- Registration-only and disabled paths remain zero-cost.
- Fast-lane under "semantic-free + default-off + parity-backed = ship it."
