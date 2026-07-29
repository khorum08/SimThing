# ANCHOR-TABLE-SURFACE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.3)
- Status: **PROBATION** (coder lane; orch owns `/clearance`)
- HD-RECEIPT: bound by handoff file
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- DA plan-review: comment [`5120052669`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5120052669) (Fable) — five binding sharpenings applied in-diff
- Board dispatch: comment `5119876551` on issue `#1332`
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## DA sharpenings (binding)

1. Dropped `falloff_params` from the typed table (no in-rung consumer; 5.4+ owns falloff as authored EML map data).
2. Core row carries `Option<BandIndex>`; i32 sentinel (`ANCHOR_BAND_NONE_POD`) minted only at POD encode.
3. Anchor table is derived/reconstructible — absent from `BoundaryDeltaEntry` / replay wire (referee + census).
4. `observation_bypass_census.sh` sources shared `lib_cfg_test_mod_spans.sh` (5.2 remand-4 brace-balanced filter); unpiped true exit required in relay.
5. Explicit Unobserved-fixture referee (zero rows for dark locus); handoff surfaces already include `simthing-mapeditor`.

Watch-item: O(rows)-per-boundary refresh cost-model comment landed on `refresh_anchor_table_magnitudes`.

## Landed contract

- One derived STEAD `AnchorTable` written only by admission mint, fused `BandCrossingDelta` updates, and typed `AnchorRemapSection`.
- GPU POD twin uploaded via `WorldGpuState::upload_anchor_table`; consumers read `AnchorTableSnapshot` only.
- Studio field/disruption + hosted observation migrated off `GpuValuesSnapshot::from_session`.

## Proof battery (local)

| Proof | Result |
|---|---|
| Focused `anchor_table_surface_0` referees | PASS — 6 passed, 0 failed |
| `cargo test -p simthing-core --lib anchor_table` | PASS — 2 passed |
| `bash scripts/ci/observation_bypass_census.sh` | PASS — unpiped EXIT:True (`PASS(observation-bypass-census): all arms green`) |
| `bash scripts/ci/plan_struct_typing_census.sh --selftest` | PASS — shared brace-balanced filter EXIT:True |
| Orientation regen | PASS — Active pointer `FIELD-SWEEP-IR-PROBE-0` |

Final head / tested_code_sha: PR-body-bound only (this file does not self-hash).
