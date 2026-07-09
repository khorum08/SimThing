# TP-STUDIO-CLAUSE-API-ADMISSION-1 Results

## Status

**DONE — DA-ADMITTED (Option A)** — 2026-07-09 executive DA admission decision.
Limited production mapeditor ClauseScript composition API is **authorized** under the constraints below.
**No implementation in this decision** — coding only on `TP-STUDIO-CLAUSE-API-1`.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-CLAUSE-API-ADMISSION-1` |
| Kind | Owner/DA admission decision (no implementation) |
| Prior | #1226 StructuralRebindReady @ `03afe3d152`; #1227 workshop class @ `0141f03c62` |
| Decision | **Option A — Admit limited production API** |

## Ruling (verbatim)

```text
DA DECISION: TP-STUDIO-CLAUSE-API-ADMISSION-1 admits a narrow, generic, non-TP-default
production ClauseScript scenario ingest composition surface limited to the
StructuralRebindReady projection contract. The admitted surface may compose existing
clausething parse/hydrate, the proven StructuralRebindReady rebind policy, simthing-spec
authority serde, and existing STEAD validation. It may not include TP defaults, fixture
defaults, UI picker, GameMode/RF attach, live-run state, lowerer heuristic expansion, or
closeout. StudioSession::from_loaded_scenario proof is required as an exit criterion of
the implementation rung (TP-STUDIO-CLAUSE-API-1), not as a second admission gate: session
hydrate is the existing generic Spec→Studio path gated by STEAD validity, which
StructuralRebindReady already proves.
```

## Rationale

| Factor | Assessment |
|---|---|
| Empty STEAD blocker (admission-0) | **Resolved** by #1226 StructuralRebindReady |
| Semantic-free utility | parse/hydrate/serde already production; rebind policy is coord-join + GalaxyMap id bind — generic if stripped of TP packaging |
| Rustification posture | Elevate the composition utility; keep TP defaults and owner-key lowerer debt out of the production surface |
| `NOT_RUN_IN_WORKSHOP` | Workshop packaging limit (no Bevy dep), **not** a missing STEAD contract. Session hydrate is Spec→Studio already production for JSON Specs |
| Option B | Rejected as extra admission gate kabuki; fold session proof into implementation exit |

## Admitted surfaces

```text
generic ClauseScript scenario ingest composition (mapeditor and/or elevated from workshop)
caller-supplied .clause path or bytes
caller-supplied source/include resolver (no production defaults)
ProjectionMode::StructuralRebindReady only (for production open)
existing clausething parse_raw_document + hydrate_scenario
StructuralRebindReady rebind policy (elevate generic form; no Tp* public names required)
existing simthing-spec authority serde
existing validate_stead_mapping_consistency / validate_scenario_links
```

## Forbidden surfaces

```text
terran_pirate_galaxy / tp_base_disc_1500 / {{FIXTURE_JSON}} production defaults
TP-FULL-TRANSPILE or Terran/Pirate wording on production public API
owner-key heuristic expansion in the elevated surface
GameMode / RF / combat_arena / palma / commitment attach
live-run theater state
UI .clause picker (separate admission after API-1)
closeout
claiming "Studio ClauseScript ingest done" before API-1 exit proofs
```

## Elevation guidance (implementation)

1. Prefer elevating **generic** rebind/composition symbols (no `Tp` prefix, no fixture Default).
2. Workshop TP modules may remain thin **callers/proofs** under `birth_track=0.0.8.5-terran-pirate`.
3. Home of production composition: `simthing-mapeditor` (Studio consumer) and/or `simthing-clausething` (pack→Spec) as appropriate — **no second authority model**.
4. **API-1 exit proofs (binding):**
   - generic API with caller-supplied resolver (no production defaults)
   - TP fixture parity only via **caller-supplied** paths (workshop/test)
   - `StudioSession::from_loaded_scenario` or `load_studio_session_from_scenario_path` PASS on StructuralRebindReady Spec
   - doctrine-scan green; no mapeditor UI picker in this rung

## Next

```text
TP-STUDIO-CLAUSE-API-1
```

**Not next:** UI picker, closeout, GameMode/RF attach.
