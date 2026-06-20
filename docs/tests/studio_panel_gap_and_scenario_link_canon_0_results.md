# STUDIO-PANEL-GAP-AND-SCENARIO-LINK-CANON-0 — Panel inset fix and canonical structural links

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added STUDIO-PANEL-GAP-AND-SCENARIO-LINK-CANON-0 PROBATION row |
| `docs/tests/studio_panel_gap_and_scenario_link_canon_0_results.md` | PROBATION | This report |
| `docs/tests/scenario_native_session_0_results.md` | PROBATION | Prior session work unchanged |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

The left-panel bottom inset fix removes a visible layout defect that made the floating panel appear docked to the window bottom. Link canonicalization hardens the structural graph projection so invalid topology (self-links, duplicate/reversed edges) cannot become future GPU feedstock.

## Left panel bottom-gap fix

- `left_panel_rect()` in `panel_layout.rs` sets bottom inset equal to left inset (`margin_x`).
- Panel height: `screen_height - margin_y - margin_x` (top keeps `margin_y`, bottom uses `margin_x`).
- Left panel body scrolls inside `ScrollArea` with `max_height` from `left_panel_content_scroll_height()`.
- Panel layout remains presentation-only (not persisted in scenario authority or studio config).

## Layout tests

`crates/simthing-mapeditor/src/panel_layout.rs`:

- `left_panel_bottom_gap_matches_left_gap`
- `left_panel_bottom_gap_survives_resize`
- `left_panel_uses_floating_area_not_docked_sidebar` (existing)
- `left_panel_content_scrolls_inside_inset_panel`

## Canonical link doctrine

`SimThingScenarioLink` validation in `simthing-spec` enforces structural adjacency edges:

- Endpoints must reference known generated system ids (string form of `system_id`).
- Self-links invalid (`ScenarioLinkError::SelfLink`).
- Direct duplicates invalid (`ScenarioLinkError::DuplicateLink`).
- Reversed duplicates invalid under undirected adjacency (`ScenarioLinkError::ReversedDuplicateLink`).
- `canonical_scenario_link_pair` / `canonical_scenario_link_key` provide deterministic undirected keys.

## Structural projection link ordering

`StudioStructuralProjection`:

- Dense link rows use `(min_dense_index, max_dense_index)` canonical pairs.
- Link index rows sorted deterministically.
- Invalid/self/duplicate/reversed links fail before GPU readiness manifest is built.

## GPU-readiness implication

`build_gpu_residency_readiness_from_scenario` fails when link validation or structural projection fails. Invalid graph links cannot produce a valid readiness manifest.

## Tests added

**simthing-spec** (10): link acceptance/rejection, canonical key determinism, deserialize rejection for self/duplicate links.

**simthing-mapeditor scenario_projection** (7 new): self-link, direct/reversed duplicate rejection, deterministic sort, canonical dense pairs, GPU readiness rejection, save/load roundtrip preserves canonical projection.

**simthing-mapeditor panel_layout** (3 new): bottom gap, resize survival, scroll height.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Files changed

- `crates/simthing-mapeditor/src/panel_layout.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/studio_panel_gap_and_scenario_link_canon_0_results.md`

## Deleted/archived artifacts

None. Self-link test fixtures replaced in-place with `two_cell_scenario()` helpers.

## Deferred work

Pathfinding, route/predecessor semantics, movement-order logic, runtime vertical-test loading, GPU kernels, RF execution arenas, heatmap rendering.

## DA status

**PROBATION** — pending owner design-authority approval.