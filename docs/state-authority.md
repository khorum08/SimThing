# State authority doctrine

Testable invariants for where numeric truth lives during ticks vs boundaries.
Prevents shadow/GPU/`SimThing.properties` drift as new features land.

See also: `docs/agents.md` (implementation map), `crates/simthing-feeder/src/patcher.rs`
(within-day patches), `crates/simthing-sim/src/boundary.rs` (boundary sync).

---

## During ticks (within-day)

**Authoritative runtime numerics:** GPU `values` (and `output_vectors` after Passes 4–6).

The CPU **shadow** (`DispatchCoordinator::shadow`) is a row-major cache of `values`.
During ticks, queued transforms are folded into GPU intent deltas and applied
directly to `values` before Pass 0. Dirty shadow rows still exist for legacy or
direct CPU-side mutation paths, but normal tick-time Set/Add/Multiply does not
read rows back or upload them through the shadow.

### Within-day patch rules (`TransformPatcher`)

| Op | Allowed mid-day? | Rationale |
|----|------------------|-----------|
| `Set` | Yes | Folded into a GPU intent delta and applied before Pass 0. |
| `Add` / `Multiply` | Yes | Folded into a GPU intent delta and applied before Pass 0, preserving same-cell operation order without CPU readback. Direct `apply_one` remains a shadow-only helper and skips these ops unless called with `ShadowFreshness::GpuSynced`. |

RMW through the shadow without GPU sync is treated as unsafe: direct `apply_one`
increments `unsafe_rmw_skipped` and leaves the row clean.

### Player / AI intents (two-phase)

`PlayerIntent` and `AiIntent` follow the same contract:

1. **Mid-day (same tick):** the intent's `transform` delta is folded into the
   same GPU intent-delta buffer as feeder patches (Set, Add, and Multiply all land).
2. **Next boundary:** the full `Overlay` is structurally attached to the tree.
3. **Subsequent ticks:** Pass 3 applies attached overlays every tick (persistent effect).

For **Set**, mid-day apply and Pass 3 re-apply are **idempotent** (same constant).
For **Add/Multiply**, mid-day apply uses integrated GPU values; after boundary attach,
Pass 3 applies the overlay persistently without double-counting on the attach tick.

Integration coverage: `player_intent_mid_day_effect_lands_on_gpu_before_boundary`,
`player_intent_add_mid_day_uses_integrated_gpu_value`, `ai_intent_mid_day_effect_and_boundary_attach`.

---

## During boundary

**Sequence:**

1. GPU `values` are read back into `coord.shadow` (integration output is GPU-only
   between boundaries).
2. Shadow is **authoritative for mutations**: expiry, fission/fusion scar, structural
   maintainer, property projection on AddChild.
3. Full shadow is uploaded back to GPU (`upload_full_shadow`).

Fusion scar (`apply_fusion_scar`) mutates the parent's row in shadow, not
`SimThing.properties` — consistent with runtime truth living in GPU/shadow.

---

If boundary growth allocates beyond the current GPU slot capacity, the boundary
protocol grows `DispatchCoordinator`, `TransformPatcher`, and `WorldGpuState`
with amortized doubling before the final sync. The CPU shadow remains the
authoritative preservation source; GPU buffers are rebuilt and then receive the
full shadow upload.

## `SimThing.properties`

**Role:** semantic presence, layout defaults, and initial seed data when a node is
created or a property is first attached.

**Not authoritative** for live numeric values after simulation starts. Do not
inspect `node.properties[..].data` expecting current Amount/Velocity/Intensity;
use GPU readback, shadow, or `BoundaryProtocol::observe` /
`BoundaryProtocol::observe_live` (one row readback for UI/debug).

AddChild projects initialized semantic property values into shadow at boundary time;
Remove zeroes tombstoned subtree rows in shadow.

---

## Reduction + aggregate thresholds

Passes 4–6 write parent aggregates into `output_vectors`. Pass 0 snapshots
`output_vectors → previous_output_vectors` before each tick's reduction.

Pass 7 aggregate alerts (`THRESH_BUF_OUTPUT`) compare **previous vs current**
reduced output. A threshold fires on **crossing**, not while a value remains above
(or below) the threshold.

---

## Fission re-fire (recurring rebellions)

**Policy (2026-05):** Rebellions and similar fission-causing events **may recur**.
A parent keeps its `FissionTrigger` registration after spawning; if the activating
Amount re-crosses the threshold in a later tick/day, a **new child may spawn**.

No one-shot latch or cooldown. Idempotency within a **single boundary tick** only
(deduplicate duplicate events for the same `(parent, template_idx)`).

Integration coverage: `fission_refires_when_amount_re_crosses_threshold`.

---

## Fission lineage

`BoundaryProtocol::fission_lineage` persists `FissionLineageRecord`s across boundaries
for `FusionTrigger` registration.

Pruned when:

- Fusion executes (`lineage_removed` from fission step), or
- Either endpoint is tombstoned (`slot_of` returns `None` after Remove, fusion,
  or reparent). `BoundaryProtocol::execute` prunes after fission/fusion (step 6)
  and again after structural mutations (step 7/8).

---

## Replay fidelity

Structural history is captured in `BoundaryDeltaEntry` (including `OverlayDissolved`,
`AggregateAlert`, fission lineage). Optional per-frame `shadow_values` checkpoints
record post-boundary integrated numerics for audit/diff — replay does not re-run GPU
passes to reconstruct floats.

---

## Invariant checklist (for reviewers)

- [x] Mid-day RMW applies through GPU intent deltas without row readback (`DispatchCoordinator::tick`).
- [x] Boundary mutation reads GPU into shadow first (`BoundaryProtocol::execute` step 1).
- [x] `SimThing.properties` is not live sim state (shadow/GPU authoritative; see expiry tests).
- [x] Aggregate thresholds use `THRESH_BUF_OUTPUT` (`aggregate_alert_registration_surfaces_at_boundary`).
- [x] Lineage pruned when endpoints disappear (`remove_after_fission_prunes_lineage`).
- [x] Overlay dissolution recorded in delta log (`OverlayDissolved` + replay driver).
