# MOBILITY-RUNTIME-1-OPEN-0 — production SimSession/GPU pass-graph wiring opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN WITH NARROWING** (Option B) → authorize **RUNTIME-1A only**.

Authorize a **CPU-only production fixture** that wires the green RUNTIME-0 composition into a
production `SimSession` surface behind an **explicit, default-off named gate** — with **no GPU
pass-graph wiring and no default schedule**. **GPU pass-graph registration is split out as RUNTIME-1B,
a separate, currently-closed later gate** (the handoff's split question, answered: yes, split).

**Why narrowed.** RUNTIME-0 proved the substrates *compose* deterministically, but everything to date
is CPU/proxy models — there is no production GPU surface yet, and `invariants.md` gates production
`SimSession`/GPU wiring as default-off, fixture-only, separately-gated (5 production-wiring rows).
Jumping straight to GPU pass-graph wiring would skip the CPU production-fixture step and press past
those rails. RUNTIME-1A is the minimal next slice; RUNTIME-1B (GPU) follows only once 1A is green.

## Reviewed files

`phase_m_gating_and_doc_policy.md`; `invariants.md` (production-wiring rows govern; no conflict, no
edit); `design_v7_8.md`; `design_v7_8_production_track.md`;
`design_v7_9_mobility_transfer_allocation_production_track.md`; workshop
`mobility_and_transfer_allocation.md` / `mapping_current_guidance.md` / `sead_self_ai_track.md`;
`phase_mobility_runtime0_opening_review_results.md` + `..._runtime0_results.md`; the
owner0_r1/econ0/idroute0_r1/reenroll0/alloc0 reports; `mobility_runtime0.rs` +
`mobility_runtime0_composition.rs`; `worklog.md`. (`docs/simthing_repoguide.md` ignored per handoff;
it has since been removed from the repo.)

## Accepted prerequisites (verified against the tree)

| Prerequisite | Result |
| --- | --- |
| SCENARIO-0 / AUDIT-0 | PASS — 13 / 8 |
| ALLOC-0 / REENROLL-0 | PASS — 15 / 16 |
| IDROUTE-0 (+R1) / ECON-0 | PASS — 20 / 20 |
| OWNER-0 (+R1) | PASS — 24 (incl. `owner_down_broadcast_reaches_every_owned_including_isolated`) |
| RUNTIME-0 composition harness | PASS — `mobility_runtime0_composition` **23** |
| `runtime0_no_simsession_passgraph_wiring` green and still true | PASS — present and passing; production wiring not yet entered |
| v7.8 M/E/T closeout preserved | PASS — c2 15 / clause_spec0 25 / met 10; `cargo check --workspace` 0 errors |

## Authorized RUNTIME-1A scope (the narrowing)

- Explicit **opt-in / default-off named gate** (config/feature flag); default `SimSession` behavior
  unchanged; zero runtime cost when disabled.
- **CPU-only** production fixture wiring the RUNTIME-0 composition outputs
  (ALLOC→REENROLL→IDROUTE→ECON→OWNER, in order) into a production `SimSession` surface.
- **No GPU pass-graph registration; no default schedule; no gameplay-facing integration.**
- Preserves: RUNTIME-0 composition order; deterministic replay; CPU/GPU parity proxy; movement writes
  only the mover's own columns; capture = owner-column flip; owner overlays reach isolated owned
  units by column presence; ECON flow separate from OWNER overlays; hard Band Alpha before soft Band
  Beta; no hard/soft silent mix.

## Explicit non-goals

GPU pass-graph wiring (→ RUNTIME-1B, separate later gate); default-on production behavior; default
`SimSession` schedule; gameplay-facing behavior; semantic/raw WGSL; designer shader code; CPU
planner/urgency/commitment; owner-as-spatial-parent; capture-as-reparenting; nested arena
reparenting; default-on Resource Flow; hard-currency through Resource Flow; Hybrid-Strata /
faction-index scaling; atlas production runtime; E-11B-5; B-1; ClauseThing/L3; FrontierV2-5;
ACT/EVENT/OBS/PIPE; invariant edits.

## Opening checks

All PASS: all substrate + RUNTIME-0 reports green; `runtime0_no_simsession_passgraph_wiring` green and
still true; RUNTIME-1A wiring is default-off behind an explicit named gate (cannot enable implicitly);
deterministic replay preserved (CPU fixture over the deterministic composition); GPU pass-graph
**split out** to RUNTIME-1B, not opened here; no semantic/raw WGSL; no CPU planner/urgency/commitment;
`simthing-sim` semantic-free; no closed-ladder reopen; battery sufficient (below, + design-authority
addition).

## Authorized RUNTIME-1 test battery (RUNTIME-1A scope; none green yet)

**Production-wiring floor:** `runtime1_explicit_opt_in_only`,
`runtime1_default_simsession_behavior_unchanged`, `runtime1_registers_named_mobility_composition_fixture`,
`runtime1_no_default_passgraph_schedule`, `runtime1_cpu_only_no_gpu_passgraph` *(design-authority
addition — enforces the 1A/1B split: RUNTIME-1A registers no GPU pass-graph)*,
`runtime1_preserves_runtime0_composition_order`, `runtime1_preserves_deterministic_replay`,
`runtime1_preserves_cpu_gpu_parity_proxy`, `runtime1_preserves_owner_overlay_isolated_unit`,
`runtime1_preserves_econ_owner_separation`, `runtime1_no_hard_soft_silent_mix`.

**Guardrails:** `runtime1_rejects_default_on_behavior`, `runtime1_rejects_semantic_or_raw_wgsl`,
`runtime1_rejects_cpu_planner_urgency_commitment`, `runtime1_rejects_owner_as_spatial_parent`,
`runtime1_rejects_capture_as_reparenting`, `runtime1_rejects_nested_arena_reparenting`,
`runtime1_rejects_default_on_resource_flow`, `runtime1_rejects_hard_currency_through_resource_flow`,
`runtime1_rejects_hybrid_strata_or_faction_index_scaling`, `runtime1_rejects_closed_ladder_reopen`,
`runtime1_rejects_unscoped_gpu_passgraph_wiring`.

**Perf/soak:** `runtime1_34k_production_fixture_soak`,
`runtime1_dirty_owner_modifier_steady_state_zero_redisperse`,
`runtime1_mobility_churn_with_owner_overlay_and_econ_clearinghouse`,
`runtime1_no_default_runtime_cost_when_disabled`.

Authorized, not implemented; no RUNTIME-1 test is green in this PR.

## Commands

```bash
cargo test -p simthing-spec --test mobility_runtime0_composition   # 23
cargo test -p simthing-spec --test mobility_scenario0_admission    # 13
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget  # 8
cargo test -p simthing-spec --test mobility_alloc0_substrate       # 15
cargo test -p simthing-spec --test mobility_reenroll0_substrate    # 16
cargo test -p simthing-spec --test mobility_idroute0_substrate     # 20
cargo test -p simthing-spec --test mobility_econ0_substrate        # 20
cargo test -p simthing-spec --test mobility_owner0_substrate       # 24
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15
cargo check --workspace                                            # 0 errors
```

## Posture attestation

Opening review only — no RUNTIME-1 implementation, no GPU pass-graph wiring, no default-on, no default
schedule, no gameplay integration, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU
planner/urgency/commitment, no Resource Flow default-on, no hard-currency through Resource Flow, no
Hybrid-Strata/faction-index scaling, no invariant changes. Substrate semantics unchanged. v7.8 M/E/T
closure, AO-WGSL-0 default-off, atlas runtime / E-11B-5 / B-1 / ClauseThing-L3 / FrontierV2-5 /
ACT-EVENT-OBS-PIPE all parked/closed.

## Next gate

**MOBILITY-RUNTIME-1A** — implement the CPU-only default-off production fixture (later PR). **RUNTIME-1B**
(GPU pass-graph registration, opt-in/non-default) is a separate, currently-closed gate, openable only
after 1A is green. Hybrid-Strata/faction-index ECON scaling remains a later slice.
