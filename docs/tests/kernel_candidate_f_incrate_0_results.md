# KERNEL-CANDIDATE-F-INCRATE-0 Results

## Status

**PROBATION** — candidate-F exact-magnitude write authority moved in-crate; cross-crate B5 residue retired. Not DA-approved.

## PR / branch / merge

- Branch: `kernel-candidate-f-incrate-0`
- Parent: [#1009](https://github.com/khorum08/SimThing/pull/1009) `094178ed7b` (DISPATCH-INCRATE-0R2)
- PR: (pending)

## Owner decision applied

Owner ruled `write_max_candidate_f_magnitude_bits` must not carry into closeout as a standing tripwire. This rung folds the last cross-crate authoritative write into the kernel before `KERNEL-CLOSEOUT-0`.

## What changed

- `write_max_candidate_f_magnitude_bits` → `pub(crate)` (kernel-internal only).
- Removed public export from `simthing-kernel` and `simthing-gpu` re-exports.
- Replaced public `AccumulatorOpSession::write_max_candidate_f_magnitude_bits` with sanctioned door `apply_candidate_f_exact_magnitude(ctx, CandidateFMagnitudeRequest)`.
- Added `CandidateFMagnitudeRequest` (gradients + target slot/col — kernel resolves and writes sealed session values internally).
- Driver `runtime_0080_0_r1a.rs` migrated to the kernel door.
- Added 2 compile-fail proofs + 1 positive GPU parity test.

## Old residue retired

| Surface | Before | After |
|---|---|---|
| `candidate_f_magnitude::write_max_candidate_f_magnitude_bits` | `pub`, re-exported via gpu | `pub(crate)`, not exported |
| `AccumulatorOpSession::write_max_candidate_f_magnitude_bits` | public cross-crate write | removed |
| `simthing_gpu::write_max_candidate_f_magnitude_bits` | public re-export | removed |

## Sanctioned candidate-F kernel door

```rust
// External crates supply non-authoritative inputs only:
pub struct CandidateFMagnitudeRequest<'a> {
    pub gradients: &'a [GradientPairGpu],
    pub target_slot: u32,
    pub target_col: u32,
}

impl AccumulatorOpSession {
    pub fn apply_candidate_f_exact_magnitude(
        &self,
        ctx: &GpuContext,
        request: CandidateFMagnitudeRequest<'_>,
    ) -> Result<CandidateFMagnitudeReport, CandidateFMagnitudeError>;
}
```

Kernel binds `values_buffer()` internally via `pub(crate) write_max_candidate_f_magnitude_bits`. External crates cannot name the write helper or pass an authoritative buffer handle.

## Boundary / B1–B8 scan

Candidate-F surface grep (`write_max_candidate_f_magnitude_bits`, `CandidateF`, `candidate_f`, `magnitude_bits`, `pub fn .*write`, `pub fn .*candidate`):

| Hit | Classification |
|---|---|
| `write_max_candidate_f_magnitude_bits` in `candidate_f_magnitude.rs` | **Sealed** — `pub(crate)` only |
| `apply_candidate_f_exact_magnitude` on session | **Sanctioned door** — kernel-internal write |
| `max_candidate_f_magnitude_bits` (public) | **Inert utility** — caller-owned ephemeral buffers; no authoritative handle; read-only max probe |
| `IndexedScatterOp::dispatch` | **Inert utility** (unchanged from 0R2) |
| `cpu_oracle_threshold_events` | **Standing tripwire** (doctrine-blessed CPU twin) |
| In-crate WGSL shader text | **Permanent residue** |

| Class | Result |
|---|---|
| B1 unsafe forge | **Clean** — `#![forbid(unsafe_code)]` |
| B2 authority token minter | **Clean** |
| B3 authoritative `&Buffer` | **Clean** — write helper not public |
| B4 POD bridge | **Clean** |
| B5 public write hook over authority | **Sealed** — candidate-F B5 residue gone |
| B6 ctx/queue pairing | **Clean** — no buffer handle pairs with public ctx |
| B7 scaffold | **Clean** |
| B8 deps | **Clean** |

## Approved residue whitelist after this rung

| Item | Evidence |
|---|---|
| `max_candidate_f_magnitude_bits(ctx, gradients)` | Inert: creates caller-owned ephemeral buffers; returns u32 bits only; never writes resolved state |
| `IndexedScatterOp::dispatch(src, dst, …)` | Inert without authoritative handles (0R2) |
| `cpu_oracle_threshold_events` | Sanctioned CPU twin tripwire |
| In-crate WGSL shader text | Permanent residue |

**Removed from whitelist:** `write_max_candidate_f_magnitude_bits` — no longer a cross-crate authoritative write.

## Compile-fail proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `external_session_candidate_f_write` | `session.write_max_candidate_f_magnitude_bits(...)` |
| `external_kernel_candidate_f_write_helper` | `simthing_kernel::write_max_candidate_f_magnitude_bits(..., target_buffer, ...)` |

**Total kernel doc compile_fail:** 28/28 (26 prior + 2 candidate-F).

## Value parity

| Harness | Result |
|---|---|
| `cargo test -p simthing-kernel apply_candidate_f --lib` | 1/1 — written cell bits match `max_candidate_f_magnitude_bits` oracle |
| `cargo test -p simthing-kernel --doc` | 28/28 |
| Driver path | `dispatch_r4_candidate_f` uses `apply_candidate_f_exact_magnitude` — same slot/col/gradient semantics |

## Performance parity

Baseline (pre-seal): `3f038a77c4` (master post-0R2 metadata).

Command: `cargo test -p simthing-sim --test c1_threshold_perf -- --nocapture`

| Run | `new_ms` |
|---|---|
| Baseline (0R2, `3f038a77c4`) | 0.1869 |
| 1 (post-seal) | 0.2783 |

Within prior 0R2 band (~0.19–0.28 ms); visibility-only — same single GPU dispatch on candidate-F hot path.

## Scope Ledger

| File | Change |
|---|---|
| `simthing-kernel/src/candidate_f_magnitude.rs` | Request/Report types; seal write helper; parity test |
| `simthing-kernel/src/accumulator_op/session.rs` | Sanctioned door replaces public write |
| `simthing-kernel/src/readback.rs` | +2 compile_fail proofs |
| `simthing-kernel/src/lib.rs` | Export door types; drop write helper export |
| `simthing-gpu/src/candidate_f_magnitude.rs` | Drop write re-export |
| `simthing-gpu/src/lib.rs` | Export door types only |
| `simthing-driver/src/runtime_0080_0_r1a.rs` | Use kernel door |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung row + closeout prerequisite |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/tests/kernel_dispatch_incrate_0_results.md` | Remove write_max from 0R2 whitelist |

## Known gaps / next

- **KERNEL-CLOSEOUT-0** — now unblocked on candidate-F seal; four mandatory doc landings (§2A) remain.
- `max_candidate_f_magnitude_bits` remains public as inert probe utility (not authoritative write).
