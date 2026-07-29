# ANCHOR-TABLE-SURFACE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.3)
- Status: **COMPLETE — DA-GRADUATED / merged #1491 @ d9544c52** (DA verified censuses true-exit 0, referee 13/0, studio 10/0, driver 146/0/13 across 66 harnesses on RTX 4080/Vulkan)
- HD-RECEIPT: `a41ced4721f0`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- DA plan-review: comment [`5120052669`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5120052669) (Fable) — five binding sharpenings applied in-diff
- Orch remand-1: comment [`5120259758`](https://github.com/khorum08/SimThing/pull/1491#issuecomment-5120259758) — GPU observation authority discharge
- Orch remand-2: comment [`5120410047`](https://github.com/khorum08/SimThing/pull/1491#issuecomment-5120410047) — GPU table writer authority (superseded by remand-3)
- Orch remand-3: comment [`5120847431`](https://github.com/khorum08/SimThing/pull/1491#issuecomment-5120847431) — governed structural GPU path + exact-generation proof
- Board dispatch: comment `5119876551` on issue `#1332`
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## DA sharpenings (binding)

1. Dropped `falloff_params` from the typed table (no in-rung consumer; 5.4+ owns falloff as authored EML map data).
2. Core row carries `Option<BandIndex>`; i32 sentinel (`ANCHOR_BAND_NONE_POD`) minted only at POD encode.
3. Anchor table is derived/reconstructible — absent from `BoundaryDeltaEntry` / replay wire (referee + census).
4. `observation_bypass_census.sh` sources shared `lib_cfg_test_mod_spans.sh` (5.2 remand-4 brace-balanced filter); unpiped true exit required in relay.
5. Explicit Unobserved-fixture referee (zero rows for dark locus); handoff surfaces already include `simthing-mapeditor`.

## Orch remand-3 discharge (`5120847431`)

1. **GPU-resident structural remaps** — `WorldGpuState::apply_anchor_remap_section` + `anchor_table_remap.wgsl`; boundary no longer read→CPU mutate→full upload.
2. **Census arm 8** fails `decode_anchor_table_from_gpu_pods` / `apply_anchor_remaps_to_table` / `refresh_anchor_table_magnitudes` / POD `read_anchor_table(` / `upload_anchor_table(` on `boundary.rs`; requires `apply_anchor_remap_section` + `upload_typed_anchor_table`.
3. **Public `&Buffer` escape removed** — no `anchor_table_buffer()`; POD upload/read are `pub(crate)`.
4. **Generation before fused dispatch** — `SimSession::run_hot_cycle` stamps `set_anchor_table_generation(day)` before ticks; POD uses `ANCHOR_GENERATION_NONE_POD = -1` so `None` ≠ `Some(0)`; referee covers gens `0/1/2`.
5. **Remap referee** exercises `apply_anchor_remap_section` (AddChild / fusion / fission), not CPU apply+upload.
6. **Studio referee** opens `StudioLiveSessionBridge` field-bearing path and asserts production `field_accretion_samples`.
7. **TP 25/0** uses canonical hydrate → `preview_install` → `apply_install_preview` on the real TP tree (not a fresh World bag).
8. **Doctrine** — six POD/oracle symbols un-exported from `lib.rs` / `simthing-gpu`; `verify_kernel_surface.py` 236/236; `scan_allowlists.py` buffer-handles + kernel-surface clean. No allowlist/TSV edits.

## Landed contract

- One derived STEAD GPU-resident `AnchorTable` POD twin as sole production observation **and** dynamic writer authority.
- Writers only: admission mint (typed upload), fused GPU threshold companion, GPU-resident structural remap.
- Studio/hosted observation via `AnchorTableSnapshot` → `read_typed_anchor_table`.

## Proof battery (local)

| Proof | Result |
|---|---|
| Focused `anchor_table_surface_0` referees | PASS — 12 passed, 0 failed |
| Studio bridge field accretion | PASS |
| GPU crossing matrix ↔ oracle | PASS |
| Successive generations incl. `0` | PASS |
| GPU remap identity (AddChild / fusion / fission) | PASS |
| TP 25 Anchored / 0 Unobserved on real install tree | PASS |
| `bash scripts/ci/observation_bypass_census.sh` | PASS — unpiped EXIT:True |
| `verify_kernel_surface.py` | PASS — 236/236 |
| `scan_allowlists.py buffer-handles` + `kernel-surface` | PASS |

Final head / tested_code_sha: PR-body-bound only (this file does not self-hash).
