# Cursor Handoff: Phase M EML-GADGET-2A/B/C Parking Packet — Temporal Gadget Substrate Review

## Goal

After R4 (Active Authority Final-SHA + EML Status Truth Pass) has landed cleanly on master, produce a consolidated parking/acceptance packet for Opus/product review of the EML-GADGET-2 temporal substrate:

- 2A Snapshot/Copy Band Fixture Proof (explicit-column previous ← current via Identity + ResetTarget at authored earlier OrderBand, Layer-3 only).
- 2A R1 Sequence parity cleanup + report accuracy.
- 2B VelocityMonitor + Decay/EMA (spec/admission/compiler/oracle only, existing EvalEML nodes, stateful-sequence CPU oracle parity).
- 2C BoundedFeedback (strict clamp-bounded recurrent accumulator with explicit min/max required; rejects unbounded recurrence at admission).

This packet is **review and parking only**. It must not implement:
- Hysteresis
- Acceleration
- Runtime gadget-stack execution
- Chained OrderBand runtime scheduling
- Dense per-cell temporal memory
- New EML opcodes
- WGSL / per-gadget GPU kernels
- Any changes to simthing-gpu EvalEML behavior
- Any simthing-sim Gadget/Personality/Memory semantics
- Production economy→mapping runtime bridge
- Default SimSession mapping wiring
- Atlas/M-4A
- Any default-on changes (MappingExecutionProfile remains Disabled; Resource Flow E-11 remains default-off)

## Binding posture (must be re-affirmed in the packet)

Every item from the long posture lists in 2A–2C and R1–R4 handoffs, including but not limited to:
- Temporal memory is explicit-column state (no hidden previous-value read).
- Layer-3 scope by default; dense per-cell separately gated.
- Snapshot/copy bands must be authored/admitted.
- Tier-2 oracle parity is stateful sequence parity.
- BoundedFeedback admission contract is binding (0 ≤ decay < 1 default; explicit clamp required when output can feed hard threshold; no positive unbounded recurrence).
- Preview ≠ runtime; PerGadgetOnly composition only.
- Every gadget requires CPU oracle + parity test.
- No driver/gpu/sim consumes CompiledEmlGadgetStack or flatten preview as execution.
- V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact (SEAD remains field → reduction → field_urgency EvalEML → Threshold + EmitEvent; no CPU planner, no production economy→mapping bridge, etc.).

## Scope of this packet

1. Consolidate evidence from the four landed slices (2A, 2A R1, 2B, 2C) into one coherent review/acceptance packet.
2. Demonstrate that the explicit-column temporal substrate satisfies the 2026-05-29 EML-GADGET-2 design gate conditions.
3. Confirm all posture and stop conditions from prior handoffs remain intact.
4. Park the consolidated substrate for Opus/product review and direction on next authorized steps.
5. Do **not** advance any runtime execution, scheduling, or new primitives.

## Deliverables

- One consolidated review/acceptance packet (e.g. `docs/reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md`).
- Light pointer updates in active guidance files (`eml_gadget_library_design_note.md`, `workshop_current_state.md`, `mapping_current_guidance.md`, `accumulator_op_v2_production_plan.md`) to reference the new parking packet as the current review artifact.
- R5 (or equivalent) test report in `docs/tests/` recording the review process, any minimal doc tweaks, exact scans/commands run, and the "ready for review" statement.

## Required verification (minimum, record in the packet report)

Run the same targeted regression set used in R1–R4 plus the two key stale scans:

- All `eml_gadget_tier2_*` tests
- `phase_m_eml_gadget_2a_snapshot_copy`
- `eml_gadget_tier1`
- `resource_economy_authoring_preview`
- Heavy driver fixtures
- `region_field_spec_admission`
- `accumulator_op_session_gpu_bridge`
- `cargo check --workspace`

Stale scans (must be clean in active docs):

```bash
rg "remain unimplemented" docs/workshop docs/accumulator_op_v2_production_plan.md docs/workshop/eml_gadget_library_design_note.md docs/workshop/mapping_current_guidance.md
rg "Final commit SHA: `\.\.\.`|... \(full list as specified|\(proposed\)" docs/tests docs/workshop docs/reviews
```

## Stop conditions (strict)

Do not proceed or claim success if:
- Any targeted test or cargo check fails.
- Active stale scans are not clean in the highest-authority production guidance files.
- The packet would require or imply runtime gadget execution, chained scheduling, new opcodes, WGSL, simthing-gpu changes, simthing-sim semantics, production bridges, default wiring, or atlas/M-4A.
- Evidence cannot be presented without contradicting V7.7 / Mapping ADR / SEAD GPU-resident default-off posture.

## Completion criteria (for the parking packet to be ready for Opus review)

1. Consolidated evidence from 2A + R1 + 2B + 2C is coherent and points back to the original 2026-05-29 design gate memo.
2. All binding posture items from prior handoffs are explicitly re-affirmed or shown to be intact.
3. Highest-authority production docs no longer contain false "remain unimplemented" statements for landed 2B/2C work.
4. Scans are clean in active docs.
5. All required tests + cargo check green.
6. Clear statement that this is a review/parking packet only — no new implementation is authorized by acceptance of this packet.
7. Explicit forward pointer to possible next gated steps (Hysteresis conditional, Resource Economy Authoring Ergonomics R2 with no runtime coupling, etc.).

## Final expected verdict wording (adapt once review completes)

PASS — Phase M EML-GADGET-2A/B/C temporal substrate parking packet landed for Opus/product review; explicit-column snapshot/copy, VelocityMonitor, Decay/EMA, and BoundedFeedback (strict clamp) are now consolidated under one review artifact with all posture and stop conditions from prior handoffs re-affirmed, highest-authority production guidance is consistent with landed state, no runtime/code changes occurred, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture remains binding. Ready for product direction on next authorized slice.

---

**After this packet is accepted (or parked with clear conditions), any future implementation handoff must be separately authorized and must not assume runtime execution or scheduling is now allowed without an explicit new gate.**