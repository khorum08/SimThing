# Cursor Handoff: Phase M-first-slice — GPU-Resident First-Slice Mapping Runtime

**Status:** **Archived — not active guidance.** M-first-slice runtime promotion was cancelled; M-4 remains parked at decision gate. No mapping runtime was implemented.

## Goal

Wire the **ADR-named first scenario-level mapping slice** into an opt-in production session pass graph. This is the **production runtime** step the Mapping ADR explicitly separates from Phase M generic natives (M-1..M-3). It exercises Layers 1–3 end-to-end on the GPU with **only adopted** optimizations.

This handoff follows M-4A evaluation (`mapping_m4a_post_merge_evaluation_test_results.md`): M-4A commits are consistent; **Option B is recommended** because the first slice requires **no atlas**.

---

## Required posture (preserve)

```text
V7.7 Mapping ADR approved at architecture level.
M-1 / M-1.1 / M-2 / M-2.1 / M-3 landed.
M-4 design note + M-4A evidence parked (atlas unimplemented).
StructuredFieldStencilOp remains live, opt-in, hardened, GPU-resident by default.
RegionFieldSpec remains designer/spec structure; MappingExecutionProfile default Disabled until explicitly enabled.
simthing-sim remains map-free (opaque AccumulatorOp registrations only).
Resource Flow defaults unchanged.
No Scatter/Gather. No dynamic nested enrollment. No D-2a. No E-11B-5.
No atlas batching. No active mask / halo. No behavioral source policy (M-5 deferred).
No semantic/map/faction/AI WGSL.
```

---

## First slice definition (ADR §First slice — binding)

Single-faction tactical suppression field on one bounded theater:

| Parameter | Value |
|---|---|
| Grid | One RegionCell grid, **≤ 32×32** (recommend **10×10** for initial wiring) |
| Operator | `source_capped_normalized` |
| Horizon | **H ≤ 8** |
| Source policy | `CallerManagedOneShotSeedThenZero` |
| Cadence | `EveryTick` |
| Optimizations | Dirty macro-region skip (**adopted**) |
| Excluded | **No atlas**, **no active mask**, **no halo**, **no cadence tier skip beyond scheduler** |
| Layer 2 | Sum reduction into one parent threat column (`SlotRange` Sum via column-aware reduction) |
| Layer 3 | One `field_urgency` EvalEML on parent |

---

## Architecture (GPU-resident)

```text
Session open (MappingExecutionProfile = Enabled, explicit scenario opt-in)
  → RegionFieldSpec RON admission (M-3) → generic stencil config + reduction binding + formula binding
  → FieldScheduler registration (M-2.1) — one (FieldId, FieldRegionId)
  → Each tick:
      FieldScheduler::schedule_and_execute (no-readback hot path — M-1.1)
        → StructuredFieldStencilOp::execute_configured (caller-managed protocol internal or orchestrated)
        → Column-aware Sum reduction (existing AccumulatorOp)
        → EvalEML field_urgency (existing C-8 path; formula class admitted at spec layer)
  → Debug/diagnostics: explicit readback only (tests, mapping debug surface)
```

**simthing-sim sees:** flat column buffers + opaque `AccumulatorOp` list. **No** RegionCell/atlas/cadence concepts.

---

## Allowed implementation scope

```text
simthing-driver: mapping session module (or extend session.rs behind MappingExecutionProfile gate)
  — compile RegionFieldSpec → runtime registrations
  — wire FieldScheduler + StructuredFieldStencilOp + reduction + EvalEML tick hook
simthing-spec: admission already landed; extend only if first-slice wiring exposes a real gap
Integration tests under simthing-driver/tests/
Mapping debug report struct (dispatch count, dirty ratio, field max/L1 — ADR §Debug surfaces)
Doc updates (production plan, todo, worklog, mapping guidance, invariants if needed)
```

---

## Not allowed

```text
No atlas packer (Option A / M-4 implementation).
No new WGSL kernels (use production structured_field_stencil.wgsl).
No StructuredFieldStencilOp behavior change unless bugfix with parity tests.
No MappingExecutionProfile default change (must stay Disabled).
No simthing-sim RegionField/map types.
No pass graph wiring into default/non-mapping scenarios.
No ActiveOnlyExperimentalNoHalo.
No source_mask / behavioral source (M-5).
No E-11B / Resource Flow default changes.
```

---

## Caller-managed source protocol (runtime obligation)

The runtime orchestrator must implement the ADR/remedial/M-4A protocol:

```text
1. Seed source identity cells (from scenario/spec — not atlas-global only).
2. Run initial stencil hop (or use execute_configured with steps=1).
3. Clear ONLY seed identity cells in source_col (never column-wide zero).
4. Run configured horizon hops.
```

CPU oracle parity tests must use `cpu_horizon` + seed-only clear (same as structured_field_stencil tests).

---

## Test battery

Save results to `docs/tests/phase_m_first_slice_runtime_test_results.md`.  
Save full log only if useful: `docs/tests/phase_m_first_slice_runtime_full.log`.

### Test 0 — Guardrail sanity

Assert/report:

```text
MappingExecutionProfile default Disabled
PipelineFlags default RF unchanged
simthing-sim map-free
No atlas / active-mask code in session hot path
M-4 remains unimplemented
StructuredFieldStencilOp unchanged except orchestration calls
```

Expected: **PASS**

### Test 1 — RON roundtrip → runtime registration

Valid first-slice RegionFieldSpec RON → compiles to stencil config + reduction + formula binding.  
Invalid specs still rejected (M-3 rejection suite unchanged).

Expected: **PASS**

### Test 2 — Single-tick GPU execution (10×10, H=8, source capped)

Seed cluster → caller-managed protocol → field non-zero at interior; gradient sign sensible (reuse M-1 t44-style checks).

Expected: **PASS**

### Test 3 — FieldScheduler integration

EveryTick cadence; dirty skip with zero false-skips on fixture; dispatch count matches horizon.

Expected: **PASS**

### Test 4 — Layer 2 reduction

Sum into parent threat column; parent value matches manual SlotRange Sum oracle.

Expected: **PASS**

### Test 5 — Layer 3 EvalEML

Parent `field_urgency` EvalEML produces finite bounded output; matches CPU oracle for whitelisted formula class.

Expected: **PASS**

### Test 6 — No-readback hot path

Production tick path uses `execute_configured` with `readback_values: false`; readback only in test/diagnostic mode.

Expected: **PASS**

### Test 7 — Default-off enforcement

Spec present but MappingExecutionProfile Disabled → no mapping dispatches on session tick.

Expected: **PASS**

### Test 8 — End-to-end replay determinism

Two identical sessions → identical dispatch counts and readback checksums (with debug readback enabled for test).

Expected: **PASS**

---

## Decision gate summary (required in results doc)

```markdown
| Test | Area | PASS / PARTIAL / FAIL | Key result |
|---|---|---|---|
| 0 | Guardrail sanity | | |
| 1 | RON → runtime | | |
| 2 | GPU stencil execution | | |
| 3 | FieldScheduler | | |
| 4 | Layer 2 Sum | | |
| 5 | Layer 3 EvalEML | | |
| 6 | No-readback hot path | | |
| 7 | Default-off | | |
| 8 | Determinism | | |
```

Final verdict: **PASS** / **PARTIAL** / **FAIL**

---

## Doc updates (required on completion)

Update with result-dependent wording:

```text
docs/accumulator_op_v2_production_plan.md  — M-first-slice PR row + status
docs/todo.md                               — first-slice status
docs/worklog.md                            — entry
docs/workshop/mapping_current_guidance.md  — runtime posture if PASS
docs/workshop/workshop_current_state.md    — HEAD + verification
docs/invariants.md                         — only if new invariant rows needed
docs/adr/mapping_sparse_regioncell.md      — only if first-slice landing note (no classification change)
```

If **PASS**, guidance wording:

```text
First-slice mapping runtime landed behind MappingExecutionProfile opt-in.
Exercises one bounded grid, caller-managed source, dirty skip, Sum + field_urgency.
No atlas. No active mask. simthing-sim remains map-free. Defaults unchanged.
```

If **PARTIAL** or **FAIL**, park runtime and document blockers; do not enable profile by default.

---

## Revert verification (if sandbox scaffolding used)

After any temporary probe files, preserve to `docs/workshop/` then remove transient tests/runtime. Run:

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture
cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture
cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture
cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture
cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
cargo check --workspace
cargo test --workspace
```

Save: `docs/tests/revert_phase_m_first_slice_to_parked_state_test_results.md` (if revert needed).

---

## Stop conditions

Stop and report rather than widening scope if:

```text
Implementation requires atlas batching or active mask.
Implementation requires new WGSL or simthing-sim map types.
MappingExecutionProfile would default to Enabled.
Behavioral source / source_mask required.
E-11B or Resource Flow regressions fail.
cargo check/test --workspace fails after landing.
Product has not explicitly authorized Option B (first-slice wiring).
```

---

## Completion criteria

```text
1. MappingExecutionProfile opt-in session wiring for first slice.
2. RegionFieldSpec → GPU stencil + reduction + EvalEML orchestration.
3. Caller-managed source protocol in runtime.
4. FieldScheduler + dirty skip integrated.
5. No-readback hot path on production tick.
6. Default-off preserved.
7. Test battery 0–8 results in docs/tests/.
8. Active docs updated (including production plan).
9. simthing-sim remains map-free.
10. Full workspace check/test pass.
```

---

## Alternative path (not this handoff)

**Option A — M-4 atlas packer:** Blocked until human + Opus sign-off ratifies M-4A algebraic-mask amendment in Mapping ADR. See M-4 design note + M-4A evidence. Do not start unless product explicitly selects Option A over first-slice wiring.
