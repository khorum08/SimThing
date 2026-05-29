# Phase M M-6A Single-Grid Active Mask Readiness Gate

**Date:** 2026-05-29  
**Base HEAD:** `1d175d01f4369488481d986afd3fe6dfb6694bd9`  
**Final commit SHA:** `8454fea`  
**Lane classification:** Tier-2 readiness gate (V7.7 §5)  
**Decision:** **DEFER M-6A**  
**Verdict:** **PASS — readiness gate completed; ActiveOnly mask admission remains deferred**

---

## Files Inspected

| Surface | Finding |
|---|---|
| `docs/tests/phase_m_m4a_atlas_readiness_gate_results.md` | Identified single-grid inactive-cell masking as separate from atlas; deferred atlas |
| `docs/invariants.md` | **`ActiveOnlyExperimentalNoHalo` never production-authorized**; only H-hop/per-hop halo with CPU-oracle parity admitted |
| `docs/workshop/mapping_current_guidance.md` | M-5 landed; atlas/M-4A deferred; no active-mask admission row yet |
| `docs/workshop/m5_gradient_extraction_design_note.md` | M-5A/B/C/D complete; no active-mask dependency |
| `docs/accumulator_op_v2_production_plan.md` | Admission rejects `ActiveOnlyExperimentalNoHalo` for production |
| `crates/simthing-spec/src/spec/region_field.rs` | **No `mask_mode` field** on `RegionFieldSpec` |
| `crates/simthing-spec/src/compile/region_field_admission.rs` | `CompiledRegionFieldMaskMode::All` only; hardcoded at compile |
| `crates/simthing-gpu/src/structured_field_stencil.rs` | Generic `active_mask` buffer; `All` + `ActiveOnlyExperimentalNoHalo` enum |
| `crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl` | Generic `use_active_mask` + `active_mask` binding; inactive cells pass-through copy |
| `crates/simthing-gpu/tests/structured_field_stencil.rs` | Ping-pong/boundary/source-cap/gradient parity tests exist for `All` mode only |
| `crates/simthing-spec/tests/region_field_spec_admission.rs` | Asserts compiled `mask_mode == All`; no mask admission cases |

---

## Product Need Analysis

### What product problem requires single-grid active masks now?

**Potential future need:** skip stencil computation on inactive cells within a single square grid — e.g. irregular geography, sparse playable area, or performance on large grids where most cells are inert.

**Current concrete scenario:** **None named.** First-slice product path uses full 10×10 grids with all cells active. M-5-gradient composition fixtures operate on full grids. No authored scenario requires active-only execution today.

### What can active mask do that M-5-gradient cannot?

Active mask controls **which cells participate in stencil execution** on one grid. M-5-gradient controls **field calculus** (derivative extraction, L3 composition). Orthogonal capabilities.

### Why this is not atlas/M-4A

| Dimension | Single-grid active mask | Atlas/M-4A |
|---|---|---|
| Scope | One grid, one coordinate space | Multi-tile packing into atlas |
| Problem | Sparse execution within grid | VRAM/dispatch batching across theaters |
| Buffer layout | Flat `width × height` unchanged | Tile packing, gutter, tile-local masks |
| Status | GPU hook exists; admission absent | Deferred (M-4A gate) |

M-4A explicitly routed inactive-cell masking here as a **separate gate**, not atlas.

### Why this is not source-mask/source-identity (M-5)

| Dimension | Active mask | Source-mask/source-identity |
|---|---|---|
| Semantics | Generic u32 per cell: active/inactive | Behavioral seed policy, source identity |
| Shader sees | `active_mask[idx] == 0` → skip stencil | Would require source identity buffer |
| Current policy | `CallerManagedOneShotSeedThenZero` | Separate M-5 track, not authorized |
| Purpose | Skip computation on inert cells | Control diffusion seeding behavior |

Active mask does **not** encode faction, ownership, threat, or source identity. Workshop archive `cpu_source_mask_model` is historical sandbox only — not production.

### Is the need L1 coupling or dense temporal memory?

No. Both remain separately gated and prohibited.

### Is the need only authoring ergonomics?

Not currently. Full-grid execution satisfies all landed product paths. Active mask would be a **runtime optimization / geography expressiveness** feature, not an authoring fix.

---

## Existing GPU Active-Mask Support Summary

### WGSL (`structured_field_stencil.wgsl`)

- Binding 3: `active_mask: array<u32>` — generic, semantic-free
- Uniform: `use_active_mask: u32`
- Behavior when `use_active_mask != 0 && active_mask[idx] == 0`:
  - **Pass-through:** copy all dimension columns unchanged from input to output
  - **No stencil computation** on inactive cell
- Active cells: normal stencil with neighbor sampling via `sample_source`
- **No halo expansion:** inactive neighbors can still be sampled by active cells (asymmetric frontier)

### Rust (`structured_field_stencil.rs`)

```rust
pub enum StructuredFieldStencilMaskMode {
    All,
    ActiveOnlyExperimentalNoHalo,  // provisional; not production-authorized
}
```

- `set_mask_mode()` toggles `use_active_mask` uniform
- `active_mask` buffer allocated at op creation; default all-ones when mode is `All`
- `readback_active_mask_ratio()` available when stats collected
- First-slice runtime and RegionField admission **hardcode `All`**

### Generic and semantic-free?

**Yes.** Shader sees only u32 mask values and generic floats/columns/weights. No faction/ownership/AI/atlas/tile semantics.

### Boundary behavior with masked cells

- **Partially specified:** inactive cells pass-through; active cells use existing `boundary_mode` (Zero/Clamp) for out-of-grid neighbor sampling
- **Under-specified:** interaction between inactive-cell frontiers and multi-hop diffusion (halo semantics explicitly absent — `NoHalo` in enum name)
- **Not tested** with active mask in production test suite

### Ping-pong with active mask

- Ping-pong infrastructure exists and is tested for `All` mode
- **No production test** combines ping-pong + `ActiveOnlyExperimentalNoHalo`

### Source-cap with active mask

- Source-cap parity tested for `All` mode
- **No production test** combines source-cap + active mask

---

## CPU Oracle / Parity Coverage Summary

| Coverage area | `All` mode | `ActiveOnlyExperimentalNoHalo` |
|---|---|---|
| CPU oracle (normalized/source-cap/gradient) | Yes — full parity suite | **No** |
| GPU/CPU parity single-step | Yes (M-5A tests) | **No** |
| Ping-pong multi-hop | Yes (`test_b_pingpong_correctness`) | **No** |
| Boundary Zero/Clamp | Yes (`structured_field_stencil_clamp_boundary_gpu_cpu_parity`) | **No** |
| Source-cap cluster | Yes | **No** |
| Gradient X/Y | Yes (M-5A) | **No** |
| Active mask enum provisional naming | N/A | Yes only (`structured_field_stencil_active_mask_provisional`) |
| RegionField admission compile | `All` only | **Not admitted** |

**Workshop archive only:** `sead_tensor_stencil_refinement_sandbox_code_preserve.rs` contains `test7_active_mask_pingpong` — historical sandbox, not production CI evidence.

**Constitutional block:** `docs/invariants.md` — "`ActiveOnlyExperimentalNoHalo` is never production-authorized; only H-hop / per-hop halo with CPU-oracle parity is admitted."

---

## Required `RegionFieldSpec` / Admission Surface (future, if authorized)

Minimal future surface (not implemented in this pass):

```rust
#[serde(default)]
pub mask_mode: RegionFieldMaskModeSpec,  // default All

pub enum RegionFieldMaskModeSpec {
    All,
    ActiveOnly,  // maps to halo-contracted mode, NOT bare NoHalo
}
```

Compile to `CompiledRegionFieldMaskMode`; default `All`; mask buffer caller/test supplied; no semantic labels; no atlas; no production runtime wiring until separately gated.

**Prerequisite before admission:** halo/frontier contract (H-hop expansion or equivalent) replacing bare `ActiveOnlyExperimentalNoHalo`.

---

## Decision

**DEFER M-6A.**

The GPU substrate is **generic and semantic-free**, but **not ready for spec/admission exposure**:

1. **Constitutional:** `ActiveOnlyExperimentalNoHalo` is explicitly not production-authorized; invariants require halo contract first
2. **Under-specified semantics:** inactive-cell pass-through without halo/frontier contract produces asymmetric diffusion at mask boundaries; behavior not production-defined
3. **Missing CPU oracle:** no CPU oracle mirrors ActiveOnly mask behavior
4. **Missing parity tests:** no GPU/CPU parity for active mask with ping-pong, boundary, source-cap, or gradient
5. **No named product scenario** requiring active-only execution on current paths
6. **Production plan** already lists rejecting `ActiveOnlyExperimentalNoHalo` for production at admission

Exposing `RegionFieldMaskModeSpec::ActiveOnly` now would wire provisional, untested, constitutionally-blocked behavior into the spec layer.

### Missing evidence for future proceed

| Gap | Required before M-6B |
|---|---|
| Halo/frontier contract | H-hop mask dilation or per-hop frontier expansion spec |
| CPU oracle | Mirror masked stencil including boundary + multi-hop |
| GPU parity suite | Ping-pong, boundary Zero/Clamp, source-cap, gradient with mask |
| Inactive-cell output contract | Deterministic behavior at mask edges documented and tested |
| Product scenario | Named use case (irregular geography / sparse grid) |
| Constitutional alignment | Replace or supersede `ActiveOnlyExperimentalNoHalo` with halo-backed mode |

### If product need emerges later

**Next implementation handoff title:**

`Phase M-6B — RegionField ActiveOnly Mask Admission`

**Implementation boundaries (only after halo contract gate):**

- `RegionFieldMaskModeSpec::All | ActiveOnly` with default `All`
- Compile to halo-contracted `CompiledRegionFieldMaskMode` (not bare NoHalo)
- CPU oracle + GPU parity for masked ping-pong/boundary/source-cap/gradient
- Mask buffer caller/test supplied; no semantic labels
- No atlas, no source-mask, no default mapping wiring, no `simthing-sim` changes
- Preserve M-5D strict-sink validation unchanged

---

## Required Scans

**Scan 1:**
```bash
rg "active_mask|use_active_mask|MaskMode|mask_mode|ActiveOnly|atlas|tile|gutter|source_mask|source identity" docs crates
```
**Result:** GPU has generic `active_mask` plumbing; `ActiveOnlyExperimentalNoHalo` marked provisional throughout; RegionField admission hardcodes `All`; atlas/source-mask references remain deferred/guardrail; first-slice tests assert runtime does not contain `ActiveOnlyExperimentalNoHalo`; workshop archive has historical sandbox usage only.

**Scan 2:**
```bash
rg "faction|ownership|owner|AI|threat|scarcity|need|routing|source identity|source_mask|atlas|tile|gutter" crates/simthing-gpu/src/shaders
```
**Result:** **No semantic matches.** Generic terms in `structured_field_stencil.wgsl`: `source_col`, `source_cap`, `sample_source`, `use_active_mask`, `active_mask`. Generic accumulator `source_slot`/`source_kind` in `accumulator_op.wgsl` — unrelated to mapping masks.

**Scan 3:**
```bash
rg "default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim|GradientXY|sqrt|L1 cross-field|dense per-cell" docs/workshop docs/accumulator_op_v2_production_plan.md docs/invariants.md
```
**Result:** Guardrail/deferred context only; no violations.

---

## Tests Run

```bash
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
```
**Result:** **26 passed; 0 failed**

```bash
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```
**Result:** **25 passed; 0 failed**

Includes `structured_field_stencil_active_mask_provisional` (enum naming only — **not** GPU parity).

**Existing ping-pong active-mask tests:** None in production CI. Archive only: `docs/workshop/archive/sead/sead_tensor_stencil_refinement_sandbox_code_preserve.rs::test7_active_mask_pingpong`.

```bash
cargo check --workspace
```
**Result:** **PASS**

No code changes beyond docs/report in this pass.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Posture Attestation

No semantic WGSL, no atlas/M-4A, no source-mask/source-identity, no default mapping wiring, no `simthing-sim` changes, no production economy→mapping bridge, no L1 coupling, no sqrt/new opcode, no active-mask admission implementation; M-5 gradient strict-sink validation unchanged; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-6A Single-Grid Active Mask Readiness Gate completed; ActiveOnly mask admission remains deferred pending missing CPU/GPU parity and halo-contract evidence, active docs and production plan updated, no implementation performed, and V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
