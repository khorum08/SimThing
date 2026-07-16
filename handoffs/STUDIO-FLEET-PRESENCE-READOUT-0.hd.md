---
rung: STUDIO-FLEET-PRESENCE-READOUT-0
kind: rung
track: 0.0.8.6
base_sha: 4d3f4467f57377478c5dda999518da5dded1b117
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed 2026-07-15 resumption of 0.0.8.6 (Phase 12) after HD-track detour; manual-progression, fresh cold-start coder. Read-only re-entry rung — NO movement/gameplay authority. Studio GUI is Windows-only; build/tests are headless CLI on Linux (see docs/agents.md cloud caveats)."
surfaces: ["crates/simthing-spec", "crates/simthing-clausething", "crates/simthing-mapeditor", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["movement authority", "new gameplay semantics", "raw property-ids in mapeditor", "kernel/WGSL changes", "CPU planner", "Spec mutation"]
required_checks: ["cargo check -p simthing-spec", "cargo check -p simthing-clausething", "cargo check -p simthing-mapeditor", "agent-scan", "focused cargo test"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "movement-or-write-authority-touched", "raw-property-id-leak-to-mapeditor"]
---
## BUILD
- Canonical spec/clausething helper that walks loaded authority for `SimThingKind::Fleet` and returns,
  per fleet: owner ref, posture, and **anchor system id**. Snapshot contract:
  `Anchored(system_id)` OR `InTransit { source_system_id, dest_system_id }`.
- Transit is expressed **only** when the sim/STEAD movement state says so; the default TP session may
  express none — the contract must still carry the InTransit variant.
- Property-id authority stays in spec/clausething (TP fleet property ids live in
  `hydrate_scenario.rs`); mapeditor consumes the helper's typed snapshot, **never raw ids**.
- Wire the helper through the existing Studio bridge to a mapeditor-consumable presence map keyed by
  generated system id (mirror the 9.x observe/bridge pattern; no new bridge surface).
## FENCES
- **Read-only**: no writes to fleet/movement/field state; no scheduling or tick changes; no new
  gameplay systems; no Spec mutation. Snapshot-consistent per tick.
- No raw property ids cross into `simthing-mapeditor` — the helper owns id→snapshot translation.
- Holds over the structural-shell fallback session (fail-soft to empty/None, fail-loud on readback error).
## EXIT-PROOF
- Headless proofs: fleet snapshot returns owner/posture/anchor for the canonical TP session;
  InTransit variant round-trips through a fixture even if the default session emits none;
  mapeditor consumes typed snapshot (compile-enforced: no raw-id path); read-only (no state mutation
  under snapshot calls). Name each regression the test catches.
- `cargo check` on the three crates + `agent_scan` green; focused `cargo test` (GPU legs skip cleanly).
- PROBATION stamp LEADS the 12.4 exit-proof cell in-diff; Active open rung advanced on graduation.
- DA authors the graduation stamp at merge (ruling 6); relay quotes this HD-RECEIPT.
