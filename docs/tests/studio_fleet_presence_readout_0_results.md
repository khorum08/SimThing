# STUDIO-FLEET-PRESENCE-READOUT-0 Results

## Status

PROBATION / proof-present / DA-review-pending.

## PR / Branch / Merge

- Branch: `coder/studio-fleet-presence-readout-0`
- PR: #1355
- Merge: not merged

## Current Defect Or Mission

Implement a read-only fleet presence/transit snapshot for loaded Studio authority without movement authority, gameplay writes, raw property-id leakage into mapeditor, or Studio GUI execution on Linux.

## Implemented Changes

- Added `simthing_spec::fleet_presence_snapshot`, returning typed anchored `FleetPresenceRecord` rows with `OwnerRef`, optional posture, and `FleetPresenceLocation`.
- Moved TP fleet posture/home-system property-id authority from Clausething-private constants to `simthing-spec`, with Clausething hydration importing the shared constants.
- Added a mapeditor-only `StudioFleetPresenceMap` projection keyed by generated system id, wired into `StudioLiveSessionBridgeReadout`.
- Added focused regression tests for the typed spec contract, canonical TP ClauseThing hydrate, and mapeditor consumption without raw fleet property ids.
- Remediation: removed the public caller-supplied transit override path. `InTransit` remains a readout contract variant with a spec-private unit fixture until authoritative sim/STEAD movement state exists.
- DA remand: removed local scan-suppression markers from the two fleet kind reads, recorded both as accounted `SPEC-LOWERER-KIND-READ` inspect sites, and deleted the bespoke mapeditor source-guard helper in favor of diff plus grep proof.

## Boundary / Constitution Checks

- HD-RECEIPT: `7404a5f3f2f6`
- ORIENT-RECEIPT: `112905597598`
- REQUIRED-ANCHORS: none
- ANCHOR-ACK working set: `field-policy-time-decisions@ae2d4c2c0c7d`, `spec-fidelity-anti-ceremony@add4dbbc267a`, `founding-ontology-invariants@b960ed2d493d`, `drift-detectors-six-line@af20f8122501`
- Read-only: helper walks `ScenarioSpec` authority and constructs cloned readout rows only; tests assert the source spec root is unchanged.
- Accounted scan inspect: `AGENT-SCAN-VERDICT: INSPECT delta_inspect=2`; both sites are `fleet_presence.rs` read-only `Fleet` enumeration, with matching `inspect_justifications.tsv` and `triage_log.tsv` rows.
- No raw property ids in mapeditor: production mapeditor source imports the typed helper and includes no TP fleet property-id constants or literals; grep proof covers `crates/simthing-mapeditor/src/`.
- No movement authority / CPU planner / Spec mutation: production snapshot remains anchored-only and exposes no caller-facing API to manufacture transit; no scheduling, movement write, driver, kernel, or WGSL code changed.

## Validation Commands

- PASS: `cargo check -p simthing-spec`
- PASS: `cargo check -p simthing-clausething`
- PASS: `cargo check -p simthing-mapeditor`
- PASS: `cargo test -p simthing-spec --test studio_fleet_presence_readout_0`
- PASS: `cargo test -p simthing-spec transit_contract_is_test_private_until_authoritative_state_exists`
- PASS: `cargo test -p simthing-clausething --test studio_fleet_presence_readout_0`
- PASS: `cargo test -p simthing-mapeditor --test studio_fleet_presence_readout_0`
- PASS: `bash scripts/ci/test_inventory_drift_check.sh`
- PASS: `bash scripts/ci/doctrine_selftest.sh`
- INSPECT accounted: `bash scripts/ci/agent_scan.sh` (`AGENT-SCAN-VERDICT: INSPECT delta_inspect=2`)
- PASS: `rg -n "TP_FLEET_POSTURE_PROPERTY_ID|TP_FLEET_HOME_SYSTEM_PROPERTY_ID|8_301_500|8_301_501" crates/simthing-mapeditor/src` returned no matches

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
- `scripts/ci/inspect_justifications.tsv`
- `scripts/ci/triage_log.tsv`

## Evidence Lifecycle

- Results doc: `docs/tests/studio_fleet_presence_readout_0_results.md`
- Evidence index: updated under `0.0.8.6 Studio Live Ops`
- Test inventory: added 3 permanent integration behavior-regression rows plus the private `transit_contract_is_test_private_until_authoritative_state_exists` unit behavior-regression row
- Design ladder row: `12.4` stamped `PROBATION / proof-present / DA-review-pending`

## Known Gaps

- Studio GUI is Windows-only; no local GUI proof in this headless Linux rung.
- Default structural-shell live session may emit no in-transit fleets; typed transit fixture is private to the spec crate until authoritative movement state exists.

## Deferred Next Rung

`STUDIO-FLEET-ICONS-0` consumes this snapshot for presentation-only fleet icons.

## DA Status

Expected route remains `DA-RESERVE(class-envelope-violation)` per remand. This relay must not self-graduate.
