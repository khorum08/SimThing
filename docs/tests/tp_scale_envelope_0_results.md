# TP-SCALE-ENVELOPE-0 Results

## Status

PROBATION / INSPECT(1). This rung proves the 1500-star disc generation -> lattice -> RF admission -> link lowering -> install path at scale, but it does not self-mark COMPLETE. The live-adapter session-open leg is attempted and skip-aware; on this machine it is blocked by a driver/kernel reduction-topology upload rejection during `initial_gpu_sync`, so compact mapping readback is not claimed.

## What changed

- Added `crates/simthing-clausething/tests/tp_scale_envelope.rs` with a focused 1500-star disc scale-envelope proof.
- Wired `MapGenResourceFlowOptions::capacity_budget` through the MapGen RF lowerer so budgeted effective caps are consumed before final `ResourceFlowSpec` validation.
- Updated the 0.0.8.5 design row and evidence index to route this rung as PROBATION / INSPECT(1), not COMPLETE.

## Accepted substrate consumed

TP-RF-CAPACITY-AMENDMENT-0 is COMPLETE — DA/Owner-cleared. This rung consumes that accepted capacity path; it does not reopen the RF capacity amendment.

## Scale envelope proof

The new test `tp_scale_envelope_disc_1500_admits_installs_with_budget`:

- Generates the 1500-star elliptical disc from seed `770421`, producer lattice request `300`, connected hyperlanes, and deterministic generation.
- Lowers the generated scenario into the existing lattice hierarchy. The honored structural frame is derived from authored placements, exceeds the standard dense-field cap, and remains within the producer lattice request.
- Consumes the accepted RF capacity budget to admit 1500 deposit participants and 1500 gridcell participants across two RF arenas with `participants_per_arena = 2048`.
- Lowers hyperlane topology through existing link/lane-coupling surfaces with widened test-time caps only.
- Confirms dense Movement-Front authoring over the scale frame typed-defers to the atlas rung instead of pretending full-grid field execution exists.
- Installs through `install_atomic` with the resolved RF capacity budget and the observed 7505-slot scale footprint.

## Load-bearing proofs

Required local validation:

- `cargo check -p simthing-mapgenerator` - PASS (existing warnings only).
- `cargo check -p simthing-clausething` - PASS (existing warnings only).
- `cargo check -p simthing-driver` - PASS (existing warnings only).
- `cargo test -p simthing-mapgenerator topology_stead` - PASS but matched 0 tests because `topology_stead` is an integration-test binary name, not a function filter.
- `cargo test -p simthing-mapgenerator --test topology_stead` - PASS, 9/9.
- `cargo test -p simthing-mapgenerator connectivity` - PASS; also ran matching connectivity/report tests.
- `cargo test -p simthing-mapgenerator --test connectivity` - PASS, 7/7.
- `cargo test -p simthing-clausething --test tp_scale_envelope tp_scale_envelope_disc_1500_admits_installs_with_budget -- --nocapture` - PASS, 1/1. Local output includes the skip-aware live-adapter inspect line below.
- `bash scripts/ci/gen_digest.sh --check` - PASS.
- `bash scripts/ci/doctrine_scan.sh` - PASS, failures=0 inspect=0, reliability RELIABLE, scanner self-test SKIPPED.

## INSPECT / triage

INSPECT(1): the local adapter exists, but `SimSession::open_from_spec` for the 7505-slot scale pack trips a caught wgpu validation panic during `WorldGpuState::upload_reduction_topology` from `initial_gpu_sync` (`Queue::write_buffer` would overrun a 24-byte destination buffer). The test reports this as a skip-aware live-adapter boundary and does not fabricate compact mapping readback. The admission/install proof remains load-bearing through `install_atomic`.

No `scripts/ci/triage_log.tsv` row has been added by this rung.

## Scope Ledger

- Phase 1 scenario content: none.
- Terran/Pirate ownership, fleets, factories, cohorts, combat, diplomacy, fronts, or `ai_will_do`: none.
- Route solver/pathfinding/predecessor surfaces: none.
- Semantic strings below spec: none added.
- CI allowlist/addendum/scanner edits: none.
- New `AccumulatorRole`: none.
- Per-tick allocation or atlas scheduler: none.
- RF capacity amendment status: consumed as accepted substrate; not reopened.

## Graduation routing

Graduation routing (for DA — why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE (local doctrine); rung still INSPECT(1) on live-adapter session proof
  Triage entries:      none
  Risk class:          data-deliverable + scale-proof
  Falsification check: Verify 1500-star disc generation/topology/admit/install path runs through the accepted widened capacity budget; verify no Phase 1 scenario content, no semantic runtime leakage, no new AccumulatorRole, no per-tick allocation, no atlas scheduler.
  Recommended posture: deep — this is the install-scale gate for every later Terran-Pirate scenario rung; accept only if the proof exercises the real install path.

## Known gaps / next

- DA/Owner review must adjudicate the live-adapter `initial_gpu_sync` reduction-topology inspect before promoting the rung.
- Phase 1+ Terran-Pirate content remains gated behind this scale proof.
