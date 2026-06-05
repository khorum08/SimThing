# MOBILITY-RUNTIME-0-OPEN-0 — production runtime integration opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN WITH NARROWING** (Option B).

`MOBILITY-RUNTIME-0` may open, **narrowed to a test-only, default-off substrate-composition
integration harness** — not production `SimSession`/GPU pass-graph wiring.

**Why narrowed, not full Option A.** Two facts decide it:
1. All six substrates (ALLOC/REENROLL/IDROUTE/ECON/OWNER) are pure `simthing-spec` metadata/proxy
   models (CPU structs + deterministic parity-proxy checksums); they reference `SimSession`/runtime
   only as *rejection strings*, never importing runtime crates. Each was validated **in isolation**;
   their **ordered composition is the genuinely-unproven step** and must be proven before any runtime
   hook.
2. `docs/invariants.md` (lines 108, 128, 161, 184) is unambiguous: production runtime / `SimSession`
   pass-graph wiring and the economy→mapping bridge are **default-off and test/fixture-only until a
   separate gated decision**. A test-only composition harness respects this; "into the runtime behind
   opt-in gates" would press toward the wiring those invariants gate separately.

So RUNTIME-0 = the **composition harness**: deterministically compose the five substrate outputs in
the documented order and prove the composition preserves every substrate invariant. **Actual
production `SimSession`/GPU pass-graph wiring is a distinct, later, currently-closed gate**
(RUNTIME-1 / production-wiring) — not authorized here. This review **authorizes** the harness only;
it does **not** implement it.

## Reviewed files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md` (targeted scan — runtime/SimSession-wiring rows govern, no conflict; no edit)
- `docs/design_v7_8.md`, `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md`, `mapping_current_guidance.md`, `field_policy_track.md`
- All v7.9 prerequisite reports (`phase_mobility_{scenario0,scenario0_acceptance,owner_band_budget_audit,alloc0_opening,alloc0,reenroll0_opening,reenroll0,idroute0_opening,idroute0,idroute0_r1,econ0_opening,econ0,owner0_opening,owner0,owner0_r1}_results.md`)
- `crates/simthing-spec/src/designer_admission/mobility_{scenario0,audit0,alloc0,reenroll0,idroute0,econ0,owner0}.rs`
- `crates/simthing-spec/tests/mobility_{scenario0_admission,audit0_owner_band_budget,alloc0_substrate,reenroll0_substrate,idroute0_substrate,econ0_substrate,owner0_substrate}.rs`
- `docs/worklog.md`

## Accepted prerequisites (verified against the tree, not only the reports)

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted | PASS — `mobility_scenario0_admission` **13** |
| MOBILITY-AUDIT-0 passed | PASS — `mobility_audit0_owner_band_budget` **8** |
| MOBILITY-ALLOC-0 | PASS — `mobility_alloc0_substrate` **15** |
| MOBILITY-REENROLL-0 | PASS — `mobility_reenroll0_substrate` **16** |
| MOBILITY-IDROUTE-0 + R1 | PASS — `mobility_idroute0_substrate` **20** |
| MOBILITY-ECON-0 | PASS — `mobility_econ0_substrate` **20** |
| MOBILITY-OWNER-0 + R1 | PASS — `mobility_owner0_substrate` **24** |
| Isolated owner-overlay completeness case named + green | PASS — `owner_down_broadcast_reaches_every_owned_including_isolated` present and passing (OWNER-0-R1) |
| v7.8 M/E/T closeout preserved | PASS — `c2_atlas_admission_relaxation` 15, `clause_spec0_frontier_v2_admission` 25, `v7_8_met_consumer_scenarios` 10; `cargo check --workspace` clean |
| Substrates isolable from Hybrid-Strata/faction-index scaling | PASS — those remain a parked later ECON slice; not referenced by the substrate models |
| Integration testable without default-on behavior | PASS — substrates are CPU/proxy models; a composition harness needs no default-on and no runtime wiring |

## Authorized RUNTIME-0 scope (the narrowing — test-only, default-off composition harness)

- Compose **only** the completed v7.9 substrate outputs, in order:
  ALLOC → REENROLL → IDROUTE → ECON → OWNER, as a deterministic CPU/driver composition with the
  existing parity proxies.
- **Test-only / default-off:** explicit opt-in; **no default `SimSession` pass-graph wiring; no GPU
  runtime hook; no default-on behavior.** Invoked only from explicit test/fixture paths.
- Composition must **preserve every substrate invariant**: deterministic replay; CPU/GPU parity
  proxy; movement writes only the moving SimThing's own authoritative columns; capture is an
  owner-column flip (never reparenting); owner-entities are columns/overlays (never spatial parents);
  modifiers are latched, DirtyOnly, blockade-immune OWNER overlays; **ECON resource circulation stays
  separate from OWNER modifier overlays**; hard Band Alpha precedes soft Band Beta; no hard/soft
  silent mixing; **isolated owned SimThings receive owner overlays by owner-column presence**.

## Explicit non-goals

No production `SimSession`/GPU pass-graph wiring (separate later gate); no default-on behavior; no
default-on Resource Flow; no hard-currency through Resource Flow; no semantic/raw WGSL; no
designer-authored shader; no CPU planner / urgency / commitment emission; no owner-entity as spatial
parent; no capture-as-reparenting; no nested arena reparenting; no Hybrid-Strata channel
partitioning; no generational faction-index slab; no later ECON scaling slice; no atlas production
runtime / sparse-residency scheduler; no E-11B-5; no B-1; no ClauseThing/L3; no FrontierV2-5 or
ACT/EVENT/OBS/PIPE reopen; no invariant edits.

## Opening checks

| Check | Result |
| --- | --- |
| All substrate reports green, incl. OWNER-0-R1 | PASS |
| Isolated owner-overlay down-broadcast completeness named + green | PASS (`owner_down_broadcast_reaches_every_owned_including_isolated`) |
| Integration isolable from Hybrid-Strata/faction-index scaling | PASS |
| Integration testable without default-on behavior | PASS (test-only composition harness) |
| No semantic/raw WGSL needed | PASS |
| No CPU planner / urgency / commitment needed | PASS |
| `simthing-sim` remains semantic-free | PASS |
| Runtime integration can preserve v7.8 + v7.9 admission boundaries | PASS (harness composes admitted substrate outputs; admission boundaries unchanged) |
| Expected RUNTIME-0 battery sufficient before implementation | PASS — see below (+ one design-authority addition enforcing the narrowing) |

## Authorized RUNTIME-0 test battery (composition-harness scope; none green yet)

**Substrate-integration floor**

- `runtime0_opt_in_only_default_off`
- `runtime0_no_simsession_passgraph_wiring` *(design-authority addition — enforces the Option-B
  narrowing: the harness must not wire into the default session pass graph; real production wiring is
  a separate later gate)*
- `runtime0_integrates_alloc_reenroll_idroute_econ_owner_in_order`
- `runtime0_preserves_deterministic_replay`
- `runtime0_preserves_cpu_gpu_parity_proxy`
- `runtime0_movement_writes_only_moving_simthing_columns`
- `runtime0_capture_remains_owner_column_flip`
- `runtime0_owner_overlay_reaches_isolated_owned_unit`
- `runtime0_econ_resource_flow_separate_from_owner_modifier_overlay`
- `runtime0_no_hard_soft_silent_mix`

**Guardrails**

- `runtime0_rejects_default_on_behavior`
- `runtime0_rejects_semantic_or_raw_wgsl`
- `runtime0_rejects_cpu_planner_urgency_commitment`
- `runtime0_rejects_owner_as_spatial_parent`
- `runtime0_rejects_capture_as_reparenting`
- `runtime0_rejects_nested_arena_reparenting`
- `runtime0_rejects_default_on_resource_flow`
- `runtime0_rejects_hard_currency_through_resource_flow`
- `runtime0_rejects_hybrid_strata_or_faction_index_scaling`
- `runtime0_rejects_closed_ladder_reopen`

**Performance / soak bars**

- `runtime0_34k_integrated_scenario_soak`
- `runtime0_dirty_owner_modifier_steady_state_zero_redisperse`
- `runtime0_mobility_churn_with_owner_overlay_and_econ_clearinghouse`

This battery is **authorized, not implemented**; no RUNTIME-0 test is marked green in this PR.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                # 16 passed
cargo test -p simthing-spec --test mobility_idroute0_substrate                 # 20 passed
cargo test -p simthing-spec --test mobility_econ0_substrate                    # 20 passed
cargo test -p simthing-spec --test mobility_owner0_substrate                   # 24 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                        # Finished (pre-existing warnings only)
```

## Posture attestation

Opening review only — no RUNTIME-0 implementation, no production `SimSession`/GPU pass-graph wiring,
no GPU kernels, no default-on flags, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no
CPU planner/urgency/commitment emission, no Resource Flow default-on, no hard-currency through
Resource Flow, no Hybrid-Strata/faction-index scaling, no invariant changes. Owner-entities remain
non-spatial; capture remains an owner-column flip; latched modifiers remain blockade-immune and
distinct from per-tick ECON flow; FIELD_POLICY decisions stay GPU-resident threshold/event outputs. v7.8
M/E/T closure (A-0/B-0/C-2), AO-WGSL-0 default-off, atlas production runtime parked, E-11B-5 parked,
B-1 closed, ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all
unchanged.

## Next gate

**`MOBILITY-RUNTIME-0`** — implement the authorized **test-only, default-off composition harness**
(later PR). The subsequent step — **production `SimSession`/GPU pass-graph wiring** — is a separate,
currently-closed gate, openable only after the composition harness is green and per invariants
108/128/161/184. The Hybrid-Strata/faction-index ECON scaling layer also remains a later,
separately-gated slice.
