# PR 11 Track A: Session Assembly

**Date:** 2026-05-22
**Status:** Accepted for PR 11 Track A implementation

## Context

`simthing-spec` exposes capability and scripted-event boundary handlers, while
`simthing-sim::BoundaryProtocol` owns the authoritative boundary sequence and
must remain unaware of the spec crate. `simthing-driver::SimSession` is the
first layer above both crates, so it can own spec runtime state and call the
handlers without reversing the dependency cleanup from PRs 9-11 Track B.

The load-bearing constraint is shadow authority. The handlers must run after
`BoundaryProtocol` reads canonical GPU values back into the CPU shadow, but
before structural boundary requests are applied. Running them directly from
`SimSession` before `BoundaryProtocol::execute` would use stale shadow values
and any handler shadow writes could be overwritten by the boundary readback.

## Decision

- Add a driver-owned `SpecSessionState`.
- Keep `simthing-sim` spec-free.
- Add feeder-level external threshold registration storage to
  `BoundaryProtocol` for capability unlocks and scripted-event triggers.
- Add a generic boundary hook to `BoundaryProtocol::execute` that runs after
  GPU value readback and alert extraction, before lifecycle, expiry,
  fission/fusion, and structural mutation.
- `SimSession` installs a `SpecSessionState`, syncs its threshold
  registrations into `BoundaryProtocol`, and uses the hook to call:
  - `CapabilityTreeBoundaryHandler::handle_capability_unlock_events`
  - `CapabilityTreeBoundaryHandler::sweep_on_prereq_met`
  - `ScriptedEventBoundaryHandler::handle_tick`
- `BoundaryRequest`s emitted by spec handlers are appended to the normal
  structural mutation request list and applied by the existing boundary path.

## V0 Scope

- Scripted events are session-global. `ScopeRef::Current` resolves through
  `SpecSessionState::scripted_current_slot`.
- `slot_to_thing` is rebuilt fresh from `SlotAllocator` each boundary.
- Diagnostics and capability notifications are collected in driver state.
- Capability runtime storage in the driver is keyed by
  `(owner_id, definition_id, tree_thing_id)` so multiple trees per owner can
  coexist. The underlying PR 5 handler still accepts per-owner maps, so the
  driver invokes it with temporary one-instance maps.
- Replay serialization of capability state, scripted cooldowns, diagnostics,
  and notification streams is documented as deferred. Structural overlay
  activations still flow through the existing boundary delta log.

## Consequences

- `simthing-driver` gains a `simthing-spec` dependency.
- `simthing-sim` gains no spec dependency.
- Empty-boundary skipping is disabled when installed spec state has scripted
  events, because predicate events and cooldowns need boundary-time ticks.
- Threshold rebuilds include external feeder-level registrations during full
  sync. Append-only integration for external registrations remains a later
  optimization.

