# MOVEMENT-DECISION-INGRESS-0 results

Status: **COMPLETE — DA-GRADUATED / merged #1675 @ `65745802`** (ruling `5227908683`).

Authority and orientation:

- Canonical handoff: `handoffs/MOVEMENT-DECISION-INGRESS-0.hd.md`
- `HD-RECEIPT: f9e4f189e200`
- Fresh coding orientation: `ORIENT-RECEIPT: 63c78f668af0`
- Orientation rule stamp: `2f75375c5ae20caf`
- Owner reorientation comment: `5227348206`

## Landed production seam

The existing first-slice field runtime now reads the compact sealed threshold-event and matching threshold-emission streams for admitted field-cell loci. Those two seals mint the existing `StructuralCommitment`; no CPU field value, diagnostic decision, destination identity, route, predecessor, or planner can mint movement.

At the boundary, `MovementCommitment::admit` reattaches the sealed `(slot, col)` to exactly one admitted field-cell identity and proves exactly one N4 edge from the mover's current parent. `apply_movement_commitments` then uses the ordinary structural `Reparent` and routed overlay delivery path. Reparent changes only tree membership and allocator residency relation: logical `SlotIndex`, subtree slots, and the value plane remain unchanged.

Each successful move performs exactly one `simthing_core::owner_channel::bind_owner` on the moved subtree root using the destination cell's purely resolved owner. Descendants inherit; there is no participant stamping, movement owner table, or materialized effective-owner copy. Neutral `unowned` remains the total ordinary owner, and no legacy owner property was introduced.

The in-transit effect is an ordinary System instruction overlay whose real origin is the deciding cell. Its only admitted lifecycle is `UntilDissolvedWith { ArrivedAt { destination } }`; `ArrivedAt` resolves from the authoritative `ChildOf(destination)` residency relation. There is no `Permanent` variant.

Consuming moves resolve through the existing CostBand algebra with depth-one throttle. Free repositioning uses the same path as observation and produces `N=0, R=V`. The production validator recomputes the admitted draw immediately before structural apply, so a direct decrement or altered residue cannot enter.

## Focused proof and planted defects

The two focused witnesses are green: `cargo test -p simthing-sim --test movement_decision_ingress_0 -- --test-threads=1` is **5 passed, 0 failed**, and `cargo test -p simthing-driver --test movement_decision_ingress_0 -- --test-threads=1` is **1 passed, 0 failed**. Both use the real GPU path.

- Real crossing: GPU-sealed event + emission mint a structural commitment; cell A → cell B moves one edge; actor/cargo slots and every value-row bit stay fixed; exactly one owner-root bind changes inherited owner; the deciding-cell-origin arrival overlay attaches.
- Field authority: changing only the sealed field locus changes B versus C without editing cell identities; reversing locus order does not change the answer; an ambiguous locus fails closed instead of invoking CPU ordering. The full production session witness clones the same admitted identities and changes only which ordinary overlay weights the field potential; that changes the sealed crossing and moves the actor east versus south. The actor's current cell is excluded from candidate scans as a topology invariant, never selected or ranked on CPU.
- CostBand: consuming movement completes one unit and conserves exactly; free repositioning completes zero and preserves `R=V`.
- Arrival: `ArrivedAt` dissolves against the authoritative `ChildOf(destination)` residency relation.
- Production-path mutants RED: raw/CPU or hard-coded destination selection, row relocation, descendant owner stamping/materialized ownership, hard-coded or synthesized origin, `AtSessionEnd`, bare `UntilDissolved`, altered/direct-decrement draw, ambiguous binding, and missing cell endpoint. These referees exercise the production sealed scan, ordinary reparent/owner/overlay paths, and pre-apply validators rather than a test-only executor.

Existing owner-channel admission guards retain `legacy_owner_properties_remaining == 0` and reject an authored Owner named `unowned`; the movement diff creates no legacy owner property or owner identity authoring path.

## Scope disposition

This rung adds no `ActionBand` production type, action registry, movement planner, `Destination`, path/predecessor record, physical-row relocation, allocator/grant work, placement/extents physics, contention, Vector CostBand, 7.2, or 8.x surface. The sequential 7.1a determination and every later row remain TODO.

## DA graduation record

DEEP-TREE ruling `5227908683`: **PASS**. Both batteries re-run by the DA at the exact head (`simthing-sim` 5 passed / 0 failed; `simthing-driver` 1 passed / 0 failed), matching the claimed counts; `clearance` / `doctrine-scan` / `doctrine-exec` green at `b9fd883d`.

The `TEST-BUDGET INSPECT 1` heuristic (five `#[test]` functions without table-driven form) was ruled **justified with no suppression and no allowlist edit** — the five are distinct mechanisms with no shared `(input, expected)` shape, and one of them already consolidates the mutant family.

**Positional-identity carry-forward (not a defect of this rung).** Commit `b9fd883d` is a DA authorization in `scripts/ci/authorized_renames.tsv`: `compile_fail_line_122 → compile_fail_line_128` in `crates/simthing-core/src/overlay.rs`. It is a **line-shift, not a deletion** — the seal-proof is unchanged and must not be reverted. Six inserted lines (the `ArrivedAt` variant plus its doc comment) moved a test identity keyed by line number. Counted at head: **100 of 1173 inventory rows (8.5%) across 31 distinct files** carry a `_line_<N>` positional identity, so any insertion above one of them shifts it, fires the deletion guard, and requires a fresh authorization. **This will recur on every rung that edits those 31 files.** It is the same logical-versus-physical identity defect the slot-identity ruling fixed for rows, and it is worth a rung of its own rather than another hundred authorizations.
