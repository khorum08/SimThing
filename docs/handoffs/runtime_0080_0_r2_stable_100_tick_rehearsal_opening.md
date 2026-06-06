# RUNTIME-0080-0-R2-IMPL-0 — stable GPU-forward 100-tick rehearsal using R1a–R1c-f

**Intended recipient:** Cursor / Codex production implementation agent  
**Authority:** RUNTIME-0080-0 after R1c-f-VERIFY-0  
**Role:** consumer implementation agent  
**Objective:** run the actual 100-tick rehearsal consumer and prove the resident runtime stack now supports the rehearsal path

## Branch

Branch from latest `origin/master` after R1c-f-VERIFY-0 merge. Open a PR; do not direct-push to master.

## Current state

R1c-f verified (`RUNTIME-0080-0-R1c-f-VERIFY-0`):

- R1a Tier-A GPU next-tick values — PASS
- R1b resident event journal — PARTIAL (full journal parity)
- R1c-a/b/c/d/e structural substrate — PASS
- R1c-f GPU-decided ZeroCohort — PASS (`structural_decisions_gpu_emitted_zero_cohort = true`)
- Umbrella `structural_decisions_gpu_emitted` — false (honest)

Full production-track battery is green. Do not add another structural substrate layer unless this consumer fails concretely.

## Purpose

Implement `RUNTIME-0080-0-R2`: a **stable 100-tick GPU-forward rehearsal** that composes R1a through R1c-f into one resident per-tick loop and compares against the R6C oracle where expected.

This is the first **consumer** test of the assembled resident stack at gameplay timescale (100 ticks), not another copy-substrate rung.

## Target stack

```text
R1a Tier-A GPU next-tick values
+ R1b resident journal
+ R1c-a/b/c/d/e structural substrate
+ R1c-f GPU-decided ZeroCohort
→ stable 100-tick rehearsal
→ exact comparison to R6C oracle where expected
→ honest list of remaining CPU-decided event classes
```

## Required scope

1. Implement `run_runtime_0080_0_r2(input) -> Runtime0080R2Report` (or equivalent named module) that runs the **full 100-tick resident loop** with GPU-forward authority where already earned.
2. Prove the rehearsal completes in **foreground under gameplay-representative wall time** (no 10+ minute harness artifacts; use OnceLock + pinned checksums like R1c-f verification).
3. Compare covered columns / journal rows / structural outcomes to R6C oracle **where the stack claims authority** (Tier-A exact columns, full journal parity set, GPU-decided ZeroCohort rows).
4. Emit an honest findings list of **remaining CPU-decided event classes** (not automatic blockers unless they prevent the rehearsal from running):
   - DamageDelta
   - MoveRequest
   - LocalBirthRequest
   - FusionRequest
   - ShipCountDelta
   - OwnerCodeFlip
5. Preserve R6C checksum `1bba891c779190a4` where the rehearsal claims whole-run parity.
6. Set `structural_decisions_gpu_emitted` umbrella false unless all classes are GPU-decided (expected false at R2).

## Read first

1. `docs/tests/runtime_0080_0_r1c_f_resident_zero_cohort_decision_results.md`
2. `crates/simthing-driver/src/runtime_0080_0_r1c_f.rs`
3. `crates/simthing-driver/src/runtime_0080_0_r1b.rs`
4. `crates/simthing-driver/src/runtime_0080_0_r1a.rs`
5. `crates/simthing-driver/src/dress_rehearsal_r6c_integrated_run.rs`
6. `docs/design_0_0_8_0_consumer_pulled_production_track.md`

## Hard do NOT

- R1c-g or another resident copy table
- M-4A / multi-atlas
- scenario reopen (0080-2)
- `docs/invariants.md` edit
- pinned-number change without explicit authorization
- default SimSession wiring
- CPU redecision of ZeroCohort
- semantic WGSL tied only to 0080-2

## Test gate

```text
cargo test -p simthing-driver --test runtime_0080_0_r2
cargo test -p simthing-driver --test runtime_0080_0_r1c_f
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run
cargo test -p simthing-driver --test runtime_0080_0_r0
cargo build --workspace
cargo fmt --all -- --check
cargo check --workspace
```

## Expected PR title

`RUNTIME-0080-0-R2-IMPL-0: stable GPU-forward 100-tick rehearsal`

## Stop conditions

Stop and return to user/Opus if R2 requires M-4A, scenario-specific GPU compute pass, CPU planner for earned GPU decisions, or invariant edits. Do not stop merely because the umbrella structural flag remains false.
