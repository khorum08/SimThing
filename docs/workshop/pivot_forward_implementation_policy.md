# Pivot-Forward Implementation Policy

**Status:** Active (2026-05-25)  
**Authority:** `docs/adr_accumulator_op_v2.md`, `docs/design_v7.md`, `docs/accumulator_op_v2_production_plan.md`

---

## 1. Production direction

AccumulatorOp v2 is the **foundational GPU execution primitive**. The target architecture is:

**Snapshot → AccumulatorOp → compact event/summary readback**

Old specialized passes (intent, overlay, reduction, threshold, velocity, intensity) are **temporary**. Each family migrates behind a flag, passes parity, defaults on, then is **deleted in S-phase**.

## 2. Legacy code classification

Every PR touching legacy GPU passes must classify interaction as exactly one of:

| Class | Allowed use |
|-------|-------------|
| **Oracle** | Parity tests comparing AccumulatorOp vs legacy |
| **Fallback** | Runtime path only because the AccumulatorOp family is incomplete |
| **Sunset** | Deletion in progress after default-on validation |
| **Bugfix** | Minimal fix to keep existing tests meaningful |

**No other legacy work is allowed.**

## 3. Forbidden posture

Do **not**:

- Keep the old path healthy long-term
- Optimize legacy shaders or passes
- Extend legacy overlay/reduction/intensity semantics
- Add new features to old pipeline because AccumulatorOp is not ready
- Split mixed overlay batches across AccumulatorOp + legacy permanently (C-3/C-4)

## 4. Migration handoff template

Every future C-family handoff must include:

```text
Pivot posture:
  AccumulatorOp path is the intended production path.
  Legacy path is oracle/fallback only.
  This PR must reduce legacy dependence or prepare a named sunset.

Sunset target:
  S-<n> — <old pass deletion>

Legacy interaction allowed:
  oracle / fallback / none

Legacy interaction forbidden:
  no new features · no optimization · no semantic expansion
```

Reviewer must verify the PR does not deepen legacy dependency.

## 5. Exactness and performance

- **Exactness blocks** → solve in AccumulatorOp (e.g. C-3 OrderBand sequencing for repeated Add)
- **Performance blocks** → build production-shaped compiler/cache (dirty overlay compiler, B-4 summary tier, segmented transfer allocator)
- **Never** optimize legacy as the primary response

Performance claims require **timestamped GPU measurements** (ADR invariant): legacy pass µs, AccumulatorOp pass µs, wall time, submission count, readback bytes.

## 6. Current migration state (2026-05-25)

| Item | Status |
|------|--------|
| C-1 Threshold scan | Landed (#97–#98), flag default false |
| C-2 Intent delta | Landed (#99–#100), flag default false |
| C-3 Overlay Add | Landed (#105–#107), Add-only + OrderBand exact f32 order; mixed → legacy fallback |
| C-4 Overlay Mul/Set | **Opus-gated** — full order-band compiler |
| B-4 Summary design | Accepted (`docs/workshop/slot_summary_b4_design.md`) |
| B-4I Summary impl | Landed — production `SlotSummaryGpu` + group checksums |
| C-INF-1 Runtime consolidation | Landed — `WorldAccumulatorRuntime` on `WorldGpuState` |
| C-INF-2 Legacy oracle harness | Landed — `legacy_oracle` module + integration tests |

## 7. Priority infrastructure (non-Opus)

1. **C-5** — WeightedMean tolerance boundary audit
2. Optional — refactor C-1/C-2/C-3 parity tests onto `run_family_oracle`

## 8. Next Opus gate

**C-4** — Overlay Multiply/Set + dirty/cached order-band compiler. Codex must not decide C-4 architecture without Opus design note.

## 9. S-phase reminder

Sunset PRs delete old shaders after: migration merged · parity green · flag default true · CI green. Old code is not a destination.
