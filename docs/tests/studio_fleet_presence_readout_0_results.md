# STUDIO-FLEET-PRESENCE-READOUT-0 Results

## Status

PROBATION / proof-present / DA-review-pending.

## PR / Branch / Merge

- Branch: `coder/studio-fleet-presence-readout-0`
- PR: PENDING
- Merge: not merged

## Current Defect Or Mission

Implement a read-only fleet presence/transit snapshot for loaded Studio authority without movement authority, gameplay writes, raw property-id leakage into mapeditor, or Studio GUI execution on Linux.

## Implemented Changes

- Added `simthing_spec::fleet_presence_snapshot` and `fleet_presence_snapshot_with_transit`, returning typed `FleetPresenceRecord` rows with `OwnerRef`, optional posture, and `FleetPresenceLocation::{Anchored, InTransit}`.
- Moved TP fleet posture/home-system property-id authority from Clausething-private constants to `simthing-spec`, with Clausething hydration importing the shared constants.
- Added a mapeditor-only `StudioFleetPresenceMap` projection keyed by generated system id, wired into `StudioLiveSessionBridgeReadout`.
- Added focused regression tests for the typed spec contract, canonical TP ClauseThing hydrate, and mapeditor consumption without raw fleet property ids.

## Boundary / Constitution Checks

- HD-RECEIPT: `7404a5f3f2f6`
- ORIENT-RECEIPT: `112905597598`
- REQUIRED-ANCHORS: none
- ANCHOR-ACK working set: `field-policy-time-decisions@ae2d4c2c0c7d`, `spec-fidelity-anti-ceremony@add4dbbc267a`, `founding-ontology-invariants@b960ed2d493d`, `drift-detectors-six-line@af20f8122501`
- Read-only: helper walks `ScenarioSpec` authority and constructs cloned readout rows only; tests assert the source spec root is unchanged.
- No raw property ids in mapeditor: production mapeditor source imports the typed helper and includes no TP fleet property-id constants or literals.
- No movement authority / CPU planner / Spec mutation: transit is an explicit typed readback override for the snapshot contract only; no scheduling, movement write, driver, kernel, or WGSL code changed.

## Validation Commands

- BLOCKED locally: `cargo check -p simthing-spec` (`cargo` not found in PATH)
- BLOCKED locally: `cargo check -p simthing-clausething` (`cargo` not found in PATH)
- BLOCKED locally: `cargo check -p simthing-mapeditor` (`cargo` not found in PATH)
- PASS: `bash scripts/ci/agent_scan.sh`
- BLOCKED locally: focused `cargo test` (`cargo` not found in PATH)

## Files Changed

- `crates/simthing-spec/src/spec/fleet_presence.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/studio_fleet_presence_readout_0.rs`
- `crates/simthing-clausething/src/hydrate_scenario.rs`
- `crates/simthing-clausething/tests/studio_fleet_presence_readout_0.rs`
- `crates/simthing-mapeditor/src/studio_fleet_presence.rs`
- `crates/simthing-mapeditor/src/studio_live_session_bridge.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_fleet_presence_readout_0.rs`
- `docs/design_0_0_8_6_studio_live_ops.md`
- `docs/tests/current_evidence_index.md`
- `scripts/ci/test_inventory.tsv`

## Evidence Lifecycle

- Results doc: `docs/tests/studio_fleet_presence_readout_0_results.md`
- Evidence index: updated under `0.0.8.6 Studio Live Ops`
- Test inventory: added 3 permanent behavior-regression rows
- Design ladder row: `12.4` stamped `PROBATION / proof-present / DA-review-pending`

## Known Gaps

- Studio GUI is Windows-only; no local GUI proof in this headless Linux rung.
- This local environment has no Rust toolchain on PATH, so cargo validation is deferred to CI or a Rust-equipped reviewer environment.
- Default structural-shell live session may emit no in-transit fleets; typed transit fixture covers the contract variant.

## Deferred Next Rung

`STUDIO-FLEET-ICONS-0` consumes this snapshot for presentation-only fleet icons.

## DA Status

Expected route remains `DA-RESERVE(gate-wiring)` per handoff. This relay must not self-graduate.
