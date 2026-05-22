# SimThing Todo Log

Current parking state after the V6 suspended-overlay and capability-fission
implementation landed on `master` as `f39fe6d`.

## Done

- [x] Add `OverlayLifecycle::Suspended { when_activated }`.
- [x] Keep suspended overlays out of CPU evaluator and GPU overlay-delta prep.
- [x] Add boundary-time `ActivateOverlay` and `SuspendOverlay` requests.
- [x] Record overlay activation/suspension in the boundary delta log.
- [x] Replay overlay activation/suspension transitions.
- [x] Add `active` attribution to observability overlay contributions.
- [x] Ensure suspended overlays do not force empty-boundary work.
- [x] Add `FissionTemplate::clone_capability_children`.
- [x] Clone capability containers on opted-in fission templates.
- [x] Allocate fresh IDs and slots for cloned capability subtrees.
- [x] Copy cloned capability shadow rows.
- [x] Remap cloned overlay `affects` from parent owner to spawned owner.
- [x] Pre-grow boundary slot headroom for cloned capability subtrees.

## Next

- [ ] Add a GPU boundary integration test proving an activated suspended overlay
      appears in the next Pass 3 delta upload and affects values on the next
      tick.
- [ ] Add an end-to-end fission replay test for a cloned capability subtree,
      verifying replay reconstructs the full spawned subtree payload.
- [ ] Decide whether capability-container names should remain hardcoded
      (`tech_tree`, `national_ideas`, `talent_tree`) or move behind a registry
      supplied by a future studio layer.
- [ ] Continue B2: reduce fission-growth boundary cost by retaining or
      append-patching GPU topology/threshold buffers only when slot ordering and
      event-kind semantics remain deterministic.
- [ ] Expand scenario loading from builtin selectors to real RON tree/registry
      definitions.
- [ ] Build the `simthing-studio` capability tree authoring layer.

## Notes

- Suspended overlays are CPU-visible and GPU-free until activated.
- Capability cloning is opt-in per `FissionTemplate` and defaults to `false`,
  preserving existing cohort/location fission behavior.
- No WGSL shader changes were needed for V6.
