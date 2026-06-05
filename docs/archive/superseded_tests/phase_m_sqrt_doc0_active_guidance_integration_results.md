# SQRT-DOC-0 R1 — Active Guidance Integration Results

**Lane:** Documentation / active-guidance integration for shader/software deterministic `sqrt` follow-on track (not implementation, not exhaustive battery, not production sqrt admission, not JIT reopening).

**Base HEAD:** `6b1ac5f267d33512c7c845de8bdfa71027b9151e`

**Branch:** `phase-sqrt-doc0-r1-active-guidance`

---

## Summary

Integrated `docs/workshop/sqrt_candidates.md` into active guidance as a separate follow-on track. Removed stale reference to deleted `phase_m_jit_desc0_kernel_descriptor_test_results.md` from the design note. M-JIT remains closed at PROD-0; native sqrt remains `ApproximateJitOnly`; exact sqrt admission remains blocked until exhaustive bit-exact proof. No implementation code changed.

---

## Files changed

| File | Change |
|---|---|
| `docs/workshop/sqrt_candidates.md` | Replaced deleted DESC-0 companion link with current retained authority; updated `validate_exact_inputs` wording to `simthing-spec` admission APIs |
| `docs/workshop/mapping_current_guidance.md` | Added shader/software deterministic sqrt status row; linked `sqrt_candidates.md` in M-JIT follow-on list |
| `docs/accumulator_op_v2_production_plan.md` | Added compact shader/software exact sqrt follow-on section |
| `docs/invariants.md` | Added exhaustive-proof row for exact sqrt authority grant |
| `docs/worklog.md` | One append-only R1 line |
| `docs/tests/phase_m_sqrt_doc0_active_guidance_integration_results.md` | This report |

**No code files changed. No deleted reports restored. No E-phase evidence touched.**

---

## Stale reference fixed

**Removed from `sqrt_candidates.md` companions:**
- `tests/phase_m_jit_desc0_kernel_descriptor_test_results.md`

**Replaced with:**
- `docs/tests/phase_m_jit_prod0_registry_shell_test_results.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`
- `docs/tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md`
- `docs/invariants.md`
- `docs/workshop/mapping_current_guidance.md`

**Scan note:** `rg "phase_m_jit_desc0_kernel_descriptor_test_results" docs` — no matches in active authority surfaces (`sqrt_candidates.md`, mapping guidance, production plan, invariants). Historical `docs/worklog.md` append-only entries and PROD-0 report deleted-file inventory retain the filename as history only; DESC-0 report was **not** restored.

---

## Active docs updated

- **mapping_current_guidance.md:** New row — Shader/software deterministic sqrt | T2 | design-open | `sqrt_candidates.md`
- **accumulator_op_v2_production_plan.md:** Follow-on section with design note link and guardrails
- **invariants.md:** Exact sqrt authority requires exhaustive proof before admission grant

---

## Confirmations

| Check | Result |
|---|---|
| M-JIT remains closed at PROD-0 | **Yes** — status table unchanged |
| Native sqrt remains `ApproximateJitOnly` | **Yes** — design note + invariants + production plan |
| Exact sqrt admission blocked until exhaustive proof | **Yes** — `max_ulp == 0` vs CPU `f32::sqrt` required |
| No production sqrt admission added | **Yes** — docs only |
| No implementation code changed | **Yes** |
| No deleted reports restored | **Yes** |
| No E-phase evidence touched | **Yes** |
| No SHA hygiene loop | **Yes** |

---

## Required scans

### Deleted DESC-0 reference scan

```
rg "phase_m_jit_desc0_kernel_descriptor_test_results" docs
```

**Result:** Matches only in historical `worklog.md` (append-only landed history) and PROD-0 report deleted-file inventory. **No active authority link remains in `sqrt_candidates.md` or updated guidance.**

### Sqrt track / guardrail scan

```
rg "sqrt_candidates|shader/software deterministic sqrt|ExactDeterministic|ApproximateJitOnly|native sqrt|mag2" ...
```

**Result:** Active docs reference `sqrt_candidates.md`; native sqrt blocked from exact authority; exhaustive proof required; `mag2` remains approximate/diagnostic.

### M-JIT closure scan

```
rg "M-JIT track.*closed|PROD-0|ProductionKernelRegistryShell" ...
```

**Result:** M-JIT closed at PROD-0; sqrt track listed as separate follow-on.

### Deferred/prohibited guardrail scan

**Result:** No authorization of default SimSession wiring, scheduler/cache, semantic WGSL, production economy→mapping bridge, or `simthing-sim` awareness in sqrt track docs.

---

## Tests / commands

| Command | Result |
|---|---|
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

Exhaustive sqrt sweep **not** run — belongs to future implementation/test-battery slice.

---

## Final verdict

**PASS** — SQRT-DOC-0 R1 landed; shader/software deterministic sqrt is integrated into active guidance as a separate follow-on track, stale references to deleted JIT intermediate reports were removed from the design note, M-JIT remains closed at PROD-0, native sqrt remains ApproximateJitOnly until exhaustive bit-exact proof, no implementation/default wiring/scheduler/cache/production bridge was added, E-phase evidence was untouched, and V7.7 / Mapping ADR / FIELD_POLICY guardrails remain intact.
