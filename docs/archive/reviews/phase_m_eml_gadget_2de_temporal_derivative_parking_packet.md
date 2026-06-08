# Phase M EML-GADGET-2D/2E Temporal-Derivative Parking Packet

**Date:** 2026-05-29  
**Status:** Submitted for Opus/product review (review/parking only)

---

## Executive Summary

This packet consolidates evidence for the Tier-2 temporal-derivative gadgets implemented after the 2A/B/C substrate parking packet:

- **2D**: Explicit-column `Hysteresis` — spec/admission/oracle landing, with high-activates state machine semantics.
- **2D R1**: Exact CMP/SELECT compiler parity correction (initial 2D stub superseded).
- **2E**: Explicit velocity-column `Acceleration` — `(current_velocity_col - previous_velocity_col) [/ dt]` over existing EvalEML arithmetic only.

The completed EML-GADGET-2 ladder now includes: 2A snapshot/copy fixture proof, 2A R1 sequence hygiene, 2B VelocityMonitor + Decay/EMA, 2C BoundedFeedback, 2D Hysteresis (+ 2D R1), 2E explicit velocity-column Acceleration, and Resource Economy Authoring Ergonomics R2 (spec/admission/preview-only).

This is a **review and parking packet only**. No new implementation is authorized by acceptance of this packet.

## Preflight Truth Patch Performed

Before assembling this packet, the following mandatory preflight was applied (recorded in the test/results report; no standalone remediation report created):

1. **`docs/tests/phase_m_eml_gadget_2e_acceleration_test_results.md`** — short final SHA `27718a4` expanded to full merge SHA `9d060867e9edc37c1f71d6031fc12a0b5412598c` (2E merge commit).

## Consolidated Evidence Summary

### 2D — Hysteresis

**Initial 2D landing:** Explicit-column `Hysteresis` spec/admission/oracle in `simthing-spec` only.

| Surface | Detail |
|---|---|
| Spec | `Hysteresis { input_col, previous_col, output_col?, on_threshold, off_threshold, off_value, on_value }` |
| Semantics | High-activates state machine: off→on when `previous == off_value && input >= on_threshold`; on→off when `previous == on_value && input <= off_threshold`; deadband hold otherwise |
| Admission | Finite thresholds with `on_threshold > off_threshold`; finite output values; distinct input/previous columns |

**2D R1 correction:** Exact CMP/SELECT compiler parity.

- `compile_hysteresis_nodes` emits a 20-node postfix tree using existing opcodes only:
  - `CMP_GE`, `CMP_LE`, `CMP_EQ`, `SELECT`
  - Arithmetic/slot/literal/return primitives as needed (`SLOT_VALUE`, `LITERAL`, `MUL`, `RETURN_TOP`)
- Postfix tree implements:
  ```text
  SELECT(on_to_off, off_value, SELECT(off_to_on, on_value, previous))
  off_to_on = (previous == off_value) && (input >= on_threshold)
  on_to_off = (previous == on_value) && (input <= off_threshold)
  ```
- `eval_eml_postfix` extended for spec-layer parity (matches `simthing-gpu` cpu_oracle CMP/SELECT semantics).

**Compiled-node parity (16/16 tests):**

- off → on transition
- on → off transition
- deadband hold
- threshold equality
- non-default output constants
- stateful sequences

**Evidence:** [`../tests/phase_m_eml_gadget_2d_hysteresis_test_results.md`](../tests/phase_m_eml_gadget_2d_hysteresis_test_results.md), [`../tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md`](../tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md)

### 2E — Acceleration

**Explicit velocity-column acceleration only.** Not position-history acceleration.

| Surface | Detail |
|---|---|
| Spec | `Acceleration { current_velocity_col, previous_velocity_col, dt?, output_col? }` |
| Formula | `acceleration = (current_velocity_col - previous_velocity_col) / dt` with `dt = 1.0` when omitted |
| Compiler | Delegates to `compile_velocity_monitor_nodes` (SUB + optional DIV + RETURN_TOP) |
| Oracle | `oracle_acceleration(current, previous, dt)` |

**Admission guarantees (11/11 tests):**

- Valid current velocity column
- Valid previous velocity column
- Current/previous velocity columns distinct
- Finite positive `dt` when present
- Rejects non-finite, zero, or negative `dt`

**Compiled-node parity:**

- `dt` omitted → default 1.0
- `dt` provided → division applied
- output_col preserved
- emits only existing arithmetic primitives (`SLOT_VALUE`, `SUB`, optional `DIV`, `RETURN_TOP`)

**Explicitly not implemented:**

- Position-history acceleration requiring `previous_previous_col`
- Dense per-cell temporal memory
- Hidden previous-value read
- Automatic snapshot/copy scheduling

**Evidence:** [`../tests/phase_m_eml_gadget_2e_acceleration_test_results.md`](../tests/phase_m_eml_gadget_2e_acceleration_test_results.md)

## Cross-Cutting Posture

All binding posture from prior EML-GADGET handoffs remains intact:

- `simthing-spec` authoring/admission/compiler/oracle only.
- No runtime gadget-stack execution.
- No chained OrderBand runtime scheduling.
- No automatic snapshot/copy scheduling.
- No hidden previous-value read.
- No new EML opcode.
- No WGSL/GPU kernel.
- No `simthing-gpu` behavior change.
- No `simthing-sim` Gadget/Personality/Memory semantics.
- No production economy→mapping bridge.
- No default SimSession mapping wiring.
- No atlas/M-4A.
- Dense per-cell temporal memory remains separately gated.
- Position-history acceleration remains separately gated.
- MappingExecutionProfile default remains Disabled.
- Resource Flow E-11 remains default-off.
- FIELD_POLICY remains GPU-resident: field propagation → parent reduction → `field_urgency` EvalEML → Threshold + EmitEvent.

## Runtime Execution Gate Assessment

### 1. What would be required to move from spec-layer gadget compilation to runtime execution?

A production runtime gadget execution path would require, at minimum:

- **Gadget-stack execution model:** A driver/session protocol that consumes `CompiledEmlGadget` output and schedules gadget evaluation at authored `OrderBand` boundaries — not merely compiling postfix trees for preview/oracle parity.
- **Output-column wiring protocol:** Explicit rules for how each gadget's `output_col` feeds the next gadget's `input_col` in a multi-gadget stack (currently `PerGadgetOnly` with `chained_runtime_deferred` diagnostic).
- **Automatic snapshot/copy scheduling protocol:** For temporal gadgets, a runtime mechanism to execute authored `Identity`+`ResetTarget` snapshot bands before update bands — currently proven only as a fixture/oracle trace, not as automatic session scheduling.
- **Chained OrderBand scheduling protocol:** True multi-band runtime ordering that wires gadget outputs across bands without manual column authorship for every intermediate state.
- **Separate ADR/product gate:** Acceptance of the above protocols, VRAM/column budget policy, and failure modes before any production execution path is added.

### 2. Which parts are already safe because they compile to existing EvalEML nodes?

Individual gadgets (Tier-1 and Tier-2) compile to postfix trees over the **existing** `EvalEML` opcode set (`SLOT_VALUE`, `SUB`, `MUL`, `DIV`, `ADD`, `CLAMP_BOUNDED`, `CMP_*`, `SELECT`, `RETURN_TOP`, etc.). The generic `simthing-gpu` EvalEML interpreter already executes these opcodes. Therefore:

- Any single gadget's compiled tree could theoretically be inserted into an existing EvalEML band **if** column values are already populated by prior authored bands.
- CPU-oracle parity tests prove compiled-node semantics match the gadget oracle for each landed gadget in isolation.
- Snapshot/copy substrate (2A) proves explicit-column temporal lag is achievable with existing `Identity`+`ResetTarget` at an earlier band — but only when authored and scheduled explicitly.

### 3. Which parts remain unsafe or under-specified?

- **Multi-gadget stacks:** `PerGadgetOnly` composition defers chained runtime; intermediate column wiring is not proven for arbitrary stacks.
- **Automatic temporal scheduling:** No runtime protocol assigns snapshot bands automatically; authors must explicitly schedule copy-before-update.
- **Gadget-stack → session integration:** No driver/gpu/sim code consumes `CompiledEmlGadgetStack` for production execution.
- **Dense per-cell temporal memory:** Layer-3 default scope only; dense per-cell history requires separate VRAM/product gate.
- **Position-history acceleration:** Requires `previous_previous_col` or deeper temporal state — not landed, separately gated.
- **Output ownership / column collision:** No runtime admission layer prevents gadget output columns from colliding across concurrent gadgets in a session.

### 4. Should runtime gadget execution remain deferred?

**Yes. Runtime gadget execution remains deferred.**

Although individual gadgets compile to existing EvalEML node templates, the repo has not yet accepted a runtime gadget-stack execution model, output-column wiring protocol, automatic snapshot/copy scheduling protocol, or chained OrderBand scheduling protocol. These require a separate ADR/product gate before any production execution path can be added.

**This packet does not authorize runtime execution, chained scheduling, or any of the above protocols.**

## Forward Direction

Acceptance of this packet does **not** authorize:

- Runtime gadget-stack execution
- Chained OrderBand runtime scheduling
- Automatic snapshot/copy scheduling
- Dense per-cell temporal memory
- Position-history acceleration
- Atlas/M-4A
- Any production economy→mapping bridge

The next authorized step (if any) must come via a separate gated handoff after product/Opus direction on this parking packet and the runtime execution gate assessment.

**Related consolidated artifacts:**

- 2A/B/C substrate: [`phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md`](phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md)
- Tier-2 design gate: [`phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](phase_m_eml_gadget_tier2_design_acceptance_opus_review.md)

---

**Test/results report:** [`../tests/phase_m_eml_gadget_2de_parking_packet_test_results.md`](../tests/phase_m_eml_gadget_2de_parking_packet_test_results.md)

**Prepared for Opus/product review.**
