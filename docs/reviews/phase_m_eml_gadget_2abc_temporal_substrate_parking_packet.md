# Phase M EML-GADGET-2A/B/C Temporal Substrate Parking Packet

**Date:** 2026-05-29  
**Status:** Submitted for Opus/product review (review/parking only)

---

## Executive Summary

This packet consolidates evidence for the explicit-column temporal gadget substrate implemented under the EML-GADGET-2 design gate (accepted 2026-05-29):

- **2A**: Snapshot/Copy Band Fixture Proof — `previous_col ← current_col` using `Identity` + `ConsumeMode::ResetTarget` at an authored earlier `OrderBand` on Layer-3 (parent/personality/strategic) columns only.
- **2A R1**: Sequence parity hygiene and report accuracy correction.
- **2B**: `VelocityMonitor` and `Decay`/`EMA` as Tier-2 temporal EML gadgets (spec/admission/compiler/oracle surfaces only, using existing `EvalEML` nodes, with stateful-sequence CPU oracle parity).
- **2C**: `BoundedFeedback` with strict clamp-bounded admission (explicit `min`/`max` required; rejects unbounded recurrence at admission).

This is a **review and parking packet only**. No new implementation is authorized by acceptance of this packet.

## Preflight Truth Patch Performed

Before assembling this packet, the following known active-documentation defects were verified and corrected as a mandatory preflight step (per handoff instructions):

1. `docs/tests/phase_m_docs_cleanup_archive_r5_test_results.md` — Final SHA placeholder corrected.
2. `docs/tests/phase_m_docs_cleanup_archive_r6_test_results.md` — Final SHA placeholder corrected.
3. `docs/workshop/mapping_current_guidance.md` — Stale "Next implementation step: **EML-GADGET-2A**" language replaced with correct reference to this consolidated parking packet.

**Important verification note**: At the moment of preflight execution, targeted scans showed that defects 1 and 2 were already absent from the live files (they had been resolved in prior remediation steps). The preflight edits were applied for absolute certainty and record cleanliness as required by the handoff. Defect 3 was also confirmed clean in active sections prior to the pointer update.

## Consolidated Evidence Summary

### 2A — Snapshot/Copy Band Fixture Proof
- Core proof: `OrderBand 0` (snapshot using `Identity` + `ResetTarget`) precedes `OrderBand 1` (update).
- Layer-3 only (explicit `SlotValue` on personality/parent slots).
- CPU-oracle sequence parity demonstrated with explicit traces.
- No new EML opcode, no WGSL, no runtime gadget execution, no hidden previous-value read.

### 2A R1 — Sequence Hygiene
- Removed confusing dual-trace comments and "re-read handoff" language from the original 2A test.
- Replaced with single coherent model + explicit `snapshot_then_update_oracle`.
- Produced clean, defensible multi-step traces showing visible lag.

### 2B — VelocityMonitor + Decay/EMA
- Added as explicit `EmlGadgetInstanceSpec` variants.
- Compile to existing `EvalEML` nodes only (SUB, MUL, DIV, LITERAL, SLOT_VALUE, ADD, CLAMP_BOUNDED, RETURN_TOP).
- Public stateful-sequence CPU oracles exported.
- Strict admission (0 ≤ decay < 1, finite positive dt when provided, distinct columns where required).
- All 2B tests green with stateful parity.

### 2C — BoundedFeedback
- Explicit `previous_col` + `input_col` + required `min`/`max` clamp in the authoring type.
- Node template uses existing `node_clamp_bounded` after linear combination.
- Strict admission contract enforced (finite decay < 1, finite gain, min < max).
- Rejects any unbounded recurrence form.
- Excellent stateful sequence parity (including clamp saturation edges).

## Posture Re-Affirmation

All binding posture from 2A–2C and R1–R7 handoffs remains intact:

- No new EML opcode.
- No new ConsumeMode.
- No WGSL or per-gadget GPU kernel.
- No runtime gadget-stack execution.
- No chained OrderBand runtime scheduling.
- No hidden previous-value read in EML.
- Temporal memory remains explicit-column state.
- Layer-3 scoped by default; dense per-cell separately gated.
- `simthing-gpu` remains generic.
- `simthing-sim` has no Gadget/Personality/Memory semantics.
- MappingExecutionProfile default remains Disabled.
- Resource Flow E-11 remains default-off.
- No production economy→mapping bridge.
- No default SimSession mapping wiring.
- No atlas/M-4A.
- No DailyResolutionBoundary or calendar semantics in `simthing-sim`.

## Active Guidance Updates

The following files were lightly updated to reference this parking packet as the current consolidated review artifact (minimal changes only; no doctrine or historical archive text was altered):

- `docs/workshop/eml_gadget_library_design_note.md`
- `docs/workshop/workshop_current_state.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`

## Forward Direction

Acceptance of this packet does **not** authorize implementation of:
- Hysteresis (2D, conditional)
- Acceleration
- Runtime gadget-stack execution
- Chained OrderBand runtime scheduling
- Dense per-cell temporal memory
- Resource Economy Authoring Ergonomics R2 with runtime coupling
- Atlas/M-4A
- Any production economy→mapping bridge

The next authorized step (if any) must come via a separate gated handoff after product/Opus direction on this parking packet.

---

**Test/results report:** [`../tests/phase_m_eml_gadget_2abc_parking_packet_test_results.md`](../tests/phase_m_eml_gadget_2abc_parking_packet_test_results.md) (exact scans, all mandated commands, preflight record, transient log check, final verdict).

**Prepared for Opus/product review.**