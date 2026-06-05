# SCENARIO-0080-2 — ATLAS-BATCH-0-CLOSE Report

**Gate:** ATLAS-BATCH-0-CLOSE
**Status:** **PASS**
**Date:** 2026-06-04
**Design authority:** Opus
**Recipient:** ChatGPT / production harness

## 1. Verdict

**PASS.** The static-atlas pre-rehearsal prerequisite (`ATLAS-BATCH-0`) is **accepted as complete and
parked.** All six rungs are closed/PASS; the NVIDIA RTX 4080 ladder lifted the adapter caveat; no parked
boundary was opened. The project is authorized to proceed to the **R1-OPEN** opening-spec gate of the
full-vertical `SCENARIO-0080-2` rehearsal. No code was added in this gate.

## 2. Evidence reviewed

Each rung was validated first-hand by the design authority as it landed (not on report): GEN (after the
spacing remedial), LOC, PACK, PACK-GPU (accept), STORE (after the compile/logic remedial), STORE-GPU
(accept; bit-exact verified). Sources: `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_*.rs`
+ matching `tests/` + `docs/tests/scenario_0080_2_atlas_batch_0_*` reports/raw-logs. NVIDIA closeout:
`docs/nvidia_fp_determinism_test.md` (CLOSED/COMPLETE), `docs/tests/nvidia_fp_temp_13_workspace_closeout.md`,
`nvidia_fp_temp_99_summary.md` (un-staled). **Full `cargo test --workspace` green on the discrete RTX
4080 (60 binaries, 0 failed)** confirmed by the design authority.

## 3. Rung-by-rung closeout

### GEN — CLOSED / PASS
Pure deterministic fixture data; no SimThing wiring / GPU / economy / FIELD_POLICY / runtime. 20×20 galaxy,
13 systems with spacing/bounds/uniqueness asserts (the spacing-band defect was caught on execution and
remediated to green).

### LOC — CLOSED / PASS
`Location`-kind gridcell primitive as fixture layout only; dense row-major cell-slot ranges
(`cell_index` single indexing home); occupants preserved as **contributors into cells** (not merged);
no PACK/STORE/GPU/economy/FIELD_POLICY/runtime.

### PACK / EC-A2a — CLOSED / PASS
CPU atlas batch packing: 3 homogeneous tile classes, algebraic `G=0` tile-local mask + CPU oracle,
coordinate round-trip, **numeric** VRAM multiplier (1.0). No GPU claims.

### PACK-GPU / EC-A2b — CLOSED / PASS (GpuVerified)
One batched `AtlasMaskGpuOp` dispatch per class == CPU oracle within **`GpuVerified` full-tile
L∞ ≤ 1e-4** — **not** bit-exact (correctly classified; the f32 diffusion stencil is `GpuVerified`, not
`ExactDeterministic`). **EC-A2b-exact remains deferred** (would require a pinned fixed-point stencil).
Revalidated on the NVIDIA RTX 4080.

### STORE / EC-A3 — CLOSED / PASS
CPU storage-shape oracle keyed `(location, cell, channel, owner)`; co-located occupants preserved
per-channel/per-owner, never blind-summed (10-pirate-shared-cell + constructed planet+patrol+pirate).
No OWNER runtime / session wiring.

### STORE-GPU / EC-A3-gpu — CLOSED / PASS (ExactDeterministic)
Whitelisted `EvalEML` `CMP_EQ`/`SELECT` owner+channel mask + `SlotRange Sum` on a real
`AccumulatorOpSession`, **38/38 `StoreOracle` entries bit-exact (`f32::to_bits`, L∞=0)** vs the CPU
oracle. Fixture composition only; no R3 / session pass-graph OWNER runtime. Revalidated bit-exact on
the RTX 4080 (integer masked sums are adapter-independent).

## 4. NVIDIA adapter caveat disposition

- **PACK-GPU:** revalidated on the discrete NVIDIA RTX 4080; EC-A2b remains **`GpuVerified` f32
  tolerance, not bit-exact**.
- **STORE-GPU:** revalidated on the RTX 4080; EC-A3-gpu **ExactDeterministic bit-exact** held
  cross-adapter (integer masked sums).
- **EC-A2b-exact:** remains **deferred** to the pinned fixed-point stencil track.
- The Intel-only adapter caveat is **lifted** — `GpuContext` now always selects a discrete GPU when
  present, and the full workspace is green on the RTX. No false bit-exact f32 claim is made.

## 5. Explicit parked boundaries (unchanged by this gate)

OWNER masked-reduction **runtime** — parked. R3 (capability-tree mask-down) — parked. M-4A
sparse-residency scheduler — parked. REENROLL — parked. R1–R7 full rehearsal — unopened. No global
default schedule; no CPU planner; no semantic/raw WGSL; no hard currency/markets/trade/`ai_budget`; no
ClauseThing/L3; no UI/realtime loop; no nested-RF depth beyond the named future rung.

## 6. Remaining deferred work

- **EC-A2b-exact** (bit-exact f32 atlas dispatch — pinned fixed-point stencil track).
- **GPU throughput is unmeasured.** The ladder proved *correctness* on the RTX, not per-tick cost;
  wall-clock cargo time is not a GPU perf metric. A separate **timestamp-query benchmark track**
  (resident buffers, batched multi-owner/location workloads) is required before any game-time
  performance claim. (Runtime is architected to avoid the test-harness costs — one persistent session,
  GPU-resident state, boundary-only readback — but that is an architectural expectation pending
  benchmark, not a measured guarantee.)
- The masked-reduction **runtime** wiring (R3) and the rehearsal rungs themselves.

## 7. Authorization for next rung

**Authorized: `R1-OPEN` — Disruption-heatmap / EC1 opening spec.** Opens *only* as a design-authority
opening-spec gate (Opus authors), **not** implementation. R1 intent (§12.5): pirate/patrol presence →
`disruption` column on gridcell SimThings → BoundedFeedback decay → diffuse to `location_status` →
reduce up to the starmap heatmap; CPU oracle; inspectable heatmap artifact. **No FIELD_POLICY movement; no
R2/R3/R4/R5; no OWNER runtime beyond what R1 explicitly consumes.** The R1 opening spec will be authored
as the next gate.

## 8. Required doc updates

- This report (new).
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` — ATLAS-BATCH-0-CLOSE → CLOSED/PASS; report
  linked; adapter disposition recorded (done 2026-06-04); active gate → `R1-OPEN`.
- `docs/worklog.md` — one entry.

## 9. §0.5 self-check

Design-authority closeout only — no new code, no gameplay/resource-flow expansion, no recursive
allocation change, no CPU planner logic, no shader/math/tolerance change, no `simthing-sim` semantic
expansion, no default session wiring. Authorizes only the next opening-spec gate (`R1-OPEN`).
