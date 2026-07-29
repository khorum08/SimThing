# ANCHOR-TABLE-SURFACE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.3)
- Status: **PROBATION** (coder lane; orch owns `/clearance`)
- HD-RECEIPT: `a41ced4721f0`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- DA plan-review: comment [`5120052669`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5120052669) (Fable) — five binding sharpenings applied in-diff
- Orch remand-1: comment [`5120259758`](https://github.com/khorum08/SimThing/pull/1491#issuecomment-5120259758) — GPU observation authority discharge
- Orch remand-2: comment [`5120410047`](https://github.com/khorum08/SimThing/pull/1491#issuecomment-5120410047) — GPU table writer authority discharge
- Board dispatch: comment `5119876551` on issue `#1332`
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## DA sharpenings (binding)

1. Dropped `falloff_params` from the typed table (no in-rung consumer; 5.4+ owns falloff as authored EML map data).
2. Core row carries `Option<BandIndex>`; i32 sentinel (`ANCHOR_BAND_NONE_POD`) minted only at POD encode.
3. Anchor table is derived/reconstructible — absent from `BoundaryDeltaEntry` / replay wire (referee + census).
4. `observation_bypass_census.sh` sources shared `lib_cfg_test_mod_spans.sh` (5.2 remand-4 brace-balanced filter); unpiped true exit required in relay.
5. Explicit Unobserved-fixture referee (zero rows for dark locus); handoff surfaces already include `simthing-mapeditor`.

Watch-item: O(rows)-per-boundary refresh cost-model comment landed on `refresh_anchor_table_magnitudes` (admission / structural-remap fallback only after remand-2).

## Orch remand-1 discharge (`5120259758`)

- `AnchorTableSnapshot::from_session` builds from `WorldGpuState::read_anchor_table` + driver-local POD decode (not `BoundaryProtocol` CPU clone).
- Census arm fails on `proto.anchor_table()` / CPU clone / production writer-staging reads.
- Disagree referee: corrupt transient CPU init → hosted + Studio seam still return GPU value.
- No CI allowlist / triage TSV edits; GPU decode kept driver-local (reach-log governance decline).

## Orch remand-2 discharge (`5120410047`)

- Persistent CPU `AnchorTable` removed from `BoundaryProtocol`; admission mint is transient upload-only.
- Fused GPU writer: `anchor_table_maintain.wgsl` companion after threshold scan (ordered `reg_idx` crossing apply + parallel magnitude refresh).
- Structural remaps: transient GPU readback → typed apply → upload (no stored CPU table).
- `BandCrossingDelta` remains wire/replay evidence only — does not mutate a CPU observation table.
- Census arm 8 fails CPU shadow writer / per-boundary full-upload substitute; allows admission + structural-remap sites only.

## Landed contract

- One derived STEAD GPU-resident `AnchorTable` POD twin as sole production observation **and** dynamic writer authority.
- Writers only: admission mint (transient), fused GPU threshold companion, typed remap (transient readback).
- Studio/hosted observation via `AnchorTableSnapshot` GPU door.

## Proof battery (local)

| Proof | Result |
|---|---|
| Focused `anchor_table_surface_0` referees | PASS — 10 passed, 0 failed |
| Studio+hosted disagree (transient CPU init) | PASS |
| GPU crossing matrix ↔ oracle | PASS (rising/falling/exact-edge/no-crossing/multi-edge) |
| GPU remap identity (AddChild / fusion / fission) | PASS |
| TP 25 Anchored / 0 Unobserved cardinality | PASS |
| `bash scripts/ci/observation_bypass_census.sh` | PASS — unpiped EXIT:True (incl. remand-2 writer arm) |
| Design / orient | 5.3 **PROBATION**; Active → `FIELD-SWEEP-IR-PROBE-0` |

Final head / tested_code_sha: PR-body-bound only (this file does not self-hash).
