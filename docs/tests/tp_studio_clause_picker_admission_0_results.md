# TP-STUDIO-CLAUSE-PICKER-ADMISSION-0 Results

## Status

**DONE — DA-ADMITTED (Option A)** — 2026-07-09 executive DA admission decision.
Narrow user-facing Studio/mapeditor `.clause` picker/menu surface is **authorized** under the constraints below.
**No picker implementation in this decision** — coding only on `TP-STUDIO-CLAUSE-PICKER-0`.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-CLAUSE-PICKER-ADMISSION-0` |
| Kind | Owner/DA admission decision (no implementation) |
| Prior | #1230 API-1 @ `820a9a2ef6`; #1231 stamp; #1232 admitted class @ `5227f08000`; #1233 stamp |
| Decision | **Option A — Admit narrow UI picker** |

## Ruling (verbatim)

```text
DA DECISION: TP-STUDIO-CLAUSE-PICKER-ADMISSION-0 admits a narrow user-facing
Studio/mapeditor .clause picker/menu surface. The picker may collect a
caller-selected .clause path and explicit resolver/source paths, then call the
existing production ClauseScript composition API from TP-STUDIO-CLAUSE-API-1 to
produce a StructuralRebindReady ScenarioSpec and hydrate a Studio session through
the existing path. The picker may not hardcode TP or fixture defaults, bypass the
production API, attach GameMode/RF/live-run state, introduce runtime/GPU/kernel
changes, or run closeout. Next rung: TP-STUDIO-CLAUSE-PICKER-0.
```

## Rationale

| Factor | Assessment |
|---|---|
| Production composition API | **Proven** #1230 — `clause_scenario_ingest` + StructuralRebindReady + STEAD/links + authority serde + `StudioSession::from_loaded_scenario` |
| Semantic novelty | **None** — picker is a UI caller of the admitted API; no new projection mode, parse path, or scenario model |
| Resolver / no-default policy | API already hard-errors on unresolved `{{...}}` placeholders (`SourceResolution`); picker must surface that, not invent defaults |
| Native dialog precedent | Studio already has `rfd` scenario JSON filedialog (`STUDIO-SCENARIO-NATIVE-FILEDIALOG-0`); `.clause` filter/menu is the analogous affordance |
| Option B (readiness report) | **Rejected** — remaining risk is implementation exit-proofable (error surface, no defaults, API-only path); a report-only gate is ceremony |
| Option C (shell only) | **Rejected** — half-admission; the production boundary is already the API; UI wiring is the admitted residual |

## Admitted surfaces

```text
minimal Studio/mapeditor UI affordance (e.g. "Open ClauseScript Scenario...")
user selects .clause file via native dialog or equivalent menu path field
explicit user-selected/configured resolver placeholder→path entries
call production clause_scenario_ingest (path/bytes + ClauseScenarioSourceResolver)
StructuralRebindReady ScenarioSpec only
Studio session hydrate via existing from_loaded_scenario / load helpers from API-1
clear error surface on missing resolver / unresolved placeholder / parse/hydrate failure
reuse existing mapeditor file-dialog machinery where appropriate (no second dialog stack required)
```

## Forbidden surfaces

```text
TP fixture/default path in production picker
terran_pirate_galaxy hardcoded picker path
tp_base_disc_1500 hardcoded picker resolver
{{FIXTURE_JSON}} auto-resolution without caller/user selection
new parser/rebind path outside production API (no UI-side duplicate ingest)
GameMode/RF/combat_arena/palma/commitment attach
live-run theater state
runtime/GPU/kernel changes
closeout
claiming full Studio scenario workflow beyond .clause → StructuralRebindReady session hydrate
```

## Implementation exit proofs (binding for TP-STUDIO-CLAUSE-PICKER-0)

1. Picker/menu action invokes production `clause_scenario_ingest` API only.
2. No parser/rebind duplicate path in UI.
3. No TP defaults or fixture defaults.
4. Unresolved placeholder prompts/error-surfaces instead of silent default.
5. Session hydrate PASS after picker flow.
6. No GameMode/RF/live-run/closeout.
7. Doctrine-scan green; clearance class for UI shape if needed (expect `no_ui_picker` class update or adjacent class — gate-wiring if class changes).

## Next

```text
TP-STUDIO-CLAUSE-PICKER-0
```

**Not next:** closeout (Owner-triggered only); GameMode/RF attach; live-run.
