# M-4A Post-Merge Evaluation — Mapping / V7.7 / SEAD Consistency

**Date:** 2026-05-19  
**Evaluated HEAD:** `bf8c189` (PR #226 — M-4A algebraic tile-local atlas masking sandbox)  
**Evaluator:** Opus follow-up after Cursor M-4A handoff

---

## Verdict

**PASS — commits consistent with V7.7, Mapping ADR discipline, SEAD principles, and production goals.** No code remedial required. One doc-drift gap (ADR classification table) addressed via **proposed-amendment** subsection (not auto-ratification). **Recommended next step:** Phase **M-first-slice** runtime wiring (ADR Option B).

---

## Consistency matrix

| Principle | Expected posture | M-4A merge (`9520a3a` / `bf8c189`) | Result |
|---|---|---|---|
| V7.7 constitutional core | No semantic WGSL; simthing-sim map-free; default-off | Docs-only; sandbox WGSL preserved in workshop only; no lib/pass graph changes | **PASS** |
| Mapping ADR | Architecture approved; no production runtime; atlas provisional | M-4 remains parked; no atlas packer; no session wiring | **PASS** |
| ADR amendment discipline | No classification change without explicit evidence + sign-off | Design note amended as proposed; ADR table unchanged until ratification | **PASS** (discipline honored; gap closed in this eval via proposed-amendment subsection) |
| SEAD three-layer model | Dense local bounded; hierarchy for strategic; no CPU planner | Probe uses generic stencil only; no Layer-2/3 wiring; no sim semantics | **PASS** |
| GPU-resident / default-off | StructuredFieldStencilOp live opt-in; MappingExecutionProfile Disabled | Production op unchanged; tests confirm default-off | **PASS** |
| Sandbox revert discipline | Preserve workshop + docs/tests; remove transient runtime | Candidate `.rs`/`.wgsl` in workshop; revert verification green | **PASS** |
| Resource Flow defaults | Unchanged | No RF flag or default changes | **PASS** |
| E-11B / D-2a / Scatter-Gather | Not touched | No changes | **PASS** |

---

## Evidence review (M-4A)

| Claim | Backing | Assessment |
|---|---|---|
| G=0 mask matches protocol oracle | max_err ≤ 0.000031 all N/H/operators | **Credible** |
| Unmasked G=0 fails | max_err 458–500 at H=8 | **Demonstrated** |
| VRAM 1.0× vs 6.76× | Test 3 tables | **Strong** |
| Physical gutter fallback | Explicit in results + design note | **Correct** |
| Coordinate cost PARTIAL | Test 4; modulo/division in WGSL | **Fair — production should use tile-local dispatch** |
| Source protocol preserved | Test 6; column-wide zero banned | **Aligns with ADR + remedial evidence** |

---

## Gaps found and disposition

| Gap | Severity | Disposition |
|---|---|---|
| Mapping ADR optimization table still gutter-only | Low (intentional deferral) | Added **Proposed amendments** subsection + M-4A evidence citation |
| `workshop_current_state.md` stale HEAD | Trivial | Fixed to `bf8c189` |
| `design_v7_7.md` atlas line gutter-only | Low | Footnote to M-4A proposed path |
| Production plan M-3 "next task" stale | Low | Updated to Option B recommendation |
| No production code regression | — | Revert ladder + workspace test green at M-4A merge |

---

## Test artifacts — keep / delete

| File | Action | Reason |
|---|---|---|
| `mapping_atlas_algebraic_mask_sandbox_test_results.md` | **Keep** | Active evidence for M-4A + future ADR ratification |
| `mapping_atlas_algebraic_mask_sandbox_full.log` | **Keep** | Full sandbox log |
| `revert_mapping_atlas_algebraic_mask_sandbox_to_parked_state_test_results.md` | **Keep** | Revert verification record |
| `phase_m4_atlas_isolation_design_note_test_results.md` | **Keep** | M-4 design note landing record (pre-M-4A; still valid) |
| `mapping_optimization_*_test_results.md` | **Keep** | ADR-cited baseline evidence |

No test results deleted — all remain ADR-cited or M-4A active evidence.

---

## Recommended next step

**Phase M-first-slice** — first-slice runtime wiring per ADR §First slice (Option B). The named first slice uses **one grid, no atlas, no active mask**; M-1..M-3 natives are green. M-4 atlas implementation (Option A) should wait for explicit product decision + Opus ratification of M-4A algebraic-mask amendment.

Handoff: [`mapping_first_slice_runtime_handoff.md`](mapping_first_slice_runtime_handoff.md)

---

## Commands run (this evaluation)

| Command | Result |
|---|---|
| `git log -5 --oneline` | `bf8c189` M-4A merge at HEAD |
| `rg atlas_mask crates/` | No production matches |
| Doc cross-check (ADR, V7.7, guidance, production plan) | Drift items patched in this eval commit |
