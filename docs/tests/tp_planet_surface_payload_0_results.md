# TP-PLANET-SURFACE-PAYLOAD-0 Results

Status: **DONE — DA-APPROVED (2026-07-02, executive DA deep review post-merge).** The DA re-ran the full
12-test suite + the ownership-columns 9 (all green), verified the surface tier resolves through the
pre-existing `simthing-spec` tier-evaluation surfaces, and confirmed live CI green. **Merge-hold breach
recorded:** PR #1079 merged while this doc read PROBATION — accepted on merits (truthful state, sound
substance), not precedent (§0.9.5).

## What changed

- Added `planet_surface_payload` authoring blocks to the scenario-container hydrator in `crates/simthing-clausething/src/hydrate_scenario.rs`.
- Owned systems (250) receive planet gridcell → mandated 1×1 surface → `Infrastructure` factory + `Cohort` under the surface tier with participant owner-flow metadata.
- Neutral systems (1250) receive planet gridcell → mandated 1×1 surface only; no factory/cohort gameplay children.
- Owned payload admits CT-2c economic modifier keys through `decode_economic_modifier_key` and lowers admitted keys to scenario `OverlaySpec` entries.
- Added targeted proof suite `crates/simthing-clausething/tests/tp_planet_surface_payload_0.rs` (12 tests).

## Payload authoring syntax

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 embedded_base_owner_siblings_ownership_and_planet_surface_payload_parse -- --nocapture`

```text
planet_surface_payload = owned_system_payload {
    applies_to = owned_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 1
    cohort_min = 1
    category_map = { pop_factory = { kind = Cohort depth = 3 } }
    resource = {
        id = "tp_minerals"
        namespace = "tp"
        name = "minerals"
        display_name = "Minerals"
    }
    modifier = {
        pop_factory_minerals_produces_mult = 0.10
        pop_factory_minerals_upkeep_add = 1
    }
}

planet_surface_payload = neutral_system_payload {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
}
```

## Owned-system payload proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 owned_systems_have_planet_surface_factory_and_cohort -- --nocapture`

All 250 owned star-system gridcells (200 Terran + 50 Pirate) carry at least one planet local gridcell, a mandated 1×1 surface gridcell, at least one `Infrastructure` factory child under the surface tier, and at least one `Cohort` child under the surface tier. Factory/cohort children carry participant owner-flow metadata bound to the system's existing owner ref column.

## Neutral-system light-payload proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 neutral_systems_have_planet_surface_without_factory_or_cohort -- --nocapture`

All 1250 neutral systems carry planet + mandated 1×1 surface and admit zero factory/cohort gameplay children under the surface tier.

## Surface-tier non-vacuity proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 surface_tier_is_non_vacuous_and_does_not_silently_collapse -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 all_planet_gridcells_enumerate_surface_tier -- --nocapture`

`evaluate_planet_child_locations` reports `surface_gridcell_tier_required = true`, `surface_gridcell_tier_present = true`, and non-zero `surface_gridcell_count` across all 1500 planets. No silent tier collapse.

## Owner-ref preservation proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 ownership_columns_remain_200_50_1250 -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 owner_refs_remain_gamesession_sibling_targets -- --nocapture`

TP-OWNERSHIP-COLUMNS-0 counts remain 200/50/1250. Owner refs still resolve to GameSession sibling owners. Star-system gridcells remain GalaxyMap children; owners remain empty of spatial children.

## RF/economy settlement proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 rf_settlement_path_exists_for_owned_surface_participants -- --nocapture`

Owned surface-tier factory/cohort participants admit through `evaluate_planet_child_rf_admission` (≥500 active participants) and scope into reduce-up buckets via `evaluate_planet_child_rf_reduce_up` with planet and star-system scope keys present — the surface→planet→star scoped RF settlement path over existing participant metadata surfaces.

## Modifier-chain admission proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 modifier_chains_admitted_through_existing_decoder_surfaces -- --nocapture`

Owned payload `pop_factory_minerals_produces_mult` and `pop_factory_minerals_upkeep_add` decode through `decode_economic_modifier_key` and lower to scenario `OverlaySpec` entries in `HydratedScenarioPack.game_mode`.

## Hard-error proof

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 unsupported_payload_fields_hard_error_with_span -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 invalid_owned_payload_counts_hard_error_with_span -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 neutral_payload_with_factory_hard_errors_with_span -- --nocapture`

Unsupported payload fields, zero `factory_min` on owned payloads, and non-zero factory/cohort mins on neutral payloads hard-error with spans.

## Load-bearing validation

Local targeted validation (bash unavailable on this host — live GitHub Doctrine Scan verified on PR #1079):

```bash
cargo check -p simthing-clausething
cargo check -p simthing-spec
cargo test -p simthing-clausething --test tp_ownership_columns_0 -- --nocapture
cargo test -p simthing-clausething --test tp_planet_surface_payload_0 -- --nocapture
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/doctrine_scan.sh
```

Local results:

- `cargo check -p simthing-clausething`: PASS
- `cargo check -p simthing-spec`: PASS
- `tp_ownership_columns_0`: PASS, 9 passed
- `tp_planet_surface_payload_0`: PASS, 12 passed

## INSPECT / triage

Local triage entries: none.

Live GitHub Doctrine Scan: PASS on PR #1079 head 9ad9aacea596160f7851f04aadc2ccde8f9cd450.
Run: 28564092572
Job: 84687704189

## Scope Ledger

- Planet/surface/factory/cohort payload only on star-system gridcells under GalaxyMap.
- Owned: ≥1 planet, mandated 1×1 surface, ≥1 factory (`Infrastructure`), ≥1 cohort per owned system.
- Neutral: ≥1 planet, mandated 1×1 surface, zero factory/cohort.
- Ownership remains owner-column/ref; no owner-as-parent; no reparenting.
- RF participant metadata on owned factory/cohort through existing `apply_participant_owner_flow_*` surfaces.
- Modifier admission through existing CT-2c `decode_economic_modifier_key` decoder.
- No fleets, ships, combat, diplomacy, AI, pathfinding, Movement-Front execution, runtime/GPU changes, new AccumulatorRole, scanner/allowlist edits, new CI workflow, second parser, or capture events.

## Graduation routing

Graduation routing (for orchestrator review — why PROBATION, not COMPLETE):

- CI verdict: PASS-RELIABLE
- Triage entries: none
- Risk class: surface-tier non-vacuity + owned-system payload authoring + RF settlement
- Falsification check: Verify every owned system has planet + mandated 1×1 surface + factory + cohort; verify every neutral system has planet + mandated 1×1 surface and no factory/cohort; verify surface tier is non-vacuous; verify TP-OWNERSHIP-COLUMNS-0 owner refs/counts remain unchanged; verify RF settles surface→planet→star→galaxy through existing resource-flow/modifier surfaces; verify no fleets, ships, combat, diplomacy, AI, pathfinding, runtime/GPU change, new AccumulatorRole, scanner/allowlist edit, second parser, third loading path, or owner-as-parent semantics.
- Recommended posture: deep — this is the first non-vacuous lower-spatial payload and economy surface consumed by later fleet/combat/diplomacy rungs.

## Known gaps / next

Parking state: `TP-PLANET-SURFACE-PAYLOAD-0` remains PROBATION — docs corrected, CI green, awaiting orchestrator/DA clearance. Grok is parked; do not start `TP-SHIPSIZE-DECODER-0`. Ready for CI Track B handoff after Fable lands the CI scaffolding refactor.

Binding follow-on from Fable DA: `HydratedScenarioPack` now carries canonical `authority_root` beside the legacy scenario-container `root`. This is acceptable for now because both are proven projections of the same hydration sources, but it is a divergence seam. By latest `TP-FULL-TRANSPILE-0`, consumers must converge onto `authority_root` or a formal Deviation must derive/retire the legacy root.