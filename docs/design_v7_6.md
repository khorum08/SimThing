# SimThing — Design v7.6 Amendment

> **Status:** Active amendment (2026-05-19). Supersedes V7.5 **development guardrails only**;
> V7.5 constitutional core and all prior v7 text in [`design_v7.md`](design_v7.md) remain
> authoritative unless explicitly amended here.
>
> **Companion:** [`design_v7.md`](design_v7.md) (V7.5 base) · [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md)

---

## 1. What changed in v7.6

V7.6 preserves the V7.5 constitutional core:

- SimThing remains the single substrate abstraction.
- `simthing-sim` remains semantic-free.
- Resource Flow remains opt-in and not global default-on.
- GPU primitives must remain deterministic, bounded, replayable, and data-parallel.
- No CPU-side AI planner or semantic sidecar may become the real game.
- No mapping/location runtime is authorized by this amendment alone.

V7.6 relaxes two **misplaced development guardrails** proven too restrictive by SEAD /
tensor-stencil / EML admission probes:

### 1.1 WGSL guardrail revision

The old blanket **"no new WGSL"** development guardrail is replaced by:

```text
No semantic or map-specific WGSL.
```

Generic tensor/field kernels are allowed when they are:

- deterministic
- bounded
- reusable
- semantic-free (no map/faction/AI/simthing-sim/Resource Flow semantics)

### 1.2 EML guardrail revision

Legacy formula whitelist rejection is **not** a substrate prohibition when a formula is
deterministic, bounded, and uses existing safe opcodes.

Formula-class admission belongs at the **RON/Designer/spec policy layer** unless runtime
safety is actually implicated.

Field formula classes now admitted at the legacy whitelist layer (designer-facing):

- `field_pressure`
- `field_urgency`
- `field_decay`
- `bounded_field_update`

(C-8 `register_formula` path was already sufficient at runtime; whitelist alignment removes
wrong-layer rejection.)

---

## 2. Evidence summary (2026-05 SEAD probes)

**SEAD operator toolkit:** Per-edge AccumulatorOp propagation over dense grids is over budget.
Hierarchy-first strategic awareness is much cheaper. Dirty/frontier skipping helps but does not
solve dense long-horizon diffusion alone.

**Tensor/stencil WGSL probe:** Generic stencil WGSL compiled and ran as general tensor math
over flat buffers, dimensions, columns, and kernel weights — no embedded semantics. Material
cost improvement vs per-edge AccumulatorOps.

**Tensor/stencil refinement:** Ping-pong buffers required and correct for H>1. Directed stencil
works when setup orientation matches `directed_mode`. Parent EvalEML works when reduction is
column-aware and parent personality fields are populated. EML `field_*` rejection was
policy/admission, not runtime inability. Long-horizon stability requires source policy,
horizon caps, and/or bounded coefficients. Active masks promising but need halo/frontier
semantics before production dependence.

---

## 3. StructuredFieldStencilOp — candidate production primitive

`StructuredFieldStencilOp` is admitted as a **live generic GPU primitive** (opt-in library
API; not wired into default production pass graph).

It is a generic structured field/tensor operation over flat buffers:

```text
width, height, n_dims, source_col, target_col
kernel coefficients (alpha_self, gamma_neighbor)
source policy, horizon count
optional active mask
ping-pong buffers (required for H > 1)
```

It must **not** know about: maps, factions, AI, stars, planets, RegionCells, Resource Flow
semantics, or simthing-sim internals.

Mapping may later **consume** it; it is not mapping runtime.

**Live code:** `crates/simthing-gpu/src/structured_field_stencil.rs`,
`crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl`

### 3.1 Production constraints

| Constraint | Value |
|---|---|
| Default operators | `normalized_stencil`, `source_capped_normalized` |
| Default tactical horizon | H ≤ 8 |
| Extended horizon | H ≤ 16 only with `allow_extended_horizon` + source cap/decay contract |
| Source policy | one-shot seed then zero (validated safe default) |
| Buffers | ping-pong required for H > 1 |
| Active mask | optional; not fully production-authorized until halo/frontier clarified |
| Parent AI bridge | column-aware SlotRange reduction → parent columns → EvalEML on later order band |

---

## 4. V7.6 greenfield admission criteria

New GPU/EML work is allowed when:

1. Generic primitive, not scenario semantics.
2. Opt-in (no production default changes).
3. Does not impair Resource Flow, E-11B, Phase T, or simthing-sim behavior.
4. Covered by regression tests and documented admission constraints.

---

## 5. Explicit non-goals (unchanged)

- No mapping/location runtime from this amendment.
- No Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5.
- No Resource Flow default-on.
- No simthing-sim arena awareness.
- No semantic WGSL.
