# TP-STUDIO-CLAUSE-PICKER-0 Results

## Status

**PROOF-PRESENT / PROBATION** — admitted narrow Studio/mapeditor `.clause` picker/menu as a **caller of production `clause_scenario_ingest` only**. Session hydrate PASS. No TP defaults, no dual parse/rebind, no GameMode/RF/live-run/closeout.

## Admitted scope

Per `TP-STUDIO-CLAUSE-PICKER-ADMISSION-0` Option A (#1235):

- Minimal UI affordance **Open ClauseScript Scenario...**
- User-selected `.clause` path + explicit resolver `TOKEN=path` entries
- Production API only → StructuralRebindReady → existing session hydrate
- Clear error surface on unresolved placeholders / API failures

## Implemented picker/menu surface

| Surface | Location |
|---|---|
| Action controller | `crates/simthing-mapeditor/src/clause_scenario_picker.rs` |
| Menu label | `OPEN_CLAUSE_SCENARIO_ACTION_LABEL` / `clause_picker_menu_label()` |
| Native dialog | `NativeClauseFilePicker` (`rfd`, `*.clause`) |
| Fake dialog (CI) | `FakeClauseFilePicker` |
| Studio UI | `app/ui.rs` Scenario panel — button + resolver multiline + clause path field |
| State | `clause_path_text`, `clause_resolver_text` on `StudioAppState` |
| App glue | `app/scenario_io.rs` — `open_native_clause_scenario_picker`, programmatic open |

Native dialog is a thin caller over the tested action boundary (`run_clause_picker_action` / `open_clause_scenario_with_picker`).

## Production API call path

```text
user selects .clause (+ explicit resolver entries)
→ ClausePickerSelection
→ ClauseScenarioIngestOptions { StructuralRebindReady, ClauseScenarioSourceResolver }
→ load_clause_studio_session_from_path  (production clause_scenario_ingest)
   → ingest_clause_scenario_path
   → save_clause_scenario_authority_to_path
   → load_studio_session_from_scenario_path
→ StudioSession adopted
```

Alternate proof path: `run_clause_picker_ingest_then_session` → `load_studio_session_from_clause_ingest_result` (`from_loaded_scenario`).

## Resolver UX / explicit source policy

- UI field: `TOKEN=path` or `{{TOKEN}}=path` lines (`parse_clause_resolver_entries`)
- Empty resolver is valid only when the clause has no `{{…}}` tokens
- No production fallback to TP fixtures

## Error surface proof

`picker_0_unresolved_placeholder_surfaces_error` — empty resolver on TP clause → `ClausePickerActionResult::Failed` with production SourceResolution message (placeholder/unresolved/resolver).

## StructuralRebindReady / session hydrate proof

- `picker_0_action_invokes_production_clause_api` — projection_mode StructuralRebindReady; stead PASS
- `picker_0_session_hydrates_after_picker_flow` — session placements non-empty; STEAD validate PASS
- `picker_0_injected_dialog_calls_action` — fake dialog selection → Loaded

## No duplicate parser/rebind proof

`picker_0_no_duplicate_parse_or_rebind_path` — picker source has no `parse_raw_document` / `hydrate_scenario` / `rebind_pack_*`; only production ingest helpers.

## No TP/default fixture proof

`picker_0_no_tp_or_fixture_defaults` — no `tp_base_disc_1500` / `terran_pirate_galaxy` / `TP-FULL-TRANSPILE` in picker module; empty default selection.

## No GameMode/RF/live-run/closeout proof

`picker_0_no_gamemode_rf_live_run_closeout` — bans attach/live-run/closeout wiring strings; menu label present in UI.

## No runtime/GPU/kernel proof

Diff limited to mapeditor presentation + tests/docs/inventory. No kernel/sim/gpu/driver edits.

## Tests

```bash
cargo test -p simthing-mapeditor --test tp_studio_clause_picker_0 -- --nocapture
cargo test -p simthing-mapeditor --test tp_studio_clause_api_1 -- --nocapture
```

8/8 picker tests PASS; API-1 suite still PASS.

## Clearance routing

Expect `DA-RESERVE(unclassified-scope)` or class-envelope against `tp-admitted-clause-api-composition` (`no_ui_picker`) unless a picker class is added (gate-wiring). Not a proof failure if constraints hold.

## Known gaps

- Resolver UX is text-field based (not multi-file dialogs per placeholder) — explicit and sufficient under admission
- No dedicated clearance class for UI picker yet (optional `TP-STUDIO-CLAUSE-PICKER-CLASS-0`)
- Full graphical dialog not driven in headless CI (FakeClauseFilePicker + action controller proven)

## Recommended next rung

```text
TP-STUDIO-CLAUSE-PICKER-CLASS-0
```

if router friction warrants a clearable class; else Owner/DA decision on Phase 8 completeness / readiness. **Not** closeout unless Owner declares workplan complete.
