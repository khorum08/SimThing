# FIELD_POLICY tensor/stencil WGSL refinement probe — workshop notes

**Purpose:** Resolve whether remaining objections from the first tensor/stencil probe are intrinsic or harness/design-policy artifacts.

## Prototype posture

- Single parametric WGSL kernel (`field_policy_tensor_stencil_refinement_prototype.wgsl`) with `variant` and `directed_mode` uniforms.
- Rust harness: `StencilRefinementPrototype` with ping-pong A/B buffers, CPU oracle, column-aware parent reduction fixture.
- No map/faction/AI semantics; no production pipeline integration.

## Variant modes (uniform `variant`)

| ID | Name | Notes |
|---|---|---|
| 0 | raw | Unnormalized neighbor sum; amplifies |
| 1 | normalized | Neighbor-mean normalization; H=8 tactical OK, H=24 blowup |
| 2 | directed | SE (0) or NW (1) neighbor pair; requires compatible source orientation |
| 3 | clamped | Per-cell cap |
| 4 | decayed_normalized | alpha+gamma ≤ 1 contract; stable but weak tactical signal |
| 5 | source_capped | Normalized + per-cell source_cap clamp |

## Key harness fixes vs prior probe

1. **Ping-pong buffers** for H>1 (no in-place read/write).
2. **Directed compatible setup:** top-left cluster + `directed_mode=NW`; bottom-right cluster + `directed_mode=SE`.
3. **Column-aware parent EML:** Sum threat/resource into parent SlotRange columns; populate aggression/risk on parent; EvalEML on **order band 1** after Sum on band 0 (same pattern as operator toolkit hybrid).
4. **Source policies:** one-shot zero, persistent, every-N, capped, decayed inject tested explicitly.

## Recommended stable operator (probe outcome)

`source_capped_normalized` (variant 5, source_cap=500) or `normalized` with authored **H≤8 horizon cap**. Decayed-normalized is stable but too weak for tactical gradient at H=8.

## Production boundary

Adoption needs: ping-pong buffer contract, source injection policy, optional active mask, column-aware parent reduction binding, designer/RON admission for field_* formula classes (C-8 register_formula sufficient at runtime).
