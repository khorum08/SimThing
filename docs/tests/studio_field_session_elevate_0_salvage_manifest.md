# STUDIO-FIELD-SESSION-ELEVATE-0 Salvage Manifest

Source reviewed read-only: PR #1405 exact head `34bb730fb24b550eeac520e83fbf5e2408a0f7c4`. RF-4 base: `c0e2202694bc9c2a329e5a6d4620a078c7ba71ea`. No merge, rebase, or wholesale cherry-pick was used.

## Accepted / already landed on the post-RF-3 base

| File/hunk family | Disposition |
|---|---|
| `Cargo.lock` dependency state | Already byte-identical on the RF-4 base; no salvage commit needed. |
| `crates/simthing-mapeditor/src/app/ui.rs` Studio loader/telemetry controls | Accepted as behavioral provenance; RF-4 retains the loader and replaces emission-only OVL claims with recursive Owner telemetry. |
| `crates/simthing-mapeditor/src/clause_scenario_picker.rs` staged clause ingest/profile preservation | Already landed; retained unchanged. |
| `crates/simthing-mapeditor/src/lib.rs` exports | Already landed; retained unchanged. |
| `crates/simthing-mapeditor/src/session.rs` authored-profile attachment | Already landed; retained unchanged. |
| `crates/simthing-mapeditor/src/studio_live_session_bridge.rs` field-bearing path/fallback/telemetry structure | Accepted selectively; rebuilt on recursive Arena RF and canonical authority rather than copied. |
| `crates/simthing-mapeditor/tests/studio_field_session_elevate_0.rs` loader/fallback/pause controls | Already landed; retained as non-load-bearing regression coverage. |
| `crates/simthing-workshop/Cargo.toml` and `tests/tp_field_session_elevate_0.rs` workshop homing | Already landed; RF-4 adds the scenario proof only in workshop. |

## Rejected / superseded

| File/hunk family | Reason |
|---|---|
| PR #1405 design/orientation status hunks | Stale pre-RF-3 governance and receipts; superseded by RF-3 graduation and RF-4 handoff identity. |
| Old `studio_field_session_elevate_0_results.md` status and executable identity | Stale blocked/Remand-5 evidence and superseded binary; cannot support current recursive RF or Owner capture. |
| Emission `source_slot`/`source_col` as the OVL load-bearing proof | Explicitly rejected by RF-4; emission remains diagnostic only. |
| Synthetic owner-shell fallback hunk | Rejected by the one-tree Owner fence. Small fixtures bind unresolved diagnostic keys to their existing root; canonical RF uses only real authority nodes. |
| Need/`weight_profile` relabel or presentation-only mirror | Rejected because no admitted GameMode/open consumer exists. Routed to bounded RF-5 approval instead. |
| Old handoff, triage, inspect, and inventory descriptions | Stale identities/counts and pre-RF-3 claims; must be regenerated from the new tested head. |

The audit found no #1405 code hunk requiring direct transplantation: accepted implementation/test files were already present on the post-RF-3 base. RF-4 changes are new, narrow recursive-RF and evidence work.
