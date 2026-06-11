# CT-1a implementation results — ClauseThing literal entity hydration parity

Status: **IMPLEMENTED / PASS**

## Scope ledger
- `crates/simthing-clausething/src/hydrate.rs` — CT-1a literal entity hydration into
  `DomainPackSpec` + install-time `seed_amount`.
- `crates/simthing-clausething/src/literal_install.rs` — admission/install snapshot via
  existing `compile_property` / `compile_overlay` and CPU `PropertyTransformDelta::apply_to_data`.
- `crates/simthing-clausething/src/error.rs` — `HydrateError` with optional `RawSpan`.
- `crates/simthing-clausething/src/lib.rs` — module wiring and public exports.
- `crates/simthing-clausething/Cargo.toml` — added `simthing-spec` + `simthing-core`
  dependencies (designer-layer only; no runtime wiring).
- `crates/simthing-clausething/tests/ct_1a_entity.rs` — 3 parity/rejection tests.
- `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity.clause` — ClauseScript fixture.
- `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity_baseline.ron` — hand-authored RON baseline.
- `crates/simthing-clausething/tests/fixtures/ct1a_unsupported_field.clause` — rejection fixture.
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 CT-1a row updated.

## Files changed
See scope ledger. No `simthing-spec` production code, `simthing-sim`, `simthing-gpu`, WGSL, or
`simthing-driver` changes.

## Fixture paths
- ClauseScript: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity.clause`
- RON baseline: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity_baseline.ron`

## Hydration mapping summary
| ClauseScript field | `simthing-spec` target |
|---|---|
| Top-level entity key | `DomainPackSpec.id` |
| `display_name` | `DomainPackSpec.display_name` |
| `property { id, namespace, name, display_name }` | `PropertySpec` |
| `property.seed_amount` | install-time seed (not in RON struct; used for CPU overlay apply proof) |
| `modifier { id, display_name, targets_property, amount_mult }` | `OverlaySpec` with `TransformOp::Multiply` on `SubFieldRole::Amount` |
| `modifier.amount_add` | `TransformOp::Add` on `SubFieldRole::Amount` (supported; unused in demo fixture) |

Default overlay metadata: `OverlayLifecycle::Permanent`, `OverlayKind::Policy`, `OverlaySource::Player`.

## Unsupported forms / rejections
Hard-error with spanned `HydrateError` for: unknown top-level entity fields (e.g.
`triggered_modifier`), unknown `property`/`modifier` sub-fields, missing required fields,
non-scalar values, both `amount_mult` and `amount_add`, neither transform op, duplicate
`property` blocks, documents without exactly one top-level entity template.

## Commands run
- `cargo test -p simthing-clausething` — pass (CT-0a 1; CT-0b 3/1 ignored; CT-0c 16/1 ignored;
  CT-0d 9/2 ignored; CT-1a 3).
- `cargo test -p simthing-clausething --test ct_1a_entity` — pass (3 tests).
- `cargo fmt --all -- --check` — clean.
- `cargo test --workspace` — **not run** (only `simthing-clausething` + docs changed; new
  `simthing-clausething → simthing-spec/simthing-core` deps validated by targeted crate tests).

## Installed-tree comparison evidence
`LiteralInstallSnapshot` via existing spec admission (`compile_property`, `compile_overlay`) +
CPU `apply_to_data` on the standard Amount column. ClauseScript-hydrated and RON-baseline paths
produce identical snapshots:
- `property_keys`: `["simthing::potency"]`
- `overlay_specs`: `ct1a_potency_boost` → `simthing::potency`, `Multiply(1.25)` on `Amount`
- `seeded_amount`: `40.0`
- `final_amount`: `50.0` (40 × 1.25)

Full driver `install_atomic` tree not exercised (driver production code out of scope); spec
compile + CPU overlay apply is the existing admission firewall proof for CT-1a.

## RON-diff / canonical diff result
Hydrated `DomainPackSpec` canonical JSON matches hand-authored RON baseline canonical JSON
(`hydrated_domain_pack_matches_ron_baseline` test).

## CPU-oracle overlay/property parity
**Tested** via `PropertyTransformDelta::apply_to_data` in `literal_install.rs` (same helper path
used by `simthing-core` CPU reference evaluator). Seeded Amount `40.0` with `Multiply(1.25)`
yields `final_amount = 50.0` identically for ClauseScript and RON paths.

## Closure questions
1. **Specified vs implemented?** Yes — one literal synthetic entity with flat property + literal
   modifier hydrates to existing `DomainPackSpec` and passes spec admission + CPU overlay apply
   parity vs RON baseline.
2. **ClauseScript fixture?** `ct1a_demo_entity.clause` (`simthing_ct1a_demo`).
3. **RON baseline?** `ct1a_demo_entity_baseline.ron`.
4. **Flat literal properties?** `id`, `namespace`, `name`, `display_name`, `seed_amount`.
5. **Literal modifier blocks?** `id`, `display_name`, `targets_property`, `amount_mult`.
6. **Same authoring/canonical form?** Yes (canonical JSON equality).
7. **Installed tree identical?** Canonically identical via `LiteralInstallSnapshot` equality.
8. **CPU-oracle parity?** Yes — `apply_to_data` on Amount column; `50.0` verified.
9. **Unsupported fields rejected?** Yes — `triggered_modifier` hard-errors with span.
10. **simthing-spec admission/install firewall?** Yes — `compile_property` / `compile_overlay`.
11. **Paradox/lab corpus committed?** No — synthetic SimThing-authored fixtures only.
12. **simthing-sim untouched?** Yes.
13. **simthing-gpu/WGSL untouched?** Yes.
14. **Runtime/default wiring untouched?** Yes.
15. **Unneeded artifacts deleted?** Yes — no scratch dumps or logs retained.
16. **Artifacts under docs/tests only?** Yes — this report only.
17. **cargo test --workspace avoided?** Yes.

## Confirmations
- No Paradox / lab corpus material committed: **confirmed**.
- `simthing-sim` untouched: **confirmed**.
- `simthing-gpu` / WGSL untouched: **confirmed**.
- No runtime/default wiring: **confirmed**.
- `cargo test --workspace` not run: **confirmed**.
