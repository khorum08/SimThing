# AccumulatorOp v2 — Production Plan

> **Status:** Active planning document. Companion to `adr_accumulator_op_v2.md`.
> The PR ladder below is the authoritative sequencing. Phases A–D are not
> calendar quarters; they are completion-gated sequences. A phase does not
> begin until all PRs in the prior phase are green and merged.

---

## Model assignment philosophy

This project has three agent tiers with different strengths:

**Opus** — deep architectural reasoning, novel design decisions, resolving
semantic ambiguity, writing analysis documents, reviewing correctness contracts.
Best for: open questions with no obvious answer, design decisions with
irreversible consequences, operations whose correctness is non-trivial.

**Composer 2.5** — implementation of well-specified tasks in existing codebases,
refactoring, adding new passes alongside existing ones, wiring new types into
established patterns. Best for: PRs with complete specs, bounded scope, clear
acceptance criteria.

**Codex 5.5** — high-throughput mechanical implementation: struct definitions,
serde derives, test boilerplate, report formatting, fixture files, doc
comments. Best for: repetitive well-typed work, test scaffolding, any PR
where the hard decisions are already made.

The ladder below assigns each PR to one of these. **Opus PRs are marked
explicitly because they require human + Opus review before merging.** Other
PRs are Composer or Codex by default and may be reviewed by a human alone.

---

## Phase A — ADR, invariants, and skeleton (no GPU changes)

All PRs in Phase A are documentation or type-system additions only. No WGSL
changes. No existing tests break.

### PR A-1 — Merge the ADR and update invariants.md

**Model:** Codex 5.5  
**Scope:** Add `docs/adr_accumulator_op_v2.md` to the repo. Append the six
new invariant rows from the ADR to `docs/invariants.md`. Add a one-line
entry to the design doc map in `design_v6.5.md` pointing to the ADR.  
**Acceptance:** CI green. No other files touched.  
**Gate:** Human review of the ADR text before merge.

---

### PR A-2 — `CombineFn` enum and `AccumulatorOp` struct (types only, no dispatch)

**Model:** Codex 5.5  
**Scope:** Add to `simthing-core`:
- `CombineFn` enum with all 12 variants from the ADR
- `GateSpec` enum (extends current `ThresholdDirection` + `LifecycleActive` +
  `OrderBand`)
- `ConsumeMode` enum (superset of the existing `ConsumeMode` from PR W-2;
  consolidate into one type)
- `AccumulatorOp` struct with `source`, `combine`, `gate`, `scale`, `consume`,
  `targets` fields
- `SourceSpec` struct with `kind`, `inputs` (fixed array + range variant),
  `weight_col`
- `InputSpec` struct (already in the workshop crate; promote to core)

All `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]`. No GPU
buffer layout or `#[repr(C)]` yet — that lands in Phase B.

Add a `validate()` method that enforces:
- `WeightedMean` combine requires `source.weight_col.is_some()`
- `EvalEML` combine requires the tree_id is <= `MAX_EML_TREE_ID`
- `SlotRange` source requires `count > 0`
- `targets` has at least one non-zero entry

**Test:** Unit tests covering `validate()` edge cases. Serde roundtrip for
every variant.  
**Acceptance:** CI green. `simthing-gpu` and `simthing-sim` are untouched.

---

### PR A-3 — `EmlExpressionRegistry` with whitelist enforcement

**Model:** Composer 2.5  
**Scope:** Add `simthing-core::eml_registry::EmlExpressionRegistry`:
- `register(tree_id: EmlTreeId, meta: EmlTreeMeta) -> Result<()>` — registers a
  tree, validates against the whitelist policy from the ADR (no
  transcendentals, ≤16 nodes, formula class whitelisted)
- `assert_whitelisted(tree_id) -> Result<()>` — called at
  AccumulatorOp registration time when combine is `EvalEML`
- `EmlTreeMeta { node_count: u32, has_transcendental: bool, formula_class: String }`

The whitelist formula classes for this ADR: `["intensity_update",
"emission_formula", "conversion_rate"]`. Any other class requires a separate
ADR amendment.

**Test:** Register valid trees, reject trees with transcendentals, reject trees
over the node limit, reject unknown formula classes.  
**Acceptance:** CI green.

---

### ⚠️ PR A-4 — Opus review: soft-aggregate tolerance policy formalization

**Model:** Opus (review and document), Codex 5.5 (implementation)  
**Why Opus:** The tolerance policy for WeightedMean has a subtle boundary
condition that the workshop data surfaces but doesn't fully resolve. The
workshop showed `max_abs_error ~3e-6` for both current AND pivot paths vs the
CPU oracle. This means the current pipeline already uses loose-tolerance
semantics for WeightedMean in production without formally acknowledging it.
Opus should answer: are there any existing system paths that currently read
a WeightedMean output and use it as a hard trigger? If yes, those paths are
already incorrect under the current architecture, and the ADR is not
introducing a new risk — it's surfacing an existing one.

**Opus task:** Review `boundary.rs`, `threshold_registry.rs`, and
`overlay_lifecycle.rs` for any code path that reads a reduced value and
uses it in a structural decision (fission trigger, overlay lifecycle,
property expiry). Cross-reference against the DimensionRegistry to find
which properties use `ReductionRule::WeightedMean`. Produce a two-page
analysis: (a) existing production exposure, (b) recommended guard pattern
(quantization or hysteresis), (c) the exact type signature for the guard.

**Implementation task (Codex 5.5 after Opus analysis):**
- Add `SoftAggregateGuard` enum: `Unguarded | Quantized { step: f32 } |
  Hysteresis { band: f32, last_committed: f32 }`
- Add `SubFieldSpec::soft_aggregate_guard: Option<SoftAggregateGuard>`
- Add `assert_no_hard_trigger_on_soft_aggregate()` validation at threshold
  registration time — panics if a `WeightedMean`-reduced column is registered
  as a fission or structural threshold without a guard

**Acceptance:** Opus analysis document committed to `docs/workshop/`. CI green.
Human sign-off on the guard pattern before merge.

---

## Phase B — Core runtime skeleton (persistent buffers, no migration)

Phase B introduces the AccumulatorOp runtime alongside the existing pipeline.
Nothing migrates yet. The two systems run in parallel.

### PR B-1 — `AccumulatorOpSession`: persistent buffer ownership

**Model:** Composer 2.5  
**Scope:** New crate `simthing-accum` (or module in `simthing-gpu`). Contains:
- `AccumulatorOpSession { device: Arc<Device>, queue: Arc<Queue>, ... }`
- Persistent GPU buffers: `op_buffer` (AccumulatorOp registrations), 
  `values_buffer` (slot × dims f32 matrix, atomic<i32>), `summary_buffer`
  (per-slot checksum), `emission_buffer` (compact EmissionRecord array),
  `emission_count` (atomic<u32>)
- `AccumulatorOpSession::new(device, queue, n_slots, n_dims) -> Result<Self>`
- `AccumulatorOpSession::upload_ops(ops: &[AccumulatorOp]) -> Result<()>` — 
  uploads the op buffer; marks dirty
- `AccumulatorOpSession::tick(band: u32) -> Result<()>` — dispatches Pass B
  for a single OrderBand
- `AccumulatorOpSession::readback_summary() -> Result<Vec<SlotSummary>>` —
  reads the summary buffer (default production path)
- `AccumulatorOpSession::readback_emissions() -> Result<Vec<EmissionRecord>>` —
  reads the compact emission buffer after `emission_count` is known
- `AccumulatorOpSession::readback_full() -> Result<Vec<f32>>` — full values
  buffer; debug only; warns if called outside test mode

The session does NOT integrate with `BoundaryProtocol` yet. It is a standalone
component that the workshop tests can drive directly.

**Test:** Create a session, upload 3 ops (transfer, constant add, sum
reduction), tick once, verify `readback_summary()` produces expected
checksums. Use the `cpu_oracle` from the workshop crate as the reference.  
**Acceptance:** CI green. `BoundaryProtocol` untouched.

---

### PR B-2 — Pass B WGSL kernel: Identity, Sum, Transfer, EmitEvent

**Model:** Composer 2.5  
**Scope:** The first production version of `accumulator_op.wgsl` with four
combine functions: `Identity`, `Sum`, `Transfer` (single-source gather +
`SubtractFromSource`), `EmitEvent` (threshold gate + atomic counter write to
emission buffer).

Atomic f32 helpers MUST be copied verbatim from
`crates/simthing-workshop/src/eml_phase5.wgsl`. Do not rewrite.

`OrderBand` dispatch: the kernel receives a uniform `current_band: u32` and
skips any op registration whose `gate.band != current_band` (or whose gate
is not `OrderBand`). The session's `tick(band)` method sets this uniform.

**Test:** GPU parity test for each of the four combine functions against the
CPU oracle. Verify atomic emission counter is correct for a 1000-op Transfer +
EmitEvent sweep.  
**Acceptance:** All 4 combine parity tests pass. Conservation holds for
Transfer (faction total decreases by exactly what factory queue increases).

---

### PR B-3 — Timestamp query plumbing

**Model:** Codex 5.5  
**Scope:** Add `wgpu::Features::TIMESTAMP_QUERY` to the device descriptor in
`AccumulatorOpSession::new`. Add `timestamp_query_set` and `resolve_buffer`
as optional fields. Add `AccumulatorOpSession::last_pass_time_us() ->
Option<u64>` — returns `None` if timestamp queries are not available on the
current backend.

Update the session's `tick()` method to optionally wrap the compute pass in
a `timestamp_writes` block when the query set is available.

**Test:** On backends that support timestamp queries (Vulkan, Metal DX12),
assert `last_pass_time_us()` returns `Some(v)` where `v < 10_000` for a
trivial 1000-op dispatch. On backends that don't, assert it returns `None`
without panicking.  
**Acceptance:** CI green. No panics on any backend.

---

### ⚠️ PR B-4 — Opus review: summary/checksum readback design

**Model:** Opus (design), Composer 2.5 (implementation)  
**Why Opus:** The summary/checksum readback path is the default production
tier from the ADR. The workshop showed summary mode wins at 3.4–4.4× over
CPU for 100k-slot runs. But the design question — what does `SlotSummary`
contain? — is not resolved by the workshop. Options:

1. **Checksum only:** `SlotSummary { slot: u32, checksum: u32 }` — diff
   against prior tick's summary to detect any change, then do a targeted
   slot readback. Minimal bandwidth; can't reconstruct state without prior
   values.

2. **Column-range checksum:** `SlotSummary { slot: u32, col_checksums: [u32; 4] }`
   — checksum per group of columns. Faster dirty detection, more bandwidth.

3. **Coarse value:** `SlotSummary { slot: u32, coarse_amount: f32,
   coarse_intensity: f32 }` — enough for boundary skip decisions without
   exact values. Allows B3-style boundary skip extension to AccumulatorOp.

The choice has direct implications for the B3 boundary-skip logic interaction
and for how replay delta logs are produced. Wrong choice here is a re-
architecture later.

**Opus task:** Evaluate the three options against: (a) compatibility with B3
boundary skip logic in `simthing-sim/src/boundary.rs`, (b) the replay/delta
log contract from `adr_accumulator_op_v2.md`, (c) whether hot-pool contention
scenarios (few pools, many requesters) change the recommendation. Produce a
two-page analysis recommending one option with explicit reasoning.

**Implementation (Composer 2.5 after Opus decision):** Implement the chosen
`SlotSummary` type and its GPU-side write in the kernel.  
**Acceptance:** Opus analysis committed. Chosen path implemented with tests.

---

## Phase C — Operation family migration

Each PR migrates one operation family from the existing passes to AccumulatorOp.
The existing pass is **NOT removed** until all parity tests pass and the PR is
merged. Every migration PR must include a bit-exact parity test against the
current pass path.

### PR C-1 — Threshold scan migration

**Model:** Composer 2.5  
**Scope:** Migrate Pass 7 (threshold scan) to AccumulatorOp using the
`Threshold` gate + `EmitEvent` consume mode. The GPU atomic counter replaces
the full-buffer scan. This is Route 1 from the optimization doc — it falls out
for free from the AccumulatorOp design.

Existing Pass 7 is retained in parallel. A feature flag
`use_accumulator_threshold_scan: bool` on `BoundaryProtocol` selects which
path runs. Default: `false` (existing path). Tests run both paths and compare.

**Parity test:** For the `fission_stress` scenario at 20k slots, run both paths
for 100 ticks and assert identical `ThresholdEvent` sequences.  
**Performance test:** Assert `readback_emissions()` from AccumulatorOp path is
at least 5× faster than `tick_event_readback_ms` from the current path at
10k registered thresholds. Use `last_pass_time_us()` from B-3.  
**Acceptance:** Both tests pass on three consecutive runs.

**Note:** This is the single PR most likely to surface the `tick_event_readback_ms`
improvement the optimization route analysis predicted. If it does NOT produce
a measurable improvement, stop and open an Opus review before migrating further.

---

### PR C-2 — Intent delta application migration

**Model:** Codex 5.5  
**Scope:** Migrate the Intent Pass (intent delta application) to AccumulatorOp
using `Identity` combine, `Always` gate. The fold logic on CPU is unchanged.
Feature-flagged with `use_accumulator_intent: bool`.

**Parity test:** Bit-exact against current intent pass for 10 scenarios.  
**Acceptance:** CI green. Feature flag default remains `false`.

---

### PR C-3 — Overlay Add migration

**Model:** Composer 2.5  
**Scope:** Migrate `TransformOp::Add` overlays from Pass 3 to AccumulatorOp
`Identity + LifecycleActive + OrderBand(0)`. The OrderBand is 0 for Add
(ancestor first is handled by registration ordering at build time).
Feature-flagged.

The key invariant: ancestor-precedes-local ordering is preserved by building
the AccumulatorOp registration buffer in tree-walk order, the same way
`build_overlay_deltas` works today. Add a `#[test]` that verifies a parent's
Add fires before a child's Add when both register in the same tick.

**Parity test:** Bit-exact against current Pass 3 for `boundary_integration.rs`
overlay scenarios.  
**Acceptance:** CI green. Existing overlay tests pass.

---

### ⚠️ PR C-4 — Opus review: Overlay Multiply/Set and OrderBand compiler design

**Model:** Opus (design), Composer 2.5 (implementation)  
**Why Opus:** The workshop overlay order-band test showed the conservative
indexed-range compiler is semantically correct but has a performance cliff
under high overlay density (density=1.0: 0.56× in run 1, 1.2–1.3× in runs
2–3, high variance). The dirty/cached-rebuild requirement is clear from the
data but the compiler design is not.

The specific question: the `OrderBand` compiler must produce a registration
buffer from the raw overlay tree without recompiling the full index every tick.
This is analogous to the B2 Approach C incremental CSR patcher for topology.
Two possible shapes:

Option A: **Delta-only compiler.** Track which overlays have changed since last
compile (via `OverlayLifecycle` transitions). Recompile only affected
parent/slot ranges. Requires tracking overlay activation/suspension events.

Option B: **Per-band dirty bits.** One dirty bit per (slot, band). When an
overlay changes, set the dirty bit for its slot and band. On tick, recompile
only registrations for dirty (slot, band) pairs.

The choice affects memory layout, the interaction with the existing
`overlay_prep.rs` path, and the handling of `Suspended` overlays (which
must not appear in the registration buffer but must re-appear on activation).

**Opus task:** Evaluate Option A vs Option B against: (a) the existing
`OverlayLifecycle` transition model from `design_v6.md`, (b) the fission-clone
path (cloned children need fresh registration compilation), (c) the
performance data from the workshop (small-scenario overhead, high-density
regression). Produce a two-page design note with a concrete recommendation
and the data structures.

**Implementation (Composer 2.5 after Opus decision):**
Implement `Product` + `LastByPriority` combine functions in the kernel.
Implement the chosen compiler path. Feature-flagged.

**Parity test:** Bit-exact against current Pass 3 for all overlay op types.  
**High-density guard test:** At overlay density=1.0, assert the compiler does
not recompile the full index when no overlays have changed since the last tick.  
**Acceptance:** Both tests pass. Opus design note committed.

---

### ⚠️ PR C-5 — Opus review: WeightedMean reduction and tolerance boundary

**Model:** Opus (review + boundary analysis), Composer 2.5 (implementation)  
**Why Opus:** WeightedMean is the operation where the tolerance policy from
A-4 is most likely to create production problems. The workshop showed the
current path is ALSO loose-tolerance (~3e-6 error) — meaning any existing
code that reads a WeightedMean-reduced value and uses it for a hard decision
is already silently wrong.

Before migrating WeightedMean to AccumulatorOp, Opus should:

1. Audit all paths in `boundary.rs` and `threshold_registry.rs` where a
   reduced value could flow into a hard structural decision. This is the
   "existing production exposure" analysis requested in PR A-4 — do it now
   if A-4's Opus work identified any exposure.

2. Confirm or deny: does the `WeightedMean` reduction path today produce
   the same ~3e-6 error vs CPU oracle as the AccumulatorOp pivot path? The
   workshop measures both and they're identical — but this should be confirmed
   in the production codebase, not just the workshop.

3. Specify the `SoftAggregateGuard` placement: which specific properties in
   the default SimThing property set (loyalty, stability, efficiency, morale)
   use WeightedMean reduction, and which of those feed threshold registrations?

**Implementation (Composer 2.5):**
`WeightedMean` and `Mean` combine functions in the kernel. Multi-input gather
over `SlotRange` with `weight_col`. Uses workgroup shared memory for
deterministic accumulation — same execution model as the current reduction
pass; just parameterized by combine function rather than hardcoded.

**Parity test:** GPU-to-GPU determinism (not bit-exact vs CPU oracle — that
is expected to fail at ~3e-6). Three consecutive runs must produce identical
results.  
**Guard test:** Assert `SoftAggregateGuard` is present on any WeightedMean
column that feeds a threshold registration.  
**Acceptance:** Both tests pass. Opus audit committed.

---

### PR C-6 — Sum, Max, Min reductions

**Model:** Composer 2.5  
**Scope:** Add `Sum`, `Max`, `Min` combine functions to the kernel. These are
the clean cases from the Phase 0 v2 analysis — one `SlotRange` registration
per parent per dimension, workgroup-local reduction, one atomic write.

**Parity test:** Bit-exact against current reduction passes for Sum (which is
bit-exact today). Max/Min are bit-exact via shared-memory reduction. Three runs.  
**Acceptance:** CI green.

---

### PR C-7 — Velocity integration migration

**Model:** Composer 2.5  
**Scope:** `IntegrateWithClamp` combine function. MultiTarget writes (Amount +
Velocity). The combine function receives `{ dt, vel_max, amount_min, amount_max }`
from the registration and applies the full `GovernedPair` semantics in one
kernel invocation.

The existing `GovernedPair` CPU struct feeds the registration at boundary
prep time — `dt` comes from the session's tick parameters, `vel_max` and
clamp bounds come from `SubFieldSpec`.

**Parity test:** Bit-exact against Pass 1 (velocity integration). Specifically
test `vel_max` clamp at the exact boundary value — this was the contingency
in the Phase 0 analysis. Feature-flagged.  
**Acceptance:** CI green. `vel_max` clamp test passes with `f32::to_bits()`
comparison.

---

### ⚠️ PR C-8 — Opus review: EML transfer + intensity migration

**Model:** Opus (integration design), Composer 2.5 (implementation)  
**Why Opus:** EML intensity and AccumulatorOp interact at the `EvalEML` combine
boundary. The workshop validated EML as a standalone harness. Integrating it
into the AccumulatorOp session means:

1. The `EmlExpressionRegistry` from A-3 must be the source of truth for tree
   IDs in `EvalEML` registrations.
2. The EML node buffer must be a persistent GPU buffer in `AccumulatorOpSession`,
   not uploaded per dispatch (the fix from the EML Phase 5 hardening handoff).
3. The tree can change at session open (when a new recipe is registered) but
   not mid-tick.

Opus should specify: (a) where the EML node buffer lives in the session
(alongside `op_buffer`? separate?), (b) how tree ID → buffer offset is
resolved in the WGSL shader, (c) whether a single dispatch can handle multiple
EML trees (different formula classes in the same tick), (d) the tree-change
protocol (can the session hot-reload an EML tree without full session teardown?
— relevant to I2/H1 deferred work).

**Implementation (Composer 2.5):**
- `EvalEML` combine in the kernel
- Persistent node buffer in `AccumulatorOpSession`
- Transfer (`CrossingFormula` + `MinAcrossInputs` + `SubtractFromAllInputs`)
  as the economic substrate

**Parity tests:**
- EML intensity bit-exact against CPU oracle (no transcendentals; bit-exact
  expected per workshop)
- Transfer conservation: exact balance across 1000 factories, 3 channels,
  100 ticks
- Conjunctive emission: emit count matches CPU reference within 2%
  (non-determinism tolerance from workshop)

**Acceptance:** All three tests pass. Opus design note committed.

---

## Phase D — Contention hardening and performance gates

Phase D is conditional: it begins only if Phase C's contention scenario
benchmarks show the v1 allocator is a production bottleneck.

### ⚠️ PR D-1 — Opus design: hot-pool allocator v2

**Model:** Opus (design only)  
**Why Opus:** This is the most open design question remaining. The workshop
showed the v1 one-invocation-per-pool allocator collapses to 0.14× CPU at 16
pools / 100k requesters. Three candidate strategies from the ADR:

1. **Segmented scan:** Divide each pool's requester range into segments of
   size N. Each GPU workgroup handles one segment. Requires a prefix-sum
   reduction to compute per-segment available balances.

2. **Prefix allocation:** Two-pass approach — first pass computes how much
   each requester would get if unconstrained; second pass applies the pool
   capacity constraint via prefix sum and scales down proportionally.

3. **Subrange partitioning:** Statically assign requesters to pool subranges
   at registration time. Each subrange gets an equal fraction of the pool.
   Simpler, less fair.

Opus task: Evaluate the three strategies against: (a) conservation guarantee
(exact vs approximate), (b) fairness (priority ordering vs proportional),
(c) GPU implementation complexity, (d) interaction with the existing
`emit_count` calculation from debt-band emission. Produce a design note with
a concrete recommendation, WGSL pseudocode, and the CPU registration change
required.

**Gate:** This PR produces a design document only. Implementation is PR D-2,
after human + Opus review of the design.  
**Acceptance:** Design note committed to `docs/workshop/`.

---

### PR D-2 — Hot-pool allocator v2 implementation

**Model:** Composer 2.5 (after Opus design from D-1)  
**Scope:** Implement the allocator strategy from D-1. Feature-flagged behind
`allocator_strategy: AllocatorStrategy` on `AccumulatorOpSession`.  
**Test:** Hotspot scenario (16 pools, 100k requesters): beat CPU by at least 2×.
Conservation remains exact.  
**Acceptance:** Both tests pass.

---

### PR D-3 — Changed-only compact logs and replay checkpoints

**Model:** Composer 2.5  
**Scope:** Production logging tier implementation:
- Default: summary/checksum readback (already in Phase B)
- Selective: changed-only compact records for production audit
  (`SummaryMode::ChangedOnly { since_tick: u32 }`)
- Replay: compact emission records per tick for replay delta log integration
- Debug: full before/after records (already gated to debug mode in Phase B)

Wire the compact emission records from `readback_emissions()` into the existing
`BoundaryDeltaEntry` log path. The emission records from AccumulatorOp become
a new delta entry variant: `AccumulatorEmission { registration_id, emit_count }`.

**Test:** Run 100-tick factory scenario; replay from compact emission records;
assert final state matches original run within tolerance.  
**Acceptance:** Replay test passes.

---

### PR D-4 — Cross-pool queue contention gate (separate ADR gate)

**Model:** Composer 2.5 (test only), Opus (design if test fails)  
**Scope:** Add a cross-pool contention test to the workshop: factories in two
separate resource pools (iron_pool and coal_pool) with shared downstream
queues. Measure conservation and performance.

This test either (a) passes, confirming the v1 allocator handles the cross-pool
case adequately, or (b) fails, triggering a separate Opus design review and ADR
amendment for cross-pool semantics.

**Acceptance:** Test exists and either passes (cross-pool gate closed) or fails
with a clearly reported failure mode (triggers separate design work).

---

## PR ladder summary

| PR | Phase | Model | Description | ADR gate |
|---|---|---|---|---|
| A-1 | A | Codex 5.5 | Merge ADR, update invariants | Human review |
| A-2 | A | Codex 5.5 | CombineFn + AccumulatorOp types | None |
| A-3 | A | Composer 2.5 | EmlExpressionRegistry + whitelist | None |
| **A-4** | **A** | **Opus + Codex** | **Soft-aggregate tolerance policy audit** | **Human + Opus sign-off** |
| B-1 | B | Composer 2.5 | AccumulatorOpSession persistent buffers | None |
| B-2 | B | Composer 2.5 | Pass B kernel: Identity/Sum/Transfer/EmitEvent | Conservation test |
| B-3 | B | Codex 5.5 | Timestamp query plumbing | None |
| **B-4** | **B** | **Opus + Composer** | **Summary/checksum readback design** | **Opus analysis** |
| C-1 | C | Composer 2.5 | Threshold scan migration | 5× readback speedup |
| C-2 | C | Codex 5.5 | Intent delta migration | Bit-exact parity |
| C-3 | C | Composer 2.5 | Overlay Add migration | Bit-exact parity |
| **C-4** | **C** | **Opus + Composer** | **Multiply/Set OrderBand compiler** | **Opus design + perf gate** |
| **C-5** | **C** | **Opus + Composer** | **WeightedMean tolerance boundary audit** | **Opus audit + guard test** |
| C-6 | C | Composer 2.5 | Sum/Max/Min reductions | Bit-exact parity |
| C-7 | C | Composer 2.5 | Velocity integration migration | vel_max clamp test |
| **C-8** | **C** | **Opus + Composer** | **EML + transfer + intensity integration** | **Opus design + 3 parity tests** |
| **D-1** | **D** | **Opus** | **Hot-pool allocator v2 design** | **Opus design note** |
| D-2 | D | Composer 2.5 | Hot-pool allocator v2 implementation | 2× CPU at hotspot |
| D-3 | D | Composer 2.5 | Changed-only logs + replay integration | Replay test |
| D-4 | D | Composer 2.5 + Opus | Cross-pool contention gate | Pass or triggers ADR amendment |

**Opus-gated PRs: A-4, B-4, C-4, C-5, C-8, D-1.** Six of twenty PRs. These are
the six PRs where the correctness or design space is genuinely open and the
cost of a wrong decision is architectural. Every other PR is fully specified by
the ADR and the workshop evidence and can be executed mechanically.

---

## Performance expectations

These are not guarantees; they are predictions based on workshop evidence.
If a PR's migration does not achieve the predicted performance, stop and open
an Opus review before proceeding.

| PR | Expected win | Basis |
|---|---|---|
| C-1 threshold migration | 5–20× reduction in `tick_event_readback_ms` | Route 1 prediction; workshop emission buffer timing |
| C-6 Sum/Max/Min | 1.2–1.9× reduction in reduction pass time at 100k+ parents | Workshop WeightedMean A/B data |
| C-7 velocity | Neutral to 1.2× | Velocity is a small fraction of current tick cost |
| C-8 transfer + emission | Current paradigm has no equivalent path | New capability, not a migration |
| D-2 hot-pool v2 | 2× CPU at 16-pool hotspot | Replaces the 0.14× measured weakness |

---

## What this plan explicitly does NOT include

- Removal of the current 8-pass pipeline. Each pass is removed only when its
  migration PR is merged, all parity tests pass, and the feature flag is
  flipped. The feature flags are permanent until removal PRs land.
- Migration of Pass 0 (snapshot). `copy_buffer_to_buffer` stays.
- EML Phase 1–4 implementation. See `docs/eml_integration_guidance.md`.
- Full economic V1 implementation (E0). Deferred per `todo.md`.
- Studio / EML authoring tools. Deferred per `todo.md`.
- Cross-pool queue contention design (deferred to D-4 gate outcome).
