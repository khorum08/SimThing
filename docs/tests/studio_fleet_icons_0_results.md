# STUDIO-FLEET-ICONS-0 Results

## Status
**PROBATION / DRAFT / remand-2 corrected** — PR [#1426](https://github.com/khorum08/SimThing/pull/1426); HD-RECEIPT `c88f057a19fc`; Owner [OVL] **not authorized**.

## Identity
| role | value |
|---|---|
| base_sha | `61abf63bba21ef95fdbd783040d69615376d7a1e` |
| implementation_code_sha | `5015dc1869eb32f5b5d00af6da5f023c4b87de9b` |
| tested_code_sha | `5015dc1869eb32f5b5d00af6da5f023c4b87de9b` |
| final_head_sha | *(docs bind tip after this battery)* |
| branch | `coder/studio-fleet-icons-0` |
| HD-RECEIPT | `c88f057a19fc` |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| remands | `5075963624`, `5076130563` |

## Remand-2 corrections
1. **Shared lifecycle planner:** `sync_fleet_icons_system` consumes `fleet_icon_entity_ops(frame, live_ids)` for Update/Despawn/Spawn — same planner as `FleetIconSceneState`.
2. **Shared cleanup collector:** production `scene_cleanup` calls `collect_galaxy_scene_cleanup_entities`; headless + production-module tests exercise the same helper (omitting fleet_icons shrinks the list).
3. **Mesh/transform conversion proofs:** pure `fleet_icon_outline_geometry` / `fleet_icon_transform_data` plus production-module tests of `fleet_icon_outline_mesh` and `fleet_icon_transform` (XZ/+Y, Bevy rotation matches pure data).
4. **Clearance identity:** PR body uses separate unbulleted `implementation_code_sha`, `tested_code_sha`, `final_head_sha`, `coverage_basis` fields.

## Proof matrix
| battery | count | result |
|---|---|---|
| unit `studio_fleet_icons` | 13 | PASS |
| unit `galaxy_render::tests` (fleet icon) | 3 | PASS |
| integration `studio_fleet_icons_0` | 11 | PASS |

## Local battery
| target | result |
|---|---|
| `cargo test -p simthing-mapeditor --lib studio_fleet_icons` | PASS 13/13 |
| `cargo test -p simthing-mapeditor --lib galaxy_render::tests` | PASS 3/3 |
| `cargo test -p simthing-mapeditor --test studio_fleet_icons_0` | PASS 11/11 |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS |
| inventory / orient / doc-budget | PASS |
| `agent_scan` | INSPECT (TEST-BUDGET justified) |

## Scope ledger
| | |
|---|---|
| Specified | Remand-2 shared production path for lifecycle, cleanup, geometry |
| Implemented | Bevy ops from planner; shared cleanup; mesh/transform proofs |
| Deferred | Owner OVL; hosted closeout artifact-expiry maintenance |
| Out of scope | new pipeline/WGSL; Spec mutation; self-graduation |

## Known gaps
- Hosted Doctrine may still stop at wall-clock closeout artifact expiry (orchestration owns maintenance; not touched here).
