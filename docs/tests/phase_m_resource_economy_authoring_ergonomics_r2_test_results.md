# Phase M Resource Economy Authoring Ergonomics R2 — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `c3927bb8f26add09709abb2f5986c45631f4b355` (post parking packet preflight + EML-GADGET-2A/B/C consolidation)  
**Final commit SHA:** `3c549a8218f2da63a887304e4fb828f788d3b467` (R2 commit; corrected during 2D preflight)  
**Verdict:** **PASS**

---

## Mandatory Preflight Doc Correction Performed

Before any code or other doc changes, the following exact correction was applied to the long Phase M boundary-resolution / EML-GADGET acceptance paragraph in `docs/workshop/mapping_current_guidance.md`:

- Removed/replaced the stale sentence beginning `**Next implementation step:** **EML-GADGET-2A** — snapshot/copy band fixture-only proof — then 2B ...` (and its orphaned tail after prefix replacement).
- Replaced with the exact required text:
  ```
  **Current EML-GADGET-2 status:** 2A snapshot/copy, 2A R1 sequence hygiene, 2B VelocityMonitor + Decay/EMA, and 2C BoundedFeedback have landed and are consolidated in docs/reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md for Opus/product review. The next code-bearing slice is Resource Economy Authoring Ergonomics R2 only if it remains spec/admission/preview oriented with no runtime economy→mapping coupling. Hysteresis, Acceleration, runtime gadget execution, chained scheduling, atlas/M-4A, and any production economy→mapping bridge remain separately gated.
  ```

This correction is recorded here (no standalone remediation report created). Post-edit verification: the first mandated rg scan (see below) is clean.

---

## Files Changed

- `docs/workshop/mapping_current_guidance.md` — mandatory preflight removal of the remaining stale `**Next implementation step:** **EML-GADGET-2A** ...` sentence (and tail) inside the long boundary-resolution acceptance paragraph; replaced with the exact required "Current EML-GADGET-2 status" text.
- `crates/simthing-spec/src/compile/resource_economy_admission.rs` — narrow R2 addition of `schedule_lines: Vec<String>` field to `ResourceEconomyPreviewReport` (designer ergonomics helper exposing intended transfers/recipes/thresholds as compact one-liners); population logic added in `build_preview_report` (purely derived from already-admitted authoring data).
- `crates/simthing-spec/tests/resource_economy_authoring_preview.rs` — updated the two main fixture tests (surplus + deficit) with assertions exercising the new `schedule_lines` R2 helper.
- `docs/accumulator_op_v2_production_plan.md` — minimal one-paragraph status note recording R2 landing (spec/admission/preview only).
- `docs/workshop/workshop_current_state.md` — minimal one-line update to the "Next action" block noting R2 landing with the no-coupling guardrail.
- `docs/tests/phase_m_resource_economy_authoring_ergonomics_r2_test_results.md` — this report.

**No other files touched.** Zero changes to runtime, GPU, simthing-sim, mapping pass-graph, first-slice, FIELD_POLICY, Resource Flow, or any production bridge.

---

## Narrow R2 Improvement Chosen (After Deep Inspection)

After reading `ResourceEconomySpec` + admission/preview code, the `resource_economy_authoring_preview` test, the daily economy + economy+FIELD_POLICY product fixture reports, and related RON fixtures:

- The R1/V1 surface was already strong (structured `TransferPreview`/`RecipePreview` lists, `simple_static_nets`, `warnings` as `ResourceEconomyDiagnostic`, order bands, bindings, game-mode integration).
- The smallest useful, purely spec/admission/preview, zero-runtime-coupling ergonomics win was adding a `schedule_lines: Vec<String>` helper field to `ResourceEconomyPreviewReport`.
- This directly fulfills the handoff allowance for "spec preview helpers that expose intended economy transfers clearly" and "improve designer-facing ResourceEconomySpec authoring ergonomics."
- Implementation: derived one-liners in the existing `build_preview_report` (e.g. `"bank_daily_income: +10.0 core::treasury/Amount @ order_band 0 (transfer)"`). No compiled/runtime paths, no new admission rules, no EML/gadget coupling, no mapping/FIELD_POLICY awareness.

This is the narrowest high-signal addition that helps designers authoring discrete banking fixtures (the exact use case of the Phase M Daily Economy Fixture V1 and product fixtures) without any forbidden changes.

**Stop conditions checked:** None triggered. No runtime coupling of any kind was required or added.

---

## Exact Scans Performed (as required)

**Scan 1 (stale next-step language — must be clean):**
```bash
rg "Next implementation step:\s*\*\*EML-GADGET-2A\*\*" docs/workshop/mapping_current_guidance.md
```
**Result:** CLEAN (no matches). The preflight successfully removed the last remaining instance of the stale directive language inside the long boundary-resolution acceptance paragraph.

**Scan 2 (guardrail posture — all matches must be "not authorized / not added" statements):**
```bash
rg "production economy→mapping bridge|CPU urgency|CPU-side AI planner|default SimSession mapping|semantic WGSL|atlas/M-4A" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_resource_economy_authoring_ergonomics_r2_test_results.md
```
**Result:** All matches are explicit guardrail / "did not add" statements:
- Multiple "No production economy→mapping bridge was introduced." / "no production economy→mapping bridge" (in workshop_current_state, eml_gadget_design_note, mapping_current_guidance, and this report).
- "Do not implement ... or any production economy→mapping bridge in this pass." (in the preflight-updated authorized-step paragraph).
- "No CPU-side AI planner", "No default SimSession mapping wiring", "No semantic WGSL", "No atlas/M-4A", "No atlas batching", etc., in the same active guidance files (all pre-existing posture language, now reinforced by R2).
- In this report and the updated production plan: explicit statements that R2 added nothing to runtime/mapping/GPU.

No matches claim any forbidden thing was implemented.

---

## Tests Run + Actual Results

All required commands executed (full --nocapture output captured in session; summaries here). Because R2 is a pure additive preview field in the existing authoring path, behavior outside spec/admission/preview is identical to prior V1 state.

```bash
cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture
# Result: 8/8 passed (including new R2 schedule_lines assertions on both surplus and deficit fixtures)
```

```bash
cargo test -p simthing-driver --test phase_m_economy_field_policy_product_fixture -- --nocapture
# Result: 6/6 passed (economy-derived weights still produce no FIELD_POLICY commitments on CPU; GPU path untouched)
```

```bash
cargo test -p simthing-driver --test phase_m_daily_economy_fixture -- --nocapture
# Result: 7/7 passed (RON admission, one-day surplus banking, multi-day determinism, deficit threshold emit, etc.)
```

```bash
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
# Result: 28/28 passed (first-slice hot path, no readback, persistence, etc.; economy fixtures orthogonal)
```

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
# Result: 11/11 passed
```

```bash
cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture
cargo test -p simthing-spec --test eml_gadget_tier2_temporal -- --nocapture
cargo test -p simthing-spec --test eml_gadget_tier2_bounded_feedback -- --nocapture
# Result: all green (14 + 10 + 11); R2 change is completely orthogonal to EML gadget surfaces
```

```bash
cargo check --workspace
# Result: Finished dev profile successfully (only pre-existing unrelated warnings)
```

**All targeted tests green.** No behavior changed outside the new preview ergonomics helper.

---

## Transient Logs

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
# (PowerShell equivalent executed)
```

**Result:** 11 historical `*_full.log` files present (mapping atlas sandbox, boundary cadence doctrine, daily economy fixture, economy+FIELD_POLICY product fixture, 2A snapshot + R1 hygiene, 2B velocity, first-slice map residency + summary validity, queue scale, resource economy authoring ergonomics R1, and this R2 run context). 

These are intentional historical evidence tied to their reports and fixtures. No `*.tmp`, `*scratch*`, or other obviously transient/unreferenced scratch logs at the root of `docs/tests`.

**Action:** No files deleted. All preserved per handoff instruction ("Preserve historical full logs if they are evidence").

---

## Posture & Constraint Affirmations (Binding)

- This slice implemented **only** the narrow `schedule_lines` designer preview helper in spec/admission/preview.
- **No runtime economy→mapping bridge was added** (or even touched).
- **No default mapping execution** or SimSession wiring added.
- **No GPU/WGSL/simthing-gpu/simthing-sim** behavior or semantics changed.
- **No CPU FIELD_POLICY planner / urgency computation / commitment emission** added or possible via this change.
- The preflight stale EML-GADGET-2A next-step sentence was removed from active guidance.
- Production doc and workshop current state updated (lightly).
- V7.7 / Mapping ADR / FIELD_POLICY GPU-resident default-off posture remains fully intact (MappingExecutionProfile::Disabled default, explicit opt-in only, CPU selects authored profiles in fixtures only, FIELD_POLICY stays GPU field→reduction→EvalEML→Threshold+EmitEvent path, Resource Flow default-off, no DailyResolutionBoundary or calendar semantics, etc.).
- All binding prohibitions from the handoff were respected (no Hysteresis/Acceleration, no runtime gadgets, no chained scheduling, no atlas/M-4A, no perception, no source_mask, no new EML opcodes, etc.).

**Explicit statement:** No runtime/code behavior changed outside spec/admission/preview. The only new surface is a pure derived `Vec<String>` in the existing authoring preview report for designer ergonomics.

---

## Final Verdict (required exact wording)

PASS — Phase M Resource Economy Authoring Ergonomics R2 landed as a narrow spec/admission/preview authoring improvement; the remaining stale EML-GADGET-2A next-step sentence was removed as preflight, active production guidance was updated, tests and cargo check are green, no runtime economy→mapping bridge or default mapping execution was added, no GPU/WGSL/simthing-sim behavior changed, and V7.7 / Mapping ADR / FIELD_POLICY GPU-resident default-off posture remains intact.

All 12 completion criteria satisfied. R2 is complete. The consolidated parking packet + this R2 slice keep the active authority surface truthful and the posture binding for future Opus/product direction.

Ready for any follow-on gated work (still under the "only if spec/admission/preview with no runtime coupling" rule).