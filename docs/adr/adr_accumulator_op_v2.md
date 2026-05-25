# ADR: AccumulatorOp v2 — GPU-Resident Gather/Combine/Gate/Scatter Primitive

**Status:** Proposed  
**Date:** 2026-05-24  
**Authors:** Architecture review (Opus 4.7 + workshop battery, Cursor Composer 2.5 implementation)  
**Supersedes:** Nothing formally. Extends the GPU pipeline specified in `design_v6.md` §10.

---

## Context

The SimThing GPU pipeline currently uses eight specialized passes (intent fold,
velocity integration, intensity update, overlay application, four-pass reduction,
threshold scan). Each pass is a separate WGSL kernel with its own buffer layout
and dispatch logic. The pipeline was designed before the project had a concrete
economic substrate — transfers, emission, and conjunctive production recipes
were not first-class operations.

Three forces now converge:

1. **The economic working doc** (`simthing_base_economic_system_working_doc.md`)
   identified that thresholded accumulation and emission is the universal
   substrate for resource interactions, and that the current pipeline has no
   native primitive for it.

2. **Phase 0 fit matrix** (`docs/workshop/accumulator_write_fit_matrix.md`)
   analysed all 15 current GPU operations against a unified primitive. Under
   the v2 framing (multi-input gather, named combine functions, OrderBand gate,
   MultiTarget writes), 14 of 15 operations fit Clean. Snapshot (memcpy) is the
   one permanent retained operation.

3. **Workshop battery** (`crates/simthing-workshop/`) ran six gate tests — EML
   intensity, WeightedMean A/B, overlay order-band semantics, multi-target
   replay, transfer/emission contention, persistent-buffer benchmark — and
   produced empirical results across three independent runs each, with
   timestamp query instrumentation for the final benchmark.

### Workshop evidence summary

| Gate | Verdict | Key numbers |
|---|---|---|
| EML Phase 5 intensity | **Strong pass** | ~1.6× overhead vs hardcoded at 100k; bit-exact for no-transcendental formula; deterministic |
| WeightedMean A/B | **Pass with tolerance policy** | 1.05–1.93× speedup; max_abs_error ~3e-6 vs CPU oracle; GPU-to-GPU deterministic |
| Overlay order-bands | **Strong semantic pass** | Bit-exact Add/Mul/Set ordering at all tested scales; 0.62–1.30× speedup |
| Multi-target replay | **Pass with logging tiers** | Conservation exact; summary mode 3.4–4.4× CPU; dense records log-volume limited at 102MB/100k-tick run |
| Transfer/emission contention | **Pass (distributed); weak pass (hotspot)** | Distributed: ~2× CPU. Hotspot (16 pools, 100k req): 0.14× CPU — 7× slower |
| Persistent buffer + timestamp | **Pass for execution model** | 3–46× faster than current GPU envelope; timestamp queries working |

---

## Decision

**Adopt AccumulatorOp v2 as the foundational GPU execution primitive for
SimThing resource interactions, overlay application, reduction, threshold
scanning, and EML-combined updates.**

The unified primitive replaces the current 8-pass specialised pipeline with a
3-pass architecture:

```
Pass 0 (retained):  Snapshot — copy_buffer_to_buffer; not a per-slot write
Pass B (new):       AccumulatorOp — unified gather/combine/gate/scatter kernel,
                    dispatched in OrderBand sequence
Pass C (lifted):    Event readback — GPU atomic counter + compact emission buffer
```

This decision is scoped to the **architectural direction and production plan**.
The workshop battery validates the approach; it is not the production
implementation. Each operation family migrates in a separate PR with its own
correctness gate. No existing tests break until the migration PR for that family
lands and passes all parity checks.

---

## The AccumulatorOp v2 Primitive

```rust
AccumulatorOp {
    source: SourceSpec {
        kind:       Constant | SlotValue | SlotRange | ConjunctiveCrossing,
        inputs:     [InputSpec; 4] | (start: u32, count: u32),
        weight_col: Option<u32>,        // for WeightedMean
    },
    combine: CombineFn,
    gate:    GateSpec,
    scale:   ScaleSpec,
    consume: ConsumeMode,
    targets: [(slot: u32, col: u32); 4], // MultiTarget
}

enum CombineFn {
    Identity,
    Sum,
    Mean,
    Max,
    Min,
    WeightedMean,                       // soft aggregate; tolerance policy applies
    Product,                            // for Multiply overlays
    LastByPriority,                     // for Set overlays
    IntegrateWithClamp { dt, vel_max, amount_min, amount_max },
    CrossingFormula { unit_cost },      // debt-band emission
    MinAcrossInputs,                    // conjunctive emit
    EvalEML { tree_id },                // Phase 5 EML; requires whitelist
}

enum GateSpec {
    Always,
    Threshold { value, direction },
    LifecycleActive,
    DirtyOnly,
    OrderBand(u32),
}

enum ConsumeMode {
    None,
    SubtractFromSource,
    SubtractFromAllInputs,
    ResetTarget,
    ScaleTarget,
    EmitEvent,
}
```

---

## Semantic scope

### Exact / integer semantics (never relaxed)

These operations must be bit-exact and conservation-verified at all times:

- Transfer between resource slots (`SubtractFromSource`)
- Queue debt-band emission (`CrossingFormula`)
- Conjunctive production emission (`MinAcrossInputs`)
- Replay checksums and delta log conservation checks
- Hard structural threshold triggers (fission, capability unlock, property expiry)
- Overlay Add on any column that feeds conservation-exact accounting

### Soft aggregate semantics (tolerance policy applies)

These operations are GPU-to-GPU deterministic but not bit-exact against the
CPU oracle (~3e-6 max_abs_error observed):

- `WeightedMean` reduce
- `Mean` reduce (same class)

**ADR tolerance policy:** Soft aggregates are acceptable for non-authoritative
computed values — faction-level efficiency scores, planet stability indices,
diplomatic pressure summaries. They must NOT drive exact replay checksums,
conservation checks, or hard structural transitions without quantization or
hysteresis guards.

### EML expression policy

The `EvalEML` combine is validated for formulas meeting all of:

- No transcendental functions (no exp, log, sin, cos)
- ≤16 nodes in the expression tree (within the 32-node budget with headroom)
- The formula class is whitelisted in `docs/eml_integration_guidance.md`

Overhead is ~1.6× vs the hardcoded kernel at 100k slots. Acceptable.

Formulas outside this envelope must use the hardcoded kernel path or remain
CPU-side per the EML integration guidance doc's Phase 4 (derived field
integration). GPU EML evaluation on formulas with transcendentals is
explicitly out of scope for this ADR.

### Hot-pool contention policy

The v1 one-invocation-per-pool allocator is semantically correct and
validated for distributed layouts (≤100 requesters per pool). It is **not**
acceptable for production hotspot layouts (few pools, many requesters) — the
workshop measured 0.14× CPU (7× slower) at 16 pools / 100k requesters.

Before AccumulatorOp can handle production hotspot transfer workloads, a v2
allocator using segmented scan, prefix allocation, or subrange partitioning
must be designed and gated. This is an explicit follow-on engineering gate.
Cross-pool queue contention is a separate gate not yet tested.

---

## Consequences

### Positive

- GPU pipeline collapses from 8 passes to 3. Each operation family is one
  WGSL kernel entry with a named combine function, not a separate pass.
- Transfer conservation is structurally enforced — `SubtractFromSource` is
  atomically bound to the target write. The two-overlay hack is eliminated.
- Threshold event readback (currently ~21ms, the dominant boundary cost at
  scale) is replaced by a GPU atomic counter + compact emission buffer. Route
  1 from the optimization doc falls out as a natural property of the
  `EmitEvent` consume mode.
- EML expressions compile to GPU-evaluable trees; intensity update and
  designer-tunable combine functions share one evaluator kernel rather than
  requiring bespoke passes.
- Persistent GPU buffers with summary/checksum readback beat the current GPU
  envelope by 3–46×. Total-validation readback is retained for debug only.

### Negative / caveats

- The `combine` field adds kernel complexity. The kernel is now a switch over
  ~12 combine variants. Complexity is concentrated in one file rather than
  spread across 8 passes — a net improvement, but the kernel requires more
  careful review.
- WeightedMean is ~3e-6 off CPU oracle. The tolerance policy must be enforced
  at design time. Any use of WeightedMean output as a hard trigger must be
  reviewed and approved.
- Overlay Multiply/Set under high overlay density (density ≈ 1.0) shows
  performance regression vs current path in one of three test runs. The
  `OrderBand` compiler requires a dirty/cached-rebuild path before production
  to avoid materialising the full indexed set every tick.
- Hot-pool contention requires a separate allocator design before production.
- Cross-pool queue contention is untested — explicitly deferred.
- The CPU oracle must be maintained for each operation family indefinitely.
  It is the only reliable regression signal for GPU-to-GPU determinism drift.

---

## Invariant additions

These extend `docs/invariants.md` and carry the same enforcement weight:

| Rule | Enforced by |
|---|---|
| Exact operations never use soft-aggregate combine fns | Code review gate; `WeightedMean` / `Mean` may not appear in conservation-critical registration paths |
| `EvalEML` combine requires a whitelist entry | `EmlExpressionRegistry::assert_whitelisted(tree_id)` checked at registration |
| `SubtractFromSource` is the only transfer mechanism | No two-overlay transfers; `TransformOp::Add` on two separate slots for the same logical transfer is a violation |
| Emission records are produced for every GPU-resolved emission | `EmissionRecord { reg_idx, emit_count }` written to compact buffer; read back for delta log |
| Persistent GPU buffer is the residency model | `AccumulatorOpSession` is created at session open and closed at session close; no per-tick device creation |
| Timestamp queries are required for performance claims | Any PR claiming a performance win must include timestamped GPU pass measurements, not just wall-clock |

---

## Out of scope

- Migration of Pass 0 (snapshot). `copy_buffer_to_buffer` is the GPU's native
  primitive for this case. It stays.
- Cross-pool queue contention. Separate gate, separate ADR.
- Hot-pool allocator v2 design. Follow-on engineering gate (see Phase D).
- Full economic V1 implementation. Deferred per `todo.md`.
- ScriptExpr / EML Phase 1–4 implementation. Governed by
  `docs/eml_integration_guidance.md`.
- Transcendental functions in EML GPU evaluation.

---

## References

- `docs/workshop/accumulator_write_fit_matrix.md` — Phase 0 fit matrix, v1 and v2 analysis
- `docs/workshop/resolver_architecture_pivot.md` — Pivot proposal and foundational retrospective
- `docs/workshop/simthing_base_economic_system_working_doc.md` — Economic substrate design
- `crates/simthing-workshop/tests/` — All workshop battery test reports
- `docs/eml_integration_guidance.md` — EML expression policy and phase ladder
- `docs/invariants.md` — Core structural invariants (extended by this ADR)
- `docs/design_v6.md` §10 — Current GPU pipeline specification
