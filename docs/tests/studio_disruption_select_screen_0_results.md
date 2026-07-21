# STUDIO-DISRUPTION-SELECT-SCREEN-0 Results

## Status
**PROBATION / DRAFT** — PR [#1420](https://github.com/khorum08/SimThing/pull/1420); DA enrollment ruling `5027107657`; Remand-5 governance `5028303162`. Production enrollment accepted through Remand-4 proofs. Owner OVL remains OPEN until orch accept + re-freeze + Owner capture. Prior freeze `715fdde4…fe58` is STOP provenance only.

## Identity SHAs
| role | full SHA |
|---|---|
| base / STOP tip | `8ffdaf610c4c9b48e76c673c155041fd9c5cde5d` |
| handoff_head | `9f1949d9cd54c997414ea7a87004741e3c423500` |
| implementation_code_sha | `d72036ce1fba5f7811e7b9a61db748f5e6d4beb5` |
| remand_4_first_tested_code_sha | `8bd4604c4187fd775529788c38d41c1a59b6b812` |
| evidence_head_sha | `0326b85985852730bde1d36fcc3adc27a1587dde` |
| remand_5_orientation_bind_sha | `1e5517c075c43b1a051eb9c53e022ee656dff337` |
| tested_code_sha | `a581097f74c980448a4720fe8b22098de2571883` |
| final_head_sha | `3aa28a2f8ab79a8345c84b63e4ace71f35867857` |

`implementation_code_sha` is production landing. Remand-4 first green battery was at `remand_4_first_tested_code_sha`. Remand-5 re-ran enrollment/readback + 12.3 + cargo check + Studio build at `tested_code_sha` (includes docs/governance through orientation regen; no executable delta vs Remand-4 tested tree). `final_head_sha` is the docs-only identity bind after that re-verify.

## DA ruling quote (`5027107657`) — operative authorization
> **GENERIC STRUCTURAL-ENROLLMENT CONTRACT: APPROVED, inline on #1420 (`system_target` on `location`)**  
> Preferred shape ADOPTED — semantics 1–7 as written, with four DA riders: §7 transitional zero-target; RF-5A spanned admission for unknown/duplicate/ambiguous/non-lattice; second-synthetic falsifier; placement-swap falsifier stays king.  
> Endorsed: exact-one `install_targets`; remove `from_values_for_test`; delete `attach_disruption_host_structural_placements`; TP explicitly places spatial field hosts.

DA `5027107657` is the authorized class-widening authority for Remand-4 surfaces outside the prior narrow `studio-live-ops-ui-clock` envelope (clausething hydrate enrollment, driver observation door internalization, canonical clause placement). Clearance may still report `DA-RESERVE(class-envelope-violation)` until orch/class harness absorbs that ruling; the sticky cites this comment explicitly.

## Production authority path implemented
1. Clause `location.system_target = "rowR_colC"` (combat vocabulary reuse).
2. Hydrate resolves onto embedded lattice → `grid_metadata` placement at that `(row,col)`.
3. `authored_live_profile_from_pack` joins `(row,col)` → generated `system_id` (`location_system_ids`).
4. Field-bearing bridge maps typed `disruption_presence` loci via structural authority + that join; exact-one install host cardinality.

Canonical TP: `terran_shipyard` → `row199_col80`; `pirate_outpost` → `row158_col110`. Neutral foundry synthetic uses vertical-seed lattice `row2_col3` / `row2_col4`.

## Remand 3 corrections retained
Typed disruption loci only; fail-loud total/partial mapping; biting 0→nonzero / refresh / max proofs; `ownership_volume` grammar stayed reverted.

## Proof matrix
| test | catches |
|---|---|
| canonical_host_system_moves_zero_to_nonzero_unrelated_stays_zero | Absent wall; open already nonzero |
| authored_system_target_swap_moves_system_id_with_zero_code_change | hard-coded system / ignored enrollment |
| two_typed_loci_on_one_enrolled_system_report_exact_max | sum/first instead of production max |
| live_map_refreshes_when_runtime_disruption_changes | frozen open map |
| structural_shell_absent_field_stays_typed_zero | shell invents nonzero |
| selected_star_telemetry_matches_live_map_and_piecewise | 12.3/live divergence |
| system_target_missing_unknown_and_duplicate_fail_loud | soft enrollment |
| structural_mapping_total_and_partial_miss_fail_loud | all-miss fail-soft |
| exact_one_install_target_host_cardinality_fail_loud | `.first()` selection |
| observation_door_unknown_property_role_and_host_fail_loud | silent observe misses |
| driver two_loci_same_system_report_exact_max_via_live_readback | test-local reduce clone |

## Local battery
### Remand-4 first green @ `8bd4604c…`
enrollment/readback (10), 12.3, 11.6, field elevate, TP elevate, TP clause author, driver observation, cargo check, Studio build, agent_scan hard FAIL=0, inventory drift, doc_budget — PASS.

### Remand-5 re-verify @ `tested_code_sha` `a581097f…`
| target | result |
|---|---|
| `cargo test -p simthing-mapeditor --test studio_live_disruption_readback_0` | PASS (10) |
| `cargo test -p simthing-mapeditor --test studio_disruption_select_screen_0` | PASS |
| `cargo check -p simthing-clausething -p simthing-driver -p simthing-mapeditor` | PASS |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS |
| `bash scripts/ci/gen_orientation.sh --check` | PASS |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `bash scripts/ci/anchor_check.sh --check` | PASS |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |

Executable tree unchanged vs Remand-4 tested code; Remand-5 re-verify binds governance tip.

## Scope Ledger
| | |
|---|---|
| Specified | Selected-star disruption screen + live STEAD readback + generic structural enrollment |
| Implemented | `system_target` hydrate enrollment + typed loci + fail-loud + production proofs |
| Proxied | none for production enrollment |
| Deferred | Owner OVL; re-freeze after orch accept |
| Out of scope | Spec mutation; WGSL; ownership_volume; synthetic attach; 12.5 |

## Conformance
piecewise YES · clamp YES · deselect YES · live STEAD map YES · system_target swap YES · exact max YES · tick refresh YES · shell 0.0 YES · 12.3 match YES · fail-loud enrollment YES · production enrollment YES · OVL OPEN

## Sticky disposition
Keep draft/open/unmerged. No re-freeze / Owner OVL until orchestration accepts Remand-5 governance return.
