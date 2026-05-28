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
| Source policy | caller-managed one-shot seed then zero (`CallerManagedOneShotSeedThenZero`; primitive does not auto-clear sources) |
| Buffers | ping-pong required for H > 1 |
| Execution horizon | `run_ping_pong` / `dispatch_ping_pong` reject steps above `config.horizon`; use `run_configured_horizon` for the configured tactical hop count |
| Active mask | `ActiveOnlyExperimentalNoHalo` — provisional; not production-authorized until halo/frontier clarified |
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

---

## 6. V7.6 guardrail hardening (2026-05-19)

`StructuredFieldStencilOp` remains a **generic opt-in toolkit primitive**. The hardening pass:

- enforces execution horizon constraints (`ExecutionHorizonExceedsConfig` when steps exceed configured horizon)
- fixes source-cap test indexing and clarifies source policy as **caller-managed**
- adds CPU/GPU clamp-boundary parity for `BoundaryMode::Clamp`
- marks active-mask behavior **provisional** via `ActiveOnlyExperimentalNoHalo`

No mapping runtime is implemented. No production pass graph is wired to
`StructuredFieldStencilOp` by default. Resource Flow defaults remain unchanged.
`simthing-sim` remains semantic-free.

---

## 7. Parked state (2026-05-19)

V7.6 `StructuredFieldStencilOp` **promotion and guardrail hardening are complete**.
Further mapping-related implementation is **parked pending the Mapping ADR**.

The next work item is **not** runtime mapping implementation. It is the **Mapping ADR**
that defines RegionCell fields, source policy, active-mask halo/frontier semantics,
cadence tiers, and column-aware parent bindings.

> **Update (2026-05-28):** The Mapping ADR is now **approved at the architecture
> level** — see [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md)
> and the [`design_v7_7.md`](design_v7_7.md) surfacing. Approval unlocks the
> **Phase M** generic natives in the production plan and names the ADR's first
> scenario-level slice. It still authorizes **no** mapping runtime (that remains a
> separately-gated later track). V7.6's parked posture (StructuredFieldStencilOp
> live, opt-in, hardened, inert by default) is unchanged.

### 7.1 Confirmed current posture

```text
V7.6 StructuredFieldStencilOp promotion and guardrail hardening are complete.

StructuredFieldStencilOp is a live generic GPU toolkit primitive for structured 2D field
propagation over flat buffers. It is not mapping runtime and contains no map/faction/AI
semantics.

The primitive is opt-in by direct API use only. Existing production pass graphs do not
invoke it.

Execution horizon guardrails are enforced:
- default tactical horizon H <= 8
- extended horizon H <= 16 only with allow_extended_horizon and stability/source policy
- run_ping_pong / dispatch_ping_pong reject steps above config.horizon
- run_configured_horizon is the preferred safe execution helper

Source policy is caller-managed:
- CallerManagedOneShotSeedThenZero means the caller seeds source cells once, clears the
  source column after the initial hop, then runs configured-horizon propagation.
- The primitive does not identify or clear source slots automatically.

Active mask remains provisional:
- ActiveOnlyExperimentalNoHalo is not production-authorized until halo/frontier semantics
  are defined.

Parent bridge is column-aware:
- local stencil fields reduce upward through SlotRange into specific parent EML input columns
- parent personality columns are populated separately
- EvalEML runs on a later order band
```

**Evidence preserved:** [`v7_6_structured_field_stencil_promotion_test_results.md`](tests/v7_6_structured_field_stencil_promotion_test_results.md),
[`v7_6_structured_field_stencil_guardrail_hardening_test_results.md`](tests/v7_6_structured_field_stencil_guardrail_hardening_test_results.md),
[`v7_6_structured_field_stencil_parked_state_test_results.md`](tests/v7_6_structured_field_stencil_parked_state_test_results.md)
