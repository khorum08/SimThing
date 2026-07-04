# TEST-PARE-STUDIO-ADMISSION-SINGLETON-BOUNDARY-0 Results

## Status

**PROBATION / DA REVIEW**. Classification-first boundary rung for two Studio/mapeditor admission singleton rows escalated by #1115 (`TEST-PARE-STUDIO-TYPEFACE-OWNER-DEEP-0`). No source edits. Merge not authorized for Grok; any future deletion from `simthing-mapeditor` requires owner-local §10.3 survivor proof.

## Mission

Produce a focused boundary ruling for the two Studio/mapeditor admission singleton rows escalated by #1115:

- `crates/simthing-mapeditor/src/studio_config.rs::studio_config_rejects_malformed_json`
- `crates/simthing-mapeditor/tests/studio_ingestion_admission_report.rs::studio_displays_unknown_gridcell_role_deferral`

This rung does not execute or compile `simthing-mapeditor`, `simthing-tools`, Bevy, desktop, windowing, typeface, or GPU surfaces. It classifies the two Studio/mapeditor admission singleton rows under `docs/ci_screening_surface.md` §10. A deletion wave may delete fossil code only; it must never rewrite a preserved representative.

## Scope

In scope:

- The two named Studio/mapeditor admission singleton rows.
- Evidence/design/index documentation required to classify them.

Out of scope:

- `simthing-tools` TYPEFACE-LADDER rows closed by #1116.
- `simthing-mapeditor` LR8 behavior rows, active Terran-Pirate Studio rows, star-nameplate visual sign-off, Admission Substrate, SimThing-Kernel, CI profiles/scanners/allowlists/workflows.

## Source rows

| path | test | boundary | ledger disposition |
|---|---|---|---|
| `crates/simthing-mapeditor/src/studio_config.rs` | `studio_config_rejects_malformed_json` | `B-T2-SIMTHING_MAPEDITOR-PARSER_SPAN_ADMISSION` | boundary_rows **KEEP**; audit **KEEP** (`selected-admission-representative`); inventory **KEEP** |
| `crates/simthing-mapeditor/tests/studio_ingestion_admission_report.rs` | `studio_displays_unknown_gridcell_role_deferral` | `B-T2-SIMTHING_MAPEDITOR-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION` | boundary_rows **KEEP**; audit **KEEP** (`selected-admission-representative`); inventory **KEEP** |

**0R ledger sync:** `test_pare_audit.tsv` and `test_inventory.tsv` now record both rows as `selected-admission-representative` / `KEEP` (matching the #1116 pattern for selected admission representatives). `test_pare_boundary_rows.tsv` already marked both **KEEP**; unchanged. This 0R syncs the selected-representative KEEP decisions into the machine-readable audit ledger so the two Studio admission singletons are no longer reported as unresolved AUDIT-BLOCKED residue. The source rows remain unchanged and no mapeditor/tools/Bevy/desktop proof was executed.

## Method

1. Read binding doctrine: `docs/ci_screening_surface.md` §10, #1115/#1116 results, boundary/inventory/audit ledgers.
2. Inspect source only (no mapeditor/tools build or test execution).
3. Cross-check `test_pare_boundaries.tsv` representative declarations and `test_pare_boundary_rows.tsv` collapse/KEEP state.
4. Apply classification rules; record in review TSV.

## Classification table

| test | decision | rationale |
|---|---|---|
| `studio_config_rejects_malformed_json` | **KEEP_SELECTED_REPRESENTATIVE** | Sole KEEP representative for parser-span admission; many src-unit/integration variants already collapsed or PARED here; tests presentation-config JSON deserialize hard error without scenario authority leakage; no non-owner-deep substitute. |
| `studio_displays_unknown_gridcell_role_deferral` | **KEEP_SELECTED_REPRESENTATIVE** | Sole KEEP representative for missing/unknown-reference admission; sibling ingestion report variants collapsed or PARED here; asserts `UnsupportedGridcellRole` deferral in Studio ingestion report for unsupported gridcell role corpus; active 0.0.8.5 Studio consumer coverage. |

## Coverage map

| admission invariant | representative | collapsed / PARED variants (sample) |
|---|---|---|
| Parser-span / malformed JSON admission (`B-T2-SIMTHING_MAPEDITOR-PARSER_SPAN_ADMISSION`) | `studio_config_rejects_malformed_json` | `startup_invalid_config_uses_defaults_and_records_warning`, hydration/scenario_projection parser-span variants, `studio_reopen_candidate_rejects_noncanonical_or_invalid_json`, `studio_rejects_invalid_structural_edit_without_partial_mutation`, `runtime_vertical_seed_gpu_validation_reports_zero_invalid_endpoints` |
| Missing/unknown reference admission (`B-T2-SIMTHING_MAPEDITOR-MISSING_OR_UNKNOWN_REFERENCE_ADMISSION`) | `studio_displays_unknown_gridcell_role_deferral` | `startup_missing_config_uses_defaults`, hydration missing-placement variants, `studio_displays_rejected_missing_gamesession_report`, structural/GPU-readiness missing-reference variants |

## Promotion targets

None. Both rows express owner-deep Studio admission invariants with no ready lower-boundary replacement; promotion would orphan live Studio coverage before a substitute exists.

## Source edits

None.

## Forbidden proof avoided

- No `cargo check` / `cargo test` on `simthing-mapeditor` or `simthing-tools`.
- No Bevy/winit/wgpu, ALSA/X/Wayland/Mesa/Vulkan, libudev/xkbcommon/xcb/egl/glx, or `apt-get` setup.
- No `workflow_dispatch` for owner-deep proof.
- Classification from ledger join + source read only.

## Proof

Local (0R):

- `bash scripts/ci/doctrine_scan.sh`: PASS `failures=0 inspect=0`
- `bash scripts/ci/gen_digest.sh --check`: PASS
- `bash scripts/ci/test_inventory_check.sh`: PASS (`rows=4070`)
- `bash scripts/ci/test_pare_boundary_check.sh`: PASS
- `bash scripts/ci/test_inventory_drift_check.sh`: PASS (`promotion-target rows=31`)
- Survivor compile floor (five non-desktop crates): PASS
- `git diff --check origin/master...HEAD`: PASS

Live (0R head — record after push):

- Doctrine Scan: pending
- Doctrine Exec: not expected (ledger/docs-only; no `workflow_dispatch`)

## Graduation routing

```text
Graduation routing:
  Risk class:          owner-deep boundary review / Studio admission singleton / DA-held
  Source edits:        none
  Ledger edits:        0R synced audit + inventory to KEEP; promotion_wave_plan +2 rows; boundary_rows unchanged (already KEEP)
  DA question:         Does Opus accept KEEP_SELECTED_REPRESENTATIVE for both singleton rows?
  Recommended posture: PROBATION / DA REVIEW — not orchestrator-mergeable
```