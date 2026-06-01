# MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE-OPEN-0 — runtime-crate fixture wiring opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN WITH NARROWING** (Option B) → **`simthing-driver` test/support fixture only**.

Authorize a CPU-only runtime-crate fixture **confined to `simthing-driver` test/support** that
**delegates** to the green `simthing-spec` RUNTIME-1A fixture model — explicit opt-in / **default-off**,
**no default `SimSession` lib-path wiring, no GPU pass-graph, no default schedule, no gameplay**. A
non-test-support production `SimSession` surface (default lib/session path) stays a **separate later
gate**.

**Why narrowed.** The invariants permit a fixture/shell "invoked only from explicit test/fixture paths
until a separate gate authorizes production wiring" (the `ProductionCandidatePreview` default-off
precedent; economy→mapping fixture-orchestration-only). Confining the runtime-crate bridge to
`simthing-driver` test/support honors that exactly while still opening the genuinely-new step (the
driver→spec delegation executing in a real runtime crate). Crate-dependency direction is respected:
`simthing-driver` already depends on `simthing-spec`, so the fixture delegates downward, never the
reverse.

## Reviewed files

`phase_m_gating_and_doc_policy.md`; `invariants.md` (fixture/production-wiring rows govern; no
conflict, no edit); `design_v7_8.md` + v7.8 track; v7.9 track; workshop
`mobility_and_transfer_allocation.md` / `mapping_current_guidance.md` / `sead_self_ai_track.md`;
`phase_mobility_runtime1_opening_review_results.md` + `..._runtime1_results.md` +
`..._runtime1a_r1_results.md` + `..._runtime0_results.md`; `mobility_runtime1a.rs` +
`mobility_runtime0.rs`; `mobility_runtime1_production_fixture.rs` + `mobility_runtime0_composition.rs`;
`worklog.md`. (`docs/simthing_repoguide.md` ignored per handoff; removed from repo.)

## Accepted prerequisites (verified against the tree)

| Prerequisite | Result |
| --- | --- |
| SCENARIO-0 / AUDIT-0 | 13 / 8 |
| ALLOC-0 / REENROLL-0 / IDROUTE-0(+R1) / ECON-0 / OWNER-0(+R1) | 15 / 16 / 20 / 20 / 24 |
| RUNTIME-0 composition harness | `mobility_runtime0_composition` **23** |
| RUNTIME-1A CPU-only spec fixture model + R1 boundary clarification | `mobility_runtime1_production_fixture` **28**; R1 report present (actual runtime-crate wiring confirmed still-closed prior to this gate) |
| v7.8 M/E/T closeout preserved | c2 15 / clause_spec0 25 / met 10; `cargo check --workspace` 0 errors |

## Authorized scope (the narrowing)

- Fixture lives in **`simthing-driver` test/support** (not the default lib/session path).
- **Explicit opt-in / default-off named gate**; default `SimSession` behavior + default pass schedule
  unchanged; **zero runtime cost when disabled** (or an explicit deterministic no-op proxy).
- **CPU-only bridge** that **delegates to the `simthing-spec` RUNTIME-1A model** — no duplicated
  substrate logic; no GPU pass-graph registration; no GPU runtime hook.
- Preserves all RUNTIME-0/1A semantics: ALLOC→REENROLL→IDROUTE→ECON→OWNER order; movement writes only
  the mover's own columns; capture = owner-column flip; owner overlays reach isolated owned units by
  column presence; ECON flow separate from OWNER overlays; no hard/soft silent mix; deterministic
  replay; no CPU planner/urgency/commitment.

## Explicit non-goals

Non-test-support / default-path production `SimSession` surface (separate later gate); RUNTIME-1B GPU
pass-graph; GPU runtime hook; default schedule; gameplay-facing behavior; semantic/raw WGSL; designer
shader; CPU planner/urgency/commitment; owner-as-spatial-parent; capture-as-reparenting; nested arena
reparenting; default-on Resource Flow; hard-currency through Resource Flow; Hybrid-Strata /
faction-index scaling; atlas runtime; E-11B-5; B-1; ClauseThing/L3; FrontierV2-5; ACT/EVENT/OBS/PIPE;
invariant edits.

## Opening checks

All PASS: invariants permit a default-off test/support fixture (no production-wiring conflict);
RUNTIME-1A-R1's boundary clarification is sufficient prerequisite evidence; fixture is CPU-only,
default-off, delegates to spec (no duplication), zero/no-op disabled cost; crate dependency direction
(driver→spec) preserved; GPU pass-graph avoided entirely; default `SimSession`/schedule unchanged; no
semantic/raw WGSL; no CPU planner/urgency/commitment; `simthing-sim` semantic-free; no closed-ladder
reopen; battery sufficient (below, + design-authority addition).

## Authorized test battery (none green yet)

**Floor:** `runtime1a_runtime_fixture_explicit_opt_in_only`,
`runtime1a_runtime_fixture_default_simsession_unchanged`,
`runtime1a_runtime_fixture_registers_named_cpu_fixture`,
`runtime1a_runtime_fixture_no_default_passgraph_schedule`,
`runtime1a_runtime_fixture_cpu_only_no_gpu_passgraph`,
`runtime1a_runtime_fixture_confined_to_driver_test_support` *(design-authority addition — enforces the
Option-B narrowing: the fixture lives in `simthing-driver` test/support, not the default lib/session
path, and delegation direction is driver→spec only)*,
`runtime1a_runtime_fixture_delegates_to_spec_fixture_model`,
`runtime1a_runtime_fixture_preserves_runtime0_composition_order`,
`runtime1a_runtime_fixture_preserves_deterministic_replay`,
`runtime1a_runtime_fixture_preserves_owner_overlay_isolated_unit`,
`runtime1a_runtime_fixture_preserves_econ_owner_separation`,
`runtime1a_runtime_fixture_no_hard_soft_silent_mix`.

**Guardrails:** `runtime1a_runtime_fixture_rejects_{default_on_behavior, semantic_or_raw_wgsl,
cpu_planner_urgency_commitment, owner_as_spatial_parent, capture_as_reparenting,
nested_arena_reparenting, default_on_resource_flow, hard_currency_through_resource_flow,
hybrid_strata_or_faction_index_scaling, closed_ladder_reopen, gpu_passgraph_registration}`.

**Perf/soak:** `runtime1a_runtime_fixture_no_default_runtime_cost_when_disabled`,
`runtime1a_runtime_fixture_34k_cpu_fixture_soak`,
`runtime1a_runtime_fixture_mobility_churn_with_owner_overlay_and_econ_clearinghouse`,
`runtime1a_runtime_fixture_dirty_owner_modifier_steady_state_zero_redisperse`.

Authorized, not implemented; no test green in this PR.

## Commands

```bash
cargo test -p simthing-spec --test mobility_runtime1_production_fixture   # 28
cargo test -p simthing-spec --test mobility_runtime0_composition          # 23
cargo test -p simthing-spec --test mobility_scenario0_admission           # 13
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget      # 8
cargo test -p simthing-spec --test mobility_alloc0_substrate              # 15
cargo test -p simthing-spec --test mobility_reenroll0_substrate           # 16
cargo test -p simthing-spec --test mobility_idroute0_substrate            # 20
cargo test -p simthing-spec --test mobility_econ0_substrate               # 20
cargo test -p simthing-spec --test mobility_owner0_substrate              # 24
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15
cargo check --workspace                                                   # 0 errors
```

## Posture attestation

Opening review only — no runtime-crate fixture implementation, no GPU pass-graph, no default-on, no
default schedule, no gameplay, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU
planner/urgency/commitment, no Resource Flow default-on, no hard-currency through Resource Flow, no
Hybrid-Strata/faction-index scaling, no invariant changes. Substrate semantics unchanged. v7.8 M/E/T
closure, AO-WGSL-0 default-off, RUNTIME-1B / atlas runtime / E-11B-5 / B-1 / ClauseThing-L3 /
FrontierV2-5 / ACT-EVENT-OBS-PIPE all parked/closed.

## Next gate

**MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE** — implement the CPU-only default-off `simthing-driver`
test/support fixture delegating to the spec RUNTIME-1A model (later PR). Beyond it (each a separate,
currently-closed gate): a non-test-support production `SimSession` surface; **RUNTIME-1B** GPU
pass-graph; then default scheduling and gameplay integration. Hybrid-Strata/faction-index ECON scaling
remains a later slice.
