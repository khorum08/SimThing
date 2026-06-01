# MOBILITY-OWNER-0-OPEN-0 — OWNER opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN** (Option A).

`MOBILITY-OWNER-0` may open as the next v7.9 implementation ladder, limited to the
**owner-relations + latched modifier overlay substrate floor + performance bars** built on the
ALLOC-0 / REENROLL-0 / IDROUTE-0(+R1) / ECON-0 substrates. This is the **final v7.9 substrate
ladder**; after OWNER-0 passes, the only remaining mobility/transfer work is **production runtime
integration**, which stays a separate, currently-closed gate.

Authorized scope is the owner-overlay substrate: owner relations as **columns/overlays (never spatial
parents)**, capture as an **owner-column flip (never reparenting)**, **latched, blockade-immune
modifier overlays** down-broadcast to local records **without spawning arena columns**, deterministic
application order, and generation/resync on owner-column change with no-silent-rebind. This review
**authorizes** OWNER-0 only; it does **not** implement it, and it opens **no** production runtime
integration. **Hybrid-Strata channel binding and the generational faction-index slab remain parked as
a later ECON scaling slice — explicitly out of OWNER-0.**

## Reviewed files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md` (targeted scan — no owner-relation/latched/overlay term; no conflict)
- `docs/design_v7_8.md`, `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md`, `mapping_current_guidance.md`, `sead_self_ai_track.md`
- `docs/tests/phase_mobility_scenario0_results.md`, `phase_mobility_scenario0_acceptance_review_results.md`, `phase_mobility_owner_band_budget_audit_results.md`
- `docs/tests/phase_mobility_alloc0_opening_review_results.md`, `phase_mobility_alloc0_results.md`
- `docs/tests/phase_mobility_reenroll0_opening_review_results.md`, `phase_mobility_reenroll0_results.md`
- `docs/tests/phase_mobility_idroute0_opening_review_results.md`, `phase_mobility_idroute0_results.md`, `phase_mobility_idroute0_r1_results.md`
- `docs/tests/phase_mobility_econ0_opening_review_results.md`, `phase_mobility_econ0_results.md`
- `crates/simthing-spec/src/designer_admission/mobility_{scenario0,audit0,alloc0,reenroll0,idroute0,econ0}.rs`
- `crates/simthing-spec/tests/mobility_{scenario0_admission,audit0_owner_band_budget,alloc0_substrate,reenroll0_substrate,idroute0_substrate,econ0_substrate}.rs`
- `docs/worklog.md`

## Accepted prerequisites (verified against the tree, not only the reports)

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted | PASS — `mobility_scenario0_admission` **13 passed**; packet admits faction (flow-pooling) + species/blueprint/tech (down-broadcast) owner columns, the exact owner-relation set OWNER-0 needs |
| MOBILITY-AUDIT-0 passed | PASS — `mobility_audit0_owner_band_budget` **8 passed**; the 13-band audit model already includes **modifier-down (1)**, so OWNER-0's overlay band fits ceiling 16 |
| MOBILITY-ALLOC-0 green | PASS — `mobility_alloc0_substrate` **15 passed** |
| MOBILITY-REENROLL-0 green | PASS — `mobility_reenroll0_substrate` **16 passed** |
| MOBILITY-IDROUTE-0 green + R1 hardened | PASS — `mobility_idroute0_substrate` **20 passed** |
| MOBILITY-ECON-0 green | PASS — `mobility_econ0_substrate` **20 passed** (clearinghouse circulation; faction flow-pooling owner already exercised) |
| v7.8 M/E/T closeout preserved | PASS — `c2_atlas_admission_relaxation` 15, `clause_spec0_frontier_v2_admission` 25, `v7_8_met_consumer_scenarios` 10; `cargo check --workspace` clean |
| OWNER isolable from production gameplay integration | PASS — OWNER-0 is column/overlay substrate accounting + parity proxy; no runtime/SimSession |
| Owner relations representable as columns/overlays, not parentage | PASS — scenario0 distinguishes flow-pooling vs down-broadcast owner disciplines; down-broadcast overlays never become arena columns |
| Latched modifier overlays testable without Resource Flow runtime | PASS — overlay application is substrate bookkeeping; ECON-0 already separates blockable per-tick flow from latched overlays |

## Authorized OWNER-0 scope (substrate only)

- Owner relations as explicit columns/overlays, never spatial parents; faction/species/blueprint/tech
  (the owner columns admitted by MOBILITY-SCENARIO-0).
- Owner-column flip for capture semantics, never reparenting.
- Latched modifier overlay substrate; **blockade-immune** latched modifiers (knowledge ≠ goods).
- Down-broadcast owner overlays applied to local records/integration columns; **no arena-column
  spawning** from down-broadcast overlays (only flow-pooling relations get aggregation columns —
  proven separate in ECON-0).
- Deterministic owner-overlay application order; generation/resync only for owner-column changes with
  explicit no-silent-rebind guardrails.
- GPU-consumable, parity-testable layouts; CPU/driver accounting + parity proxy, following the
  ALLOC/REENROLL/IDROUTE/ECON pattern.

## Explicit non-goals (enforced at designer/scenario admission)

No owner-entity as spatial parent; no capture-as-reparenting; no nested arena reparenting; no
production gameplay integration; no production `SimSession` wiring; no semantic/raw WGSL; no
designer-authored shader; no global/CPU AI planner; no CPU urgency; no CPU commitment emission; no
default-on behavior; no default-on Resource Flow; no hard-currency through Resource Flow; no ECON
reimplementation; **no Hybrid-Strata channel binding; no generational faction-index slab; no
faction-index scaling layer** (later ECON slice); no invariant edits.

## Opening checks

| Check | Result |
| --- | --- |
| SCENARIO accepted | PASS |
| AUDIT passed (OrderBand ceiling covers OWNER-0 modifier-down) | PASS (modifier-down already in the audited 13 ≤ 16) |
| ALLOC / REENROLL / IDROUTE(+R1) / ECON substrates green | PASS |
| OWNER isolable from production gameplay integration | PASS |
| Owner relations representable as columns/overlays, not parentage | PASS |
| Capture remains owner-column flip, not reparenting | PASS (scenario0 + ECON-0 reject capture-as-reparenting) |
| Latched modifier overlays testable without Resource Flow runtime | PASS |
| Blockade-immune overlays and blockable per-tick flows remain distinct | PASS (latched-immune vs per-tick-cut) |
| No default-on Resource Flow / hard-currency-through-RF needed | PASS (rejected) |
| No semantic/raw WGSL needed | PASS |
| No CPU planner / urgency / commitment emission | PASS |
| `simthing-sim` remains semantic-free | PASS |
| Expected OWNER test battery sufficient before implementation | PASS — see below |

## Authorized OWNER-0 test battery (to implement in a later PR — none green yet)

**Substrate floor**

- `owner_column_overlay_applies_deterministically`
- `owner_capture_is_column_flip_not_reparenting`
- `owner_latched_modifier_overlay_persists`
- `owner_blockade_immune_modifier_stays_latched`
- `owner_down_broadcast_does_not_spawn_arena_columns`
- `owner_generation_resync_on_owner_column_change`
- `owner_cpu_gpu_parity_layout`

**Guardrails**

- `owner_rejects_owner_as_spatial_parent`
- `owner_rejects_capture_as_reparenting`
- `owner_rejects_nested_arena_reparenting`
- `owner_rejects_default_on_resource_flow`
- `owner_rejects_hard_currency_through_resource_flow`
- `owner_rejects_production_simsession_wiring`
- `owner_rejects_semantic_or_raw_wgsl`
- `owner_rejects_cpu_planner_urgency_commitment`
- `owner_rejects_hybrid_strata_or_faction_index_scaling_layer`
- `owner_keeps_production_runtime_integration_parked`

**Performance bars**

- `owner_overlay_multi_cell_scale`
- `owner_concentration_one_owner`
- `owner_scale_soak_34k`

**Additional authorized (carried from the workshop OWNER architecture, consistent with OWNER-0):**
`owner_cohort_homogeneity_via_fission` (pop cohort stays homogeneous; partial change fissions a new
cohort), `owner_dirtyonly_amortized` (no owner-set change → zero dispersal cost), and
`owner_band_budget_audit` (the standing open audit — interleaved circulations incl. modifier-down fit
`max_orderband_depth`; OWNER is where modifier-down lands, so this audit is exercised here).

This battery is **authorized, not implemented**; no OWNER test is marked green in this PR.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                # 16 passed
cargo test -p simthing-spec --test mobility_idroute0_substrate                 # 20 passed
cargo test -p simthing-spec --test mobility_econ0_substrate                    # 20 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                        # Finished (pre-existing warnings only)
```

## Posture attestation

Opening review only — no OWNER implementation, no owner-overlay/modifier code, no GPU kernels, no
production gameplay integration, no production `SimSession` wiring, no default-on flags, no
semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU planner/urgency/commitment emission,
no Resource Flow default-on, no hard-currency through Resource Flow, no Hybrid-Strata/faction-index
scaling, no invariant changes. Owner-entities remain non-spatial; capture remains an owner-column
flip; latched modifiers remain blockade-immune and distinct from per-tick blockable flows; SEAD
decisions stay GPU-resident threshold/event outputs. v7.8 M/E/T closure (A-0/B-0/C-2), AO-WGSL-0
default-off, ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all
unchanged.

## Next gate

**`MOBILITY-OWNER-0`** — implement the authorized owner-overlay substrate floor + performance bars
above (later PR, substrate-only). It is the **last v7.9 substrate ladder**; once green, the v7.9
mobility/transfer substrate is complete and the only remaining mobility work is **production runtime
integration** (separate, currently-closed gate). The Hybrid-Strata / faction-index ECON scaling layer
also remains a later, separately-gated slice.
