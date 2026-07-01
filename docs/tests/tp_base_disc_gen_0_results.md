# TP-BASE-DISC-GEN-0 Results

Status: **PROBATION** - implementation proof complete; orchestrator review required before merge.

## Scope

`TP-BASE-DISC-GEN-0` emits the canonical Phase 1 base galaxy artifact:

- Artifact: `crates/simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json`
- Scenario id: `tp_base_disc_1500`
- Fixture SHA-256: `aab8b0d2bd229bca620986d1c56ac8f658c0842b432f6131b66ac5bf99266ac0`
- Fixture bytes: `889808`
- Star-name corpus source: `simthing-authored:tp_base_disc_star_names_v1`
- Name assignment mode: `stellaris_star_names_seeded_shuffle_no_replacement_cycle`

## Canonical Seed + Params

Recorded in scenario provenance as `generator_profile_id`, `generator_params_json`, `name_corpus_source`, and `name_assignment_mode`.

| Field | Value |
|---|---:|
| seed | `770421` |
| shape | `elliptical` |
| stars | `1500` |
| lattice edge | `300` |
| target hyperlanes | `5000` |
| max hyperlane distance | `7.0` |
| ensure connected | `true` |
| partitions | `none` |
| cluster count | `5` |
| cluster radius | `400.0` |

## Proofs

- Byte-identical regeneration from recorded metadata: PASS
- Studio Generate parity for the same profile and corpus: PASS
- `map_quality_status = PASS`: PASS
- Seeded name determinism by exact `system_id -> display_name` assignment: PASS

The canonical artifact path normalizes generated SimThing ids before save so repeated generation and Studio generation converge byte-for-byte without changing live Studio allocation behavior.

## Load-Bearing Validation

Local targeted validation:

```bash
cargo check -p simthing-mapgenerator
cargo check -p simthing-mapeditor
cargo check -p simthing-clausething
cargo test -p simthing-mapgenerator --test topology_stead
cargo test -p simthing-mapgenerator --test connectivity
cargo test -p simthing-mapeditor --test tp_base_disc_gen tp_base_disc_regenerates_byte_identically_from_recorded_metadata -- --nocapture
cargo test -p simthing-mapeditor --test tp_base_disc_gen tp_base_disc_studio_generate_path_matches_canonical_artifact -- --nocapture
cargo test -p simthing-mapeditor --test tp_base_disc_gen tp_base_disc_names_are_seeded_and_map_quality_passes -- --nocapture
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/doctrine_scan.sh
```

## INSPECT / Triage

Local doctrine scan: PASS, failures=0, inspect=0.
Local gen_digest --check: PASS.
Live PR-head CI: pending after push.

## Scope Ledger

- Runtime code touched only to add canonical base-disc generation/proof helpers and optional scenario provenance metadata.
- No Phase 1 overlay content.
- No Terran/Pirate scenario content.
- No scanner or allowlist edits.
- No new `AccumulatorRole`.
- No catch-unwind path.
- No semantic changes below the spec boundary.

## Graduation Routing

CI verdict: local PASS; live PR-head CI pending after push

Triage entries: none locally

Risk class: canonical producer artifact + Studio parity + ScenarioSpec provenance

Falsification check: regenerate from recorded seed/params byte-identically; prove Studio Generate path emits identical canonical artifact; assert map quality PASS; assert seeded Stellaris-style names are deterministic and unique for all 1500 systems; verify no Phase 1 overlay content, scanner/allowlist edit, new `AccumulatorRole`, or lower-boundary semantic change.

Recommended posture: deep - this fixture becomes the Phase 1 base galaxy for later embedding and overlay rungs.

## Known Gaps / Next

Next active rung after orchestrator clearance is `TP-BASE-EMBED-0`.
