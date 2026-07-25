# STUDIO-FLEET-ICONS-0 Results

## Status
**PROBATION / DRAFT / remand-1 corrected** — PR [#1426](https://github.com/khorum08/SimThing/pull/1426); HD-RECEIPT `c88f057a19fc`; Owner [OVL] **not authorized** until orch accept.

## Identity
| role | value |
|---|---|
| base_sha | `61abf63bba21ef95fdbd783040d69615376d7a1e` (12.3 merge) |
| implementation_code_sha | `4b24a90a5d4f082468745f2256873dbad1191926` |
| tested_code_sha | `4b24a90a5d4f082468745f2256873dbad1191926` |
| final_head_sha | *(docs bind commit after battery)* |
| branch | `coder/studio-fleet-icons-0` |
| HD-RECEIPT | `c88f057a19fc` |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| remand | `5075963624` |

## Remand-1 corrections
1. **Production renderer seam:** `MeshOutlineFleetIconRenderer` implements `FleetIconRenderer`; Bevy `sync_fleet_icons_system` applies only `production_fleet_icon_render_frame` draw plans. Dummy second backend shares the same draw-contract fingerprint; wrong-descriptor bypass diverges.
2. **Per-system star blur scale:** `admitted_base_max_star_blur_world` / `admitted_base_star_blur_by_system` from star `sprite_scale` + unselected near visual (not `selected_star_scale_multiplier`). Each icon capped ≤75% of its own anchor star blur.
3. **Scene cleanup:** `scene_cleanup` includes `fleet_icons`; reveal strips pending markers; pure cleanup-id falsifier bites.
4. **Nose / legibility:** Bevy-consistent yaw; transformed local nose faces star/destination (dot>0.99); map-plane mesh normal +Y legible top-down.
5. **Production lifecycle:** headless `FleetIconSceneState` proves side flip, add/remove, tint update, zero fleets, cleanup re-open with overlapping fleet ids.
6. **PR body / identities:** refreshed at handback.
7. **Hosted artifact-expiry:** left to orchestration (no unrelated deletions).

## Proof matrix
| battery | count | result |
|---|---|---|
| unit `studio_fleet_icons` | 11 | PASS |
| integration `studio_fleet_icons_0` | 11 | PASS |

## Local battery (required by remand)
| target | result |
|---|---|
| `cargo test -p simthing-mapeditor --lib studio_fleet_icons` | PASS 11/11 |
| `cargo test -p simthing-mapeditor --test studio_fleet_icons_0` | PASS 11/11 |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/gen_orientation.sh --check` | PASS |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |
| `bash scripts/ci/agent_scan.sh` | INSPECT delta_inspect=2 (TEST-BUDGET justified) |

## Scope ledger
| | |
|---|---|
| Specified | Remand-1 production seam, per-system scale, cleanup, nose proofs, lifecycle |
| Implemented | Seam frame + Bevy apply; admitted star blur; cleanup; nose/plane; lifecycle tests |
| Proxied | none |
| Deferred | Owner OVL; hosted artifact-expiry maintenance |
| Out of scope | new pipeline/WGSL; movement authority; Spec mutation; self-graduation |

## Known gaps
- Owner [OVL] not authorized.
- Hosted Doctrine Scan wall-clock artifact-expiry is repo-wide debt (orchestrator routes separately).
