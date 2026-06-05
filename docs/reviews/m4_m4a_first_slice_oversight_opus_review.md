# Opus Oversight Memo — M-4 / M-4A Ratification + First-Slice Runtime Readiness

**Date:** 2026-05-28
**Authority:** Opus 4.8, acting as the mapping/FIELD_POLICY design/oversight authority under
explicit human delegation ("you are the design authority… full authority to raise
guardrails up to the designer-facing studio layer… push and merge when you're done").
**Scope:** Decide what is ratified / amended / deferred / rejected before the next
mapping implementation step. **This memo authorizes no atlas implementation and no
production mapping runtime beyond what has already landed.**

**Supersedes the "pending Opus sign-off" status** on the M-4A amendment recorded in:
- `docs/adr/mapping_sparse_regioncell.md` (atlas row + "Proposed amendments" subsection)
- `docs/workshop/mapping_atlas_batching_isolation_design_note.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/design_v7_7.md` §2.4

---

## Executive verdict

| Decision | Verdict |
|---|---|
| **Oversight verdict** | **PASS WITH CONDITIONS** |
| **M-4A ratification** | **RATIFY WITH CHANGES** — `AlgebraicTileLocalMask (G=0)` is the **preferred isolation candidate** for homogeneous square atlas batches; `PhysicalGutter (G≥H)` is **downgraded to fallback**; `LocalBoundsMetadata` **remains deferred**. Atlas batching itself stays **Provisional and unimplemented**; ratification of the *isolation policy* is **not** authorization to *implement* atlas. |
| **First-slice runtime** | **ACCEPTED** as a stable base through M-first-slice-R3. No remedial PR required. The per-slot queue-write cost shape is a documented precondition for the *atlas/multi-field* step, **not** a blocker for the named first product slice. |
| **Required next handoff** | **Option 3 — product scenario fixture.** Build one tiny product-facing first-slice scenario on the landed runtime. **Do NOT begin M-4 atlas implementation** (Option 4) until the named multi-theater scenario, approved VRAM budget, and a gate-passing M-4 implementation PR all exist. Option 2 (R3 readiness) is already done (PR #232). |

PASS is conditioned on the stop conditions below remaining enforced. The conditions are
not new work; they are the existing guardrails, which I have verified are real in code.

---

## Evidence reviewed

**Architecture / contract docs:** `mapping_sparse_regioncell.md` (ADR, approved
2026-05-28), `design_v7_7.md`, `mapping_atlas_batching_isolation_design_note.md` (M-4
design note, parked), `mapping_current_guidance.md`, `accumulator_op_v2_production_plan.md`
§"Phase M".

**Evidence docs:** `mapping_atlas_algebraic_mask_sandbox_test_results.md` (M-4A),
`mapping_atlas_algebraic_mask_candidate_notes.md`, and the first-slice series
`phase_m_first_slice_runtime_test_results.md` (R0, 11/11),
`…_r1_no_readback_correctness_test_results.md` (R1, 20/20),
`…_r2_gpu_bridge_test_results.md` (R2, 24/24),
`…_r3_readiness_test_results.md` (R3, 28/28).

**Code surfaces verified directly (not taken on the reports' word):**
- `crates/simthing-spec/src/compile/region_field_admission.rs` — `request_atlas_batching`
  is **rejected at admission** (`compile_region_field_preview`); grid caps (10 / 32, ≤1024
  cells), horizon caps (8 default / 16 extended only with `allow_extended_horizon` +
  `SourceCappedNormalized` + `source_cap`), v1-only `CallerManagedOneShotSeedThenZero`,
  and formula-class whitelist (`field_urgency` admitted) all enforced before any GPU touch.
- `crates/simthing-spec/src/compile/region_field_budget.rs` — VRAM estimator already plumbs
  `SingleGridNoAtlas`/`AlgebraicTileLocalMask` (1.0×) and `PhysicalGutter` ((N+2G)²/N² =
  6.76× at N=10,G=8) as **estimate-only** policies; over-budget rejects at the spec layer.
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — hot path
  (`run_reduction_and_eml(readback_report=false)`) bridges stencil→accumulator via GPU
  `zero_values_buffer` + `copy_values_prefix_from_buffer` + queue `write_slot_col_values`,
  ticks bands 0/1, returns `(None,None)`, and keeps `reduction_stencil_readbacks = 0`.
  Seed protocol is **seed-only clear** (`zero_cell_values` on the seeded cells), never
  column-wide `source_col` zeroing.

**Independent verification run (this review, real GPU, this machine):**
`cargo test -p simthing-driver --test phase_m_first_slice_runtime` →
**`28 passed; 0 failed`**, including `test_r2_a_hot_path_no_hidden_reduction_readback`,
`test_r1_d_seed_only_clear_gpu_resident`, `test_8_default_off_enforcement`, and
`test_r3_a_readiness_report_hot_path_shape`. The GPU-resident no-readback invariant is
not just claimed in the report — it holds in the running binary.

**Reconciliation note.** The handoff was authored at the R2 point and recommended
"Option 2 (R3) first, then Option 3." By the time this review ran, **R3 had already
merged (PR #232, HEAD `6f7701f`)**. The docs that reference "M-first-slice-R3 landed" are
therefore correct, not premature. The only working-tree deltas at review time were
nondeterministic workshop bench-report `.txt` timing churn (unrelated to mapping); they
are intentionally left out of this change.

---

## M-4A decision (detailed answers to the M-4 design note's six questions)

1. **Is `AlgebraicTileLocalMask` admissible as generic, semantic-free WGSL? — YES.**
   The kernel is `contribution = neighbor_value * valid_tile_local_neighbor`: pure
   boundary algebra over flat buffers, dimensions, columns, and kernel weights. It carries
   no map / faction / AI / `simthing-sim` semantics, which is exactly the V7.6/V7.7 bar for
   admissible WGSL. M-4A evidence: full-tile **protocol-oracle parity** (CPU max error
   ≤ 0.000031; GPU masked error 0.0) across N∈{5,10,20,32}, H∈{1,4,8}, Normalized and
   SourceCappedNormalized; unmasked G=0 diverges (458–500) exactly as it must.

2. **Preferred M-4 isolation policy for homogeneous square batches? — YES (as the
   preferred *candidate*).** It resolves the gutter VRAM tax (1.0× vs 6.76× at N=10,H=8)
   without correctness loss, and is competitive-to-faster at scale (1.69 ms vs 4.11 ms at
   64 tiles). "Preferred" governs *which design an implementation PR should pursue first*;
   it does not mark atlas as Adopted or implemented.

3. **Does `PhysicalGutter` remain only as fallback? — YES.** Downgraded to the
   conservative fallback, mandatory whenever algebraic masking is not configured or not
   admitted, or for any layout that is not homogeneous-square.

4. **Does mixed-size `LocalBoundsMetadata` remain deferred? — YES.** Unchanged. It needs
   its own implementation ADR/PR; nothing here promotes it.

5. **Modulo/division coordinate derivation vs tile-local dispatch for the first
   implementation? — EITHER is permitted, gated on the acceptance test, not on the
   coordinatization method.** Coordinate derivation is an implementation detail, not a
   constitutional matter, provided global buffer-bounds checks precede every load and
   full-tile protocol-oracle parity passes. The PR **must** report both isolation policy
   and the coordinatization method chosen, and **should** measure modulo/division against
   tile-local dispatch (M-4A Test 4 found division plausible at ~1.6–5.8 ms but recommended
   tile-local dispatch where cheap). Start simple if it passes the gate; record the number.

6. **Is the acceptance gate sufficient? — YES, ratified as binding** (see "Ratified M-4
   acceptance gate" below). The design note §11 checklist is sound; I am promoting it from
   "indicative" to **required**, and binding the normalization, source-clear, VRAM, and
   no-semantic-WGSL conditions explicitly.

**Constitutional risk assessment.** Low, conditional on one discipline: **mask fever**.
The algebraic-mask pattern (`flat buffers + RON-authored relationships → generic
masks/gates → semantic-free GPU transforms → hierarchy reduction → EML interpretation`) is
genuinely powerful and correctly described in the design note §4. The risk is that it
invites a bespoke mask shader per gameplay concept. **Ratified guardrail:** a new
algebraic mask is admissible only when it is *generic, bounded, opt-in, designer/RON-
governed, semantic-free, and parity-tested against a protocol-faithful oracle.* M-4A
clears that bar for tile boundaries. It does **not** pre-authorize masks for
fog/perception, supply, ownership, source identity, or active frontier — each remains its
own gated decision (design note §4.5). There is no constitutional erosion in the masking
*representation* itself; the erosion risk is purely in over-applying it, and the guardrail
above is the brake.

---

## First-slice runtime decision

1. **Does R2/R3 satisfy the Mapping ADR's first-slice intent? — YES.** The ADR's named
   first slice is one bounded grid (≤32×32), `source_capped_normalized` at H≤8, one-shot
   seed-then-zero, `EveryTick`, dirty skip, Sum reduction into one parent threat column,
   one `field_urgency` `EvalEML` on the parent, **no atlas / no active mask / no
   perception**. `FirstSliceMappingSession` is exactly that shape and nothing more.

2. **FIELD_POLICY / AI-as-SimThing discipline? — YES.** The decision path is
   stencil → `SlotRange` Sum → `EvalEML`, producing a parent urgency *column*. There is no
   CPU map planner. The intended consumer — commitments as `Threshold` + `EmitEvent`
   crossings over that column — is not yet built, which is correct for a first slice.

3. **Is the GPU-resident path adequate
   (`StructuredFieldStencilOp → AccumulatorOpSession → SlotRange Sum → EvalEML`)? — YES.**
   The hidden GPU→CPU→GPU staging that existed before R2 is gone. Verified in code and in
   the passing `test_r2_a` (`reduction_stencil_readbacks=0`). Debug readback remains
   explicit and gated by `set_debug_readback_allowed`.

4. **Are the generic GPU bridge helpers acceptable substrate additions? — YES.**
   `zero_values_buffer`, `copy_values_prefix_from_buffer`, `write_slot_col_values`, and
   `values_buffer()` on `AccumulatorOpSession` are generic flat-buffer operations with no
   mapping semantics. They belong in the substrate and are reusable beyond mapping.

5. **Remaining observability before product-scenario work? — Adequate; one named caveat.**
   R3's `FirstSliceReadinessReport` exposes dispatch counts, `gpu_bridge_bytes_copied`,
   `gpu_bridge_slot_col_writes`, budget estimate, execution flags, and an
   *informational-only* `hot_path_wall_ms` (correctly **not** a CI stability gate). The one
   open item is the **scale cost shape**: the bridge uses per-slot queue writes for child
   resource values and parent weights (102 writes at 10×10). This is fine for the first
   slice and is now *reported*, which is the right posture. It must be replaced with a
   preinitialized resource column / generic fill helper / fill kernel **before** multi-field
   or atlas scale — that is a precondition on the *atlas* step, not on the product fixture.

---

## Risks / caveats

- **Scale cost shape (tracked, not blocking).** Per-slot queue writes are O(cells) on the
  bridge. Acceptable and reported at 10×10; must be redesigned (measured) before atlas /
  multi-field. Carry this caveat forward verbatim.
- **`hot_path_wall_ms` is informational only.** Do not let it become a flaky CI gate.
- **Mask fever (constitutional).** See M-4A risk assessment. The brake is the genericity
  guardrail, now ratified.
- **Ratification ≠ implementation.** The single most likely misreading of this memo is to
  treat "M-4A ratified" as "build the atlas packer next." It is not. The admission layer
  must keep rejecting `request_atlas_batching` until a gate-passing M-4 implementation PR
  lands. **Do not relax that admission check as part of any product-fixture work.**

---

## Required next step

**Option 3 — product scenario fixture (next handoff; Composer-class implementation PR).**
Build one tiny product-facing first-slice scenario that drives the landed
`FirstSliceMappingSession` (single grid, `source_capped_normalized`, H≤8, EveryTick, Sum →
`field_urgency`), asserting the GPU-resident no-readback invariant and the readiness report
shape end-to-end from a RON spec. **No atlas, no active mask, no perception, no
`source_mask`, no new WGSL, no default-on execution, no `simthing-sim` awareness.** This is
the cheapest path that turns the runtime into evidence about *product* behavior without
touching any provisional/deferred feature.

**Not now:** Option 4 (atlas). It waits on (a) this ratified M-4A amendment [done in docs by
this memo], (b) a *named* multi-theater scenario that actually needs batching, (c) an
*approved* VRAM budget for that scenario, and (d) an M-4 implementation PR that satisfies
the ratified acceptance gate below.

### Ratified M-4 acceptance gate (binding for any future atlas implementation PR)

A future M-4 atlas implementation PR is admissible **only** if it satisfies **all** of:

1. **Full-tile protocol-oracle parity** — bit-exact (or an explicitly documented,
   narrower-than-t44 approved tolerance) on all useful cells per tile after horizon
   completion, against a CPU oracle that replays the *same* per-tile seed-clear + gutter/
   mask + boundary protocol the GPU uses. **t44/corridor agreement alone is rejected.**
2. **Fixed-denominator zero-boundary normalization** (renormalization-by-valid-neighbors is
   deferred — M-4A Test 5 showed edge amplification, max err 321 vs oracle).
3. **Safe atlas-global buffer bounds** — every neighbor load is bounds-checked against the
   atlas buffer before the algebraic mask is applied; no out-of-buffer reads.
4. **Caller-managed seed-only clearing per tile** after the setup hop —
   **column-wide `source_col` zeroing is banned** (corrupts propagated state; err 256 vs 0).
5. **VRAM accounting reported at/before pack time** — `vram_multiplier`,
   `vram_overhead_percent`, `estimated_atlas_bytes`, and the chosen isolation policy. A
   packer that cannot report VRAM accounting must refuse to pack.
6. **Refuse-to-pack** when `gutter < effective_horizon` and no algebraic mask / local-bounds
   isolation is configured.
7. **Homogeneous square tiles per batch** (v1) — mixed `grid_size` in one batch rejected.
8. **No semantic WGSL; no `simthing-sim` awareness; no default pass-graph wiring** — packer
   lives in the driver/substrate behind the mapping profile; defaults unchanged.
9. **No `ActiveOnlyExperimentalNoHalo`; no behavioral source policy / `source_mask` before
   M-5.**

Items 1–9 are the design note §11 checklist promoted from indicative to required, plus the
normalization and bounds conditions made explicit.

---

## Stop conditions (escalate to human; do not land)

Reject / do not land any path that:
- requires semantic / map / faction / AI WGSL, or `simthing-sim` map awareness;
- turns mapping execution on by default, or wires the production pass graph to mapping;
- implements the atlas packer without a PR satisfying the ratified acceptance gate above;
- relaxes the admission rejection of `request_atlas_batching` outside such a PR;
- ships an active mask without halo, or any `source_mask` / behavioral source policy before
  M-5;
- uses column-wide `source_col` zeroing;
- accepts an atlas on t44/corridor agreement alone;
- ships atlas without VRAM-multiplier reporting;
- introduces a CPU-side AI map planner.

These restate the ADR, design-note §12, and the handoff's non-negotiables. Nothing in this
memo loosens any of them.

---

## Doc amendments made alongside this memo

- `docs/adr/mapping_sparse_regioncell.md` — atlas optimization-doctrine row and "Proposed
  amendments" subsection updated from "pending Opus sign-off" to **ratified (Opus,
  2026-05-28): algebraic tile-local mask G=0 is the preferred isolation candidate for
  homogeneous square batches; physical gutter is fallback; atlas remains Provisional and
  unimplemented**. First-slice update note advanced to R3 + accepted-as-stable-base.
- `docs/workshop/mapping_atlas_batching_isolation_design_note.md` — status header and §3/§4
  updated to reflect ratification; §11 checklist marked **binding acceptance gate**; the
  "Opus still needs to decide" list in §4.7 replaced with the recorded decisions.
- `docs/design_v7_7.md` §2.4 — optimization-doctrine wording updated from "M-4A evidence
  proposes… pending Opus ADR amendment" to the ratified classification.
- `docs/workshop/mapping_current_guidance.md` — M-4A/decision-gate sections updated to
  ratified status and Option 3 as the named next step.
- `docs/invariants.md` — atlas Mapping row updated to name algebraic tile-local mask as the
  preferred isolation candidate (physical gutter fallback), parity + VRAM-reporting
  conditions unchanged.
- `docs/accumulator_op_v2_production_plan.md` §"Phase M" — M-4 row and decision-gate text
  updated to record the ratification and the Option 3 next step.

All amendments are **decision/classification only**. No code is changed; the admission
layer continues to reject atlas batching; `MappingExecutionProfile::default()` remains
`Disabled`; `simthing-sim` remains map-free.
