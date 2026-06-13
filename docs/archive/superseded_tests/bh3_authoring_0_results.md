# BH-3-AUTHORING-0 — ClauseThing field-operator authoring bridge

> **Artifact lifecycle: PROBATION** (supports next two landed handoffs only; promote/archive/delete later)

## Verdict

**PASS** — ClauseThing fixture authors a BH/PALMA field-operator profile, lowers to generic
`simthing-spec` surfaces, passes existing admission previews, and reaches existing driver GPU
bridge helpers without enabling runtime execution. `simthing-sim` remains ClauseThing-blind.

## Files changed

| Area | Path |
|---|---|
| Authoring hydrate | `crates/simthing-clausething/src/hydrate_field_operator.rs` |
| Public API | `crates/simthing-clausething/src/lib.rs` |
| Fixtures | `crates/simthing-clausething/tests/fixtures/bh3_*.clause` |
| Parse tests | `crates/simthing-clausething/tests/bh3_authoring_parse.rs` |
| Admission tests | `crates/simthing-spec/tests/bh3_operator_spec_admission.rs` |
| Install bridge tests | `crates/simthing-driver/tests/bh3_authoring_installs_existing_operator.rs` |
| Dev wiring | `crates/simthing-driver/Cargo.toml`, `Cargo.lock` |
| Production docs | `docs/design_0_0_8_1_border_hack_track.md`, `docs/design_0_0_8_1.md`, `docs/tests/fable_review_bh2_track_packet.md` |

No changes to `simthing-sim`. No new GPU kernels. No movement/pathfinding/border services.

## Authoring syntax accepted (provisional, project-local)

Top-level pack block (example id `simthing_bh3_field_operator`) with nested blocks:

- Grid: `grid_size`, `source_col`, `target_col`, `n_dims`, `alpha_self`, `gamma_neighbor`, `horizon`
- `saturating_flux { u_sat, chi, choke_output_col }`
- `field_impedance { base_w_col, choke_a_col, choke_b_col, weight_a, weight_b, output_w_col }`
- `field_stress { operator = overlap|mismatch, choke_a_col, choke_b_col, output_col }`
- `threshold_feedstock { parent_slot, urgency_col, threshold, direction = upward, event_kind }`
- `parent_formula { formula_class, weight_pressure, weight_resource }`

Caps: max 1 impedance profile, max 1 stress profile per pack (BH-3 v0).

## Lowered spec shape

`HydratedFieldOperatorPack`:

- `game_mode.region_fields[0]` → `RegionFieldSpec` with `SaturatingFlux`, optional
  `FirstSliceCommitmentSpec`, auto-derived `RegionFieldReductionSpec` when threshold feedstock
  is authored, `parent_formula`, `MappingExecutionProfile::Disabled`
- `w_impedance_compose: Option<WImpedanceComposeSpec>`
- `stress_compose: Option<StressComposeSpec>`

Generic names only in spec/runtime surfaces; ClauseThing nouns do not leak into `simthing-sim`.

## Admission guardrails exercised

- Missing / zero `u_sat` rejected
- Invalid `chi` (> CFL 0.25) and non-finite values rejected
- Distinct W column bindings enforced (no aliasing across base/choke/output)
- Unbounded impedance profile fanout rejected
- Commitment requires reduction + `field_urgency` parent formula (derived on lower)
- Presence alone is default-off (`MappingExecutionProfile::Disabled`)

## Tests run

```text
cargo test -p simthing-clausething --test bh3_authoring_parse          # 4/4 PASS
cargo test -p simthing-spec --test bh3_operator_spec_admission       # 4/4 PASS
cargo test -p simthing-driver --test bh3_authoring_installs_existing_operator
```

Driver install test binary passes when invoked directly (bridge admission + GPU config plumbing).
On this Windows agent host, `cargo test` occasionally reports `os error 740` on first spawn of
the fresh test executable; CI/Linux and direct invocation succeed. No workspace gate run (parser/spec/bridge only).

## Artifact lifecycle classification (pre-handoff audit)

| Artifact | Classification |
|---|---|
| `docs/tests/fable_review_0_0_8_1_result.md` | LIVE_GUARDRAIL |
| `docs/tests/fable_review_bh2_track_packet.md` | LIVE_GUARDRAIL |
| `docs/tests/bh2d_ct4b_100tick_scenario_observations.md` | CURRENT_EVIDENCE |
| `docs/tests/r1_default_workspace_purge_results.md` | CURRENT_EVIDENCE |
| `docs/archive/superseded_tests/` | ARCHIVE |
| **This report** | PROBATION |

No superseded artifacts deleted in this handoff (audit found no stale active proof scaffolding to remove).

## Deleted / superseded artifacts

None in this PR.

## Remaining risks

- Authoring syntax is provisional; Paradox-parity naming deferred.
- `threshold_feedstock` auto-derives a minimal reduction binding; explicit reduction authoring not yet exposed.
- Driver session install wiring for authored packs is a follow-on consumer handoff (BH-3 does not wire `SimSession` ticks).
- Candidate F §0.7 unchanged; BH hot paths remain sqrt-free.
