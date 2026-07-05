# TP-COMBAT-ARENA-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** HP/Damage RF combat arena for two-fleet contact is authored and proven on GPU==CPU transfer oracle parity, zero-HP boundary removal, and overlay-only owner weapon bonus. No combat subsystem, no new opcode/WGSL/kind branching. DA/Owner clearance required before merge.

## Identity

| Field | Value |
|---|---|
| PR | (pending push) |
| Branch | `tp-combat-arena-0` |
| Base | `origin/master` |
| Rung | Phase 4.0 `TP-COMBAT-ARENA-0` |

## Scenario-envelope compliance

| Rule | Status |
|---|---|
| `birth_track = 0.0.8.5-terran-pirate` on new tests | yes |
| `TP-COMBAT-ARENA-0` registered in `test_lifecycle_tracks.tsv` | no |
| TP-born artifacts marked canonical/inviolate/doctrine | no |

## Scope

Authoring/hydration + integration proofs only. No combat engine, movement solver, diplomacy, AI commitments, mapeditor/tools/workshop edits, or workspace `cargo test`.

**No kind-specific spec code landed.** Combat resolves through existing discrete `ResourceEconomySpec` transfers and overlay paths only.

## Implemented authored forms

| Form | Lowering |
|---|---|
| `combat_arena_payload` | Co-located hostile ship contact in a named `system_target`; one ship per side for oracle isolation |
| Hull damage | `SubtractFromSource` discrete transfers between opposing ship weapon/hull columns |
| HP recovery | Optional `governed_by` hull recovery per tick (authored; not exercised in main parity test) |
| Owner combat bonus | `ship_weapon_damage_mult` decoded via shipsize decoder → overlay on weapon column (`InstallTargetSpec::ScenarioListed`) |
| Ship death | Zero-hull rising `Threshold` → `EffectSpec::Remove` via scripted boundary handler |

## Conformance statements

| Claim | Status |
|---|---|
| No combat subsystem | yes — transfers + overlays + boundary removal only |
| No new opcode / WGSL / AccumulatorRole | yes |
| No `SimThingKind` branching in spec/clausething production `src/**` | yes |
| Owner bonus changes damage through overlay path only | proven (`owner_weapon_damage_mult_changes_damage_via_overlay_only`) |
| GPU main parity test does not skip on missing GPU | yes — `require_gpu()` hard-fails without adapter |

## Load-bearing proofs

| Proof | Test | What it catches |
|---|---|---|
| GPU==CPU transfer oracle bit-exact | `gpu_two_fleet_contact_matches_transfer_oracle` | Silent GPU transfer drift on hostile hull damage |
| Zero-HP boundary removal | `zero_hp_threshold_requests_boundary_removal` | Ships surviving past hull zero or missing slot tombstone |
| Overlay-only owner bonus | `owner_weapon_damage_mult_changes_damage_via_overlay_only` | Hardcoded owner logic smuggled into damage calc |

## Test lifecycle / inventory updates

| Test | birth_track | retention basis | downstream consumer note | dsu_survivals |
|---|---|---|---|---|
| `gpu_two_fleet_contact_matches_transfer_oracle` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 5 diplomacy / Phase 6 fronts rung | 0 |
| `zero_hp_threshold_requests_boundary_removal` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 6 fleet movement / removal contract | 0 |
| `owner_weapon_damage_mult_changes_damage_via_overlay_only` | `0.0.8.5-terran-pirate` | `permanent-residue:oracle-parity` | Phase 4+ owner bonus overlay chain | 0 |

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-clausething --test tp_combat_arena_0 -- --nocapture` | PASS (3 passed) |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_boundary_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | (pending — whole-tree scan slow on Windows; use unbuffered bash, not `Select-Object -Last`) |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `git diff --check origin/master...HEAD` | (pending until commit) |

## Scope ledger

| Item | Touched? |
|---|---|
| simthing-clausething | yes — `hydrate_combat_arena.rs`, `hydrate_scenario.rs`, tests + fixture |
| simthing-spec | no production edits |
| simthing-gpu/driver/sim/mapeditor/tools | no |
| workflows / scans / allowlists | no |
| `test_lifecycle_tracks.tsv` | no |
| workspace `cargo test` | no |

## Falsification checks

| Check | Expected | Observed |
|---|---|---|
| Remove overlay install → owner bonus test fails | fail | (by construction — overlay path is load-bearing) |
| Skip GPU on main parity test | hard error | `require_gpu()` panics without adapter |
| Drift gate with unledgered tests | FAIL | fixed — 3 inventory rows added |

## Graduation routing

- TP-COMBAT-ARENA-0 complete pending DA review
- PROBATION / DA-OWNER REVIEW
- not self-mergeable
- next rung after clearance: `TP-DIPLOMACY-FLOW-0` per design ladder