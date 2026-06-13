# SimThing 0.0.8.2 — ClauseThing Closeout Ladder

> **Status: DESIGN / READY FOR CURSOR EXECUTION (2026-06-13, executive design authority).**
> This is the planning artifact that closes the ClauseThing / BH / PALMA tracks to a **park
> point** before any Bevy/editor work. It is not an implementation PR. It pins the schema
> judgments so the implementation rungs are Cursor-mechanical, and it names the single deferred
> boundary (field-export/corpus) precisely so the future editor track inherits a clean seam.
>
> **0.0.8.2 rationale:** this closeout spans and parks three 0.0.8.1 tracks (ClauseThing
> production, Border Hack, PALMA integration). It earns its own minor-version doc because it
> defines the cross-track park line; the editor/corpus track will be a separate future minor
> version sitting on the seam defined in §10. No constitution change — `design_0_0_8_1.md` §0.7
> (Candidate F) and `simthing_core_design.md` (principle-level) are untouched.

---

## 1. Current-state assessment (verified 2026-06-13)

**ClauseThing hydration surfaces that exist and lower to generic `simthing-spec` structs today:**

| Surface | Lowers to | Status |
|---|---|---|
| `hydrate_entity_pack` (CT-1a) | `DomainPackSpec` (properties + overlays) | CLOSED |
| `hydrate_category_economy` (CT-2c) | `GameModeSpec`: RF arenas, `RegionFieldSpec`, gated/`value:` rates, `pressure_binding`, ai_will_do `field_urgency`, `FirstSliceCommitmentSpec` + `CommitmentEffectSpec` | CLOSED |
| `hydrate_field_operator` (BH-3 provisional) | `RegionFieldOperatorSpec::SaturatingFlux`, W-impedance compose, stress compose, threshold feedstock structs | **PROBATION** |
| `hydrate_resource_flow` | `ResourceFlowSpec` | CLOSED |
| capability / tradition trees (CT-1c) | `CapabilityTreeSpec` | CLOSED |
| All parsing | jomini text path → `RawDocument` (`parse_raw_document`) | CLOSED |

**Driver/runtime spine (0.0.8.1, accepted):** `open_from_spec` installs a `GameModeSpec`; the
session loop runs RF arena bands → on-device pressure scatter → stencil heatmap (incl.
`SaturatingFlux`) → reduce → ai_will_do EML → GPU edge-detected commitment scan → journaled
crossing → authored `CommitmentEffectSpec` via `BoundaryRequest::AttachOverlay`. PALMA W/D exists
at driver level (BH-2C: `WImpedanceComposeOp` → `GpuInterleavedW` → resident D + compact probe).

**The closure gap (precise):** every hydrator consumes **one top-level entity/fixture**. There is
**no scenario-container import** — no single ClauseScript document declaring multiple
location/SimThing nodes plus adjacency, composing the above sub-blocks. PALMA W/D has a driver
bridge but **no authored binding from ClauseScript**. There is **no canonical end-to-end sample**
(parse → lower → admit → install → exercise) as one scenario. These three are the spine of the
ladder; everything else is guardrail-hardening and lifecycle hygiene.

**Contamination note (adjudicated):** the Bevy/editor discussion is **excluded** from this ladder
(editor is deferred until closure; the handoff forbids editor work). It is *useful* only in one
way: it tells me where to draw the closure line. The field-export/corpus consumer — the surface
the editor will save/load/export heatmap animations as JEPA training corpora through — is named
as the **deferred next-track boundary** in §10, with the intrinsic-vs-ambient-dimensionality
discipline pinned as the constraint its future schema must honor. No export code lands here.

## 2. ClauseThing closeout definition

ClauseThing is **closed enough to park for editor work** when a single sample ClauseScript
scenario: imports; lowers into generic `GameModeSpec`/`Scenario`/`RegionFieldSpec`/`ResourceFlowSpec`
surfaces with **zero `simthing-sim` semantic leakage**; declares SaturatingFlux/Gu-Yang field
operators, PALMA W/D feedstock, FIELD_POLICY thresholds, and overlays/properties/children;
admits and installs through the driver; exercises the GPU-resident field/operator path under a
focused test or accepted compact probe; routes a bounded commitment → `BoundaryRequest` feedstock
path (exists, CT-3b+4a Line 3R); and is documented as to exactly what is complete vs deferred.
**This is authoring/import/runtime-feedstock closure — not playable-game closure.**

## 3. Schema adjudications (executive design authority — spent here so PRs are mechanical)

**A1 — Scenario container.** A scenario is one top-level `scenario { … }` block: `metadata`,
N × `location { id … }` (alias for a SimThing node), `link { from to }`, and the existing
`field_operator` / `region_field` / `resource_flow` / `commitment` sub-blocks. It hydrates to the
existing `(GameModeSpec, Scenario)` pair. Locations become **children of root**; no new
`SimThingKind`, no new sim type. `hydrate_scenario` is a composing front-end over the existing
per-block parsers — not a new lowering target.

**A2 — Adjacency / links (the highest leakage-risk decision).** `link { from = a to = b }` lowers
to **bounded admission-time topology metadata** consumed by the RegionField grid binding: in v1,
locations map to grid cells and adjacency is the existing N4 grid neighborhood; explicit `link`
entries are admitted, validated (both endpoints exist, bounded fan-out), and recorded as
scenario `install_targets`/cell-placement metadata the driver already consumes. **A link is never
a graph object, edge struct, or topology engine in `simthing-sim`.** Non-grid / arbitrary-graph
topology is **deferred** (precise boundary: the day a consumer needs non-grid adjacency, it opens
a topology-spec rung; until then, grid adjacency is the only representable form). The grammar has
**no production** for routes/edges-as-objects — unrepresentable, not merely rejected.

**A3 — PALMA W/D feedstock authoring.** `palma_feedstock { w_source = <Choke|Named(col)> d_output_col = N }`
lowers to the existing `WImpedanceComposeSpec` + min-plus traversal field config (BH-2C bridge).
Authored config may choose the W source column and the D output column. The grammar has **no
production** for `route`, `path`, `plan`, `predecessor`, `waypoint`, `movement_order`, or
destinations. PALMA stays a generic traversal/field substrate; D is a field, never a route.

**A4 — Candidate F is not implicated by any closeout rung.** SaturatingFlux is linear arithmetic
+ `clamp` + products; PALMA min-plus is `D = W + min(N4 D)` (no sqrt); thresholds are scalar
compares. No closeout rung introduces spatial magnitude, Euclidean distance, gradient norm, or a
parity-sensitive exact path. **No rung routes through Candidate F, and none needs to.** The one
tripwire: if any rung is tempted to author a "Euclidean distance field," it **stops for design
review** — min-plus D is impedance/Manhattan distance and carries no sqrt; an exact Euclidean
consumer is out of closeout scope and would route through `m_jit_mag_f_from_exact_mag2` per §0.7.

**A5 — Deferred boundary (the editor/corpus seam, named not built).** The bounded
commitment → `BoundaryRequest` path is complete and in scope. The **field-export/corpus consumer**
is deferred: a future read-only compact tap on the reduction-pass columns (pressure, choke, W, D,
stress) at authored ticks/scales, populating the inert `export_meta` placeholder (CT-0b) and
closing `FIELD-MOVIE-DATASET-0`. **Pinned constraint for that future schema** (so it is not
designed wrong later): the export *input* stack must be regime-distinct independently-sourced
fields (genuine intrinsic dimensionality), with derived readouts (choke/W/D/stress) tagged as
**held-out probe targets, never input padding**; complexity is dialed by field count + coupling +
C_u saturation, audited by measuring corpus intrinsic dimensionality against authored field count.
None of this is implemented in 0.0.8.2.

## 4. PR ladder table

| PR | Title | Owner | Theme | Depends on |
|---|---|---|---|---|
| 1 | Closeout index + artifact lifecycle census | Cursor | lifecycle | — |
| 2 | Scenario-container grammar + location/property/overlay/children lowering | Cursor (DA review) | A,B | 1 |
| 3 | Adjacency/link grammar → grid-topology lowering | Cursor (DA review) | A,B | 2 |
| 4 | BH-3 SaturatingFlux authoring closure (admission guardrails) | Cursor | C,E | 2 |
| 5 | PALMA W/D feedstock authoring + generic lowering | Cursor | D | 2,4 |
| 6 | FIELD_POLICY threshold feedstock unification under scenario | Cursor | B | 2 |
| 7 | Canonical sample ClauseScript scenario + parse/lower test | Cursor | F,G | 3,4,5,6 |
| 8 | Driver admit/install closure for the sample + GPU path exercise | Cursor (DA review) | B,G | 7 |
| 9 | Test battery + artifact promotion/deletion pass | Cursor | G | 8 |
| 10 | Closeout report + production-docs + ledger close | Cursor (DA sign-off) | H | 9 |

"DA review" = executive design authority reviews the merge diff (no fresh design pass needed; the
judgment is pre-spent in §3). No rung requires a new Opus *design* pass; §3 closed those gates.

## 5. Rare Opus / design-authority gates

The schema-authority, leakage-boundary, and event-architecture decisions the handoff reserves for
Opus are **adjudicated in §3** of this document. The only residual design-authority involvement is
**merge-time review** of the three leakage-sensitive PRs (2, 3, 8) and **sign-off** on the
closeout report (10). A rung must **stop and escalate** (not improvise) only if it hits a §9 stop
condition — chiefly: adjacency needing a graph engine (A2 boundary breached), PALMA needing a
route production (A3 breached), or a Euclidean-distance consumer appearing (A4 tripwire).

## 6. Cursor-granular PR handoffs

### PR 1 — Closeout index + artifact lifecycle census
Owner: Cursor
Purpose: Ground the ladder; classify every ClauseThing/BH/PALMA artifact before code moves.
Scope: docs only.
Files likely touched: `docs/design_0_0_8_2_clausething_closeout_ladder.md` (append census table); none else.
Implementation steps: Inventory `crates/simthing-clausething/{src,tests}`, BH/PALMA reports under
`docs/tests/`, and the field_operator probation artifacts. Classify each
LIVE_GUARDRAIL / PROBATION / CURRENT_EVIDENCE / ARCHIVE / DELETE. Mark `hydrate_field_operator`
provisional tests PROBATION.
Tests: none.
Docs: census appended here.
Artifact cleanup: delete any DELETE-class scratch now; archive nothing yet.
Acceptance: every artifact classified; no unclassified proof scaffolding remains active.
Stop conditions: an artifact cannot be classified (→ PARTIAL, §9.8).

### PR 2 — Scenario-container grammar + core declarations
Owner: Cursor (DA review)
Purpose: Add `hydrate_scenario` composing front-end (A1) — multi-location scenario document.
Scope: parse + lower; no new sim/driver types.
Files likely touched: `crates/simthing-clausething/src/hydrate_scenario.rs` (new), `src/lib.rs`
(export), `docs/clausething/ClauseThing_Spec.md` (grammar §).
Implementation steps: Parse top-level `scenario { metadata, location*, … }`; each `location`
hydrates a SimThing node (id + properties + overlays + children) reusing existing block parsers;
assemble children-of-root; emit `(GameModeSpec, Scenario)`. Reuse `hydrate_entity_pack` /
`hydrate_category_economy` block parsers — do not fork them.
Tests: `ct_scenario_parse_lowers_multilocation` (parse a 3-location stub → assert N children,
properties/overlays present, canonical-JSON stable).
Docs: ClauseThing_Spec scenario grammar section.
Artifact cleanup: none.
Acceptance: multi-location scenario lowers to generic specs; no new `SimThingKind`; no sim import.
Stop conditions: scenario shape needs a sim-side type (→ escalate, A1 breach).

### PR 3 — Adjacency / link grammar → grid-topology lowering
Owner: Cursor (DA review)
Purpose: `link { from to }` → bounded grid-adjacency/install-target metadata (A2).
Scope: grammar + admission validation; no graph object.
Files likely touched: `hydrate_scenario.rs`, `crates/simthing-spec/src/...` (only if a bounded
topology-metadata field is needed on an existing spec; prefer `Scenario.install_targets` /
cell placement), `ClauseThing_Spec.md`.
Implementation steps: Parse `link`; validate both endpoints exist and fan-out ≤ a small authored
cap; record as cell-placement/install-target metadata. **No edge struct, no graph, no route.**
Tests: `ct_scenario_links_lower_to_grid_adjacency`; `ct_scenario_link_unknown_endpoint_rejected`;
`ct_scenario_link_fanout_cap_rejected`.
Docs: ClauseThing_Spec adjacency note + the A2 "no graph object" rule.
Artifact cleanup: none.
Acceptance: links lower to bounded grid metadata; grammar has no route/edge production.
Stop conditions: a consumer needs non-grid arbitrary-graph adjacency (→ defer per A2; do not build).

### PR 4 — BH-3 SaturatingFlux authoring closure
Owner: Cursor
Purpose: Promote `hydrate_field_operator` from PROBATION to closed; complete admission guardrails (C/E).
Scope: hydration guardrails + admission, over existing `RegionFieldOperatorSpec::SaturatingFlux`.
Files likely touched: `src/hydrate_field_operator.rs`, `crates/simthing-spec/src/compile/region_field_admission.rs` (reuse existing CFL/bounds), tests.
Implementation steps: Hard, spanned errors for: missing `u_sat`, invalid/`>0.25` `chi`
(CFL, dt=1.0 per §3 A4 / BH track), non-finite values, unknown output binding, unbounded fanout;
default-off (operator presence enables nothing). Preserve doctrine: symmetric `(C_i+C_j)/2` flux,
zero-flux boundary, register-transient C, 13-cell diamond — these live in the GPU op already; the
authoring layer must not weaken them.
Tests: `ct_field_op_saturating_flux_admits`; rejection tests for each guardrail; `*_default_off`.
Docs: BH track §status → BH-3 authoring closed; ClauseThing_Spec field_operator grammar.
Artifact cleanup: promote the PROBATION field_operator tests to CURRENT_EVIDENCE or delete superseded ones.
Acceptance: authored SaturatingFlux profiles admit/install or hard-error; doctrine preserved; default-off.
Stop conditions: authoring would require weakening symmetric-flux/zero-flux doctrine (→ escalate).

### PR 5 — PALMA W/D feedstock authoring + lowering
Owner: Cursor
Purpose: `palma_feedstock { w_source d_output_col }` → `WImpedanceComposeSpec` + min-plus config (D).
Scope: grammar + lowering over existing BH-2C bridge; **no pathfinding productions**.
Files likely touched: `hydrate_field_operator.rs` or `hydrate_scenario.rs`, `ClauseThing_Spec.md`, tests.
Implementation steps: Parse `palma_feedstock`; bind W source (Choke column or Named flow column)
and D output column to the existing compose/min-plus config. Grammar has **no** `route`/`path`/
`predecessor`/`plan`/`destination` token.
Tests: `ct_palma_feedstock_lowers_to_w_compose`; `ct_palma_feedstock_no_route_production`
(assert the parser rejects/has-no-grammar-for a `route {}` block); `*_default_off`.
Docs: ClauseThing_Spec PALMA feedstock note + the A3 "D is a field, not a route" rule;
`design_0_0_8_1_palma_pathfinding_integration_guide.md` cross-reference.
Artifact cleanup: none.
Acceptance: imported scenario may configure W/D feedstock; may not declare routes/plans/predecessors.
Stop conditions: feedstock would need destination/route semantics (→ escalate, A3 breach).

### PR 6 — FIELD_POLICY threshold feedstock unification
Owner: Cursor
Purpose: Expose ai_will_do/`field_urgency` threshold + commitment feedstock under the scenario container (B).
Scope: wire the existing category-economy commitment/threshold path through `hydrate_scenario`.
Files likely touched: `hydrate_scenario.rs`, tests, `ClauseThing_Spec.md`.
Implementation steps: Allow a scenario to declare `commitment { … }` (threshold + weights +
optional `effect`) reusing the CT-3b+4a hydration; ensure it composes with locations.
Tests: `ct_scenario_threshold_feedstock_lowers`; `ct_scenario_commitment_effect_optional`.
Docs: ClauseThing_Spec commitment/threshold note.
Artifact cleanup: none.
Acceptance: scenario-level FIELD_POLICY thresholds lower to `FirstSliceCommitmentSpec`; effect optional.
Stop conditions: threshold needs CPU planner logic (→ escalate; decisions stay GPU-side).

### PR 7 — Canonical sample ClauseScript scenario
Owner: Cursor
Purpose: The expressive small sample that proves closure (F).
Scope: fixture + parse/lower test only.
Files likely touched: `docs/clausething/examples/sample_scenario.clause` (canonical),
`crates/simthing-clausething/tests/fixtures/closeout_sample.clause`, `tests/ct_closeout_sample.rs`.
Implementation steps: Author a scenario with 3–5 locations, ≥1 link, ≥1 property block, ≥1 overlay,
one SaturatingFlux field operator, one PALMA W/D binding, one FIELD_POLICY threshold, one compact
probe/reportable output. Test: parse → lower → assert generic spec shape (no sim nouns).
Tests: `ct_closeout_sample_parses_and_lowers`.
Docs: examples/README pointer.
Artifact cleanup: none.
Acceptance: sample imports and lowers; every required element present; original (not Paradox) content.
Stop conditions: sample cannot be expressed in the minimal grammar (→ grammar gap, escalate).

### PR 8 — Driver admit/install closure for the sample
Owner: Cursor (DA review)
Purpose: Prove admit → install → GPU-resident path exercise for the sample (B/G).
Scope: driver test using `open_from_spec`; reuse existing install + session loop.
Files likely touched: `crates/simthing-clausething/tests/ct_closeout_install.rs` (GPU-gated),
possibly a thin driver test helper — **no new driver runtime surface** unless escalated.
Implementation steps: Hydrate sample → `open_from_spec` → assert install (region field +
SaturatingFlux op + PALMA W/D + commitment installed; default-off honored when profile disabled);
run a few ticks; assert compact probe/threshold event (no full-field CPU decision readback).
Tests: `ct_closeout_sample_installs_and_runs` (skips cleanly without GPU).
Docs: none beyond the closeout report.
Artifact cleanup: none.
Acceptance: sample admits/installs; GPU path exercised; compact readback only; no sim leakage.
Stop conditions: install needs a new sim-aware surface, or full-field readback to decide (→ escalate).

### PR 9 — Test battery + artifact promotion/deletion
Owner: Cursor
Purpose: Consolidate a fast, focused closure battery; promote/delete PROBATION artifacts (G).
Scope: test organization + lifecycle; no new runtime.
Files likely touched: clausething tests; `docs/tests/` (delete superseded).
Implementation steps: Ensure the battery covers parse / lower / admit / install / default-off /
each rejection case / semantic-free lowering / no-sim-leakage / PALMA-not-pathfinding /
SaturatingFlux-not-border-service / compact-probe-only. Promote PROBATION field_operator artifacts
to CURRENT_EVIDENCE or delete; archive only historically useful. **No** report-checksum,
disabled/reenabled theater, prior-rung parity ledgers, or >60s default tests.
Tests: the consolidated battery green.
Docs: none.
Artifact cleanup: delete superseded; archive sparingly under `docs/archive/superseded_tests/`.
Acceptance: battery fast + focused; no proof theater; no unclassified scaffolding.
Stop conditions: battery cannot stay fast/focused (→ PARTIAL, §9.7).

### PR 10 — Closeout report + docs + ledger close
Owner: Cursor (DA sign-off)
Purpose: Document exactly what is complete vs deferred; close the tracks (H).
Scope: docs only.
Files likely touched: `docs/tests/clausething_closeout_results.md` (new, CURRENT_EVIDENCE);
`docs/design_0_0_8_1_clausething_production_track.md`, `docs/design_0_0_8_1_border_hack_track.md`,
`docs/design_0_0_8_1_palma_pathfinding_integration_guide.md`, `docs/clausething/ClauseThing_Spec.md`,
`docs/tests/fable_review_bh2_track_packet.md`, this doc's ledger.
Implementation steps: Write the closeout report (§7 below); flip the track ledgers to
CLOSED/PARKED with pointers; record the deferred export/corpus boundary (§10) verbatim.
Tests: `cargo fmt --all -- --check`; `cargo test -p simthing-clausething`; `-p simthing-spec`;
`-p simthing-driver`. Workspace only if a shared harness/runtime file changed.
Docs: all of the above.
Artifact cleanup: final superseded-report sweep.
Acceptance: §10 closeout criteria all met; docs state complete vs deferred; Candidate F unmoved.
Stop conditions: any §10 criterion unmet (→ PARTIAL with the precise gap).

## 7. Test strategy

Focused, fast, GPU-skipping-clean. Cover, once each: parse sample; lower to generic spec; admit;
install; default-off; each rejection case (missing `u_sat`, bad `chi`, non-finite, unknown
binding, unbounded fanout, unknown link endpoint, route-production-absent); semantic-free lowering
(grep-style assert: no ClauseThing noun in the emitted spec); no `simthing-sim` ClauseThing
leakage; PALMA feedstock config is not a pathfinding service (no route grammar); SaturatingFlux is
not a border service (choke is a column); compact probe/readout only. **Forbidden:** report
checksum stability, disabled/reenabled proof theater, broad prior-rung parity ledgers, large slow
batteries, any default test >60s.

## 8. Artifact lifecycle strategy

Every PR opens with an inventory + classification pass. New report/test artifacts start
**PROBATION**; a PROBATION artifact supports at most two later landed handoffs before promotion,
archive, or deletion. DELETE items are deleted in the PR that finds them. Archive only
historically useful proof markdown under `docs/archive/superseded_tests/`. New visibility only
under `docs/tests/`. **No PASS is marked while stale/unclassified proof scaffolding is active.**
The `hydrate_field_operator` provisional artifacts are PROBATION until PR 4 promotes or deletes them.

## 9. Stop conditions (any → PARTIAL with the precise gap; do not improvise)

1. Existing docs too contradictory to ladder safely. 2. Import scope cannot be bounded.
3. PALMA consumption would require pathfinding semantics (A3 breach). 4. SaturatingFlux
consumption would require a border service. 5. Candidate F exactness boundary unclear (A4 tripwire
— a Euclidean-distance consumer appears). 6. `simthing-sim` would need ClauseThing nouns.
7. Test battery cannot stay fast/focused. 8. An artifact cannot be classified.

## 10. Final closeout acceptance criteria

ClauseThing track is **CLOSED / PARKED** when: the sample scenario parses, lowers to generic
`GameModeSpec`/`Scenario` surfaces, admits, installs, and exercises the GPU-resident field path
under a focused/compact test; SaturatingFlux + PALMA W/D + FIELD_POLICY threshold +
overlays/properties/children are all authorable with no `simthing-sim` semantic leakage; the
bounded commitment → `BoundaryRequest` feedstock path is exercised; the closeout report states
complete-vs-deferred; Candidate F authority is unmoved (§0.7); no editor/Bevy work was required;
and **the deferred boundary is recorded precisely**:

> **Deferred to a future minor version (the editor/corpus track):** the read-only
> field-export/corpus consumer — a compact strided tap on the reduction-pass columns at authored
> ticks/scales, populating `export_meta`, closing `FIELD-MOVIE-DATASET-0`. Its future schema is
> bound by the intrinsic-vs-ambient discipline (A5): regime-distinct independently-sourced input
> fields; derived readouts as held-out probe targets, never input padding; complexity dialed by
> field count + coupling + C_u saturation; honesty-audited by measuring corpus intrinsic
> dimensionality against authored field count. The Bevy/editor UI sits on top of this seam. None
> of it is built in 0.0.8.2.

The closeout target is **authoring/import/runtime-feedstock closure, not playable-game closure.**

## 11. Docs update map

- **New:** this doc; `docs/tests/clausething_closeout_results.md` (PR 10).
- **Updated:** `design_0_0_8_1_clausething_production_track.md` (scenario-import + closeout
  pointer); `design_0_0_8_1_border_hack_track.md` (BH-3 authoring closed); `palma_pathfinding_integration_guide.md`
  (authored W/D feedstock entry); `clausething/ClauseThing_Spec.md` (scenario + field_operator +
  PALMA + commitment grammar); `fable_review_bh2_track_packet.md` (BH-3 promotion note).
- **Untouched (binding):** `design_0_0_8_1.md` §0.7 stays the Candidate F home;
  `simthing_core_design.md` stays principle-level — the Candidate F artifact chain does **not**
  move there.
