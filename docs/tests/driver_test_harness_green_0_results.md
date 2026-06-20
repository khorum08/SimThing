# DRIVER-TEST-HARNESS-GREEN-0 — simthing-driver package test hygiene

**Status:** PROBATION  
**Date:** 2026-06-18  
**Base:** `master` after PR #766 / SIMTHING-SIM-DEVDEP-SEAM-0  
**Branch:** `driver-test-harness-green-0`

## Orientation answers (pre-edit)

| Question | Answer |
|---|---|
| Exact bh3 failure? | Not a Rust compile error. Windows `os error 740` (elevation required) when Cargo spawns integration-test binaries whose **crate names contain `install`** — UAC installer-name heuristic (same class as PR8-WIN-HYGIENE). PR #766 mislabeled this as compile failure. |
| bh3 classification? | **ACTIVE_GUARD** / LIVE_GUARDRAIL (BH-3-AUTHORING-0 install bridge). Superseded as a **standalone binary** only; guard retained by merge into `ct_bh3_closeout_sample_driver.rs`. |
| Production driver logic needed? | **No** — test/harness rename and consolidation only. |
| Scenario/sim/GPU/Studio semantics change? | **No**. |
| `cargo test -p simthing-driver terran_pirate` after fix? | **PASS** (exit 0). |
| Targeted Terran Pirate tests? | **PASS** — compile 5/5; resident tick 3/3 executed + 1 filtered. |
| Gu-Yang/PALMA deferred? | **Yes** — STEAD §10; no implementation in this PR. |
| Deleted/retained artifacts? | Deleted `bh3_authoring_installs_existing_operator.rs` (guard merged). Renamed `palma_path_5_install_session_property.rs` → `palma_path_5_session_property.rs` (UAC heuristic). Retained all live ledger files; no scratch/temp dumps added. |

## Why this is not hygiene

Gu-Yang/SaturatingFlux falloff borders and PALMA reach fields are driver/sim/gpu integration surfaces. Package-level `cargo test -p simthing-driver terran_pirate` must run honestly so agents cannot hide unrelated harness breakage behind narrower `--test` invocations. PR #766 restored the sim-owned seam but left this validation gate SKIP/blocked.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated — DRIVER-TEST-HARNESS-GREEN-0 row added |
| `docs/tests/simthing_sim_devdep_seam_0_results.md` | PROBATION source (PR #766) | Retained unchanged |
| `docs/tests/driver_test_harness_green_0_results.md` | PROBATION (this file) | Created |
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated § DRIVER-TEST-HARNESS-GREEN-0 |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority fixture | Untouched |
| `bh3_authoring_installs_existing_operator.rs` | ACTIVE_GUARD binary (UAC-blocked name) | Deleted; guard merged into `ct_bh3_closeout_sample_driver.rs` |
| `palma_path_5_install_session_property.rs` | ACTIVE_GUARD binary (UAC-blocked name) | Renamed to `palma_path_5_session_property.rs` |

## bh3_authoring failure summary

```
cargo test -p simthing-driver terran_pirate
  -> builds all driver integration-test binaries
  -> spawns bh3_authoring_installs_existing_operator-*.exe
  -> os error 740: The requested operation requires elevation
```

Direct PowerShell execution of the same `.exe` succeeds; the failure is Cargo child-process spawn under Windows UAC installer-name detection when `install` appears in the binary stem. Documented precedent: PR8-WIN-HYGIENE / `ct_bh3_closeout_sample_driver.rs` rename.

## Classification of bh3_authoring

**ACTIVE_GUARD** — fast BH-3 field-operator install bridge (ClauseThing hydrate → spec admission → driver GPU bridge surfaces, default-off, forbidden-token scan). Related live guards remain in `simthing-clausething/tests/bh3_authoring_parse.rs` and `simthing-spec/tests/bh3_operator_spec_admission.rs`.

## Repair action and rationale

1. **Merged** `bh3_authoring_installs_existing_operator` into `ct_bh3_closeout_sample_driver.rs` as `bh3_field_operator_install_bridge_surfaces_without_runtime_action` — preserves guard coverage without an `install`-named Cargo binary.
2. **Deleted** standalone `bh3_authoring_installs_existing_operator.rs` — superseded as a separate binary; not deleted without replacement.
3. **Renamed** `palma_path_5_install_session_property.rs` → `palma_path_5_session_property.rs` — second pre-existing UAC blocker on the same package-filtered command (documented in `docs/tests/r1_default_workspace_purge_results.md`).

No production driver logic, scenario authority, sim tick, GPU primitive, shader, or Studio behavior changed.

## Proof that simthing-sim seam remains clean

- `crates/simthing-sim/Cargo.toml` has no `simthing-driver`, `simthing-mapeditor`, or `simthing-spec` dev-dependencies.
- Sim tests do not import driver/spec/mapeditor or deserialize scenario JSON.
- Terran Pirate scenario→driver→sim resident GPU proof remains in `simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs`.

## Terran Pirate driver/sim/GPU proof preservation

- Driver compile: 4 slots, 4 ops, Sum-over-INPUT_LIST, expected fork adjacency — **PASS** (5/5 compile tests).
- AO-WGSL-0 compatibility asserted — **PASS**.
- Resident GPU tick CPU/GPU parity `[20, 80, 20, 20]` when adapter available — **PASS** (3/3 executed tests).
- ProofReadback does not leak into subsequent None tick — **PASS**.
- Scenario authority not mutated by tick/proof readback — **PASS**.

## Production None-tick/readback preservation

Unchanged — READBACK-SCOPE-0 / RESIDENT-TICK-0 behavior preserved; validation sweep green on `debug_readback_scope` and resident-tick proof test.

## Gu-Yang / PALMA contract carried forward (STEAD §10 — deferred)

**Gu-Yang falloff borders:** grid N4 over structural `(col,row)`; `saturating_flux_choke_threshold` + `structured_field_stencil`; saturating flux choke column + bounded field readout; bounded theater first slice; typed atlas deferral; no border/frontline service.

**PALMA reach:** grid N4 over structural `(col,row)`; `min_plus_stencil` + `w_impedance_compose`; D is a field, W is impedance/feedstock; bounded theater first slice; typed atlas deferral; no routes, predecessors, came_from, path objects, or pathfinding engine.

Hyperlane link adjacency uses bounded `AccumulatorOp` Sum-over-INPUT_LIST. Gu-Yang/PALMA use grid N4 structural adjacency within bounded theaters. Do not conflate.

## Big-endian / portable byte-proof backlog (deferred)

- Explicit little-endian byte helpers
- Cross-platform byte-order evidence
- Replacing host-endian bytemuck casts in canonical artifact byte proofs

## Forbidden-token scan

Changed test sources scanned for new route/predecessor/pathfinding/border/frontline service semantics — **none introduced**. Existing FORBIDDEN_HOT_PATH guard lists in merged test unchanged in intent.

## Tests added/changed/deleted

| Action | File |
|---|---|
| Added test fn | `ct_bh3_closeout_sample_driver.rs` — `bh3_field_operator_install_bridge_surfaces_without_runtime_action` |
| Deleted | `bh3_authoring_installs_existing_operator.rs` |
| Renamed | `palma_path_5_install_session_property.rs` → `palma_path_5_session_property.rs` |

## Validation commands

Run with `CARGO_BUILD_JOBS=1` on Windows to avoid linker PDB contention when building the full driver integration-test matrix.

| Command | Status | Notes |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | |
| `cargo check -p simthing-driver` | PASS | |
| `cargo test -p simthing-driver terran_pirate` | PASS | Package filter runs; Terran Pirate tests 8 executed |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_compile` | PASS | 5/5 |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_resident_tick` | PASS | 3/3 executed, 1 filtered |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver bh3_field_operator` | PASS | 1/1 merged BH-3 guard |
| `cargo test -p simthing-driver --test palma_path_5_session_property` | PASS | 7/7 after rename |
| `cargo check -p simthing-sim` | PASS | |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS | |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS | |
| `cargo check -p simthing-spec` | PASS | |
| `cargo test -p simthing-spec` | PASS | Full suite |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS | |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS | |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS | |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS | |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS | |
| `git diff --check` | PASS | |

## Windows / resource-limit notes

- Parallel driver integration-test linking can hit `LNK1201`/`LNK1285` corrupt PDB on Windows; `cargo clean -p simthing-driver` + `CARGO_BUILD_JOBS=1` resolves.
- UAC installer-name heuristic (`os error 740`) affects Cargo-spawned test binaries with `install` in the crate stem; fix is rename (PR8 precedent), not `#[ignore]` or feature gates.

## Files changed

- `crates/simthing-driver/tests/ct_bh3_closeout_sample_driver.rs`
- `crates/simthing-driver/tests/bh3_authoring_installs_existing_operator.rs` (deleted)
- `crates/simthing-driver/tests/palma_path_5_session_property.rs` (renamed from `palma_path_5_install_session_property.rs`)
- `docs/tests/driver_test_harness_green_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

## Deleted / archived artifacts

- `crates/simthing-driver/tests/bh3_authoring_installs_existing_operator.rs` — guard merged; UAC-blocked binary removed.

## Deferred work

- Gu-Yang falloff borders implementation
- PALMA reach field implementation
- Big-endian/portable byte-proof hardening

## DA status

**PROBATION** — no DA/owner sign-off claimed. SIMTHING-SIM-DEVDEP-SEAM-0 and DRIVER-TEST-HARNESS-GREEN-0 remain PROBATION until promoted by DA/owner approval.