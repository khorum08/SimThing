# AO-WGSL-0-ACCEPT — Design-Authority Acceptance Review Results

## Verdict

**ACCEPT AO-WGSL-0 — Generic semantic-free AccumulatorOp WGSL performance path accepted.**
Compatible OrderBand plans may use the default-off fast path; unsupported shapes fall back.
Parity is proven for flat-star, A-0 D=3/D=4 Resource Flow, and B-0 transfer OrderBand fixtures.
Designer-authored semantic/raw WGSL remains rejected at admission; stale filename bans remain
deleted. A-0 remains pending separate design-authority review.

One narrow **performance remediation** was folded into acceptance per the endgame-scale performance
mandate (see §"Performance remediation"). It is semantically inert (bit-exact parity preserved) and
touches only the AO-WGSL-0 fast path; the legacy accepted path is byte-identical.

## Base HEAD

`10932fb` (master, merged PR #362) + AO-WGSL-0-ACCEPT remediation commit.

## Reviewer

Design authority (Opus 4.8 lane), acting under the project-owner performance mandate for endgame
scale.

## Scope

This is a review + gate decision. It does **not** accept A-0, open E-11B-5, B-1, Line C runtime,
ClauseThing/L3, FrontierV2-5, or ACT/EVENT/OBS/PIPE.

## Review questions and findings

| # | Question | Finding |
|---|---|---|
| 1 | Authorized by WGSL-GUARD-0/R1 corrected doctrine? | **Yes.** Generic GPU-resident WGSL via named production gate is permitted; AO-WGSL-0 is such a gate. Shader is static project code. |
| 2 | WGSL path generic and semantic-free? | **Yes.** `execute_orderband_bands` only consumes slot/dim/band indices, role/combine/gate/consume mode ids, buffers, strides, clamps, weights. Static token scan finds no `faction/planet/star/map/AI intent/economy/ClauseThing/ClauseScript/scenario`. |
| 3 | `accumulator_op_generic.wgsl` only a contract marker? | **Yes.** It documents allowed/forbidden concepts and points to the authoritative kernel in `accumulator_op.wgsl`. |
| 4 | `execute_orderband_bands` preserves global OrderBand sequencing? | **Yes.** One band per dispatch, sequential dispatches in one compute pass; WebGPU inserts inter-dispatch storage barriers, so band N's writes are visible to band N+1. Confirmed by bit-exact parity vs the per-pass legacy path. |
| 5 | Honestly described as sequential band dispatches, not cross-band fusion? | **Yes.** Code, shader comment, and report all state single band per dispatch; no in-shader band loop. |
| 6 | `classify_ao_wgsl0_plan` accepts only bounded compatible shapes? | **Yes.** Accepts ALWAYS/ORDER_BAND gates with a bounded combine allow-list; everything else falls back. |
| 7 | Unsupported shapes routed to fallback, not mis-executed? | **Yes.** Threshold/affine-intent/crossing/product/last-by-priority/empty/unknown gate or combine → `Fallback`; `run_resource_flow_bands_with_fast_path` only takes the fast path when `ao_wgsl0_fast_path_compatible`. |
| 8 | Default-off posture preserved? | **Yes.** `PipelineFlags::use_accumulator_wgsl_fast_path` defaults false; verified by `ao_wgsl0_no_default_on_resource_flow_or_hard_currency_reroute`. |
| 9 | `simthing-sim` flag acceptable? | **Yes.** It is a generic boolean pipeline plumbing flag with no map/faction/AI/economy semantics; no `ArenaSpec/ResourceFlowSpec/HierarchyNode` awareness added. |
| 10 | A-0/B-0/C-2 semantics preserved? | **Yes.** Resource Flow / transfer / atlas admission unchanged; the fast path is an alternate encode of the same ops. |
| 11 | CPU/oracle parity adequate? | **Yes.** RF: `GpuVerifiedApproximate`, max_abs_error/l_inf = 0.0 on D=3 leaves; D=4 buffer bit-exact. Transfer: `GpuVerifiedExact`, bit-exact, conservation 0.0. |
| 12 | Performance claims bounded/non-overclaimed? | **Yes.** Report flags timings include queue sync, are noisy on tiny fixtures, and are not production-grade. |
| 13 | Semantic-WGSL designer admission guard still tested? | **Yes.** `DesignerAdmissionRequest::SemanticWgsl` → `report.accepted == false` in AO-WGSL-0 and C-2 suites. |
| 14 | Generated artifacts avoided? | **Yes.** No `target/`, `*.replay.ldjson`, `.claude/worktrees/`, scratch logs committed. |
| 15 | Accept as generic GPU performance option? | **Yes — with the narrow performance remediation below.** |

## Performance remediation (folded into acceptance)

**Observed gap:** the landed fast path (`encode_orderband_fast_into`) reduced N compute passes to one
pass, but still created a fresh uniform buffer **and** a 13-binding bind group **per band, per tick**.
That is O(n_bands) GPU resource allocation + validation churn every tick. At endgame scale (deep
hierarchies → many OrderBands, evaluated every tick) this churn dominates and undercuts the
"performance path" intent.

**Remediation (narrow, semantic-free):**
- New `orderband_fast_layout` identical to the execute layout except binding 4 (tick uniform) is
  `has_dynamic_offset: true`. Only the fast pipeline uses it; the legacy `execute_pipeline`/layout is
  untouched.
- A single growable `orderband_fast_uniform` holds every band's `AccumulatorTickParams` at the
  device `min_uniform_buffer_offset_alignment` stride, written once per tick with one
  `queue.write_buffer`.
- A single bind group is built per encode and reused across bands via per-band dynamic offsets.

**Result:** per-tick GPU allocations for the band loop drop from O(n_bands) buffers + O(n_bands) bind
groups to **one buffer write + one bind group**, independent of band count. The uniform buffer is
reused across ticks (grown only when band count increases).

**Safety:** bit-exact parity preserved — `ao_wgsl0_generic_kernel_matches_existing_ao_for_*`
(flat-star, A-0 D=3/D=4, B-0 transfer) and `ao_wgsl0_replay_reproducibility` all pass unchanged. The
legacy per-band path bytes are unchanged. Wall-clock on the bounded D=3 fixture remains queue-sync
dominated (~1.3–1.7×); the remediation's value is the removed allocation scaling term, not the
micro-fixture wall-clock.

## Files changed in this pass

| Path | Change |
|---|---|
| `crates/simthing-gpu/src/accumulator_op/session.rs` | Dynamic-offset fast-path layout + growable uniform + single bind group; `encode_orderband_fast_into` rewritten; `uniform_entry_dynamic`, `ao_wgsl0_uniform_stride`, `ensure_orderband_fast_uniform`, `create_orderband_fast_bind_group` added |
| `docs/design_v7_8_production_track.md` | AO-WGSL-0 marked ACCEPTED; stale V7.8-CLEAN-0 note corrected |
| `docs/design_v7_8.md` | Acceptance noted |
| `docs/workshop/mapping_current_guidance.md` | Status updated to accepted |
| `docs/workshop/field_policy_track.md` | Acceptance note |
| `docs/worklog.md` | Acceptance entry |
| `docs/tests/phase_ao_wgsl0_acceptance_review_results.md` | This report |

## Guardrails confirmed (no authorization for)

designer-authored raw WGSL · semantic WGSL from designer/spec admission · scenario-generated shader
source · map/faction/AI/economy semantics in WGSL · global filename-based WGSL ban restoration ·
A-0 acceptance · E-11B-5 · Policy B · selector rerun · slot compaction · default-on Resource Flow ·
hard-currency through Resource Flow · B-1 · Line C runtime / sparse-residency scheduler ·
ClauseThing/ClauseScript/L3 · FrontierV2-5 · ACT-5/EVENT-3/OBS-5/PIPE-1 · simthing-sim semantic
awareness beyond the generic pipeline flag · invariant changes. **None authorized.**

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `AO-WGSL-0\|accumulator_op_generic\|execute_orderband_bands\|use_accumulator_wgsl_fast_path` | Named gate present | PASS |
| `accepted_wgsl_baseline\|ACCEPTED_WGSL\|WGSL whitelist\|no_new_wgsl` | No active ban; historical only | PASS — only historical report refs + test names |
| `raw WGSL\|SemanticWgsl\|semantic WGSL\|designer-authored WGSL` | Designer rejection active | PASS |
| forbidden semantics in `shaders/` + `accumulator_op/` | None except marker comment | PASS |
| `E-11B-5\|Policy B\|default-on Resource Flow\|hard-currency through Resource Flow` | Parked/rejected | PASS |
| `B-1\|sparse-residency\|atlas runtime\|M-6A\|M-5` | Parked/deferred | PASS |
| `ClauseThing\|L3\|FrontierV2-5\|ACT-5\|EVENT-3\|OBS-5\|PIPE-1` | Parked/rejected | PASS |
| `ArenaSpec\|ResourceFlowSpec\|HierarchyNode\|Resource Flow\|AO-WGSL-0` in simthing-sim | Generic flag only | PASS — only the pipeline flag |
| `target/` / `*.replay.ldjson` / `.claude/worktrees/` / scratch logs | None committed | PASS |

## Commands

```bash
cargo test -p simthing-driver --test phase_ao_wgsl0_accumulator_op_performance -- --nocapture  # 12/12
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture     # 19/19
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture                  # 11/11
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture                    # 12/12
cargo test -p simthing-driver --test phase_t_b0_d2a_hard_currency_ordering -- --nocapture       # 11/11
cargo test -p simthing-spec --test c2_atlas_admission_relaxation -- --nocapture                 # 15/15
cargo check --workspace                                                                         # ok
```

## Completion criteria

1. Implementation code reviewed (not only report). ✔
2. Acceptance decision explicit. ✔ (ACCEPT)
3. Production track marks AO-WGSL-0 accepted. ✔
4. (n/a — accepted) one narrow remediation named and applied. ✔
5. Semantic-free shader verified. ✔
6. Compatibility/fallback verified. ✔
7. Default-off posture verified. ✔
8. Parity evidence verified. ✔
9. Performance evidence bounded/non-production. ✔
10. Semantic-WGSL admission rejection verified. ✔
11. Generated-artifact absence verified. ✔
12. Stale C-0 sequencing note corrected. ✔
13. Review saved in docs/tests. ✔
14. Required checks pass. ✔
15. v7.8 constitution / production-track split intact. ✔

## Final verdict

**ACCEPT AO-WGSL-0 — Generic semantic-free AccumulatorOp WGSL performance path accepted as a generic
GPU performance option.** Compatible OrderBand plans use the default-off fast path (now O(1)
per-tick allocations via dynamic-offset uniform + single bind group); unsupported shapes fall back to
the existing accepted path. Parity proven for flat-star, A-0 D=3/D=4 Resource Flow, and B-0 transfer
OrderBand. Designer-authored semantic/raw WGSL remains rejected at admission; stale global filename
bans remain deleted; A-0/B-0/C-2 semantics and runtime posture unchanged. **A-0 remains pending
separate design-authority review.**
