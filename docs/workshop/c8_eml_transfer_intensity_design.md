# C-8 EML + Intensity + Transfer / Emission Design Gate (Opus design memo)

**Author:** Opus 4.7
**Date:** 2026-05-25
**Gate for:** Codex 5.5 / Composer 2.5 implementation PR series — `feat(gpu): C-8a/b/c/d EML + intensity + transfer + emission`
**Status:** Accepted (design); implementation PRs follow separately
**Implementer:** Mixed — **Codex 5.5** for mechanical parts (registry, type tables, doc-replacements); **Composer 2.5** for the WGSL interpreter, transfer substrate, and integration
**Companion:** `docs/adr_accumulator_op_v2.md`, `docs/design_v7.md` §2 + §4 + §6 + §7, `docs/accumulator_op_v2_production_plan.md` PR C-8, `docs/workshop/pivot_forward_implementation_policy.md`, `docs/workshop/c4_overlay_orderband_compiler_design.md` (C-4), `docs/workshop/c5_weighted_mean_reduction_design.md` (C-5)

---

## TL;DR

> **Migrate now, but build the runway.** C-8's production baseline keeps the
> A-3 ExactDeterministic whitelist intact (no transcendentals, ≤16 nodes,
> deterministic IEEE-754 ops). The EML substrate is rebuilt around an
> **execution-class taxonomy** and a **consumer validation matrix** so
> future PRs can admit `SoftDeterministic`, `FastApproximate`, and
> `CpuOracleOnly` classes without reformatting the GPU node table or
> reworking the registry.
>
> **Conservation never compromises.** Transfer (`CrossingFormula +
> MinAcrossInputs + SubtractFromAllInputs`) admits **`ExactDeterministic`
> only** — no soft, no fast, no exceptions. Hard structural triggers
> continue to require `ExactDeterministic` unless a future ADR amendment
> licenses guarded soft formulas.
>
> **Persistent GPU buffers.** EML nodes live in a session-owned buffer for
> the lifetime of the session; tree mutations occur only at boundary
> sync and increment a generation counter that invalidates affected
> op registrations. **No per-dispatch upload.**
>
> **Bounded WGSL lookup.** `AccumulatorOpGpu.combine_a = tree_range_index`
> — a direct array index into `eml_tree_ranges[]`, not a `tree_id` that
> the shader would have to search for. CPU resolves `tree_id → range_index`
> at registration time.
>
> **Staged delivery (C-8a → C-8d).** Infrastructure first (registry +
> persistent buffer + interpreter), then intensity migration, then
> transfer, then emission. Each stage is its own merge-able PR with a
> default-false flag.
>
> **Active docs updated immediately** (in this same PR) so future agents
> read the execution-class framing rather than "EML never has
> transcendentals." See §15 for the doc edit list.

---

## 1. EML execution classes

The current `EmlTreeMeta { node_count, has_transcendental, formula_class }`
schema records *whether* a tree has transcendentals but does not name
its execution policy. C-8 promotes this into a first-class enum.

```rust
/// Execution policy for an EML formula. Determined at registration time
/// by static analysis of the tree (opcodes used, node count, structural
/// determinism), then attached to every consumer site that references
/// the tree. Consumer admissibility is gated by this class — see the
/// validation matrix in §2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmlExecutionClass {
    /// IEEE-754 deterministic ops only. No transcendentals, no
    /// implementation-defined math, no unbounded loops, no
    /// time/random dependencies. Bit-exact CPU↔GPU expected within
    /// the workshop's measured tolerance class (typically 0 ULPs for
    /// add/mul/min/max/clamp chains; otherwise ≤ 1 ULP).
    ExactDeterministic,

    /// Bounded approximations of deterministic functions: polynomial
    /// approximations, fixed-iteration Newton-Raphson, lookup tables,
    /// rational approximations. Deterministic GPU-to-GPU; specified
    /// max_abs_error vs CPU oracle. **Requires `SoftAggregateGuard` on
    /// any sub-field that feeds a hard structural trigger from this
    /// class.** Not yet enabled in production.
    SoftDeterministic,

    /// Native / vendor-fast GPU math (`sin`, `cos`, `exp`, `log`,
    /// approximate `rsqrt`, etc.) where the WGSL backend may select
    /// implementation-defined intrinsics. Not deterministic across
    /// vendors. Not replay-safe under the current replay model. Not
    /// yet enabled in production.
    FastApproximate,

    /// Reference / testing class. Evaluated only on CPU (against the
    /// CPU oracle), never admitted to a production GPU session. Used
    /// for golden-value tests, developer experiments, and formulas
    /// not yet promoted to any GPU class.
    CpuOracleOnly,
}
```

### 1.1 ExactDeterministic — C-8 production baseline

Allowed opcodes (initial set; Codex extends the validator to whitelist
these at registration time):

| Opcode family | Operations |
|---|---|
| Constants & references | `LITERAL_F32`, `SLOT_VALUE(slot, col)`, `PARAM(idx)` |
| Arithmetic | `ADD`, `SUB`, `MUL`, `NEG` |
| Division | `DIV` (with explicit divisor-zero handling — see §1.1.1) |
| Min/max/clamp | `MIN`, `MAX`, `CLAMP_BOUNDED`, `CLAMP_FLOORED` |
| Comparison | `CMP_LT`, `CMP_LE`, `CMP_GT`, `CMP_GE`, `CMP_EQ` |
| Select | `SELECT(cond, a, b)` |
| Absolute value | `ABS` |
| Output | `RETURN_TOP` |

**Disallowed in `ExactDeterministic`:** `SIN`, `COS`, `TAN`, `EXP`, `LOG`,
`POW`, `SQRT`, `RSQRT`, `FRACT`, `MOD`, random/time intrinsics, any
opcode whose IEEE-754 behavior is implementation-defined across WGSL
backends, unbounded loops, recursion.

**Special:** `SQRT` is *probably* deterministic enough on modern GPUs
(IEEE-754 spec requires `sqrt` to be correctly rounded) but Codex must
not add it to `ExactDeterministic` without a documented vendor
cross-check. Default: leave `SQRT` in `SoftDeterministic` for now.

#### 1.1.1 Division and divisor-zero handling

Division is allowed only when:
1. The divisor is a literal constant that the validator can prove is
   non-zero, OR
2. The expression is wrapped in `SELECT(cmp(divisor != 0), divide_result, fallback)`.

This is a CPU-side validator rule; the shader does not detect zero
divisors at runtime. (Future SoftDeterministic / FastApproximate classes
may use unwrapped division if their tolerance contract covers it.)

### 1.2 SoftDeterministic — future-prep, not C-8 production

The class **exists in the registry and validator** at C-8, but no
production consumer admits it. Codex must add registration + matrix
tests that prove the class is recognized and gated:

```rust
#[test]
fn soft_deterministic_formula_can_register_but_not_feed_transfer() {
    let mut reg = EmlExpressionRegistry::new();
    let tree_id = reg.register(meta_with_class(EmlExecutionClass::SoftDeterministic))?;
    let err = reg.assert_consumer_admissible(tree_id, EmlConsumerKind::TransferConservation)
        .unwrap_err();
    assert!(matches!(err, EmlRegistryError::SoftClassNotAdmissibleForTransfer { .. }));
}
```

Future PRs that wish to enable a `SoftDeterministic` formula in
production must:
1. Add a dedicated opcode (e.g. `POLY_APPROX_EXP`) to the GPU
   interpreter under a feature flag.
2. Document its `max_abs_error` and reference inputs in the registry
   metadata.
3. Land registration + tolerance + CPU-oracle tests.
4. Open the consumer matrix narrowly (e.g. allow intensity but not
   transfer).

C-8 does NOT implement any SoftDeterministic opcodes.

### 1.3 FastApproximate — future-prep, not C-8 production

WGSL natively supports `sin`, `cos`, `exp`, `log`, `pow`, `sqrt`,
`inverseSqrt`, `tan`. These are implementation-defined; results may
vary across vendors and driver versions. **Replay safety is the
blocker:** the v7 replay model assumes deterministic GPU evaluation,
and a `FastApproximate` formula breaks that contract.

A future ADR amendment can license `FastApproximate` in two ways:
1. **Non-authoritative consumers** (visual fields, observability)
   where divergence has no game-state consequence.
2. **Deterministic-replay refactor**: record per-tick formula outputs
   in the compact log so replay reproduces the recorded path rather
   than re-evaluating. Separate Opus design.

C-8 does NOT implement any FastApproximate opcodes; the validator
rejects them from all production consumers.

### 1.4 CpuOracleOnly — testing / reference

For tests that compare GPU output against a richer CPU evaluator (e.g.
the workshop's `eml_phase5` harness). These trees are never uploaded
to a session's node buffer. The CPU oracle's evaluator may include
opcodes that no GPU class supports yet.

---

## 2. Consumer validation matrix

Registration-time rule, enforced by
`EmlExpressionRegistry::assert_consumer_admissible(tree_id, consumer)`:

| Consumer | `ExactDeterministic` | `SoftDeterministic` | `FastApproximate` | `CpuOracleOnly` |
|---|---|---|---|---|
| **TransferConservation** (`SubtractFromAllInputs`, `CrossingFormula`) | ✅ | ❌ (admission requires conservation-preserving proof; deferred indefinitely) | ❌ | ❌ |
| **HardThreshold** (FissionTrigger / FusionTrigger / PropertyExpiry / CapabilityUnlock) | ✅ | ⚠️ allowed only with `SoftAggregateGuard::Quantized` or `Hysteresis` per A-4 | ❌ | ❌ |
| **SoftThreshold** (AggregateAlert, observability) | ✅ | ✅ | ⚠️ allowed only with explicit non-authoritative flag | ❌ |
| **Intensity** (replaces legacy `intensity_update.wgsl`) | ✅ (C-8 baseline) | ⚠️ future flag, tolerance documented | ⚠️ future, non-authoritative + replay refactor | ❌ |
| **Emission** (count of discrete events) | ✅ | ✅ with documented tolerance (≤2 %) | ✅ with documented tolerance + non-authoritative + replay refactor | ❌ |
| **DebugOracle** (test paths only) | ✅ | ✅ | ✅ | ✅ |
| **Production GPU registration** (anything not above) | ✅ | ❌ | ❌ | ❌ (never admitted to a session) |

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EmlConsumerKind {
    TransferConservation,
    HardThreshold,
    SoftThreshold,
    Intensity,
    Emission,
    DebugOracle,
}

bitflags::bitflags! {
    /// Set of consumer kinds for which this formula has been pre-cleared.
    /// At registration the validator computes the maximum mask the formula
    /// could admit based on its execution class; each consumer site that
    /// references the tree narrows the mask further (e.g. an intensity
    /// site sets `Intensity` only).
    pub struct EmlConsumerMask: u32 {
        const TRANSFER_CONSERVATION = 1 << 0;
        const HARD_THRESHOLD        = 1 << 1;
        const SOFT_THRESHOLD        = 1 << 2;
        const INTENSITY             = 1 << 3;
        const EMISSION              = 1 << 4;
        const DEBUG_ORACLE          = 1 << 5;
    }
}
```

The matrix's invariant: **adding a row to a class' allowed column never
relaxes any other class's row.** Future expansions of `SoftDeterministic`
to allow `Intensity` do not retroactively admit it to
`TransferConservation`.

---

## 3. Registry metadata changes

Extend `crates/simthing-core/src/eml_registry.rs`:

```rust
/// Replaces the current EmlTreeMeta in C-8.
#[derive(Clone, Debug, PartialEq)]
pub struct EmlFormulaMeta {
    pub tree_id:         EmlTreeId,

    /// Static classification of the formula. Determined by analyzing
    /// opcodes used in the compiled tree (Codex's
    /// `classify_tree(nodes: &[EmlNodeGpu]) -> EmlExecutionClass`).
    pub execution_class: EmlExecutionClass,

    /// Pre-cleared consumer kinds based on `execution_class` plus any
    /// per-formula opt-ins. Default for `ExactDeterministic` is "all
    /// production consumers." Default for `SoftDeterministic` is
    /// "SoftThreshold + DebugOracle only" until a per-formula opt-in
    /// extends it.
    pub allowed_consumers: EmlConsumerMask,

    /// CPU-validated max absolute error vs the canonical CPU oracle.
    /// `None` for ExactDeterministic (bit-exact assumed). Required for
    /// SoftDeterministic. Optional for FastApproximate.
    pub max_abs_error: Option<f32>,

    /// True if the formula produces identical f32 bits across runs on
    /// the same GPU device. ExactDeterministic and SoftDeterministic
    /// are guaranteed deterministic; FastApproximate is not.
    pub deterministic_gpu: bool,

    /// True if any production consumer that registers this formula on
    /// a hard structural threshold must also attach a
    /// `SoftAggregateGuard` to the sub-field. Computed from class +
    /// reductions used in the formula.
    pub requires_guard_for_hard_threshold: bool,

    /// Node count and other structural facts.
    pub node_count:   u32,
    pub has_loops:    bool,
    pub has_recursion: bool,

    /// Optional name for diagnostics; not used for dispatch.
    pub display_name: String,
}
```

The legacy `EmlTreeMeta { node_count, has_transcendental, formula_class }`
becomes deprecated. Codex provides a `From<EmlTreeMeta> for EmlFormulaMeta`
that maps:
- `has_transcendental == false && node_count ≤ 16` → `ExactDeterministic`
- `has_transcendental == true` → `FastApproximate` (and reject at admission)

Existing call sites that build `EmlTreeMeta` are migrated to the new
struct in C-8a.

### 3.1 Validation extensions

```rust
impl EmlExpressionRegistry {
    /// Register a formula. Computes execution class, validates opcodes
    /// against the class' opcode set, computes default consumer mask.
    pub fn register(&mut self, meta: EmlFormulaMeta) -> Result<EmlTreeId, EmlRegistryError>;

    /// Verify the formula is admissible for the given consumer site.
    /// Called by every `AccumulatorOpBuilder::*_with_eml(...)` builder.
    pub fn assert_consumer_admissible(
        &self,
        tree_id: EmlTreeId,
        consumer: EmlConsumerKind,
    ) -> Result<(), EmlRegistryError>;

    /// Resolve a tree_id to its persistent node-buffer range index.
    /// Returns `None` if the tree has not been uploaded to the GPU
    /// session yet (the boundary sync upload sets this).
    pub fn tree_range_index(&self, tree_id: EmlTreeId) -> Option<u32>;
}

#[derive(Debug, thiserror::Error)]
pub enum EmlRegistryError {
    #[error("EML opcode {opcode:#x} not allowed in execution class {class:?}")]
    OpcodeNotAllowedInClass { opcode: u32, class: EmlExecutionClass },
    #[error("EML formula execution class {class:?} not admissible for consumer {consumer:?}")]
    ClassNotAdmissibleForConsumer { class: EmlExecutionClass, consumer: EmlConsumerKind },
    #[error("EML formula {tree_id:?} has not been uploaded to a session yet")]
    TreeNotUploaded { tree_id: EmlTreeId },
    #[error("EML formula registration would exceed MAX_EML_NODES")]
    NodeBudgetExceeded { requested: u32, max: u32 },
    // ... existing variants retained for backward compat
}
```

---

## 4. Persistent GPU node-buffer layout

EML programs live in two persistent GPU buffers, owned by either the
`AccumulatorOpSession` that evaluates them or (preferred) by
`WorldAccumulatorRuntime` so multiple EML-capable sessions can share.

**Recommendation: ownership lives on `WorldAccumulatorRuntime`.** The
runtime already coordinates per-family sessions; the EML program table
is a peer of `summary: Option<WorldSummaryRuntime>`. Specifically:

```rust
pub struct WorldAccumulatorRuntime {
    // ... existing fields ...

    /// C-8 EML program table. `None` until the first EML-capable
    /// session uploads a formula. Shared by all sessions that
    /// reference tree ranges (intensity, transfer, emission).
    pub eml: Option<EmlGpuProgramTable>,
}

pub struct EmlGpuProgramTable {
    pub node_buffer:  wgpu::Buffer,           // array<EmlNodeGpu>
    pub range_buffer: wgpu::Buffer,           // array<EmlTreeRangeGpu>
    pub generation:   u64,
    /// CPU mirror of range_buffer; mirrors are kept in sync at
    /// boundary sync time.
    pub ranges: Vec<EmlTreeRangeGpu>,
    pub node_capacity: u32,
    pub range_capacity: u32,
    pub node_used: u32,
    pub range_used: u32,
}
```

### 4.1 GPU node layout

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmlNodeGpu {
    /// Opcode discriminant. See §1.1 for the ExactDeterministic set;
    /// future PRs add opcodes here.
    pub opcode: u32,
    /// Flag bits. Reserved at C-8 (always 0); future PRs may stake
    /// bits for opcode-specific behavior.
    pub flags:  u32,
    /// Operand 0. For LITERAL_F32, `bitcast<f32>(a)` is the literal.
    /// For SLOT_VALUE, `a` is the slot index. For arithmetic, `a` is
    /// typically an operand index into the same node's compute output.
    pub a:      u32,
    /// Operand 1. Same conventions.
    pub b:      u32,
    /// Operand 2. Same conventions; used by SELECT (cond, a, b).
    pub c:      u32,
    /// Operand 3 / output index / reserved.
    pub d:      u32,
}
```

24 bytes per node. At 16 nodes (the current MAX), one formula is 384 B
on GPU. With a default capacity of 64 trees × 32 nodes (room to grow
from the current 16-node limit) the node buffer is 48 KB —
insignificant.

### 4.2 GPU range layout

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmlTreeRangeGpu {
    /// Start index into the node buffer.
    pub node_offset: u32,
    /// Number of nodes in this tree.
    pub node_count:  u32,
    /// Execution class as u32 (matches EmlExecutionClass discriminant).
    /// The shader can branch on this if it ever supports multiple
    /// classes in one kernel; C-8 baseline ignores it (only one class
    /// is admitted).
    pub execution_class: u32,
    /// Reserved.
    pub flags: u32,
}
```

16 bytes per range. 64 ranges = 1 KB. Trivial.

### 4.3 Why `tree_range_index` in the op, not `tree_id`

`tree_id` is stable across the session lifetime and is the API the
spec layer authors against. But the GPU can't search for it
efficiently. `tree_range_index` is the post-upload position in
`eml_tree_ranges[]` — a direct array index in the WGSL shader.

CPU resolves `tree_id → range_index` at registration time and stores
that in the `AccumulatorOpGpu.combine_a` field:

```rust
AccumulatorOp {
    combine: CombineFn::EvalEML { tree_id: 7 },
    // ... after encode, GPU op has combine_a = range_index_for_tree_7
}
```

Codex extends `encode_combine` to resolve `tree_id → range_index` via
the registry's `tree_range_index(tree_id)` method. If the tree hasn't
been uploaded yet, return `EncodeError::EmlTreeNotUploaded { tree_id }`.

### 4.4 Generation-based invalidation

Tree registration / replacement happens **only at boundary sync**. When
a tree is added, removed, or replaced:

1. `WorldAccumulatorRuntime::eml.generation += 1`.
2. Affected `AccumulatorOpGpu` registrations (those whose `combine` is
   `EvalEML { tree_id }`) are recomputed because their `range_index`
   may have changed.
3. Op buffers (intent, intensity, transfer, emission) are re-uploaded
   if their generation cached at last upload != current generation.

The simplest implementation: every EML-bearing session caches its
`eml_generation_at_upload`; the per-family `sync_*` path compares
against the current runtime generation and re-uploads if mismatched.

### 4.5 Capacity and growth

Initial capacity: 64 trees × 32 nodes/tree = 2048 nodes. Codex
implements a `grow_to` strategy that doubles capacity when full:

```rust
impl EmlGpuProgramTable {
    pub fn ensure_capacity(&mut self, ctx: &GpuContext, n_trees: u32, n_nodes: u32) {
        // ...
    }
}
```

Buffer growth recreates both buffers via `create_buffer` + copy from
old → new. Generation increments (existing data preserved bit-exactly
but consumers must re-resolve range indices).

### 4.6 Capacity exhaustion error

If `ensure_capacity` cannot grow (e.g. n_nodes > 2^16), return
`EmlRegistryError::NodeBudgetExceeded`. Production callers should
treat this as a session-open error, not a per-tick runtime error.

---

## 5. WGSL interpreter design

The C-8 interpreter is a **flat stack machine** over a node array.
Stack-machine bytecode is the standard pattern for embedded
expression evaluators: bounded stack, no recursion, deterministic
opcode dispatch.

### 5.1 Encoding policy: postfix (reverse Polish)

Trees are linearized into **postfix order**: child nodes appear in the
array before their parent operator. The interpreter walks the array
linearly, maintaining a small per-thread stack:

```wgsl
const EML_OP_LITERAL_F32: u32 = 0u;
const EML_OP_SLOT_VALUE:  u32 = 1u;
const EML_OP_PARAM:       u32 = 2u;
const EML_OP_ADD:         u32 = 10u;
const EML_OP_SUB:         u32 = 11u;
const EML_OP_MUL:         u32 = 12u;
const EML_OP_NEG:         u32 = 13u;
const EML_OP_DIV:         u32 = 14u;
const EML_OP_MIN:         u32 = 20u;
const EML_OP_MAX:         u32 = 21u;
const EML_OP_CLAMP_BOUNDED: u32 = 22u;
const EML_OP_CLAMP_FLOORED: u32 = 23u;
const EML_OP_ABS:         u32 = 24u;
const EML_OP_CMP_LT:      u32 = 30u;
const EML_OP_CMP_LE:      u32 = 31u;
const EML_OP_CMP_GT:      u32 = 32u;
const EML_OP_CMP_GE:      u32 = 33u;
const EML_OP_CMP_EQ:      u32 = 34u;
const EML_OP_SELECT:      u32 = 40u;
const EML_OP_RETURN_TOP:  u32 = 50u;

const EML_STACK_MAX: u32 = 16u;
```

Per-thread stack as a fixed-size local array; max depth = 16 (sized to
match the current node-count limit so a worst-case tree can be
evaluated without overflow). The validator at registration time
asserts that no formula's evaluation requires deeper than
`EML_STACK_MAX` slots.

### 5.2 Eval function

```wgsl
struct EmlEvalCtx {
    /// Range index into eml_tree_ranges[].
    range_idx: u32,
    /// SlotValue formulas read from this base slot (passed by the caller).
    eval_slot: u32,
    /// Caller-provided parameter slots (e.g. dt for intensity).
    param0: f32,
    param1: f32,
    param2: f32,
    param3: f32,
}

fn eml_eval(ctx: EmlEvalCtx) -> f32 {
    let range = eml_tree_ranges[ctx.range_idx];
    var stack: array<f32, 16>;
    var sp: u32 = 0u;

    for (var i: u32 = 0u; i < range.node_count; i = i + 1u) {
        let node = eml_nodes[range.node_offset + i];
        switch node.opcode {
            case EML_OP_LITERAL_F32: {
                stack[sp] = bitcast<f32>(node.a);
                sp = sp + 1u;
            }
            case EML_OP_SLOT_VALUE: {
                // node.a = column index; reads from ctx.eval_slot.
                stack[sp] = atomic_read_f32_at(linear_idx(ctx.eval_slot, node.a));
                sp = sp + 1u;
            }
            case EML_OP_PARAM: {
                // node.a = param index (0..3).
                let v = select(
                    select(ctx.param0, ctx.param1, node.a == 1u),
                    select(ctx.param2, ctx.param3, node.a == 3u),
                    node.a >= 2u
                );
                stack[sp] = v;
                sp = sp + 1u;
            }
            case EML_OP_ADD: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs + rhs; sp = sp - 1u;
            }
            case EML_OP_SUB: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs - rhs; sp = sp - 1u;
            }
            case EML_OP_MUL: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs * rhs; sp = sp - 1u;
            }
            case EML_OP_NEG: {
                stack[sp - 1u] = -stack[sp - 1u];
            }
            case EML_OP_DIV: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                // Validator guarantees rhs != 0 (either literal-nonzero
                // or wrapped in SELECT(cmp_ne_0, ..., fallback)).
                stack[sp - 2u] = lhs / rhs; sp = sp - 1u;
            }
            case EML_OP_MIN: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = min(lhs, rhs); sp = sp - 1u;
            }
            case EML_OP_MAX: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = max(lhs, rhs); sp = sp - 1u;
            }
            case EML_OP_CLAMP_BOUNDED: {
                // node.a = bitcast<u32>(lo), node.b = bitcast<u32>(hi)
                let v = stack[sp - 1u];
                stack[sp - 1u] = clamp(v, bitcast<f32>(node.a), bitcast<f32>(node.b));
            }
            case EML_OP_CLAMP_FLOORED: {
                let v = stack[sp - 1u];
                stack[sp - 1u] = max(v, bitcast<f32>(node.a));
            }
            case EML_OP_ABS: {
                stack[sp - 1u] = abs(stack[sp - 1u]);
            }
            case EML_OP_CMP_LT: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs < rhs); sp = sp - 1u;
            }
            case EML_OP_CMP_LE: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs <= rhs); sp = sp - 1u;
            }
            case EML_OP_CMP_GT: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs > rhs); sp = sp - 1u;
            }
            case EML_OP_CMP_GE: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs >= rhs); sp = sp - 1u;
            }
            case EML_OP_CMP_EQ: {
                let rhs = stack[sp - 1u]; let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs == rhs); sp = sp - 1u;
            }
            case EML_OP_SELECT: {
                // 3-arg: cond, true_val, false_val (stacked in that order)
                let f_val = stack[sp - 1u];
                let t_val = stack[sp - 2u];
                let cond  = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(f_val, t_val, cond);
                sp = sp - 2u;
            }
            case EML_OP_RETURN_TOP: {
                return stack[sp - 1u];
            }
            default: {
                // Validator guarantees no unknown opcodes reach here.
                return 0.0;
            }
        }
    }
    // Implicit return: top of stack at end of program.
    return stack[sp - 1u];
}
```

### 5.3 Why not tier 2/3/4 in C-8

The interpreter is "tier 1" — generic dispatch. Tiers 2/3/4 (compact
bytecode, specialized kernels per formula class, deterministic
approximations, fast approximate math) are explicit future work. The
node layout in §4.1 does not preclude any of them — the `flags` field
and `combine_b..d` of `AccumulatorOpGpu` are reserved for future
opcode-specific or class-specific dispatch hints.

### 5.4 Stack overflow / underflow safety

Validator-side (CPU):
- Compute the max stack depth reached by simulating execution at
  registration time. Reject formulas exceeding `EML_STACK_MAX`.
- Compute final stack depth at `RETURN_TOP`; must be ≥ 1.
- Reject programs that finish with stack underflow.

Shader-side:
- No runtime checks. Validator's CPU-side analysis is the gate.
- A future debug-build variant could `debug_assert!(sp < EML_STACK_MAX)`
  on host re-evaluation against the CPU oracle.

---

## 6. Intensity migration mapping

The legacy `intensity_update.wgsl` implements a hard-coded formula:

```
if |velocity| > velocity_threshold:
    intensity += build_coefficient * |velocity| * dt
else:
    intensity -= decay_coefficient * intensity * dt
intensity = clamp(intensity, 0.0, 1.0)
```

This is the canonical EML intensity formula. Codex compiles it into a
postfix EML program (~16 nodes) at session open:

```
// param0 = dt
// slot = current slot (intensity sub-field slot)
// col 1 (velocity) and col 2 (intensity) used as SLOT_VALUE inputs

PARAM 0                                // dt
SLOT_VALUE col=1 (velocity)            // velocity
ABS                                    // |velocity|
LITERAL bitcast<u32>(velocity_threshold)
CMP_GT                                 // |vel| > threshold
LITERAL bitcast<u32>(build_coeff)      
SLOT_VALUE col=1
ABS
PARAM 0
MUL MUL                                // build_coeff * |vel| * dt
SLOT_VALUE col=2 (intensity)
ADD                                    // intensity + build_coeff * |vel| * dt
SLOT_VALUE col=2
LITERAL bitcast<u32>(decay_coeff)
MUL
PARAM 0
MUL
SLOT_VALUE col=2
SUB                                    // intensity - decay * intensity * dt
SELECT                                 // if |vel| > threshold: build branch else decay branch
LITERAL bitcast<u32>(0.0)
LITERAL bitcast<u32>(1.0)
CLAMP_BOUNDED                          // clamp to [0, 1]
RETURN_TOP
```

(Exact node count and order are Codex's to lay out; this sketch
illustrates the formula shape.)

**Registration shape** for intensity:

```rust
AccumulatorOp {
    source:  SourceSpec::Constant(0.0),     // SourceSpec ignored — EvalEML reads via SLOT_VALUE opcodes
    combine: CombineFn::EvalEML { tree_id }, 
    gate:    GateSpec::OrderBand(intensity_band),
    scale:   ScaleSpec::Identity,
    consume: ConsumeMode::ResetTarget,       // overwrite intensity column
    targets: vec![(slot, intensity_col)],
}
```

One `AccumulatorOp` per slot per intensity-bearing property — same
shape as legacy intensity (which dispatches one thread per slot per
property).

### 6.1 Parity gate

Bit-exact CPU↔GPU. The CPU oracle re-implements the EML interpreter
in Rust (it's a stack machine; trivial port). The intensity migration
is bit-exact because every op is `ExactDeterministic`.

### 6.2 Order in the pipeline

Intensity executes after velocity (which sets the `Velocity` column)
and before overlay application (which may modify intensity-relevant
columns). Codex routes the intensity dispatch to an `OrderBand` between
velocity and overlay bands.

### 6.3 Adapter from `IntensityBehavior` to EML

Codex provides:

```rust
impl IntensityBehavior {
    /// Compile this behavior into an EML formula meta + node list.
    /// Returns the meta to register and the nodes to upload to the
    /// runtime's eml.node_buffer.
    pub fn compile_to_eml(&self) -> (EmlFormulaMeta, Vec<EmlNodeGpu>);
}
```

Existing call sites that build `IntensityBehavior` are not changed;
the session-open path calls `compile_to_eml` and registers per behavior.

---

## 7. Transfer substrate

Transfer must move resources from N source cells to M target cells
exactly (no f32 accumulation drift; no negative balances; no
double-counting).

### 7.1 The three primitives

| Combine | Consume | Use case |
|---|---|---|
| `Identity` | `SubtractFromSource` | Single-source single-target transfer (faction pool → factory queue) |
| `MinAcrossInputs` | `SubtractFromAllInputs` | Conjunctive recipe (5 iron + 3 energy + 2 labor → 1 unit) |
| `CrossingFormula { unit_cost }` | `SubtractFromSource` + `EmitEvent` | Debt-band emission (queue crosses → emit floor(queue/unit_cost) units, decrement queue) |

### 7.2 Auxiliary input-list buffer

`AccumulatorOpGpu` has space for one source slot/col and four target
slot/col pairs. Conjunctive recipes need up to 4 input cells per
recipe. Two options:

**(a) Use the four target slots for inputs** (reinterpret) — fragile;
collides with multi-target writes; rejected.

**(b) Add an auxiliary `InputList` buffer.** New persistent buffer on
the session:

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorInputGpu {
    pub slot: u32,
    pub col:  u32,
    /// For conjunctive recipes: per-input unit_cost. Otherwise 1.0.
    pub unit_cost: f32,
    pub flags: u32,   // reserved
}

pub struct AccumulatorInputListTable {
    pub buffer: wgpu::Buffer,
    pub entries: Vec<AccumulatorInputGpu>,
    pub generation: u64,
}
```

`AccumulatorOpGpu` encodes the input-list slice via:
- `source_kind = SOURCE_INPUT_LIST` (new variant)
- `source_slot = input_list_offset`
- `source_count = input_list_count`

This is the same pattern as `EmlGpuProgramTable` — persistent table,
generation-based invalidation, CPU-side range resolution at registration
time.

**Recommend option (b).** Codex implements `AccumulatorInputListTable`
on `WorldAccumulatorRuntime` (sibling of `eml`).

### 7.3 `MinAcrossInputs` + `SubtractFromAllInputs` kernel

Pseudo-WGSL:

```wgsl
fn execute_conjunctive(op: AccumulatorOpGpu) {
    if op.source_kind != SOURCE_INPUT_LIST { return; }

    let n = op.source_count;
    let base = op.source_slot;  // offset into input_list buffer

    // Compute floor(min over inputs of (input_value / unit_cost)).
    var min_units: f32 = 1e30;
    for (var i: u32 = 0u; i < n; i = i + 1u) {
        let input = input_list[base + i];
        let v = atomic_read_f32_at(linear_idx(input.slot, input.col));
        let units = v / input.unit_cost;
        min_units = min(min_units, units);
    }
    let emit_count = floor(max(min_units, 0.0));

    if emit_count == 0.0 { return; }

    // Subtract emit_count * unit_cost from each input.
    for (var i: u32 = 0u; i < n; i = i + 1u) {
        let input = input_list[base + i];
        let idx = linear_idx(input.slot, input.col);
        let old = atomic_read_f32_at(idx);
        // Single-writer invariant: only one conjunctive op writes to
        // any (slot, col) per band. Same OrderBand discipline as
        // overlay OrderBands. Plain atomic load + store, no CAS.
        atomic_store_f32_at(idx, old - emit_count * input.unit_cost);
    }

    // Add emit_count to target (per ConsumeMode::EmitEvent rules).
    if op.consume == CONSUME_EMIT_EVENT {
        let out_idx = atomicAdd(&emission_count, 1u);
        if out_idx < tick_params.emission_capacity {
            emissions[out_idx].reg_idx = op_idx;
            emissions[out_idx].emit_count = u32(emit_count);
        }
    }
    // Other consume modes for transfer: see §7.4.
}
```

### 7.4 Conservation invariants

Codex must add registration-time and test-time invariants:

| Invariant | Check |
|---|---|
| Each input slot/col appears at most once per band | Validator like C-4 overlay OrderBand |
| `unit_cost > 0` for all conjunctive inputs | Registration time |
| `EmlExecutionClass == ExactDeterministic` when transfer uses EML | Consumer validation matrix (§2) |
| Total subtracted from inputs = `emit_count * Σ(unit_cost_i)` | Test invariant — assert per-tick conservation |
| No input balance goes negative | Validator at planning time AND runtime clamp in kernel |

The "no negative" runtime clamp: in `execute_conjunctive`, before
subtracting, take `min(old, emit_count * input.unit_cost)` so the
final value is `max(0, old - debit)`. Then conservation is checked at
test time over many ticks; if the clamp ever activates, the planner
over-credited and a fission test catches it.

### 7.5 Transfer must reject Soft/Fast classes

Per §2, transfer admits only `ExactDeterministic`. The encoder rejects:

```rust
fn encode_transfer_op(op: &AccumulatorOp, registry: &EmlExpressionRegistry)
    -> Result<AccumulatorOpGpu, EncodeError>
{
    if let CombineFn::EvalEML { tree_id } = op.combine {
        registry.assert_consumer_admissible(tree_id, EmlConsumerKind::TransferConservation)?;
    }
    // ...
}
```

This compiles to the registry error `ClassNotAdmissibleForConsumer` if
a designer tries to attach a Soft formula to a transfer registration.

---

## 8. Emission substrate

Emission is the **non-conservation** sibling of transfer: it records
that an event happened, not what was moved. Tolerance is admissible
here because emission counts are probabilistic / discretized in many
cases.

### 8.1 Why tolerance is allowed for emission

A debt-band emission `floor(queue_value / unit_cost)` is exact for
`ExactDeterministic` formulas. But a future SoftDeterministic formula
that computes `emit_count` from a soft aggregate (e.g. "factory
efficiency × labor pool") inherits the soft tolerance — and that's
acceptable because emission count is informational, not
conservation-critical.

The production-plan note (§"Emission tolerance: ≤2 % vs CPU
reference") applies to *future SoftDeterministic formulas only*.
ExactDeterministic emission formulas are bit-exact.

### 8.2 Tolerance isolation

The matrix in §2 prevents tolerance leakage:

- `TransferConservation` ❌ Soft/Fast — conservation is exact.
- `Emission` ✅ Soft (≤2 %) — non-conservation.
- `HardThreshold` ⚠️ Soft only with `SoftAggregateGuard`.

So a formula could be Soft, admitted for Emission, and rejected from
Transfer. A modder cannot "accidentally" use it for transfer because
the encoder validation rejects.

### 8.3 EmissionRecordGpu unchanged

`EmissionRecord { reg_idx, emit_count }` is the existing record (from
C-1/C-3). C-8 reuses it for transfer-emit and conjunctive-emit. No
schema change.

### 8.4 Compact emission buffer overflow

Existing `emission_capacity` policy applies: if `emission_count >
capacity`, readback returns `EmissionOverflow` and CI fails. Codex
must size the emission buffer for the worst-case conjunctive recipe
fan-out (e.g. 64 KB at default).

---

## 9. Pipeline integration

Per the handoff target shape:

```
optional Accumulator intent
snapshot
optional Accumulator velocity
Accumulator EvalEML / intensity / transfer / emission   ← C-8 lands here
Accumulator overlay
Accumulator reduction
threshold
world summary
one command encoder / one submit
```

### 9.1 New PipelineFlags

```rust
pub struct PipelineFlags {
    // ... existing C-1..C-7 flags ...
    pub use_accumulator_eml:       bool,   // EML infrastructure on
    pub use_accumulator_intensity: bool,   // intensity migration (requires use_accumulator_eml)
    pub use_accumulator_transfer:  bool,   // transfer substrate (requires use_accumulator_eml)
    pub use_accumulator_emission:  bool,   // emission substrate (requires use_accumulator_eml)
}
```

All default `false`. `use_accumulator_eml` is the gate for the
runtime's `eml: Option<EmlGpuProgramTable>` initialization. The other
three are per-family flags that can be set independently once `_eml`
is on.

### 9.2 Order in the encoder

After snapshot/velocity, before overlay:

1. **Intensity** (if `use_accumulator_intensity`) — one band, all
   intensity ops dispatched.
2. **Transfer** (if `use_accumulator_transfer`) — one band per
   transfer order (recipe → emission record). Typically a single band
   suffices.
3. **Emission** events flush to the compact buffer as a side effect
   of transfer or intensity registrations that use `CONSUME_EMIT_EVENT`.

Each is encoded into the **same command buffer** as the rest of the
tick (C-1 pattern).

### 9.3 No CPU-mediated fallback

C-8 production paths must NEVER call any CPU evaluator for intensity,
transfer, or emission. The CPU oracle in
`crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs` (EML evaluator
mirror) is for tests and the legacy oracle harness only.

---

## 10. Staged delivery

### C-8a — EML infrastructure (no production consumers)

**Scope:** Registry refactor (`EmlExecutionClass`, `EmlFormulaMeta`,
`EmlConsumerMask`, `assert_consumer_admissible`), persistent
`EmlGpuProgramTable` on `WorldAccumulatorRuntime`, `EmlNodeGpu` /
`EmlTreeRangeGpu` types, WGSL interpreter, CPU oracle mirror, tree
generation protocol, capacity growth. No consumer yet — flag
`use_accumulator_eml` gates the table's existence. Validator + tests
prove future Soft/Fast formulas register but cannot reach production.

**Files (Codex 5.5):**

```
crates/simthing-core/src/eml_registry.rs        — refactor to EmlFormulaMeta + classes
crates/simthing-core/src/accumulator_op.rs      — EmlConsumerKind, EmlConsumerMask
crates/simthing-gpu/src/accumulator_op/types.rs — EmlNodeGpu, EmlTreeRangeGpu, opcode constants
crates/simthing-gpu/src/accumulator_op/eml_program_table.rs  ← NEW
crates/simthing-gpu/src/accumulator_op/runtime.rs — add `eml: Option<EmlGpuProgramTable>`
crates/simthing-gpu/src/accumulator_op/encode.rs — tree_id → range_index resolution
crates/simthing-gpu/src/shaders/accumulator_op.wgsl — interpreter in §5
crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs — CPU EML evaluator mirror
crates/simthing-sim/src/boundary.rs — PipelineFlags::use_accumulator_eml
crates/simthing-sim/src/gpu_sync.rs — boundary-sync EML tree upload + generation check
docs/design_v7.md, docs/accumulator_op_v2_production_plan.md, etc. — §15
```

**Tests:**

```
c8a_eml_tree_table_upload_roundtrip
c8a_eval_eml_exact_gpu_matches_cpu_oracle_bit_exact
c8a_multiple_eml_trees_one_dispatch
c8a_tree_generation_reupload_invalidates_ops
c8a_no_mid_tick_tree_mutation
c8a_eml_execution_class_validation_rejects_transcendental_in_exact
c8a_eml_execution_class_validation_rejects_fast_approx_in_exact_consumer
c8a_eml_execution_class_validation_allows_exact_in_transfer
c8a_eml_execution_class_validation_requires_guard_for_soft_hard_threshold
c8a_soft_deterministic_formula_can_register_but_not_feed_transfer
c8a_fast_approx_formula_rejected_from_hard_threshold
c8a_cpu_oracle_only_formula_rejected_from_production_gpu_registration
c8a_node_buffer_capacity_growth_preserves_existing_trees
c8a_stack_depth_validator_rejects_overflowing_formula
c8a_persistent_node_buffer_no_per_dispatch_upload
```

### C-8b — Intensity migration

**Scope:** Compile `IntensityBehavior` → EML formula at session open;
register one intensity op per slot per intensity-bearing property;
add intensity band to the dispatcher; flag-on path replaces legacy
`intensity_update.wgsl` dispatch. Legacy intensity stays as oracle /
flag-off path until S-2.

**Tests:**

```
c8b_intensity_legacy_vs_accumulator_bit_exact
c8b_intensity_exact_deterministic_only_in_baseline
c8b_intensity_no_cpu_mediated_production_path
c8b_intensity_compiled_formula_matches_legacy_for_default_behavior
c8b_intensity_persistent_node_buffer_no_per_dispatch_upload
```

### C-8c — Transfer substrate

**Scope:** `AccumulatorInputListTable` persistent buffer;
`MinAcrossInputs` + `SubtractFromAllInputs` kernel paths;
single-channel `Identity + SubtractFromSource` for simple transfers
(no input list needed); conservation invariants + tests. Encoder
rejects Soft/Fast EML formulas from transfer registrations.

**Tests:**

```
c8c_transfer_single_factory_conserves_exactly
c8c_transfer_1000_factories_3_channels_100_ticks_conserves_exactly
c8c_transfer_insufficient_inputs_min_across_inputs
c8c_transfer_subtract_from_all_inputs_exact
c8c_transfer_no_negative_balances
c8c_transfer_rejects_soft_or_fast_eml_formula
c8c_transfer_input_list_capacity_growth
c8c_transfer_input_list_generation_invalidates_ops
```

### C-8d — Emission substrate

**Scope:** `CrossingFormula` + `EmitEvent` consume; conjunctive
emission via `MinAcrossInputs + EmitEvent`; compact `EmissionRecord`
buffer (already exists from C-1); compaction tests; tolerance policy
explicit for future Soft formulas.

**Tests:**

```
c8d_conjunctive_emission_cpu_reference_within_2_percent
c8d_conjunctive_emission_exact_when_exact_deterministic_formula
c8d_emission_record_compaction_capacity
c8d_emission_overflow_behavior
c8d_emission_tolerance_does_not_apply_to_transfer
c8d_crossing_formula_debt_band_emission
```

### Combined acceptance

```
c8_combined_c1_c2_c4_s4_c7_c8_all_flags_on
  — intent + velocity + EML/intensity + overlay + reduction + threshold
    + summary all GPU-resident; no CPU-mediated production path
c8_combined_no_cpu_mediated_eml_evaluation
c8_combined_persistent_node_buffer_survives_100_ticks
c8_combined_tree_change_at_boundary_invalidates_only_affected_ops
```

---

## 11. Implementer boundaries (Composer 2.5 / Codex 5.5)

For C-8a (registry + infrastructure): **Codex 5.5** mechanical.
For C-8b (intensity): **Codex 5.5** mechanical (legacy → EML formula
compilation is a defined adapter).
For C-8c (transfer): **Composer 2.5** — the conservation invariants
and the input-list table benefit from architectural judgment that
Codex may not have context for.
For C-8d (emission): **Codex 5.5** mechanical (reuses C-8c patterns).

### 11.1 Pivot posture (every C-8 PR)

```
Pivot posture:
  AccumulatorOp EML / intensity / transfer / emission is the production direction.
  Legacy intensity_update.wgsl is oracle/flag-off only after C-8b.
  Legacy CPU evaluators are test-only.
  No CPU-mediated production evaluation.
  Persistent GPU buffers; no per-dispatch upload.

Sunset target:
  S-2 — delete legacy intensity_update.wgsl after C-8b default-on + 7 days CI green.
  (Transfer and emission are new capabilities; no legacy code to sunset.)

Legacy interaction allowed:
  Oracle / parity tests only.

Legacy interaction forbidden:
  no new features · no optimization · no semantic expansion.
```

### 11.2 Stop-and-report (per handoff §"Stop-and-report conditions")

The C-8 series must stop and request Opus review if:

1. Persistent EML node buffer cannot be implemented without per-dispatch upload.
2. Tree lookup requires unbounded WGSL search.
3. Execution-class validation cannot be enforced at registration time.
4. Transfer conservation cannot be exact for `ExactDeterministic`.
5. `SubtractFromAllInputs` cannot be represented without a new auxiliary input buffer (this memo accepts this — the input-list table is the solution; but if implementation hits a different snag, escalate).
6. Emission tolerance would leak into transfer or hard-trigger semantics.
7. Any production path would evaluate EML / intensity / transfer / emission on CPU.
8. C-8 requires changing C-7 velocity or S-4 reduction semantics.
9. Future-prep metadata requires breaking `AccumulatorOpGpu` reformat.

---

## 12. Sunset implications

### S-2: intensity legacy deletion

After C-8b defaults on + 7 days CI green:

```
S-2 PR checklist:
- [ ] use_accumulator_intensity defaulted to `true`
- [ ] CI green at flag=on for 7+ days
- [ ] All parity tests pass with flag=on
- [ ] Delete:
      - crates/simthing-gpu/src/shaders/intensity_update.wgsl
      - intensity_update pipeline + bind group layout
      - WorldGpuState intensity_params buffer (if not used elsewhere)
      - gpu_sync intensity upload branch
- [ ] Update design_v7.md §4.3 to remove the intensity entry
- [ ] Add SUPERSEDED annotation to design_v6.md §10 intensity entry
```

### No S-phase for transfer/emission (new capability)

Transfer and emission have no legacy code to delete — these are new
production capabilities. There is no sunset PR; the first time they
default on, they are the only path.

---

## 13. Active doc updates required by this PR

The same PR that lands this memo must update active docs so future
agents see the execution-class framing rather than "EML is
permanently exact-only." Specific edits:

### 13.1 `docs/design_v7.md`

- Line 100 (`EvalEML†` in the constitution table): footnote
  changes from "EvalEML: requires whitelist entry; no transcendentals;
  ≤16 nodes" to reference the execution-class framing.
- Line 112 (footnote): replace with the execution-class language from
  §1 above.
- Line 234 (Pass 2 entry, intensity migration target): note that
  intensity uses `ExactDeterministic` EML in C-8 baseline.
- Lines 503–516 (Stage 3 EML section): replace the permanent "no
  transcendentals" framing with the C-8 baseline + future-prep
  language.
- Line 559 (invariant table): change "EvalEML requires whitelist
  entry" to "EvalEML requires execution-class registration and
  consumer admissibility check" — and note that the
  `ExactDeterministic` whitelist is the C-8 production subset, not the
  permanent ceiling.

### 13.2 `docs/adr_accumulator_op_v2.md`

- Line 44 (workshop evidence table): unchanged (factual workshop
  outcome).
- Line 105 (CombineFn enum sketch): comment changes to "Phase 5 EML;
  validated via execution-class registration (see C-8 design memo)."
- Lines 163–173 (EML expression policy section): rewrite from
  "validated for formulas meeting all of: no transcendentals, ≤16
  nodes, whitelisted formula class" to the execution-class framing.
  Keep the `ExactDeterministic` constraints intact as the C-8 baseline
  but frame them as the C-8 admission policy, not a permanent ADR
  rule.
- Line 272 (invariant table): update the row.

### 13.3 `docs/accumulator_op_v2_production_plan.md`

- Lines 76, 95 (`EmlTreeMeta` sketch): replace with `EmlFormulaMeta`
  sketch referencing this memo.
- Lines 86–101 (A-3 entry): add a note that A-3's `EmlTreeMeta` is
  refactored to `EmlFormulaMeta` in C-8a.
- Line 562 (C-8 entry, "EML intensity bit-exact"): keep the assertion
  for the `ExactDeterministic` baseline; add a sub-bullet that future
  Soft/Fast formulas may have documented tolerance.
- C-8 entry status (§ around line 530): "Design landed; staged
  implementation (C-8a → C-8d) follows."

### 13.4 `docs/workshop/workshop_current_state.md`

- §2 (Open migration work): mark C-8 as design-landed; staged
  implementation pending.
- §6 (Active workshop documents): add this memo to the table.
- §10 (Migration handoff template): no changes; the template already
  matches the C-8 pivot-forward posture.

### 13.5 `docs/todo.md`

- Update the AccumulatorOp v2 progress table: add C-8 design landed
  row.
- Add the next Opus gate: D-1 (hot-pool allocator design) — already
  in the production plan.

### 13.6 `docs/worklog.md`

- Append a "C-8 design memo landed" entry per the standard
  session-by-session format.

### 13.7 `crates/simthing-core/src/eml_registry.rs` doc comments

- `EmlTreeMeta` type doc: add a `// Deprecated by C-8 — see
  EmlFormulaMeta.` annotation.
- `MAX_EML_TREE_NODES` doc: clarify this is the
  `ExactDeterministic` C-8 baseline limit; future classes may
  increase the budget.
- `WHITELISTED_FORMULA_CLASSES`: rename or wrap in deprecation
  comments; the new gating mechanism is `EmlExecutionClass +
  EmlConsumerMask`.

(These code-side annotations are doc-comment changes, not code
behavior changes — the actual refactor lands in C-8a.)

---

## 14. Non-goals (explicit)

C-8 design does NOT:

- Implement any `SoftDeterministic` opcodes (future PR).
- Implement any `FastApproximate` opcodes (future PR + ADR amendment).
- Implement specialized hot-formula kernels (Tier 2/3/4) — future
  optimization PRs.
- Change C-7 velocity integration or S-4 reduction semantics.
- Modify the A-4 `SoftAggregateGuard` or its enforcement.
- Modify the B-4 `SlotSummaryGpu` shape.
- Default any C-8 flag to `true` (separate PR per family).
- Touch the v6.5 spec layer / `simthing-driver` / `simthing-spec`
  contracts.
- Add CPU-mediated production paths for intensity / transfer /
  emission.

---

## 15. Sign-off checklist

- [x] (1) `EmlExecutionClass` defined (§1)
- [x] (2) Consumer validation matrix defined (§2)
- [x] (3) Registry metadata changes specified (§3)
- [x] (4) Persistent GPU node buffer layout (§4)
- [x] (5) Tree-range mapping + bounded WGSL lookup (§4.3)
- [x] (6) Tree generation + invalidation protocol (§4.4)
- [x] (7) WGSL EvalEML interpreter design (§5)
- [x] (8) Intensity migration mapping (§6)
- [x] (9) Transfer substrate + exact conservation policy (§7)
- [x] (10) Emission mapping + tolerance rationale isolated from
      conservation (§8)
- [x] (11) Feature flag / staging plan (§9, §10)
- [x] (12) Tests with named cases (§10 per stage + §10 combined)
- [x] (13) Implementation boundaries (§11)
- [x] (14) Legacy intensity sunset path (§12)
- [x] (15) Required active-doc updates listed (§13)
- [x] Acceptance criteria from handoff satisfied
- [ ] Human sign-off on the design (this PR requests it)

---

## References

- `docs/adr_accumulator_op_v2.md` — ADR; framing updates per §13.2
- `docs/design_v7.md` §2, §4, §6, §7 — framing updates per §13.1
- `docs/accumulator_op_v2_production_plan.md` PR A-3 + PR C-8 —
  framing updates per §13.3
- `docs/workshop/pivot_forward_implementation_policy.md` —
  posture authority
- `docs/workshop/workshop_current_state.md` — routing
- `docs/workshop/soft_aggregate_tolerance_audit.md` (A-4) — guard policy
- `docs/workshop/c4_overlay_orderband_compiler_design.md` (C-4) —
  OrderBand single-writer pattern reused here
- `docs/workshop/c5_weighted_mean_reduction_design.md` (C-5) — soft
  aggregate tolerance pattern
- `crates/simthing-core/src/eml_registry.rs` — current `EmlTreeMeta` /
  `EmlExpressionRegistry` (refactored in C-8a)
- `crates/simthing-gpu/src/shaders/intensity_update.wgsl` — legacy
  intensity (oracle after C-8b)
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` — C-8 target
  kernel (extended in C-8a)
- `crates/simthing-gpu/src/accumulator_op/runtime.rs` —
  `WorldAccumulatorRuntime` (extended in C-8a)
