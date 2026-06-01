# MOBILITY-RUNTIME-1B-OPEN-0 — GPU pass-graph registration gate

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Tier-2 gate (GPU pass-graph = a named Tier-2
threshold under v7.9 §2.1). Opening review only — no implementation.

## Verdict

**OPEN WITH NARROWING** → **non-scheduled GPU pass-graph node *registration* in `simthing-driver`
test/support only.**

Authorize an **opt-in, default-off** named GPU pass-graph **node registration** that **delegates to
the green RUNTIME-1A CPU fixture** and carries the existing CPU/GPU-proxy parity classification —
**no scheduled GPU dispatch, no new/semantic/raw WGSL, no default schedule, no gameplay, no default
production path.**

**Why narrowed (pivotal fact):** the RUNTIME-1A driver fixture is **pure CPU** (0 wgpu/dispatch/WGSL
references) delegating to the spec model; the "GPU" side of parity is a deterministic checksum proxy.
**There is no mobility shader to dispatch** — the composition is CPU accounting (slab allocation,
masked reductions). So "GPU pass-graph registration" can only honestly mean *registering a
non-scheduled node*, not executing a kernel. A real scheduled GPU dispatch would require a generic
GPU execution path that does not exist, and is a **separate later slice** — not opened here.

## Files reviewed

`invariants.md` (governing doctrine + closure posture; not edited); `design_v7_8.md` §2.4–§2.5;
`design_v7_9...track.md` §2.1 + runtime rows; `phase_mobility_runtime1a_runtime_fixture_results.md`;
`crates/simthing-driver/tests/support/mobility_runtime1a_fixture.rs` (confirmed CPU-only, no GPU);
`crates/simthing-driver/tests/mobility_runtime1a_runtime_fixture.rs`.

## Evaluation

| Check | Finding |
| --- | --- |
| CPU fixture evidence sufficient prerequisite | PASS — driver fixture 21, spec fixture 28, composition 23, `cargo check` 0 errors |
| Can stay opt-in / non-default | PASS — registration behind a named default-off gate |
| Avoids default schedule + gameplay | PASS — non-scheduled node; no gameplay path |
| Stays semantic-free (no raw/semantic WGSL) | PASS — registration adds **no shader**; none exists to add |
| Preserves CPU/GPU-proxy parity + honest classification | PASS — delegates to the spec parity proxy; classification unchanged |
| Preserves no CPU planner / urgency / commitment | PASS |
| Avoids reopening atlas runtime / E-11B-5 / B-1 / ClauseThing-L3 / FrontierV2-5 / ACT-EVENT-OBS-PIPE / Hybrid-Strata / faction-index | PASS — registration delegates to the existing accepted path only |

## Authorized RUNTIME-1B scope (the narrowing)

- Opt-in, default-off, **non-scheduled** GPU pass-graph **node registration** in `simthing-driver`
  test/support.
- **Delegates** to the green RUNTIME-1A CPU fixture (no duplicated logic); preserves
  ALLOC→REENROLL→IDROUTE→ECON→OWNER order, deterministic replay, CPU/GPU-proxy parity classification,
  isolated owner-overlay delivery, ECON/OWNER separation, hard/soft separation.
- **No scheduled GPU dispatch; no new/semantic/raw WGSL; no new designer shader surface; no default
  schedule; no gameplay path; no default production path.**

## Explicit non-goals

Scheduled GPU dispatch / kernel execution (separate later slice); default schedule; gameplay; default
production path; semantic/raw WGSL; new shader surface; CPU planner/urgency/commitment;
owner-as-spatial-parent; capture-as-reparenting; default-on Resource Flow; hard-currency through
Resource Flow; Hybrid-Strata/faction-index scaling; atlas runtime; E-11B-5; B-1; ClauseThing/L3;
FrontierV2-5; ACT/EVENT/OBS/PIPE; invariant edits.

## Authorized test battery (none green yet)

Floor: `runtime1b_explicit_opt_in_only`, `runtime1b_default_schedule_unchanged`,
`runtime1b_registers_named_gpu_passgraph_fixture`, `runtime1b_no_default_passgraph_schedule`,
`runtime1b_non_scheduled_registration_no_gpu_dispatch` *(design-authority addition — enforces the
narrowing: registration only, no scheduled kernel execution, no new shader)*,
`runtime1b_delegates_to_runtime1a_fixture`, `runtime1b_preserves_runtime0_composition_order`,
`runtime1b_preserves_deterministic_replay`,
`runtime1b_preserves_cpu_gpu_parity_or_honest_approx_classification`,
`runtime1b_preserves_owner_overlay_isolated_unit`, `runtime1b_preserves_econ_owner_separation`,
`runtime1b_no_hard_soft_silent_mix`.
Guardrails: `runtime1b_rejects_{default_on_behavior, semantic_or_raw_wgsl,
cpu_planner_urgency_commitment, owner_as_spatial_parent, capture_as_reparenting,
default_on_resource_flow, hard_currency_through_resource_flow, hybrid_strata_or_faction_index_scaling,
closed_ladder_reopen}`.
Perf: `runtime1b_no_default_runtime_cost_when_disabled`,
`runtime1b_34k_gpu_fixture_soak_or_precise_blocker` (a precise blocker is acceptable here, since
non-scheduled registration runs the CPU delegate — a real GPU soak belongs to the later dispatch slice).

Authorized, not implemented; no test green.

## Commands

```bash
cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture   # 21
cargo test -p simthing-spec --test mobility_runtime1_production_fixture   # 28
cargo test -p simthing-spec --test mobility_runtime0_composition          # 23
cargo check --workspace                                                   # 0 errors
```

## Posture attestation

Opening review only — no implementation, no scheduled GPU dispatch, no shader, no default schedule,
no gameplay, no default path, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU
planner/urgency/commitment, no invariant changes. Closed/parked ladders unchanged.

## Next gate

**MOBILITY-RUNTIME-1B** — implement the non-scheduled GPU pass-graph node registration (later PR).
Beyond it (separate, currently-closed): a real **scheduled GPU dispatch** (needs a generic GPU
execution path); a non-test-support default `SimSession` path; default schedule; gameplay.
