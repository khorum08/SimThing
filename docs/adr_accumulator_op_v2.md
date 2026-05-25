# ADR: AccumulatorOp v2 — GPU-Resident Gather/Combine/Gate/Scatter Primitive

**Status:** Proposed  
**Date:** 2026-05-24  
**Authors:** Architecture review (Opus 4.7 + workshop battery, Cursor Composer 2.5 implementation)  
**Supersedes:** `design_v6.md` §10 (GPU pipeline). All other sections of v6 remain
authoritative until `design_v7.md` is accepted.  
**Completed by:** `design_v7.md` (specification of the v7 architecture this ADR enables).

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

**Important:** The workshop established that the *current* pipeline is also
loose-tolerance on WeightedMean. Any existing code path that reads a
WeightedMean-reduced value and uses it as a hard structural trigger is already
incorrect under the current architecture. PR A-4 audits this exposure before
any migration begins.

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

- GPU pipeline collapses from 8 passes to 3.
- Transfer conservation is structurally enforced — `SubtractFromSource` is
  atomically bound to the target write. The two-overlay hack is eliminated.
- Threshold event readback (currently ~21ms, the dominant boundary cost at
  scale) is replaced by a GPU atomic counter + compact emission buffer.
- EML expressions compile to GPU-evaluable trees; intensity update and
  designer-tunable combine functions share one evaluator kernel.
- Persistent GPU buffers with summary/checksum readback beat the current GPU
  envelope by 3–46×.
- Resource interactions become a native primitive enabling economic V1
  (transfer, debt-band emission, conjunctive production) to ship as first-class
  operations rather than CPU-mediated approximations.

### Negative / caveats

- The `combine` field adds kernel complexity (~12 combine variants).
- WeightedMean is ~3e-6 off CPU oracle. Tolerance policy must be enforced.
- Overlay Multiply/Set under high overlay density (density ≈ 1.0) shows
  performance regression in some conditions. Dirty/cached-rebuild compiler
  required.
- Hot-pool contention requires a separate allocator design.
- Cross-pool queue contention is untested — explicitly deferred.
- The CPU oracle must be maintained for each operation family indefinitely.

---

## Old pipeline sunset policy

Each existing pass is removed in a dedicated sunset PR, sequenced after its
migration PR has been merged, all parity tests have passed, and the feature
flag has been set to default-on for at least one release cycle.

**Sunset sequence (one PR each, Codex 5.5):**

| Pass | Migration PR | Sunset PR | Removal scope |
|---|---|---|---|
| Intent fold | C-2 | S-1 | Delete `intent_fold.rs`, `intent_fold.wgsl`; remove feature flag |
| Pass 2 (intensity) | C-8 | S-2 | Delete `intensity_update.rs`, `intensity_update.wgsl` |
| Pass 3 (overlay) | C-3, C-4 | S-3 | Delete `overlay_prep.rs`, overlay WGSL; remove `overlay_prep` module |
| Pass 4–6 (reduction) | C-5, C-6 | S-4 | Delete `reduction.rs`, reduction WGSL |
| Pass 1 (velocity) | C-7 | S-5 | Delete `velocity_integration.rs`, velocity WGSL; update `GovernedPair` to use AccumulatorOp builder |
| Pass 7 (threshold scan) | C-1 | S-6 | Delete threshold scan WGSL; retire `tick_event_readback_ms` label (superseded by `pass_c_readback_us`) |

Each sunset PR must:
1. Set the feature flag default to `true` (AccumulatorOp path) in a
   preparatory commit
2. Run CI at that state — if CI fails, the sunset is blocked until the
   migration PR is fixed
3. Delete the old code only after CI passes with the flag defaulted on
4. Remove the feature flag itself in the same PR

No sunset PR merges while any downstream test references the deleted code.

---

## Design document policy

`design_v6.md` §10 (GPU pipeline) is superseded by this ADR the moment it is
accepted. The canonical pipeline specification moves to `design_v7.md`, which
is maintained as a living document updated by each migration PR's author.

Specifically:
- Each migration PR (C-1 through C-8) includes a diff to `design_v7.md`
  updating §4 (GPU pipeline passes) to reflect the current state
- Each sunset PR (S-1 through S-6) removes the corresponding old-pass
  description from `design_v7.md`
- `design_v6.md` §10 is annotated with a header: "SUPERSEDED — see
  design_v7.md §4 for current pipeline specification"

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
| Old pass code is never deleted without a green CI run at default-on flag | Enforced by sunset PR checklist |
| `design_v7.md` §4 is updated by each migration PR | PR template checklist item |

---

## Out of scope

- Migration of Pass 0 (snapshot). `copy_buffer_to_buffer` stays permanently.
- Cross-pool queue contention. Separate gate, separate ADR.
- Hot-pool allocator v2 design. Follow-on engineering gate (Phase D).
- ScriptExpr / EML Phase 1–4 implementation. See `docs/eml_integration_guidance.md`.
- Transcendental functions in EML GPU evaluation.

---

## References

- `docs/design_v7.md` — v7 architecture specification (companion to this ADR)
- `docs/workshop/accumulator_write_fit_matrix.md` — Phase 0 fit matrix
- `docs/workshop/resolver_architecture_pivot.md` — Pivot proposal
- `docs/workshop/simthing_base_economic_system_working_doc.md` — Economic substrate
- `crates/simthing-workshop/tests/` — Workshop battery test reports
- `docs/eml_integration_guidance.md` — EML expression policy
- `docs/invariants.md` — Core structural invariants (extended by this ADR)
- `docs/design_v6.md` §10 — SUPERSEDED by this ADR; see design_v7.md §4
