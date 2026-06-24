# STELLARIS-STAR-NAMESPACE-INGESTION-0 results

## Status

PROBATION — Stellaris star_names corpus ingestion and deterministic Studio name assignment.

## PR / branch / merge

- Branch: `codex/stellaris-star-namespace-ingestion-0`
- PR: (pending)
- Merge: (pending)

## Current defect or mission

Studio-generated star-system gridcells have no semantic name in `SimThingScenarioSpec`. Hydration synthesizes `System N Gridcell`, while Bevy nameplates render raw `SIM-NNNNNN` ids. The legacy Stellaris `star_names` ClauseScript pool is not consumable by Studio.

## Implemented changes

- Added a narrow ClauseThing consumer for the top-level literal `star_names` pool. It accepts quoted and unquoted scalar entries, preserves authored order and duplicates, and fails closed on missing, duplicate, empty, or structurally invalid pools.
- Added deterministic seeded assignment: generated system ids are sorted; catalog entries are shuffled by a fixed SplitMix64/Fisher-Yates policy; names are used without replacement within a cycle and reshuffled after exhaustion.
- Added `STAR_SYSTEM_DISPLAY_NAME_PROPERTY_ID` plus ScenarioSpec metadata accessors.
- Added an optional `GenerationProfile.star_name_corpus_path`. Empty preserves old behavior; `SIMTHING_STELLARIS_RANDOM_NAMES_PATH` is a fallback for non-dialog callers.
- Replaced immediate Generate execution with a three-choice corpus dialog. It defaults to the copied Stellaris corpus path; OK generates with the displayed path, None clears naming and retains `SIM-######` nameplates, and Cancel performs no generation.
- Studio adoption reads/parses the corpus and writes selected names to star-system Location authority before building projections.
- Hydration, `StudioScenarioDocument`, and `StudioGalaxyViewModel` project semantic names from authority. Bevy nameplates use the semantic name when present and retain `SIM-######` otherwise.
- Scenario save/reopen preserves the selected names without re-reading the external corpus.

## Boundary / constitution checks

- ScenarioSpec remains save/load authority.
- Names are semantic CPU-side metadata; no GPU structural projection or shader changes.
- MapGenerator remains a structural producer and does not gain corpus or localization ownership.
- Corpus loading is optional/default-off and must not alter existing generation when no source is configured.
- Existing legacy generated-World-root behavior is not silently broadened into a root migration in this rung.

## Validation commands

- PASS — `cargo check -p simthing-clausething -p simthing-spec -p simthing-mapeditor`.
- PASS — `cargo test -p simthing-clausething stellaris_names --lib` (2 passed).
- PASS — `cargo test -p simthing-mapeditor configured_stellaris_pool_names_authority_view_and_save_load --lib` (1 passed; authority, view projection, and save/load).
- PASS — `cargo test -p simthing-mapeditor editor_settings_default_roundtrip --lib` (1 passed; the added defaulted profile field preserves settings serialization).
- PASS — `cargo test -p simthing-mapeditor generation_name_dialog_choices_gate_generation_and_corpus --lib` (OK/None/Cancel semantics and default path).
- PASS — with `SIMTHING_STELLARIS_RANDOM_NAMES_PATH=C:\Users\mvorm\Clauser\Paradox\vanilla\common\random_names\base\00_random_names.txt`, `cargo test -p simthing-mapeditor studio_session_requires_hydrated_grid --lib` (the copied real corpus parsed and a 3,000-star session hydrated).
- PASS — `cargo fmt -p simthing-clausething -p simthing-spec -p simthing-mapeditor -- --check`.
- PASS — `git diff --check`.
- PARTIAL — `cargo fmt --all` hit Windows error 206 (workspace command-line/path length); package-scoped format and format-check passed.

## Files changed

- `crates/simthing-clausething/{Cargo.toml,src/lib.rs,src/stellaris_names.rs}`
- `crates/simthing-spec/src/{lib.rs,spec/mod.rs,spec/scenario.rs}`
- `crates/simthing-mapeditor/{Cargo.toml,src/generation.rs,src/session.rs,src/hydration.rs,src/view_model.rs,src/studio_scenario_document.rs,src/app/ui.rs,src/app/galaxy_render.rs}`
- `Cargo.lock`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/stellaris_star_namespace_ingestion_0_results.md`

## Evidence lifecycle

- This report is PROBATION evidence until focused tests pass and DA review decides promotion.
- The production synthesis records the capability as PROBATION. The live evidence index was not given a pending/local row because it explicitly forbids stale pending entries.

## Known gaps

- Studio generation currently creates a legacy `World` root while canonical ingestion expects `Scenario -> GameSession -> GalaxyMap`; root migration is out of this rung.
- The Stellaris engine's exact RNG and exhaustion/collision policy are unavailable; SimThing's deterministic policy must be documented as compatibility behavior, not claimed engine equivalence.
- Localization-backed names outside the literal `star_names` pool remain deferred.

## Deferred next rung

- Canonicalize generated Studio scenarios and extend the namespace consumer to localization keys and the other Stellaris name families.

## DA status

PROBATION — user-requested implementation; formal promotion/closed-track amendment not yet recorded.
