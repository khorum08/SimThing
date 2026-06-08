# A-0-ACCEPT-0 — Design-Authority Acceptance Review Results

## Verdict

**ACCEPT A-0; CLOSE PROMOTED M/E/T.** A-0 accepts the static nested Resource Flow first slice.
Authored nested participants materialize into D=3/D=4 Resource Flow layouts; per-parent contiguous
SlotRange enforcement, non-contiguous rejection (no compaction), and reserved-gap exclusion are
proven; existing AccumulatorOp OrderBand execution produces **bit-exact** GPU/CPU oracle parity;
Resource Flow remains opt-in/default-off; hard-currency stays Phase T. Together with C-2 map-batching
designer-surface closure and B-0 hard-currency ordering closure, **all promoted v7.8 M/E/T lines are
now closed for their current named scenarios; no implementation gate remains open.**

No dynamic enrollment, Policy B, selector rerun, wildcard expansion, gap-child promotion, slot
compaction, indirection-list SlotRange, default-on Resource Flow, hard-currency-through-Resource-Flow,
CPU fallback, boundary-time allocator, simthing-sim awareness, B-1, Line C runtime, L3, FrontierV2-5,
or ACT/EVENT/OBS/PIPE expansion is authorized. WGSL doctrine remains corrected: generic GPU kernels
may be introduced through named gates; semantic/raw WGSL from designer/spec admission is rejected.

This review made **no code changes**; it is a gate decision validated against the final tree.

## Base HEAD

`dd01b4c` (master, post AO-WGSL-0-ACCEPT / PR #363).

## Reviewer

Design authority (Opus 4.8 lane). All guardrails raised to the designer-facing simthing-spec layer.

## Scope confirmation

A-0 only. Does NOT open E-11B-5, Policy B, Line B/B-1, Line C runtime, ClauseThing/L3, FrontierV2-5,
or ACT/EVENT/OBS/PIPE.

## Code reviewed (not only reports)

| File | What was verified |
|---|---|
| `arena_hierarchy.rs` | Band math `total_bands(D)=3D−1`, `integration(D)=3D−2`; `build_execution_plan` selects `build_nested_layout` only via `has_nested_participants` (an `ArenaParticipant` child that itself contains a participant child); recursive `verify_child_contiguity`; `nested_hierarchy_materialization_report`. |
| `arena_allocation_plan.rs` | Re-verifies contiguity per node before planning (rejects, no compaction); reset→upsweep(`Sum` SlotRange)→downsweep(broadcast + `EvalEML` child-share disburse)→`IntegrateWithClamp` integration at `integration_band`; `child_range` = `(children[0].slot, children.len())` (active block only). |
| `arena_allocation_oracle.rs` | CPU oracle mirrors the GPU plan exactly, sharing `child_share_cpu` with the GPU EvalEML tree → exact parity reference. |
| `arena_participant.rs` | Active children allocated contiguously (`slots_are_contiguous`); reserved gaps allocated in a **separate exclusive gap block** (`reserve_exclusive_gap_block`); `nested_fission_gap_report` proves gaps fall outside the active child span and the sibling range; gap pool consumed LIFO with `Reject` on exhaustion. |
| `phase_e_a0_nested_resource_flow_static.rs` | 19 tests covering all claims (see below). Fission helper is used **only to deliberately break contiguity and prove rejection** — not to enable dynamic enrollment. |
| `wgsl_path.rs` / `session.rs` (AO-WGSL-0) | Default-off fast path is semantics-preserving; no A-0 semantic change. |
| `simthing-sim` | No `HierarchyNode`/`ResourceFlowSpec`/`ArenaSpec`/`E-11B`/`Resource Flow` awareness. |

## Review questions and findings

| # | Question | Finding |
|---|---|---|
| 1 | Static nested first-slice scope satisfied? | **Yes** — static materialization + D=3/D=4 parity, no dynamic enrollment. |
| 2 | Authored topology → production-equivalent nested layouts? | **Yes** — `materialize_arena_participants` builds `ArenaParticipant` subtree; `build_nested_layout` reconstructs the `HierarchyNode` tree from allocated slots. |
| 3 | `build_execution_plan` chooses nested only when nested participants exist? | **Yes** — `has_nested_participants` gate; otherwise flat-star. |
| 4 | Per-parent child contiguity enforced? | **Yes** — recursive `verify_child_contiguity`, re-verified in `plan_arena_allocation`. |
| 5 | Non-contiguous active children reject without compaction? | **Yes** — `HierarchyError::NonContiguousChildren`; test `a0_noncontiguous_nested_children_reject`. |
| 6 | Reserved-gap children excluded from active SlotRanges? | **Yes** — separate exclusive gap block; `child_range` counts only active children; `nested_fission_gap_report.gap_outside_active_child_span`. |
| 7 | D=3/D=4 OrderBand + integration-band math correct? | **Yes** — D=3: 8 bands, integration 7; D=4: 11 bands, integration 10. |
| 8 | Existing AccumulatorOp v2 GPU path executes nested allocation? | **Yes** — depth-generic `Sum`/`EvalEML`/`IntegrateWithClamp` OrderBands; bit-exact parity. |
| 9 | AO-WGSL-0 preserves/improves A-0 execution without semantic change? | **Yes** — default-off, semantics-preserving; A-0 parity unchanged with fast path on/off. |
| 10 | CPU/GPU oracle parity adequate and honestly classified? | **Yes** — `GpuVerifiedApproximate`; D=3/D=4 `max_abs_error = 0.0` (bit-exact in fixtures). |
| 11 | `use_accumulator_resource_flow` default false preserved? | **Yes** — `a0_resource_flow_flag_default_false_unchanged`. |
| 12 | Hard-currency stays Phase T, not Resource Flow? | **Yes** — `use_accumulator_transfer=false`, `resource_economy_registry=None`. |
| 13 | Dynamic enrollment / E-11B-5 avoided? | **Yes** — `last_resource_flow_dynamic_enrollment_report.is_none()`. |
| 14 | Policy B / selector rerun / wildcard / compaction / indirection-list avoided? | **Yes** — source scans + behavior; `child_range` is direct contiguous SlotRange, no indirection. |
| 15 | WGSL-GUARD-0/R1 removed stale bans without weakening semantic-WGSL admission? | **Yes** — no `accepted_wgsl_baseline` code; `SemanticWgsl` rejection active at designer admission. |
| 16 | AO-WGSL-0 acceptance downstream concern for A-0? | **No** — default-off, semantics-preserving. |
| 17 | Accept A-0 as Line A static nested first slice? | **Yes.** |
| 18 | Are all promoted M/E/T lines closed for current named scenarios? | **Yes** — A (this), B (B-0), C (C-2). |

## A-0 test results (19/19)

`a0_static_nested_d3/d4_materializes_from_authored_topology`, `a0_nested_children_contiguous_per_parent`,
`a0_noncontiguous_nested_children_reject(_without_compaction)`,
`a0_reserved_gap_slots_excluded_from_active_slotranges` / `..._stay_outside_active_child_slotranges`,
`a0_d3/d4_orderband_budget_and_integration_band`, `a0_d3/d4_gpu_cpu_oracle_parity`,
`a0_replay_reproducibility`, `a0_resource_flow_flag_default_false_unchanged`,
`a0_hard_currency_not_routed_through_resource_flow`, `a0_no_new_wgsl_roles_or_cpu_fallback`,
`a0_no_dynamic_enrollment_policy_b_selector_rerun_or_compaction`,
`a0_no_b1_c_runtime_l3_frontierv2_5_or_act_event_obs_pipe`, `a0_no_simthing_sim_semantic_awareness`,
`a0_nested_static_children_are_contiguous_per_parent`.

## Parity table

| Fixture | Class | max_abs_error | l_inf | conservation | replay |
|---|---|---:|---:|---:|---|
| A-0 D=3 (7 participants, 8 OrderBands, 4 leaves) | GpuVerifiedApproximate | 0.0 | 0.0 | 0.0 | reproducible (bit-identical) |
| A-0 D=4 (7 participants, 11 OrderBands) | GpuVerifiedApproximate | 0.0 | 0.0 | 0.0 | reproducible |

## Guardrails confirmed (no authorization for)

E-11B-5 dynamic enrollment · Policy B Reevaluate · selector rerun · wildcard/dynamic selector
expansion · automatic gap-child promotion · slot compaction · indirection-list SlotRange · default-on
Resource Flow · hard-currency through Resource Flow · CPU production fallback · boundary-time
allocator/hot pool · Line B/B-1 · Line C production runtime/sparse-residency scheduler ·
ClauseThing/ClauseScript/L3 · FrontierV2-5 · ACT-5/EVENT-3/OBS-5/PIPE-1 · simthing-sim Resource
Flow/spec awareness · semantic/raw WGSL from designer/spec admission · invariant changes. **None
authorized.** Generic WGSL remains allowed only through named production gates; the stale filename
whitelist stays deleted.

## Artifact absence verified

`crates/simthing-driver/tests/support/accepted_wgsl_baseline.rs`, `.claude/worktrees/...`,
`crates/simthing-workshop/target/workshop/eml_phase5_rich_report_100k.md`, `demo.replay.ldjson` — all
absent. No committed `target/` or `*.replay.ldjson` artifacts.

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `A-0 / E-11B / nested Resource Flow / phase_e_a0` | present, accepted | PASS |
| `E-11B-5 / dynamic enrollment / Policy B / selector rerun / wildcard / slot compaction / indirection-list` | parked/rejected | PASS |
| `use_accumulator_resource_flow / default-on RF / hard-currency through RF` | opt-in/default-off; separate | PASS |
| `accepted_wgsl_baseline / ACCEPTED_WGSL / no_new_wgsl` | historical only | PASS |
| `semantic WGSL / SemanticWgsl / raw WGSL` | rejected at designer/spec admission | PASS |
| `AO-WGSL-0 / execute_orderband_bands / use_accumulator_wgsl_fast_path` | accepted, default-off; no A-0 semantic change | PASS |
| `B-1 / C-runtime / sparse-residency / M-6A / M-5` | Line B closed; Line C runtime deferred | PASS |
| `ClauseThing / L3 / FrontierV2-5 / ACT-5 / EVENT-3 / OBS-5 / PIPE-1` | parked/rejected | PASS |
| `ArenaSpec / ResourceFlowSpec / HierarchyNode / E-11B` in simthing-sim | no awareness | PASS |

## Commands

```bash
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture  # 19/19
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture               # 11/11
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture                 # 12/12
cargo test -p simthing-driver --test phase_ao_wgsl0_accumulator_op_performance -- --nocapture # 12/12
cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture                    # 13/13
cargo test -p simthing-spec --test c2_atlas_admission_relaxation -- --nocapture             # 15/15
cargo test -p simthing-spec --test v7_8_met_consumer_scenarios -- --nocapture               # 10/10
cargo check --workspace                                                                     # ok
```

## Promoted M/E/T closure

Promoted v7.8 M/E/T lines are closed for current named scenarios: **Line C/M** map batching closed at
the designer surface (C-0/C-1/C-2), **Line B/T** hard-currency ordering closed at the narrow smoke
level (B-0), and **Line A/E** static nested Resource Flow closed at the first nested slice (A-0). No
implementation gate remains open. Future nested dynamic enrollment (E-11B-5), production atlas runtime
/ sparse-residency scheduler, mixed-kind/multi-band hard-currency ordering, or ClauseThing/L3 each
require a separate named scenario / product authorization.

## Completion criteria

1–18 all satisfied (implementation + A-0-R1 + WGSL-GUARD-0/R1 + AO-WGSL-0 context reviewed; explicit
ACCEPT; production track updated; materialization/contiguity/gap/parity verified; opt-in/default-off
and hard-currency separation verified; no widening authorized; WGSL doctrine preserved; report saved;
required checks pass; v7.8 constitution/production-track split intact).

## Final verdict

**ACCEPT A-0; CLOSE PROMOTED M/E/T** — A-0 accepts the static nested Resource Flow first slice.
Together with C-2 map-batching designer-surface closure and B-0 hard-currency ordering closure, all
promoted v7.8 M/E/T lines are closed for their current named scenarios. Future nested dynamic
enrollment, production atlas runtime, mixed-kind hard-currency ordering, ClauseThing/L3, FrontierV2-5,
or ACT/EVENT/OBS/PIPE work requires separate product authorization or a named scenario. Correct WGSL
doctrine remains: generic GPU kernels may be introduced through named gates; semantic/raw WGSL from
designer/spec admission is rejected.
