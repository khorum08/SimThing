# Phase M M-4A Atlas Readiness Gate — Product Need, Minimum Surface, and GPU Safety Decision

**Date:** 2026-05-29  
**Base HEAD:** `9bbcb39612be803b8af04af2ee7a9f26ff203060`  
**Final commit SHA:** `753fbc8`  
**Lane classification:** Tier-2 readiness gate (V7.7 §5)  
**Decision:** **DEFER M-4A**  
**Verdict:** **PASS — readiness gate completed; M-4A remains deferred**

---

## Files Inspected

| Surface | Finding |
|---|---|
| `docs/invariants.md` | Atlas provisional; no semantic WGSL; strict-sink rule enforced (M-5D) |
| `docs/design_v7_7.md` | Mapping ADR provisional atlas; Phase M natives gated |
| `docs/workshop/phase_m_gating_and_doc_policy.md` | Tier-2 gate for M-4A; no implementation in readiness pass |
| `docs/workshop/mapping_current_guidance.md` | Atlas/M-4A deferred; M-5 landed; M-4 isolation policy ratified, implementation blocked |
| `docs/workshop/m5_gradient_extraction_design_note.md` | M-5A/B/C/D complete; no atlas dependency |
| `docs/workshop/mapping_atlas_batching_isolation_design_note.md` | §11 binding gate; algebraic tile-local mask preferred; implementation blocked until named scenario + VRAM budget |
| `docs/accumulator_op_v2_production_plan.md` | M-4 packer explicitly not next |
| `crates/simthing-spec/src/spec/region_field.rs` | `request_atlas_batching: bool` — rejected at admission |
| `crates/simthing-spec/src/compile/region_field_admission.rs` | `CompiledRegionFieldMaskMode::All` only; atlas flag rejected |
| `crates/simthing-gpu/src/structured_field_stencil.rs` | Generic `active_mask` buffer + `StructuredFieldStencilMaskMode` |
| `crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl` | Generic `use_active_mask` + `active_mask` binding; no atlas/tile semantics |
| M-5A/B/C/D/R1 tests and reports | Single-grid substrate green; no multi-tile runtime |

---

## Product Need Analysis

### What can atlas/M-4A do that M-5-gradient cannot?

Atlas/M-4A addresses **multi-tile batching**: packing multiple independent RegionField grids into one atlas dispatch for **VRAM efficiency and dispatch amortization**. It does not add field-calculus capabilities.

M-5-gradient provides single-grid **Gradient X/Y**, **SlotRange Sum**, **L3 EMA/WeightedAccumulator**, **strict-sink admission**, and product-facing need/routing fixtures on **one grid per field**. These are orthogonal concerns.

### Immediate need classification

| Need | Belongs to | M-4A required now? |
|---|---|---|
| Masking inactive cells on one grid | Generic `active_mask` (existing WGSL hook; RegionField admission hardcodes `All`) | No — separate sparse-execution gate, not atlas |
| Multi-theater / multi-region batching | Atlas/M-4 packer | **Potential future** — no named product scenario today |
| Source identity / behavioral seed policy | Separate `M-5` source-mask track | No — not M-4A |
| L1 cross-field coupling | Separately gated / prohibited | No |
| Dense per-cell temporal memory | EML-GADGET-2 / separately gated | No |
| Authoring ergonomics for gradient composition | M-5B/C fixtures + `compile_region_field_frame_preview` | No — already sufficient |

### Is the need actually source-mask (M-5) rather than atlas (M-4A)?

For seed clearing, source identity, and behavioral source policy — **yes, that is the separate M-5 track**, not M-4A. Current posture: `CallerManagedOneShotSeedThenZero`; no source_mask buffer in production.

### Is the need only authoring ergonomics?

For gradient/L3 composition patterns demonstrated by M-5B/C — **yes, existing substrate + fixtures are sufficient**. No atlas required.

### Concrete product scenario?

**None named.** Production plan and mapping guidance require a **named multi-theater scenario**, an **approved VRAM budget**, and a **§11-gate-passing M-4 PR** before atlas implementation. First-slice posture remains **single 10×10 grid, no atlas**.

---

## Existing-Substrate Sufficiency Analysis

M-5-gradient substrate satisfies current authorized product paths:

- Single-target Gradient X/Y with CPU/GPU parity (M-5A)
- Multi-field L3 composition fixtures with integrated CPU-oracle evidence (M-5B+R1)
- Product-facing need/routing signal fixture (M-5C)
- Frame-level gradient strict-sink admission (M-5D + R1 grouped helper)

`request_atlas_batching: true` is **rejected at admission**. No production multi-field atlas runtime exists. First-slice runtime operates on one region field per scenario.

**Conclusion:** M-5-gradient does not block or replace atlas; atlas is simply **not justified by a current product need**.

---

## Minimal M-4A Surface Proposal (if product need emerges later)

**Do not implement in this pass.** If a named multi-theater scenario is approved:

### Recommended path: Option B — Algebraic tile-local atlas mask (homogeneous square batches)

Ratified preferred isolation per `mapping_atlas_batching_isolation_design_note.md` and M-4A sandbox evidence:

- Generic tile-local valid-neighbor mask in WGSL (no faction/ownership semantics)
- Flush-packed tiles (`G=0`) with protocol-oracle parity vs standalone per-tile oracle
- Physical gutter `G≥H` as fallback only
- Driver-side generic atlas packer behind opt-in mapping profile
- VRAM accounting + §11 acceptance checklist
- `request_atlas_batching` admission unlock only after gate pass

### Not recommended as M-4A

| Option | Reason |
|---|---|
| A — Generic active-cell mask (single grid) | Already partially exists (`use_active_mask`); not atlas; separate sparse-execution gate |
| C — Source-mask/source-identity | Separate M-5 track |
| D — Do nothing | **Current decision** |

---

## GPU/WGSL Layout Risk Analysis

Current production WGSL (`structured_field_stencil.wgsl`):

- Flat grid: `width × height × n_dims`
- Bindings: uniform params, input/output values, optional `active_mask` (generic u32 per cell)
- Per-direction weights support M-5A gradient (no semantic names)
- **No atlas tile metadata, gutter, or tile-local mask in production WGSL**

Atlas implementation risks (future, if authorized):

| Risk | Severity |
|---|---|
| Buffer layout / stride changes for packed tiles | High |
| Neighbor sampling at tile boundaries | High — core correctness issue M-4A sandbox addressed |
| Ping-pong correctness with atlas-global coordinates | Medium |
| Uniform/param expansion for tile origin/pitch | Medium |
| CPU protocol oracle complexity (seed-clear per tile, boundary mode) | High |
| Semantic leakage (faction/ownership in shader) | Blocked by guardrails |
| `simthing-sim` pressure | Blocked — packer must live in driver |
| Performance regression on single-grid path | Medium — must remain default-off |

Existing `source_cap`, boundary mode, and ping-pong tests apply to **single-grid** path only; atlas would require a **separate** oracle and parity suite per §11.

---

## CPU Oracle / Parity Requirement (future)

Any authorized M-4A implementation must provide:

- Protocol oracle matching GPU atlas semantics (gutter or algebraic mask, per-tile seed-clear, boundary mode)
- Full-tile parity vs standalone per-tile oracle (M-4A sandbox demonstrated this for algebraic mask)
- No t44-only acceptance (per M-4 design note §11)

---

## Stop Conditions (for future implementation handoff)

Future M-4A implementation must stop if it requires:

- Semantic WGSL (faction/ownership/AI/threat)
- Default SimSession mapping wiring
- `simthing-sim` map awareness
- Source-mask without separate M-5 gate
- Atlas without CPU protocol oracle plan
- Weakening M-5D strict-sink validation
- L1 coupling, dense temporal memory, sqrt, GradientXY, production economy→mapping bridge

---

## Decision

**DEFER M-4A.**

No concrete named product scenario requires multi-theater atlas batching today. M-5-gradient substrate satisfies current gradient/composition product paths. Remaining related needs (source identity, single-grid sparse mask, queue-write scale) belong to **separate gates**, not M-4A.

### If product names a multi-theater scenario later

**Next implementation handoff title:**

`Phase M-4 — Algebraic Tile-Local Atlas Packer (Homogeneous Square Batches)`

**Implementation boundaries:**

- Generic driver-side atlas packer (opt-in mapping profile only)
- Tile-local algebraic mask WGSL extension (semantic-free; CPU-oracle-backed)
- §11 acceptance gate + VRAM accounting
- Unlock `request_atlas_batching` admission only after gate pass
- No default mapping wiring; no `simthing-sim` changes; no production economy→mapping bridge
- Preserve all M-5 gradient semantics and strict-sink admission unchanged

---

## Required Scans

**Scan 1:**
```bash
rg "atlas|M-4A|mask_mode|MaskMode|active mask|gutter|tile|tiling|source_mask|source identity" docs crates
```
**Result:** Extensive **deferred/guardrail/design-note** references; `request_atlas_batching` rejected at admission; `CompiledRegionFieldMaskMode::All` hardcoded in RegionField admission; GPU has generic `active_mask` + `ActiveOnlyExperimentalNoHalo` (provisional); M-4A sandbox artifacts in docs/workshop archive only — **no production atlas implementation**.

**Scan 2:**
```bash
rg "semantic WGSL|new WGSL|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim|GradientXY|sqrt|L1 cross-field|dense per-cell" docs/workshop docs/accumulator_op_v2_production_plan.md docs/invariants.md
```
**Result:** Guardrail/deferred context only; no new violations.

**Scan 3:**
```bash
rg "mask|atlas|faction|ownership|owner|AI|threat|source|identity" crates/simthing-gpu/src/shaders
```
**Result:** Generic terms only in `structured_field_stencil.wgsl`: `source_col`, `source_cap`, `use_active_mask`, `active_mask`, `sample_source`. No faction/ownership/atlas/tile semantics. `accumulator_op.wgsl` has generic `source_slot`/`available`/`unit_cost` — unrelated to mapping atlas.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Sanity Tests (no code changes in this pass)

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
```
**Result:** **26 passed; 0 failed**

```bash
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```
**Result:** **25 passed; 0 failed**

```bash
cargo check --workspace
```
**Result:** **PASS**

---

## Posture Attestation

No semantic WGSL, no default mapping wiring, no simthing-sim changes, no source-mask/source-identity work, no atlas/M-4A implementation, no L1 coupling, no sqrt/new opcode, no production economy→mapping bridge, no ResourceEconomySpec→mapping coupling; M-5 gradient strict-sink admission unchanged; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-4A Atlas Readiness Gate completed; M-4A remains deferred because the current M-5-gradient substrate satisfies named product needs and remaining needs belong to separate gates, active production guidance updated, no implementation performed, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
