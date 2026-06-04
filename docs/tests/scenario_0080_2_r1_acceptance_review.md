# SCENARIO-0080-2-R1-ACCEPT-0 — R1 Disruption Heatmap / EC1 Acceptance Review

**Verdict:** **R1 ACCEPTED / IMPLEMENTED-PASS**
**Gate:** `R1` — Disruption heatmap / EC1
**Implementation reviewed:** `SCENARIO-0080-2-R1-IMPL-0` (PR #511, merged at `ce82b72`)
**Date:** 2026-06-04
**Acceptance authority:** Opus (design authority)
**Method:** first-hand source review + **re-ran all test evidence locally** (not accepted on report).

## 1. Verdict

`SCENARIO-0080-2-R1-IMPL-0` **satisfies R1 / EC1** and is **accepted and closed as implemented-pass.**
EC1 is met: a **non-trivial disruption heatmap over real galactic gridcell SimThings, produced by
pirate/patrol occupant presence through the pinned `BoundedFeedback` recurrence (not hand-seeded),
verified against a CPU oracle, and emitted as a deterministic inspectable artifact.** No §11 stop
condition was crossed. R2 is **not** opened by this acceptance; the next engineering action requires a
separate `R2-OPEN` gate (or another explicit Opus authorization).

## 2. Files reviewed

- `docs/scenarios/scenario_0080_2_r1_disruption_heatmap_opening_spec.md` (the gate)
- `crates/simthing-driver/src/dress_rehearsal_r1_disruption_heatmap.rs` (implementation + CPU oracle)
- `crates/simthing-driver/tests/dress_rehearsal_r1_disruption_heatmap.rs` (34 tests)
- `crates/simthing-driver/src/lib.rs` (opt-in module export; no default wiring)
- `docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md` (impl report + emitted artifact)
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_{gen,loc,store}.rs` (layout dependency)
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md`, `docs/design_0_0_8_0_consumer_pulled_production_track.md`,
  `docs/invariants.md` (binding rules only — not edited), `docs/workshop/mapping_current_guidance.md`.

## 3. Test evidence (re-run by the acceptance authority, 2026-06-04)

| Command | Result (re-run) | Report claim | Match |
|---|---|---|---|
| `cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap` | **34 passed; 0 failed** | 34 passed | ✓ |
| `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen` | **6 passed; 0 failed** | 6 passed | ✓ |
| `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_loc` | **9 passed; 0 failed** | 9 passed | ✓ |
| `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store` | **11 passed; 0 failed** | 11 passed | ✓ |
| `cargo test -p simthing-driver --test demo_0080_1` | **24 passed; 0 failed** | 24 passed | ✓ |
| `cargo test -p simthing-driver --test default_schedule_0080_1` | **30 passed; 0 failed** | 30 passed | ✓ |
| `cargo check --workspace` | **clean** (only 2 pre-existing warnings) | PASS | ✓ |

The only `cargo check` warnings are the **pre-existing** unused-import warnings (`EmlConsumerKind` in
simthing-core; `RF_CONTINUED_STATIC_512` in simthing-driver) — disclosed honestly in the impl report and
unrelated to R1. No new errors or warnings introduced by R1.

## 4. Evidence summary (against the acceptance checklist)

**Shape / layout (✓):** `GALAXY_SIDE = 20`; 400 row-major cells; `cell_index = y*20 + x` is the only dense
cell home (asserted per-cell); 13 ATLAS-BATCH-0 systems preserved as inert galactic occupants; `disruption`
= column 0, `location_status` = column 1, distinct. Occupants remain **contributors** (`separated_entries`)
into cells — the 400 grid cells stay cells, never replaced by occupants.

**Source production (✓):** pirate fleet → `+20.0` (`PirateDisruption` channel, `Pirate` owner); patrol
fleet → `−15.0` (`PatrolSuppression`, `Terran`); non-fleet system occupants → `0.0` (`InertSystem`).
Co-located pirate/patrol/inert contributors stay separated by `(channel, owner)` **before** `input_cell`
is netted (`input_cell = pirate_contribution − patrol_suppression`). Verified by the co-located test
(`input_cell == 5.0` for one pirate + one patrol) and the no-blind-sum test.

**Pinned recurrence (✓):** `DECAY=0.80, GAIN=1.00, FLOOR=0.0, CEILING=100.0, PIRATE_EMIT=20.0,
PATROL_SUPPRESS=15.0, H_WEIGHT=0.25` all present as named constants and asserted in tests.
`bounded_feedback_next(prev, input) = clamp(prev*0.80 + input*1.0, 0, 100)`; closed-form check
`bounded_feedback_next(10,5) == 13.0`. The recurrence is the existing whitelisted
`EmlGadgetInstanceSpec::BoundedFeedback` (reported in `bounded_feedback_gadget`).

**CPU-oracle parity (✓):** determinism (same inputs → identical field + checksum); no-source decay
(`50.0 → 40.0`, i.e. `0.8×`); **lone pirate converges monotonically to 100.0** over 32 ticks (the
`d* = 20/(1−0.8) = 100` saturation); **two patrols vs one pirate floors at 0.0** (`input = −10`); floor
and ceiling hold. *Note:* the `cpu_oracle` accessor and the run path share `execute_model`, so the parity
field is a **determinism** guarantee; the recurrence **math** is independently verified by the closed-form
expectation tests above — EC1's "verified against a CPU oracle" is satisfied in substance.

**Strict-sink diffusion (✓):** `source_col (0) ≠ target_col (1)`; diffusion writes only `location_status`
and never overwrites `disruption` (asserted: disruption unchanged after the pass); one dense von-Neumann
pass over 400 cells; falloff reaches the 4 neighbors and decays with distance (center > adjacent >
distance-two); **no edge-wrap / inter-tile bleed** (cells at the opposite grid edge stay 0.0).

**Deterministic heatmap artifact (✓):** 400-cell table, top-8 hotspots, total disruption, max cell,
occupied-cell count, stable checksum (`17de0080304b3da7`), markdown render; deterministic across replay
(full report equality + identical markdown + stable checksum). Hotspot rank-1 is the canonical pirate cell
`(4,14)` index 284 at 100.0; patrol cells remain suppressed at 0.0 with lower `location_status` than the
hotspot.

**Optional GPU cross-check (✓, honestly absent):** not implemented / not run; reported as
`gpu_cross_check = "NotRunCpuOraclePrimary"`. **No `GpuVerified` and no f32 bit-exact claim is made.** The
opening spec authorized CPU oracle as primary and GPU as optional; a source guard test asserts the R1
fixture contains no `create_shader_module` / `.wgsl` / `simthing_gpu`.

**Note (not a blocker):** the canonical *starting* field is intentionally sparse — one saturated hotspot
(10 stacked pirates) plus isolated suppressed patrol cells (total disruption 100). This is a faithful,
non-trivial consequence of the canonical fleet disposition, not a model defect; a richer spatial field
emerges only once fleets move/spread, which is the deferred R4/R5 work. EC1 ("non-trivial nonzero field,
hotspot near pirates, suppression near patrols") is fully met.

## 5. Stop-line confirmation (all held)

No SEAD movement · no GradientXY consumption · no R2 recursive reduce-up · no R3 mask-down · no
R4/R5/R6 · no REENROLL · no M-4A sparse-residency · no blockade/divert gate (the `CEILING=100` is field
saturation only, not the §6 gate) · no default `SimSession` pass-graph change (opt-in/default-off fixture;
`default_simsession()` is a no-op) · no new shader/WGSL/GPU kernel · no f32 bit-exact GPU claim. Each is
enforced both by report flags and by `forbidden`-request rejection diagnostics (verified by the rejection
tests) plus the source guard. No `docs/invariants.md` edit was required or made.

## 6. Next posture

- **R1 accepted / closed as implemented-pass.**
- **R2 remains unopened.** The recursive multi-tier reduce-up (system 10×10 → galactic) + faction-economy
  coupling + blockade/divert are the named R2 scope (§12.5) but are **not** authorized by this acceptance.
- Next engineering action requires a separate **`R2-OPEN`** opening-spec gate (Opus authors), routed as a
  distinct gate per the one-rung-at-a-time §0.5 cadence — or another explicit Opus authorization.

## 7. §0.5 self-check

Acceptance review only — **no code changed** (docs-only PR), **no `docs/invariants.md` edit**, no R2
implementation, no scope expansion, no shader/math/tolerance change, no `simthing-sim` semantic change, no
default session wiring. Verdict + status updates + next-gate posture only.
