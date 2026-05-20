# State authority doctrine

Testable invariants for where numeric truth lives during ticks vs boundaries.
Prevents shadow/GPU/`SimThing.properties` drift as new features land.

See also: `docs/agents.md` (implementation map), `crates/simthing-feeder/src/patcher.rs`
(within-day patches), `crates/simthing-sim/src/boundary.rs` (boundary sync).

---

## During ticks (within-day)

**Authoritative runtime numerics:** GPU `values` (and `output_vectors` after Passes 4–6).

The CPU **shadow** (`DispatchCoordinator::shadow`) is a row-major cache of `values`.
It is uploaded to the GPU for dirty rows **before** Pass 0 each tick.

### Within-day patch rules (`TransformPatcher`)

| Op | Allowed mid-day? | Rationale |
|----|------------------|-----------|
| `Set` | Yes | Absolute write; safe even if shadow lags integration by one tick. |
| `Add` / `Multiply` | No (skipped) | Read-modify-write needs current GPU-integrated value; shadow may be stale. Counted in `PatcherStats::unsafe_rmw_skipped`. |

Long-term, Add/Multiply may become GPU-side delta commands with explicit readback
authority. Until then, use **boundary-only** RMW or **Set** mid-day.

### Player / AI intents (two-phase)

`PlayerIntent` and `AiIntent` follow the same contract:

1. **Mid-day (same tick):** the intent's `transform` delta is applied to the shadow
   via `apply_one` — **Set only** lands; Add/Multiply are skipped like any patch.
2. **Next boundary:** the full `Overlay` is structurally attached to the tree.
3. **Subsequent ticks:** Pass 3 applies attached overlays every tick (persistent effect).

For **Set**, mid-day apply and Pass 3 re-apply are **idempotent** (same constant).
For **Add/Multiply**, only the attached overlay drives GPU integration after boundary;
there is no mid-day double-count.

Integration coverage: `player_intent_mid_day_effect_lands_on_gpu_before_boundary`,
`ai_intent_mid_day_effect_and_boundary_attach`.

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

## `SimThing.properties`

**Role:** semantic presence, layout defaults, and initial seed data when a node is
created or a property is first attached.

**Not authoritative** for live numeric values after simulation starts. Do not
inspect `node.properties[..].data` expecting current Amount/Velocity/Intensity;
use GPU readback, shadow, or `BoundaryProtocol::observe`.

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

## Fission lineage

`BoundaryProtocol::fission_lineage` persists `FissionLineageRecord`s across boundaries
for `FusionTrigger` registration.

Pruned when:

- Fusion executes (`lineage_removed` from fission step), or
- Either endpoint is tombstoned (`slot_of` returns `None` after Remove, fusion,
  or reparent). `BoundaryProtocol::execute` prunes after fission/fusion (step 6)
  and again after structural mutations (step 7/8).

---

## Invariant checklist (for reviewers)

- [ ] Mid-day patch uses only `Set`, or documents why GPU readback was done first.
- [ ] Boundary mutation that changes numerics updates shadow (or reads GPU first).
- [ ] New code does not treat `SimThing.properties` as live sim state.
- [ ] Threshold on aggregates uses `THRESH_BUF_OUTPUT`, not leaf `values`.
- [ ] Lineage records are pruned when endpoints disappear.
