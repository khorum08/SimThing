# RUNTIME-0080-0-R1c-f Opening Handoff — cross the GPU structural-decision boundary (stop building copy-substrate)

**Intended recipient:** Cursor / Codex production implementation agent
**Authority:** RUNTIME-0080-0 production track after R1c-e
**Predecessors:** R1c-a … R1c-e (resident free-list mark → allocation → membership apply → compaction/lineage staging → compacted-view apply), all IMPLEMENTED / PASS
**Decision flag still false:** `structural_decisions_gpu_emitted` (false since R1b; unmoved across a–e)

## 1. Why this handoff exists (the drift call)

R1c-a through R1c-e built real, largely necessary resident structural substrate: a GPU free-list, an allocator, a membership table, a compaction map + lineage staging, and a compacted-view apply. That machinery is now **deep enough**. Continuing to add resident-table sub-rungs would be ceremony, for three concrete reasons measured from the landed code and reports:

1. **The one discriminator has not moved in five rungs.** `structural_decisions_gpu_emitted` is the single flag that turns R1c from PARTIAL to PASS (R1c handoff §5). It is explicitly `false` in every one of R1c-a/-b/-c/-d/-e. The substrate grew five layers deep; the defining flag did not move once.
2. **The "GPU apply" degenerated to identity-copy plumbing.** R1c-b used a genuine `CombineFn::Min` (a real GPU *selection*). R1c-c, -d, and -e use **only `CombineFn::Identity`**: the CPU computes every value, host-writes it into a GPU staging slot, the GPU copies staging→committed, and the CPU reads it back to compare against the same CPU oracle that produced the plan. The disabled-writer negative control proves rows *flow through GPU memory* — **not** that the GPU *decided* or *computed* anything.
3. **None of it is the 100-tick rehearsal.** R1c-c/-d/-e consume the aggregate `structural_events_from_gpu_journal` (the full ~247-row journal flattened across all 100 ticks) and do a **one-shot batch apply** against a single resident snapshot. There is no per-tick resident structural loop, no double-buffered journal drain, no tick-boundary swap for structural state. The structural side has never run as a resident per-tick loop.

The rigor (opt-in/default-off, disabled-transform negative control, CPU-oracle parity, complete-shadow preservation) is **correct and must be kept** — that is the anti-faking protocol, not the problem. The problem is **substrate sprawl without crossing the decision boundary**, plus the "Next horizon" pointer now reading **"M-4A / multi-atlas"**, which is a parked stop-line that does not flip the flag and must not be pulled forward.

## 2. The boundary to cross (the whole point of R1c-f)

Today, for every structural event, the **CPU decision witness decides that the event exists** (`R1aBoundaryWitness::step_tick_capture_events` runs the R6C tick kernels), and the GPU stages/copies/remaps the resulting rows. R1c-f must move the **decision of at least one structural event class onto the GPU**, computed from **resident state** via a generic reduce/threshold/emission-band — not an identity copy of a CPU-decided row.

> If the CPU still runs a tick kernel to decide whether the event happens and the GPU only copies the answer, R1c-f is PARTIAL. The GPU must *produce* the event from resident state.

**Pick the single cleanest class first — `ZeroCohort` (departure).** It is the most honestly GPU-decidable:

- The decision is a pure **threshold on resident state**: `num_ships_after == 0 ⇒ emit ZeroCohort(slot)`. `num_ships` is already a resident Tier-A value column (R1a) updated by the resident `DamageDelta` reduction.
- The GPU computes it with a **generic, semantic-free `Threshold` + `EmitEvent` / emission-band** over the resident `num_ships` column — the exact §0.2/§0.3 shape already used in R6/R6B and admissible under the §2.4 / §4a gate. No scenario-specific WGSL, no new semantic op.
- The CPU witness **stops deciding ZeroCohort**. The GPU emits the ZeroCohort journal rows; the CPU boundary pass only *applies* them (it already does, via `r1b_apply_boundary_events`).
- Result: `structural_decisions_gpu_emitted` flips **true for the ZeroCohort class** (honestly scoped partial of the umbrella), proven by exact parity of the GPU-emitted ZeroCohort rows against the CPU oracle and a disabled-emitter negative control.

This is a vertical slice through the boundary, not another horizontal substrate layer. Later sibling rungs do the same for `DamageDelta` (reduce), the move `StepOpportunity` threshold (R4 magnitude → `MoveRequest`), `LocalBirthRequest` (production threshold), and `FusionRequest` — each flipping one more class until the umbrella flag is unconditionally true.

## 3. Required scope for R1c-f

Implement `run_runtime_0080_0_r1c_f(input) -> Runtime0080R1cFReport`. It must:

1. Run the resident loop so the GPU **decides** `ZeroCohort` from the resident `num_ships` state via a generic `Threshold`/emission-band reduction (no `CombineFn::Identity` copy of a CPU-decided ZeroCohort row).
2. **Remove ZeroCohort from the CPU decision witness's emitted set** for this run (the witness may still advance other classes; the discriminator is that ZeroCohort is no longer CPU-decided).
3. Emit the GPU-decided ZeroCohort rows into the resident event journal; the CPU boundary pass only **applies** them.
4. Prove **exact parity** of GPU-emitted ZeroCohort rows vs the R6C CPU oracle (same ticks, same slots), measured from GPU values.
5. Disabled-emitter **negative control**: turning off the GPU ZeroCohort emitter yields zero ZeroCohort rows and fails parity; re-enabling restores it.
6. Set `structural_decisions_gpu_emitted_zero_cohort = true` and the umbrella `structural_decisions_gpu_emitted` only if/when *all* classes are GPU-decided (still false here — say so).
7. Preserve R1a Tier-A, R1b journal parity, R1c-a/-b/-c/-d/-e contracts, and the R1c complete-shadow serialize→reload→continue round-trip.

### Strongly preferred (the rehearsal direction)

Drive the GPU ZeroCohort decision **per-tick across the R6C 100-tick loop**, with the journal as the decoupling queue (handoff §4 of the R1c opening: GPU loop ahead, batched/lagged readback, boundary-cadence CPU drain). If a true per-tick resident structural loop is too large for one rung, it is acceptable to decide ZeroCohort per-tick from each tick's resident `num_ships` **while reusing the existing journal round-trip**, but state explicitly in the report whether the decision ran per-tick or one-shot, and name the per-tick-loop integration as the next rung. **Do not** regress to a flattened one-shot aggregate apply and call it per-tick.

## 4. Hard "do NOT" list (this is where the ceremony was)

- **Do NOT** add another resident-table copy sub-rung (R1c-g "resident X table", etc.) before a structural *decision* class is GPU-authoritative. a–e is enough plumbing.
- **Do NOT** pull M-4A / multi-atlas / sparse-residency scheduler forward. It is parked, out of scope, and does not flip the flag. Correct the production-track "Next horizon" line accordingly.
- **Do NOT** implement the GPU "decision" as `CombineFn::Identity` over a CPU-staged answer. The combine must be a real reduce/threshold over resident state. An Identity copy of a CPU-decided ZeroCohort row is an automatic PARTIAL.
- **Do NOT** delete or thin the complete CPU shadow; keep the serialize→reload→continue round-trip green.
- **Do NOT** strip the rigor (opt-in/default-off, disabled-transform negative control, CPU-oracle parity, preservation summaries). That rigor is the proof the boundary was really crossed — keep it; just point it at a real GPU decision.

## 5. Required tests (`crates/simthing-driver/tests/runtime_0080_0_r1c_f.rs`)

Prove the *decision moved*, not a flag:

1. `r1c_f_opt_in_default_off`
2. `r1c_f_gpu_decides_zero_cohort_from_resident_num_ships` (the decision is a GPU threshold/reduction, not an identity copy)
3. `r1c_f_cpu_witness_does_not_decide_zero_cohort` (witness no longer emits ZeroCohort; if it still ran the kernel to decide, fail)
4. `r1c_f_zero_cohort_rows_read_from_gpu_values`
5. `r1c_f_zero_cohort_parity_matches_r6c_oracle`
6. `r1c_f_disabled_zero_cohort_emitter_fails_parity` / `r1c_f_reenabled_zero_cohort_emitter_restores_parity`
7. `r1c_f_sets_structural_decisions_gpu_emitted_zero_cohort_true`
8. `r1c_f_umbrella_structural_decisions_gpu_emitted_remains_false_until_all_classes` (honest gate)
9. `r1c_f_preserves_r1a_tier_a_source_of_truth`
10. `r1c_f_preserves_r1b_event_journal_parity`
11. `r1c_f_preserves_r1c_a_b_c_d_e_contracts` (one test or five)
12. `r1c_f_preserves_r1c_complete_shadow_contract`
13. `r1c_f_no_identity_copy_substitution_for_decision` (asserts the ZeroCohort op uses a reduce/threshold combine, not `Identity`)
14. `r1c_f_no_m4a_or_multi_atlas`
15. `r1c_f_no_invariant_edit_or_scenario_reopen`
16. `r1c_f_domain_neutral_terms_only`
17. `r1c_f_report_checksum_stable`

A test must never pass merely because a report field says so; the decision authority is proven by the data-flow (GPU threshold over resident `num_ships`), not a string.

## 6. Report

`docs/tests/runtime_0080_0_r1c_f_resident_zero_cohort_decision_results.md` — verdict; the GPU decision op (combine fn + gate, showing it is reduce/threshold not Identity); whether the decision ran per-tick or one-shot; ZeroCohort parity vs oracle; disabled-emitter negative control; per-class flag `structural_decisions_gpu_emitted_zero_cohort = true`, umbrella flag still false with the remaining classes listed; preservation summaries; exact commands; stable checksum. Update `design_0_0_8_0_consumer_pulled_production_track.md` (replace the M-4A "Next horizon" with the class-by-class decision-boundary plan), `worklog.md`, and `workshop/mapping_current_guidance.md`. **Do not** edit `docs/invariants.md`.

## 7. Stop conditions (return to Opus)

Stop if R1c-f appears to require: a scenario-specific GPU compute pass or semantic WGSL tied only to 0080-2; a new *semantic* op (a generic semantic-free `Threshold`/emission-band is admissible under §2.4 / §4a); a CPU planner or CPU redecision of ZeroCohort; M-4A / multi-atlas; pinned-number change; scenario reopen; `docs/invariants.md` edit; loosening the R4 f32 bound; or no discrete GPU in the run environment.

## 8. Read first

1. `docs/handoffs/runtime_0080_0_r1c_resident_decision_opening.md` (§2 complete-shadow invariant, §4 GPU-non-starvation, §5 what flips PARTIAL→PASS)
2. `crates/simthing-driver/src/runtime_0080_0_r1b.rs` (journal + `step_tick_capture_events` decision witness; this is where ZeroCohort must stop being CPU-decided)
3. `crates/simthing-driver/src/runtime_0080_0_r1c_b.rs` (`CombineFn::Min` — the one prior rung that made a real GPU decision; model the ZeroCohort threshold on this, not on c/d/e)
4. `crates/simthing-driver/src/dress_rehearsal_r6c_integrated_run.rs` (`r1b_apply_boundary_events` ZeroCohort apply; R6/R6B threshold + emission-band shapes to reuse)
5. `docs/tests/runtime_0080_0_r1c_{c,d,e}_*_results.md` (the rungs that are identity-copy plumbing — do not extend the pattern)

## 9. Terminology

PR #539 domain-neutral terms: `FieldPolicy`, `field_agent`, `selection`, `extraction`, `resident event journal`, `resident membership table`, `GPU-decided structural event row`, `disabled-transform parity check`. Do not reintroduce normalized terms.
