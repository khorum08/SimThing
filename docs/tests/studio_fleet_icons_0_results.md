# STUDIO-FLEET-ICONS-0 Results

## Status
**PROBATION / DRAFT** — PR [#1426](https://github.com/khorum08/SimThing/pull/1426); HD-RECEIPT `c88f057a19fc`; Owner icon-base rider binding; [OVL] open until Owner screenshot.

## Identity
| role | value |
|---|---|
| base_sha | `61abf63b` (12.3 merge) |
| tested_code_sha | `9824ad0c46d5dd9d50a7459b175ef13d0aeb56cc` |
| branch | `coder/studio-fleet-icons-0` |
| HD-RECEIPT | `c88f057a19fc` |
| ORIENT-RECEIPT | `2c9fde39d1d6` |

## What landed
1. **Renderer-agnostic descriptor layer** (`studio_fleet_icons.rs`): silhouette id, owner tint, anchor/transit placement, side, orientation, scale — no Bevy/render types.
2. **One-site silhouette DATA** (`FLEET_ICON_SILHOUETTE_DESTROYER` outline table); look change is a one-site edit.
3. **Narrow `FleetIconRenderer` seam** + `RecordingFleetIconRenderer` + `DummySecondFleetIconBackend` (forward-compat falsifier consumes identical descriptors).
4. **Placement laws:** selected-owner anchored fleets Right; others Left (mirror); transit at 0.30 along source→dest; arrival snaps to Anchored; scale ≤ 75% base max star blur.
5. **Existing-mechanism mesh path** (`sync_fleet_icons_system`): outline → Mesh + StandardMaterial only; no new pipeline/WGSL.
6. **Studio_ops Telemetry** fleet icon table (owner / placement / side / scale) for Owner OVL.
7. First-landing docs: 12.3 **DA-GRADUATED** @ `#1420`/`61abf63b`; pointer → 12.5; orientation regenerated.

## Proof matrix
| test | catches |
|---|---|
| selected_owner_right_others_left_mirror | wrong side / non-mirror |
| transit_thirty_percent_and_orientation_toward_dest | wrong lane fraction / orientation |
| arrival_snap_to_anchor_slot | sticky transit after arrival |
| scale_bound_seventy_five_percent_of_base_max_star_blur | oversize icons |
| dummy_second_backend_consumes_identical_descriptors | backend-coupled descriptors |
| silhouette_is_one_site_data | scattered look hardcoding |
| mapeditor_presence_map_feeds_descriptors | 12.4 wire break |
| mesh_draw_plans_are_outline_pose_only | new pipeline requirement |
| fleet_icons_module_has_no_wgsl_or_spec_mutation_surface | fence drift |
| unit battery in `studio_fleet_icons.rs` (9) | same laws at unit layer |

## Local battery
| target | result |
|---|---|
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --lib studio_fleet_icons` | PASS 9/9 |
| `cargo test -p simthing-mapeditor --test studio_fleet_icons_0` | PASS 9/9 |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/gen_orientation.sh --check` | PASS |

## Scope ledger
| | |
|---|---|
| Specified | Tiny fleet icons from 12.4 snapshot; descriptor base + narrow renderer seam; OVL telemetry |
| Implemented | Descriptor layer, seam, mesh draw plans, Bevy sync, ops rows, inventory, 12.3 stamp |
| Proxied | none |
| Deferred | Owner OVL screenshot |
| Out of scope | movement authority; Spec mutation; new WGSL/pipeline; 12.5 self-graduation |

## Known gaps
- Owner [OVL] open — screenshot against Studio_ops fleet icon table.
- Default sessions may express no InTransit; transit is contract-proven via fixture records.
