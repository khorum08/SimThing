# Codex/Cursor Handoff 7 — `ATLAS-BATCH-0-STORE-GPU` Contract

**Recipient model: Cursor** · **Role: production implementation agent**
**From:** Opus (design authority) · **Date:** 2026-06-03 · **Gate:** authored + accepted.
**Predecessors:** GEN / LOC / PACK (EC-A2a) / PACK-GPU (EC-A2b GpuVerified) / STORE (EC-A3 CPU) all PASS.

> ## DESIGN-AUTHORITY RULINGS (locked)
> **(§3) Route → Option B: fixture-only driver harness around existing generic primitives.** Verified:
> `AccumulatorOpSession` is public; `CombineFn::{EvalEML, Sum}`, `ScaleSpec::ByColumn` exist; `CMP_EQ` /
> `SELECT` opcodes are **whitelisted**; `EmlExpressionRegistry::{new, register}` is driver-usable (a
> fixture registers its own owner-mask tree — **no core whitelist edit**); precedent `mobility_idroute0`
> already masks by identity. **No new WGSL, no new `CombineFn`/`AccumulatorRole`, no `simthing-gpu`/
> `-core` edits.** Cursor **calls** these public APIs; it must not edit them.
> **(§5) Parity → bit-exact `f32::to_bits()` (ExactDeterministic)** over **integer-valued** fixture
> contributions: a masked CMP_EQ select + a fixed-order contiguous `Sum` of integers is exactly
> representable and order-deterministic (the `c6` exact-reduction-parity lineage). **Fallback:** if the
> GPU masked path shows genuine ordering nondeterminism, fall back to **GpuVerified (expect L∞ = 0 for
> integer sums)** and **explicitly state exact parity deferred** — do **not** make a false exact claim.
> **Scope note:** STORE-GPU proves the OWNER **masked-reduction composition** in a fixture against the
> STORE CPU oracle. It does **not** wire it into a session pass graph and does **not** implement R3
> capability-tree mask-down — the OWNER masked-reduction *runtime* and R3 stay parked.

---

## 1. Harness

**Fixed base (cite on handoff back):** 1) `docs/design_0_0_8_0.md` §0 · 2) `docs/invariants.md`
(Scenario Proof; **AccumulatorOp v2**; `EvalEML` whitelist; **`GpuVerified` vs `ExactDeterministic`**;
semantic-free `simthing-sim`) · 3) `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5
(ladder; STORE closure; STORE-GPU = this gate; OWNER routing §12.4) · 4)
`docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` · 5) `crates/simthing-core/src/accumulator_op.rs`
(the masked-reduction vocabulary — `EvalEML`/`CMP_EQ`/`Sum`/`ScaleSpec::ByColumn`) · 6)
`docs/workshop/field_policy_track.md`.

**Rung-local (ephemeral):** `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs` (the CPU
oracle = `StoreOracle`); `docs/tests/scenario_0080_2_atlas_batch_0_store_report.md`;
`crates/simthing-sim/tests/c8a_eml_infrastructure.rs` (worked example of building an
`EmlExpressionRegistry` + `AccumulatorOpSession` from a test). **Do not grow the fixed base.**

## 2. Established decisions — DO NOT re-derive (Opus-locked)

- **Fixture-only, GPU-gated, deterministic.** `store_gpu.rs` includes STORE privately
  (`#[path = "dress_rehearsal_atlas_batch_0_store.rs"] mod store;`). **No `lib.rs` export.** **Call**
  `simthing-gpu`/`simthing-core` public APIs; **edit neither** (nor `simthing-sim`).
- **The STORE CPU oracle (`StoreOracle`) is the authority.** GPU output must match its entries keyed by
  `(location_id, cell_index, channel, owner)`. Do not re-derive a second oracle; consume STORE's.
- **GPU path = existing primitives only:** materialize the fixture contributions into a values buffer
  with owner + channel columns; register an **`EvalEML` CMP_EQ owner-mask** tree in a local
  `EmlExpressionRegistry`; reduce co-located contributions into per-`(channel, owner)` cell columns via
  `CombineFn::Sum` (+ `ScaleSpec::ByColumn` for the mask) on an `AccumulatorOpSession`; read back; compare
  to `StoreOracle`. Integer-valued contributions (exactly f32-representable) → bit-exact target.
- **Values are integers, generic** (no gameplay recipes/rates). This is the first rung to materialize
  owner/channel **numeric columns** (STORE kept typed descriptors) — that materialization is in scope;
  gameplay computation is not.
- **No R1/R2/R3/R4:** no diffusion/BoundedFeedback, no economy/stockpile, no capability mask-down, no
  FIELD_POLICY/gradient/exact-sqrt/threshold/`BoundaryRequest`, no movement/REENROLL, no combat, no session wiring.

## 3. Deliverables

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store_gpu.rs` — the masked-reduction fixture
  (materialize columns; build registry + ops + session; run; compare to `StoreOracle`); `STATUS_PASS`
  const stating **EC-A3-gpu (masked-reduction parity vs STORE oracle); parity = ExactDeterministic
  bit-exact (or GpuVerified-fallback if noted)**.
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store_gpu.rs` (tests §4).
- `docs/tests/scenario_0080_2_atlas_batch_0_store_gpu_{report.md, status_row.md,
  cargo_test_2026_06_03.txt, parity_2026_06_03.txt}`.

## 4. Tests (target `dress_rehearsal_atlas_batch_0_store_gpu`)

CPU/metadata (always run): 1) `store_gpu_status_matches_gate` (id `ATLAS-BATCH-0-STORE-GPU`; EC-A3-gpu
only; no R1/R2/R3/R4/economy/FIELD_POLICY/movement/combat) · 2) `store_gpu_consumes_accepted_store_oracle`
(consumes `StoreOracle`; mutates STORE not at all) · 8) `no_semantic_shader_or_gameplay_inputs` (the GPU
path receives only generic keys/values/owner/channel masks/buffers — no map/faction/gameplay) · 9)
`no_r1_r2_r3_r4_behavior`.

GPU-gated (`SIMTHING_RUN_GPU_TESTS=1`; skip ≠ PASS): 3) `gpu_parity_full_store_table` (GPU == `StoreOracle`
under the §5 parity standard) · 4) `gpu_preserves_10_pirate_shared_cell_channels` (the 10 canonical
pirate fleets sum only into `PiratePresence` / `FleetStrength(Pirate)`) · 5)
`gpu_preserves_constructed_planet_patrol_pirate_distinction` · 6)
`gpu_owner_indexed_entries_do_not_blind_sum_by_position` (Terran vs Pirate at one cell never merge) · 7)
`gpu_channel_entries_do_not_blind_sum_by_position`. If env-gated and skipped, that is **not** PASS
evidence; the raw log must show the GPU tier ran on an adapter.

## 5. Raw evidence

`$env:SIMTHING_RUN_GPU_TESTS=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt`
**Use `*>&1` (PowerShell), not `2>&1`** — `2>&1` on a native exe wraps cargo stderr as `NativeCommandError`
and a non-zero exit *looks like a crash* (this bit STORE). Raw log must show: the command/env var, GPU
tier ran, adapter name, **0 ignored**, `N passed; 0 failed`. Parity report (`…_parity_2026_06_03.txt`):
adapter, CPU oracle entry count, GPU entry count, co-location cases, **exact-match status (or L∞)**,
owner/channel leakage checks, whether EC-A3-gpu closed, any skipped tests. If no adapter / skipped: keep
the diagnostic log, do **not** mark PASS, do **not** close the production doc, return **blocked** to Opus.

## 6. Docs update (AFTER green, GPU tier actually ran)

`docs/design_0_0_8_0_consumer_pulled_production_track.md`: mark **`ATLAS-BATCH-0-STORE-GPU` PASS for
EC-A3-gpu**; state the **parity standard used** (ExactDeterministic bit-exact, or GpuVerified-fallback
with exact deferred); state the **CPU STORE oracle was matched**; list the proven co-location cases +
adapter; state the **OWNER masked-reduction is proven as a fixture composition only — not wired into a
session; R3 stays parked**; R1/R2/R4 unimplemented; M-4A sparse-residency parked; REENROLL parked. One
`docs/worklog.md` entry. Do not mark PASS until the raw log + parity report are committed.

## 7. Files Cursor MAY edit / MUST NOT edit

**MAY create/edit:** the four `store_gpu` files; **after green only** —
`docs/design_0_0_8_0_consumer_pulled_production_track.md`, `docs/worklog.md`.
**MUST NOT edit:** `docs/design_0_0_8_0.md`; `docs/invariants.md`;
`…/dress_rehearsal_atlas_batch_0_{gen,loc,pack,pack_gpu,store}.rs` + accepted artifacts;
`crates/simthing-driver/src/lib.rs`; **`crates/simthing-gpu/**`, `crates/simthing-core/**`,
`crates/simthing-sim/**`** (call their public APIs only — editing any is a stop condition).

## 8. Stale artifacts

Delete only superseded duplicate STORE-GPU logs after a clean final GPU-enabled green log. Never delete
accepted GEN/LOC/PACK/PACK-GPU/STORE artifacts. If none: "Deleted obsolete artifacts: none found."

## 9. Stop conditions — STOP and escalate if it seems to need

new WGSL · a new `CombineFn`/`AccumulatorRole` · editing `simthing-gpu`/`-core`/`-sim` · a non-whitelisted
EML opcode (anything beyond `CMP_EQ`/`SELECT`/arithmetic/`Sum`) · runtime `match kind` on a real
`SimThingKind` · CPU map planner · economy/stockpile/recipe · disruption heatmap/diffusion ·
capability-tree mask-down · FIELD_POLICY/exact-sqrt/threshold/`BoundaryRequest` · REENROLL/movement · combat ·
session pass-graph wiring · M-4A sparse-residency. **STORE-GPU proves a generic masked storage/reduction
parity vs the STORE oracle — nothing more.**

## 10. Handoff-back format

```
Recipient model: Opus  ·  Role: design authority
ATLAS-BATCH-0-STORE-GPU implemented (EC-A3-gpu; OWNER masked-reduction fixture composition only).
GPU tier ran: yes/no · adapter: <name> · parity: ExactDeterministic bit-exact | GpuVerified(L∞=<>)
Raw log: …store_gpu_cargo_test_2026_06_03.txt (N passed; 0 failed)  ·  oracle entries == GPU entries: <n>
Co-location cases proven on GPU: 10-pirate; constructed planet+patrol+pirate.
Deliverables / deviations / open questions: <...>
§0.5 self-check: holds 1–6 — <one line>
```

## 11. §0.5 self-check (this contract)

Holds 1–6: STORE-GPU proves a **generic** owner/channel masked-reduction (the §12.4 OWNER mechanism)
on GPU against the accepted CPU oracle, via existing whitelisted primitives — no new WGSL, no
`match kind`, no `simthing-sim` semantics, no default wiring, no allocation outside the recursive tree,
no CPU planner. The fixture-composition-only scope keeps the OWNER masked-reduction *runtime* + R3
parked; the ExactDeterministic-with-honest-GpuVerified-fallback parity standard prevents a false exact
claim while pursuing the strongest provable result (integer masked sums, unlike PACK-GPU's f32 diffusion).
