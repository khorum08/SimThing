# AccumulatorOp v2 — Production Plan

> **Status:** Active planning document. Companion to `adr_accumulator_op_v2.md`.
> The PR ladder below is the authoritative sequencing. Phases A–D are not
> calendar quarters; they are completion-gated sequences. A phase does not
> begin until all PRs in the prior phase are green and merged.
>
> **Pivot posture (2026-05-26):** AccumulatorOp v2 is the production direction.
> Legacy reduction (S-4), legacy intensity (S-2), legacy overlay (S-3),
> legacy threshold (S-6), legacy velocity (S-5), and legacy intent (S-1) are
> **deleted**. The only retained old operation is snapshot
> (`copy_buffer_to_buffer`). See
> [`docs/workshop/workshop_current_state.md`](workshop/workshop_current_state.md) and
> [`docs/workshop/pivot_forward_implementation_policy.md`](workshop/pivot_forward_implementation_policy.md).

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

**Phase M EML-GADGET-2A snapshot/copy fixture proof landed (2026-05-29, PASS).**
It proves that temporal snapshot/copy bands can be authored using existing substrate primitives: Identity combine + ResetTarget at an earlier OrderBand, copying current_col into previous_col before the update band.
No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No temporal gadget implementation landed.
EML-GADGET-2A snapshot/copy fixture proof + R1 sequence hygiene landed. 2B VelocityMonitor + Decay/EMA landed. 2C BoundedFeedback landed. 2D Hysteresis landed with 2D R1 exact CMP/SELECT compiler parity. 2E explicit velocity-column Acceleration landed in simthing-spec authoring/admission/compiler/oracle only. Dense per-cell temporal memory remains deferred. No runtime gadget execution or chained scheduling. No new opcode/WGSL/GPU kernel. No simthing-sim semantics. No production economy→mapping bridge or default mapping wiring. No atlas/M-4A.
No hidden previous-value read was introduced.
Temporal memory remains explicit-column state.
Temporal memory remains Layer-3 scoped by default; dense per-cell temporal memory remains separately gated.
No simthing-sim Gadget/Personality/Memory semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.

**Phase M EML-GADGET-2A R1 hygiene landed.**
It keeps the original 2A snapshot/copy proof intact and cleans the multi-step sequence test/report so the evidence precisely shows previous_col capturing current_col before the update band while current_col advances afterward.
No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No temporal gadget implementation landed.
EML-GADGET-2A snapshot/copy fixture proof + R1 sequence hygiene landed. 2B VelocityMonitor + Decay/EMA landed. 2C BoundedFeedback landed. 2D Hysteresis landed with 2D R1 exact CMP/SELECT compiler parity. 2E explicit velocity-column Acceleration landed in simthing-spec authoring/admission/compiler/oracle only. Dense per-cell temporal memory remains deferred. No runtime gadget execution or chained scheduling. No new opcode/WGSL/GPU kernel. No simthing-sim semantics. No production economy→mapping bridge or default mapping wiring. No atlas/M-4A.
No hidden previous-value read was introduced.
Temporal memory remains explicit-column state.
Temporal memory remains Layer-3 scoped by default; dense per-cell temporal memory remains separately gated.
No simthing-sim Gadget/Personality/Memory semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.

**Consolidated EML-GADGET review state:** 2D Hysteresis + 2D R1 exact CMP/SELECT compiler parity and 2E explicit velocity-column Acceleration are consolidated in [`docs/reviews/phase_m_eml_gadget_2de_temporal_derivative_parking_packet.md`](reviews/phase_m_eml_gadget_2de_temporal_derivative_parking_packet.md). [`docs/reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md`](reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md) consolidates 2A/R1/2B/2C. Runtime gadget execution, chained scheduling, dense per-cell temporal memory, position-history acceleration, atlas/M-4A, and production economy→mapping bridge remain separately gated.

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

> **C-8 evolution note:** The A-3 `EmlTreeMeta { node_count,
> has_transcendental, formula_class }` schema is **refactored to
> `EmlFormulaMeta { tree_id, execution_class, allowed_consumers,
> max_abs_error, deterministic_gpu, requires_guard_for_hard_threshold,
> ... }` in C-8a**. The A-3 whitelist becomes the `ExactDeterministic`
> execution-class admission policy; the framework is extended for
> future `SoftDeterministic` / `FastApproximate` / `CpuOracleOnly`
> classes per `docs/workshop/c8_eml_transfer_intensity_design.md`. The
> A-3 string-class check (`["intensity_update", "emission_formula",
> "conversion_rate"]`) is superseded by the typed `EmlConsumerKind` +
> admissibility matrix.

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

**Shipped scope (post-hardening):** persistent session + bootstrap non-emitting
kernel only. Supports non-contended Identity/Sum and clamped SlotValue transfer.
Rejects duplicate same-band writes/consumes at upload. `SlotSummary` is now the
production B-4I group-checksum tier (32 B/slot); see B-4 design memo.

**Shipped scope (B-2):** hardens the bootstrap kernel with explicit scale
encoding, clamped non-contended SlotValue transfer, same-band contention
rejection, compact `EmitEvent` records, atomic `emission_count`, emission
overflow reporting, and CPU oracle parity. Bootstrap validation treats
`GateSpec::Always` as a wildcard band: an Always op may not write or consume
any cell that another op writes or consumes in any OrderBand. This is
conservative and prevents accidental races in the non-contended bootstrap
kernel. Later allocator/order-band work may relax this only with explicit
deterministic semantics.

---

### PR B-2 — Pass B WGSL kernel: Identity, Sum, Transfer, EmitEvent

**Model:** Composer 2.5  
**Scope:** Expand the bootstrap `accumulator_op.wgsl` into the first production
Pass B kernel with four combine functions: `Identity`, `Sum`, `Transfer`
(single-source gather + `SubtractFromSource`), `EmitEvent` (threshold gate +
atomic counter write to emission buffer).

B-1 shipped the persistent session and a **bootstrap** subset. B-2 hardens that
kernel with `EmitEvent`, `emission_count` atomic increments, emission capacity
handling, and CPU oracle parity. B-2 still does not implement threshold-gated
migration, WeightedMean, EvalEML, overlay Product/LastByPriority, conjunctive
production, contended allocation, or `BoundaryProtocol` integration.

C-1 owns threshold scan migration; B-4 owns final summary/checksum design.

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

**Shipped scope (B-3):** optional timestamp query plumbing for
`AccumulatorOpSession`. Timestamp support is feature-detected. Unsupported
backends return `None` from `last_pass_time_us()` and still execute normally.
The B-3 readback is synchronous for testability; future production profiling
may batch or sample timestamp readbacks.

---

### ⚠️ PR B-4 — Opus review: summary/checksum readback design

**Design status:** **Accepted** — see
[`docs/workshop/slot_summary_b4_design.md`](workshop/slot_summary_b4_design.md).
Selected layout: column-group checksums + whole-slot checksum + reserved
`flags` word, no semantic values, no GPU-side previous-summary comparison.

**Implementation (B-4I):** **Landed** — production `SlotSummaryGpu` (32 B/slot),
WGSL `write_summaries` group checksums, CPU oracle parity tests. Full readback
remains debug/test only.

**World integration (PR #111):** **Landed** — `WorldSummaryRuntime` on
`WorldAccumulatorRuntime` writes summaries from integrated `WorldGpuState.values`
after C-1/C-2/C-3 tick passes; `WorldGpuState::readback_accumulator_summary()`.
Standalone `AccumulatorOpSession` summary path unchanged for kernel/oracle tests.

**Pivot posture:** Summary tier is production infrastructure, not optional polish.
Legacy pass readback is not the long-term change-detection path.

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

**Status:** Landed; refined and integrated. Parity green; perf gate reframed
per [`c1_perf_reframe_memo.md`](workshop/c1_perf_reframe_memo.md) — the
original 5× projection was based on a workshop baseline that did not match
the production codebase's already-compact readback path. New gate is
no-regression (`ratio ≥ 1.0`) with a 1.5× soft warning. Single-submission
integration via `Pipelines::run_tick_pipeline_with_threshold_scan` captures
the structural per-tick savings.

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
**Performance test (reframed):** Assert AccumulatorOp readback does not
regress vs the legacy Pass 7 readback path at 10k registered thresholds
(ratio ≥ 1.0, warn below 1.5×). See `c1_perf_reframe_memo.md` for why the
original 5× projection was reframed. The structural win lives in the
single-submission pipeline integration; total tick wall time is the right
metric for that, not isolated readback.  
**Acceptance:** Parity test passes bit-exact; perf test asserts no
regression.

**Note:** This is the single PR most likely to surface the `tick_event_readback_ms`
improvement the optimization route analysis predicted. If it does NOT produce
a measurable improvement, stop and open an Opus review before migrating further.

---

### PR C-2 — Intent delta application migration

**Model:** Codex 5.5  
**Scope:** Migrate the Intent Pass (intent delta application) to AccumulatorOp
using a C-2-specific `COMBINE_AFFINE_INTENT` GPU combine (`value = value * mul + add`).
The CPU fold logic on the feeder/patcher path is unchanged.
Feature-flagged with `use_accumulator_intent: bool` (default `true` after S-1).

**Implementation note:** Folded `IntentDelta` rows encode as affine AccumulatorOp
registrations; the Accumulator intent pass runs in the same tick command buffer
as Passes 0–6 (before snapshot), not as a second submission. Combined C-1/C-2
test verifies intent-before-threshold ordering. S-1 deleted old
`intent_delta.wgsl`; disabling AccumulatorOp intent with pending intents now
rejects loudly instead of falling back.

**Parity test:** Bit-exact against current intent pass for 10 scenarios.  
**Acceptance:** CI green. S-1 sunset tests use AccumulatorOp/CPU golden coverage.

---

### PR C-3 — Overlay Add migration

**Status:** Landed (#105–#107). Add-only → AccumulatorOp with OrderBand exact f32
order. The historical mixed-batch fallback was removed by C-4 while retaining the
`use_accumulator_overlay_add` flag name for compatibility. **Sunset target:** S-3
(after C-4 default-on validation).

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

**Implementation note:** C-3 established the per-cell OrderBand exactness rule
for Add-only batches. C-4 extends that rule to all Add/Multiply/Set overlay
batches and removes the mixed-batch fallback when the accumulator overlay flag is
enabled. S-3 deletes the legacy Pass 3 runtime branch; overlay workloads now
require the AccumulatorOp OrderBand path.

---

### PR C-INF-1 — WorldAccumulatorRuntime consolidation

**Status:** Landed. `WorldGpuState` holds one `accumulator_runtime: Option<WorldAccumulatorRuntime>`
instead of three sidecar sessions. C-1/C-2/C-3 use per-family `AccumulatorOpSession` instances
inside the runtime envelope; tick dispatch take/put matches pre-consolidation behavior.

**Pivot posture:** Stop accumulating per-family `Option<AccumulatorOpSession>` sidecars on
`WorldGpuState`. New migrations register into `WorldAccumulatorRuntime` op sets.

**Acceptance:** C-1/C-2/C-3 tests green; flags default false; no shader deletion.

---

### PR C-INF-2 — Legacy oracle harness

**Status:** Landed. `simthing-sim::legacy_oracle` defines `LegacyOracleRun`,
`run_family_oracle`, comparison helpers, and `OracleCapture`. Integration tests in
`c_inf_legacy_oracle_harness.rs` (intent + threshold smoke). Legacy paths invoked
only from oracle tests or explicit fallback.

**Acceptance:** No runtime tick dependency on oracle harness.

---

### PR Pivot-forward remedial — Authoritative flags + world summary (#111)

**Status:** Landed. Feature flags are authoritative: disabling
`use_accumulator_intent` or `use_accumulator_threshold_scan` clears stale runtime
sessions via boundary sync (`clear_intent` / `clear_threshold`); overlay already
cleared on flag-off. `WorldSummaryRuntime` provides B-4 summaries from integrated
world values. `OracleExactness::ToleranceAbsEpsilon` replaces mislabeled ULP tolerance.

**Pivot posture:** Strengthen AccumulatorOp runtime authority; no legacy expansion.

**Acceptance:** Flag-off stale-session tests; world summary matches full-value readback;
C-1/C-2/C-3 parity green; flags remain default false.

---

### PR C-4 — Overlay Multiply/Set and OrderBand compiler

**Status:** Landed behind the overlay AccumulatorOp flag. See
[`docs/workshop/c4_overlay_orderband_compiler_design.md`](workshop/c4_overlay_orderband_compiler_design.md).
Implemented: reuse `build_overlay_deltas` unchanged; new
`plan_overlay_orderband` extends the C-3 per-cell band pattern to mixed
Add/Mul/Set; two-tier cache (revision counter on `BoundaryProtocol` +
equality check on the cached `(deltas, ranges)`); new
`ConsumeMode::AddToTarget` plus shader-side `ScaleTarget` / `ResetTarget`
make the (combine, consume) mapping clean. C-3's `(Identity, None)` ≡
add hack is replaced by `(Identity, AddToTarget)`.

**Implementer:** **Codex 5.5** (was previously listed as Composer 2.5;
the memo specifies enough detail that Codex can execute mechanically).

**Model:** Opus (design), Codex 5.5 (implementation)  
**Why Opus:** The workshop overlay order-band test showed the conservative
indexed-range compiler is semantically correct but has a performance cliff
under high overlay density (density=1.0: 0.56× in run 1, 1.2–1.3× in runs
2–3, high variance). The dirty/cached-rebuild requirement is clear from the
data but the compiler design is not.

**Implementation:** Full Add/Multiply/Set overlay batches route through
AccumulatorOp OrderBands when `use_accumulator_overlay_add` is true. The flag
name is retained from C-3 for compatibility, but the path is now the full C-4
overlay compiler. The pipeline still runs in one command buffer and executes the
overlay bands at the original overlay point, before reduction and world summary.
S-3 removes the legacy shader/pipeline branch; CPU/golden tests now cover
overlay parity.

**Parity test:** Bit-exact against CPU/golden canonical overlay order for all overlay op types.  
**High-density guard test:** At overlay density=1.0, assert the compiler does
not recompile the full index when no overlays have changed since the last tick.  
**Acceptance:** C-4 parity and no-change cache tests pass. Opus design note
committed. S-3 deletes the old overlay shader/pipeline after C-4 validation.

---

### ✅ PR S-3 — Legacy overlay sunset

**Status:** Landed locally.

**Scope:** Delete the legacy Pass 3 overlay runtime path after C-3/C-4 migrated
Add/Multiply/Set overlays to AccumulatorOp OrderBands.

**What shipped:**
- Deleted `crates/simthing-gpu/src/shaders/transform_application.wgsl`.
- Removed `overlay_pipeline`, `overlay_layout`, legacy overlay bind-group
  creation, and legacy overlay dispatch from `Pipelines`.
- `use_accumulator_overlay_add` now defaults **true** and is mandatory for
  active overlay workloads. Disabling it with overlay deltas panics with the S-3
  deletion message rather than falling back.
- C-3/C-4 overlay parity tests now use CPU/golden canonical overlay order rather
  than the deleted shader path.
- Added `s3_overlay_sunset.rs` guards for shader absence, default accumulator
  routing, flag-off rejection, CPU golden Add/Multiply/Set parity, and cache
  rebuild after lifecycle activation.

**Acceptance:** S-3 overlay sunset, C-3/C-4 overlay parity, and GPU pass tests
green locally. No CPU production overlay path was added.

---

### ✅ PR C-5 — WeightedMean / Mean soft reductions → AccumulatorOp

**Status:** **Landed** (#122; design in
[`docs/workshop/c5_weighted_mean_reduction_design.md`](workshop/c5_weighted_mean_reduction_design.md)).

**What shipped:** `use_accumulator_reduction_soft` flag (default false);
`ReductionSoft` session on `WorldAccumulatorRuntime`; `plan_reduction_orderband`;
linear-loop `COMBINE_MEAN` / `COMBINE_WEIGHTED_MEAN` in `accumulator_op.wgsl`;
two-buffer model preserved (`values` → memcpy → `output_vectors` reductions);
legacy `reduction.wgsl` skips soft columns when flag on (exact columns unchanged).

**Parity:** GPU-to-GPU bit-identical (three runs); legacy vs AccumulatorOp abs
tolerance `1e-5`; A-4 guard tests unchanged. S-4 pending.

---

### ✅ PR C-6 — Sum, Max, Min, First exact reductions → AccumulatorOp

**Status:** **Landed** (#124).

**What shipped:** `use_accumulator_reduction_exact` flag (default false; requires
soft flag). `ReductionPlanMode::AllRules` extends `plan_reduction_orderband` with
Sum / Max / Min / First. AccumulatorOp WGSL linear-loop gather for exact rules.
When soft+exact flags are on, full reduction runs through AccumulatorOp with no
legacy `reduction.wgsl` dispatch. C-5 bridge (soft only) unchanged. S-4 pending.

**Parity:** Sum/Max/Min/First bit-exact vs legacy; mixed soft+exact within 1e-5;
combined all-flags integration test green.

### ✅ PR S-4 — Legacy reduction sunset

**Status:** **Landed** (#126).

**What shipped:** Deleted `reduction.wgsl`, legacy reduction pipeline/bind groups,
`skip_soft_columns`, C-5/C-6 exact fallback branch, and legacy dispatch counters.
`run_accumulator_reduction_passes` is the sole reduction dispatch. `ReductionPlanMode`
removed — `plan_reduction_orderband` plans all rules. Reduction flags default on
(both required). Tests use CPU oracle golden; `s4_reduction_sunset.rs` added.

**Preserved:** topology upload, `child_starts` / `child_indices` / `depth_slots`,
column rules, THRESH_BUF_OUTPUT semantics, GPU-resident two-buffer reduction.

---

### PR C-7 — Velocity integration migration

**Status:** Landed (#127); sunset complete locally. `use_accumulator_velocity`
default **true** after S-5. Legacy `velocity_integration.wgsl` and pipeline
wiring are deleted; disabling AccumulatorOp velocity with governed pairs now
rejects loudly.

**Model:** Composer 2.5  
**Scope:** `IntegrateWithClamp` combine function. MultiTarget writes (Amount +
Velocity). Clamp metadata (`vel_max`, `clamp_min`, `clamp_max`, `clamp_kind`) in
uploaded ops; `dt` via `AccumulatorTickParams.dt_bits` (not per-tick op rebuild).
Legacy-exact semantics: amount integrate + velocity pinning at floor/ceiling only.

**Parity test:** Bit-exact against Pass 1 (velocity integration). Specifically
test `vel_max` clamp at the exact boundary value — this was the contingency
in the Phase 0 analysis. Feature-flagged.  
**Acceptance:** CI green. `vel_max` clamp test passes with `f32::to_bits()`
comparison.

---

### ✅ PR C-8 — EML transfer + intensity + emission migration

**Design half status:** **Accepted** — see
[`docs/workshop/c8_eml_transfer_intensity_design.md`](workshop/c8_eml_transfer_intensity_design.md).

**C-8a implementation status:** **Landed (#129)** — infrastructure only; see worklog 2026-05-19.

**C-8a remedial status:** **Merged** (#130) — program-table accounting, boundary skip, admissibility hardening.

**C-8b status:** **Landed (#131)** — intensity EvalEML migration; `use_accumulator_intensity` (default **true**); legacy `intensity_update.wgsl` **deleted (S-2 #138)**.

**C-8b remedial status:** **Landed (#132)** — intensity op upload cache keys on `IntensityEmlOpPlanSignature` (EML generation + world/op-plan shape); slot growth and entry/layout changes force op reupload; unchanged formulas skip EML table churn via `replace_formula_if_changed`.

**C-8b landed:**
- `IntensityBehavior` → `ExactDeterministic` EML (22 nodes; `MAX_EML_TREE_NODES`/`EML_STACK_MAX` raised to 32).
- Production intensity routes through AccumulatorOp `EvalEML` after velocity, before overlay.
- `dt` via tick params; persistent EML buffers; no per-dispatch upload.

**S-2 landed (#138):**
- Deleted `intensity_update.wgsl`, legacy Pass 2 pipeline/bind group wiring, and `IntensityParams` buffer.
- `use_accumulator_intensity` + `use_accumulator_eml` default **true**; disabling intensity with registered `IntensityBehavior` panics at boundary validation.
- C-8b parity tests use CPU/EML golden oracle only; `s2_legacy_intensity_sunset.rs` validates default path and rejection semantics.

**C-8b remedial landed:**
- Intensity op upload cache now keys on EML generation plus world/op-plan shape (`IntensityEmlOpPlanSignature`).
- Slot growth and intensity entry/layout changes force op reupload.
- Identical formula/shape boundaries skip EML table and op reupload.
- Intensity remains GPU-resident through EvalEML.

**C-8c landed (#133):**
- Transfer substrate routes through AccumulatorOp (`use_accumulator_transfer`, default false).
- `AccumulatorInputListTable` provides persistent GPU input lists (generation-based skip; no per-dispatch upload).
- `MinAcrossInputs` + `SubtractFromAllInputs` support conjunctive exact transfer; single-source `SubtractFromSource` for fixed-amount moves.
- `TransferConservation` admits `ExactDeterministic` only.
- No CPU-mediated production transfer.

**C-8c remedial landed (#134):**
- Transfer planner rejects same-band consumed-input contention (policy A).
- Same-target contention remains allowed via atomic target adds.
- Single-source `output_scale != 1.0` rejected until explicitly supported.
- Invalid unit costs and non-finite transfer values rejected before GPU upload.
- Input-list table generation invalidates on nonempty→empty clear.
- Defensive single-source debit clamp in WGSL (not transactional reservation).

**C-8d landed (#135):**
- GPU-resident emission substrate added through AccumulatorOp (`use_accumulator_emission`, default false).
- `EmissionRecordGpu` schema remains `{ reg_idx, emit_count }`; stable `reg_idx` via `combine_b`.
- ExactDeterministic emission formulas are bit-exact; Soft/Fast emission remains future-gated by explicit tolerance policy.
- `TransferConservation` remains ExactDeterministic only; emission tolerance does not leak into transfer or hard thresholds.
- No CPU-mediated production emission; no per-dispatch EML upload.
- Tick placement after transfer, before overlay.

**C-8d remedial landed (#136):**
- Emission op-plan signature includes stable `reg_idx`, constant value bits, and `max_emit` state.
- EvalEML tree IDs derived/validated from the formula variant (parallel field must match or be absent).
- `max_emit` explicitly rejected until shader clamp is implemented.
- Emission remains GPU-resident through AccumulatorOp; transfer conservation unchanged.

**C-8 complete (completion gate, #137):**
- Full GPU-resident C-8 block validated: EML + intensity + transfer + emission in one tick pipeline.
- Persistent EML/input-list/op reuse across ticks with varying `dt`.
- `c8_full_pipeline_integration.rs` exercises all flags together.

**S-2 complete:** Legacy intensity deleted; production intensity is EvalEML-only (see S-2 landed above).

Selected:
- **Execution-class taxonomy** (`EmlExecutionClass::{ExactDeterministic, SoftDeterministic, FastApproximate, CpuOracleOnly}`) plus a **consumer admissibility matrix** that gates which classes may feed which consumers.
- **C-8 production baseline admits `ExactDeterministic` only** — `SoftDeterministic`/`FastApproximate` register structurally but no production consumer admits them yet.
- **Persistent GPU node buffer + tree-range table** on `WorldAccumulatorRuntime.eml: Option<EmlGpuProgramTable>`. Generation-counter-based invalidation; no per-dispatch upload.
- **Bounded WGSL lookup:** `AccumulatorOpGpu.combine_a = tree_range_index` (resolved CPU-side at registration); the shader never searches by `tree_id`.
- **Flat stack-machine interpreter** in WGSL; fixed-depth stack; postfix-encoded nodes.
- **Auxiliary `AccumulatorInputListTable`** for `MinAcrossInputs + SubtractFromAllInputs` (conjunctive recipes need 4+ inputs; `AccumulatorOpGpu`'s target slots are reserved for write targets).
- **Staged delivery: C-8a (infra) → C-8b (intensity) → C-8c (transfer) → C-8d (emission)**. Transfer/emission flags remain default-off until explicitly enabled; EML + intensity default-on after S-2.

**Implementer mix:** **Codex 5.5** for C-8a, C-8b, C-8d; **Composer 2.5** for C-8c (transfer's conservation invariants and the input-list table benefit from architectural judgment).

**Opus design resolved:** C-8 design landed in [`docs/workshop/c8_eml_transfer_intensity_design.md`](workshop/c8_eml_transfer_intensity_design.md).

Implemented:
- `WorldAccumulatorRuntime.eml: Option<EmlGpuProgramTable>`
- persistent node/range buffers
- CPU-side `tree_id` → `tree_range_index` resolution
- `AccumulatorOpGpu.combine_a = tree_range_index`
- no WGSL tree-id search
- generation-based table invalidation
- multi-tree dispatch through per-op tree range indices

**Parity tests:**
- EML intensity bit-exact against CPU oracle for `ExactDeterministic`
  baseline formulas (C-8b). Future `SoftDeterministic` intensity formulas
  may admit tolerance under per-PR opt-in.
- Transfer conservation: exact balance across 1000 factories, 3 channels,
  100 ticks (C-8c). `ExactDeterministic` only — Soft/Fast classes are
  structurally rejected from transfer paths.
- **C-8d baseline:** `ExactDeterministic` emission formulas are bit-exact.
- **Future Soft/Fast emission:** tolerance gate remains future work; any 2%
  tolerance applies only to future explicitly-gated emission behavior and
  must not leak into `TransferConservation` or hard thresholds.

**Acceptance:** All stage tests pass. Design memo linked above.

**Open after C-8:** production transfer/emission registration ownership
(spec/builder integration) — **design memo landed 2026-05-27** at
[`docs/reviews/transfer_emission_registration_ownership_opus_review.md`](reviews/transfer_emission_registration_ownership_opus_review.md);
Cursor implementation ladder lives in Phase T below. Shared-input cross-pool
contention (D-1); Soft/Fast EML classes remain future-gated.

---

## Phase D — Contention hardening and performance gates

Phase D is conditional: it begins only if Phase C's contention scenario
benchmarks show the v1 allocator is a production bottleneck.

### ⚠️ PR D-1 — Discrete-transaction contention analysis memo

**Status (2026-05-27):** **Done** — [`docs/reviews/d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md). Original hot-pool allocator v2 design scope is
**dissolved for the continuous-flow case** by the Resource Flow Substrate ADR
(`docs/adr/resource_flow_substrate.md`). Continuous flow eliminates per-tick
shared-pool contention architecturally — the workshop's 16-pool / 100k
requester regime cannot arise under the Resource Flow Substrate because no
shared pool slot is written at tick time. Hierarchical fanout distributes
contention across tree depth.

**D-1 memo** evaluates whether *discrete*
transactions (construction commits, treaty payments, emergency spend) reach
contention scales that justify a GPU allocator at all. **Conclusion:** layered
same-band consumed-input rejection (T-2, C-8c, bootstrap) is sufficient for
current Phase T workloads; D-2 GPU allocator remains deferred; prefer driver-only
boundary transaction scheduling (D-2a) if cross-band ordering is needed.

**Model:** Opus (memo only)  
**Gate:** Memo committed to [`docs/reviews/d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md). No implementation PR. **Done (2026-05-27).**
**Output:** Recommendation either confirming D-2 deferral or motivating its
revival as a narrower scope.

---

### PR D-2 — DEFERRED INDEFINITELY

**Status (2026-05-26):** Deferred indefinitely pending discrete-transaction
workload that demonstrates need. Continuous-flow workloads are addressed
by Phase E (Resource Flow Substrate). If D-1 memo concludes discrete
transactions need a GPU allocator, this PR's scope will be re-defined at
that time. The original "hot-pool allocator v2" design is no longer
applicable.

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
| **B-4** | **B** | **Opus + Composer** | **Summary/checksum readback design** | **Opus analysis — accepted** |
| **B-4I** | **B** | **Composer** | **Production SlotSummary protocol (32 B/slot group checksums)** | **CPU/GPU oracle tests** |
| **C-INF-1** | **C-infra** | **Composer** | **`WorldAccumulatorRuntime` consolidation** | **C-1/C-2/C-3 tests green; sidecars shimmed** |
| **C-INF-2** | **C-infra** | **Composer** | **Legacy oracle harness** | **Legacy invoked only in oracle tests** |
| **Pivot remedial** | **C-infra** | **Composer** | **Authoritative flags + `WorldSummaryRuntime`** | **Flag-off clears sessions; world summary parity** |
| C-1 | C | Composer 2.5 | Threshold scan migration | 5× readback speedup |
| C-2 | C | Codex 5.5 | Intent delta migration | Bit-exact parity |
| C-3 | C | Composer 2.5 | Overlay Add migration | Bit-exact parity |
| C-4 | C | Opus + Codex 5.5 | Multiply/Set OrderBand compiler | Landed behind flag |
| **C-5** | **C** | **Opus + Composer** | **WeightedMean tolerance boundary audit + soft reductions** | **Landed (#121 design, #122 impl)** |
| C-6 | C | Composer 2.5 | Sum/Max/Min/First exact reductions | **Landed (#124)** |
| C-7 | C | Composer 2.5 | Velocity integration migration | **Landed (#127)** |
| **C-8** | **C** | **Opus + Composer** | **EML + transfer + intensity + emission** | **Landed + S-2 sunset** |
| **D-1** | **D** | **Opus (memo only)** | **Discrete-transaction contention analysis memo** | **Done** — [`d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md) |
| D-2 | D | — | **Deferred; revive only if D-1 proves need** | n/a |
| D-3 | D | Composer 2.5 | Changed-only logs + replay integration | Replay test |
| D-4 | D | Composer 2.5 + Opus | Cross-pool contention gate | Pass or triggers ADR amendment |

**Remaining Opus-gated PRs: A-4, B-4, D-1.** C-4 and C-8 Opus design have
landed. These are the PRs where the correctness or design space is genuinely open and the
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
| D-2 deferred | n/a | Continuous-flow contention dissolved by Resource Flow ADR; no active performance target unless D-1 revives a narrower discrete-transaction GPU allocator |

---

## What this plan explicitly does NOT include

- Removal of legacy passes before their sunset PRs land. **S-1 through S-6 are complete**; snapshot is the only retained non-Accumulator GPU operation.
- Migration of Pass 0 (snapshot). `copy_buffer_to_buffer` stays.
- EML Phase 1–4 implementation. See `docs/eml_integration_guidance.md`.
- Complete game-content economy design, balancing, Studio tooling, or final scenario content. This plan **does** include Economic V1 substrate/builders (E-1 through E-3) and Resource Flow infrastructure (E-7 through E-11); it does **not** include full content/economy authoring.
- Studio / EML authoring tools. Deferred per `todo.md`.
- D-2 hot-pool allocator v2 implementation (deferred indefinitely unless D-1 memo revives a narrower discrete scope).

---

## Phase E — Economic V1 integration

Phase E begins after C-8 is merged. It delivers the economic substrate as a
first-class production capability built on AccumulatorOp primitives. This is
the integration layer between the GPU primitive and the spec/driver layer that
modders and Studio will author against. It is not a new engine — it is the
AccumulatorOp primitive expressed through the spec session model.

### PR E-1 — `EmitOnThreshold` as a first-class AccumulatorOp registration builder

**Status:** **Landed** (#144) — first-class `emit_on_threshold(...)` builder in `simthing-core`;
re-registration helpers compile to existing C-1/C-8d threshold+EmitEvent registrations.
Output-buffer registrations must use `emit_on_threshold_registrations_to_gpu` /
`upload_threshold_ops` (plain `AccumulatorOp` does not carry buffer selector).
No new GPU primitive.

**Model:** Composer 2.5  
**Scope:** `AccumulatorOpBuilder::emit_on_threshold(...)` in `simthing-core` constructs
the C-1 threshold + `EmitEvent` registration shape:

```rust
pub fn emit_on_threshold(
    source_slot: u32,
    source_col:  u32,
    threshold:   f32,
    direction:   ThresholdDirection,
) -> AccumulatorOp
```

`EmitOnThresholdRegistration` + `rebuild_emit_on_threshold_ops` support session-open
and boundary threshold refresh. `refresh_emit_on_threshold_debt_band` advances debt-band
threshold values after emission without Resource Flow registry machinery.

**Test:** `crates/simthing-sim/tests/e1_emit_on_threshold_builder.rs` — op-shape parity
with C-1, upward/downward/either/no-crossing, debt-band re-registration, S-6 intact.  
**Acceptance:** Tests pass. C-1/C-8d/S-6 regressions remain green.

---

### PR E-2 — SPLIT: discrete transfer + continuous-flow participant builders

**Model:** Codex 5.5
**Scope:** Split E-2 into two builders to match the ADR's discrete-vs-continuous
separation. **E-2A can land before E-8/E-9; E-2B must land after or with E-8/E-9**
because it depends on `AccumulatorRole` / `ArenaRegistry` semantics.

#### PR E-2A — `resource_transfer_discrete(...)`

**Status:** **Landed** — first-class exact discrete transfer builder in `simthing-core`;
compiles to C-8c `SubtractFromSource` single-source transfer shape. No new GPU primitive.
E-2B remains blocked on E-8/E-9.

```rust
pub fn resource_transfer_discrete(
    source_slot: u32,
    source_col:  u32,
    target_slot: u32,
    target_col:  u32,
    amount:      f32,
) -> AccumulatorOp
```

Sets `combine: Identity`, `gate: Always`, `scale: Constant(amount)`,
`consume: SubtractFromSource`. GPU upload bridge:
`discrete_transfer_registrations_to_transfer` → `plan_transfer_ops` / `sync_transfer_accumulator`.

**Test:** `crates/simthing-sim/tests/e2a_resource_transfer_discrete_builder.rs` — exact
debit/credit, insufficient-source clamp, zero no-op, invalid amount rejection, C-8c shape parity.  
**Acceptance:** Tests pass. C-8c exact transfer regressions remain green.

#### PR E-2B — Resource Flow enrollment compilation

**Status:** **Done (E-2B-1…4 static session-open slice; E-2B-5 Policy A dynamic fission enrollment)** — authored `EnrollmentSelectorSpec` on `ArenaSpec`; `resolve_resource_flow_enrollment` resolves `InstallTargetSpec` → live `ExplicitParticipantSpec` at session install; `react_to_fission_resource_flow_enrollment` admits fission children as arena-root sibling participants (inherit-only, contiguous append). Existing E-10R/E-10R2/E-10R3 scaffold and E-11 flat-star sync reused. **No legacy `resource_flow_participant` AccumulatorOp builder.**

```rust
pub enum EnrollmentSelectorSpec {
    ExplicitOnly,
    InstallTarget(InstallTargetSpec),
}
```

**Test:** `resource_flow_enrollment_roundtrip`, `resource_flow_enrollment_compile`, `resource_flow_enrollment_session`.  
**Acceptance:** RON-native selector enrollment → session open → E-11 flat-star allocation without `fill_explicit_participants`. Flag default false.

---

### PR E-3 — Conjunctive recipe builder + lift CPU-side N≤4 cap

**Model:** Composer 2.5
**Scope:** Two parts:

**(a)** Add `AccumulatorOpBuilder::conjunctive_recipe(...)`:

```rust
pub fn conjunctive_recipe(
    inputs:      &[(SlotId, SubFieldRole, f32)],  // (slot, col, unit_cost) — arbitrary N
    target_slot: SlotId,
    target_col:  SubFieldRole,
    max_per_tick: u32,
) -> AccumulatorOp
```

(E-3R: builder parameter renamed to `throttle_hint_max_per_tick`; not enforced on GPU.)

Sets `source: ConjunctiveCrossing`, `combine: MinAcrossInputs`,
`consume: SubtractFromAllInputs`. The recipe IS the registration.
Conservation structurally enforced. E-3 emits all affordable exact units;
`throttle_hint_max_per_tick` is registration metadata only (E-3R).

**(c) E-3R remedial:** Rename/harden `max_per_tick` → `throttle_hint_max_per_tick`;
document that per-tick throttling is not GPU-enforced until a later cap mechanism.
E-4 RON must not promise recipe throttling without that mechanism.

**(b)** Lift the `inputs.len() > 4` CPU-side cap in
`crates/simthing-core/src/accumulator_op.rs::AccumulatorOp::validate`. The GPU
input-list table (C-8c, binding 10) already supports arbitrary N via
`ensure_capacity`. The 4-input limit is a stale CPU-side holdover from the
pre-input-list inline-array layout. Remove the limit; add a test exercising
N=8 inputs.

**Test:** (a) Run the multichannel `factory_1k` fixture (iron/energy/labor) and
an N=8 fixture through the production builder. Conjunctive recipe builder must
preserve **exact per-recipe conservation** for `ExactDeterministic` fixtures.
Any tolerance applies only to historical workshop baseline comparisons and must
not weaken conservation tests. (b) Unit test that `AccumulatorOp::validate`
accepts N>4 conjunctive inputs.
**Acceptance:** Both tests pass.

---

### PR E-4 — Economic V1 RON fixture format and session integration

**Model:** Composer 2.5  
**Scope:** Define the modder-facing RON format for economic properties and
wire it into `simthing-spec`'s session assembly:

```ron
// economic_resource.ron
(
    property: "iron_ore",
    namespace: "economy",
    kind: Resource,
    accumulator: (
        initial_pool: 100000.0,
        transfer_rate: 1.0,
    ),
    recipe_input: (
        unit_cost: 5.0,
    ),
)
```

The `simthing-driver` session assembly translates RON resource specs into
`AccumulatorOp` registrations (E-1 through E-3 builders) at session open.
No changes to `simthing-sim`. The sim stays spec-free.

**E-3R gate:** Do not wire RON `throttle_hint_max_per_tick` (or legacy
`max_per_tick`) as an enforced production cap. E-3 emits all affordable exact
recipe units; per-tick throttling requires a later explicit GPU-resident cap.

**Test:** A three-channel faction/factory session assembled from RON fixtures
produces the same emission counts and conservation as the direct-builder test
in E-3.  
**Acceptance:** RON → session → 100-tick run → conservation check passes.

---

### PR E-5 — Economic V1 compact log integration

**Model:** Composer 2.5  
**Scope:** Wire economic emission records into the existing `BoundaryDeltaEntry`
log:
- New variant: `BoundaryDeltaEntry::AccumulatorEmission { property_id, emit_count, tick }`
- Written from the compact emission buffer readback in `AccumulatorOpSession`
- Replay: applying `AccumulatorEmission` entries to a fresh session reproduces
  the final state within the soft-aggregate tolerance

**Test:** Record 100 ticks, replay from delta log, assert final resource totals
match within 1%.  
**Acceptance:** Replay test passes.

---

### PR E-6 — Update design_v7.md and economic docs (covered by v7.5 bump)

**Status (2026-05-26):** **Substantially landed via the v7.5 bump** that
accompanied the Resource Flow Substrate ADR. `design_v7.md` §2 (constitution),
§5.1 (Pattern 4), §5.4 (renamed to per-recipe conservation), §5.5 (continuous
flow conservation), §9 (invariants pointer), §10 (read order) all landed.

**Remaining E-6 scope:** Update `design_v7.md` §6 (Logging tiers) to clarify
that allocator disbursements do not produce emission records (they are not
threshold-gated), and surface via summary diff only. Add a worked example to
§5 showing a complete Pattern 4 arena (e.g. food: faction → planets → districts
with one inbound coupling from trade_access).

**Model:** Codex 5.5
**Acceptance:** Logging-tier clarification landed. Worked example landed.

---

## Phase E continued — Resource Flow Substrate landing (E-7 through E-11)

These five PRs land the Resource Flow Substrate per
`docs/adr/resource_flow_substrate.md`. The substrate is a registration
discipline on top of AccumulatorOp v2; no new GPU primitive is introduced.

**PR sequencing:** E-7 through E-10 landed (#149–#153). **Pre-E-11 prerequisites landed** (E-10R, E-8R, E-7R, E-10R2, E-10R3). **E-11 allocation execution landed** (#159, `8a628ca`). E-1, E-3, E-5 remain independent.

### PR E-7 — `governed_by` planner generalization

**Model:** Composer 2.5
**Scope:** Generalize the C-7 `IntegrateWithClamp` planner from special-casing
`(Amount, Velocity)` governed pairs to supporting arbitrary `(Named, Named)`
pairs. The kernel `COMBINE_INTEGRATE_CLAMP` branch in
`crates/simthing-gpu/src/shaders/accumulator_op.wgsl` is **unchanged** — it
already operates on `(governed_offset, governing_offset, dt, clamp_bounds)`
and does not depend on role names. Only the planner needs to compile arbitrary
governed pairs.

This enables `Balance` integrating from `Flow` (the core of Pattern 4) without
touching the velocity-integration path.

**Test:** Bit-exact parity against the existing C-7 velocity integration for
`(Amount, Velocity)` pairs. New test: `(Named("balance"), Named("flow"))` pair
integrates correctly on a synthetic arena fixture.
**Acceptance:** Existing C-7 tests still green; new governed-pair test passes.

---

### PR E-8 — `accumulator_spec: Option<AccumulatorSpec>` lands on `SubFieldSpec`

**Model:** Codex 5.5
**Scope:** Add the planned `accumulator_spec` field to
`crates/simthing-core/src/property.rs::SubFieldSpec`. Schema per
`docs/adr/resource_flow_substrate.md` §"Substrate shape":

```rust
pub struct SubFieldSpec {
    // existing fields unchanged
    #[serde(default)]
    pub accumulator_spec: Option<AccumulatorSpec>,
}

pub struct AccumulatorSpec {
    pub role:     AccumulatorRole,
    pub log_tier: LogTier,
}

pub enum AccumulatorRole {
    IntrinsicFlow,
    AllocatedFlow { arena: ArenaName },
    Balance(BalanceSpec),
    AllocatorWeight { arena: ArenaName },
}

pub struct BalanceSpec {
    pub unit_cost: Option<f32>,
    pub num_count_source: Option<NumCountSource>,
}

pub enum NumCountSource {
    Static(u32),
    Column { property_id: SimPropertyId, role: SubFieldRole },
}

pub type ArenaName = String;
```

**Critical invariant:** `AccumulatorRole` is **compile-time spec metadata only**.
It must not become runtime semantic branching in `simthing-sim`. By the time
`AccumulatorOp` registrations reach the sim crate, the role has compiled away
into specific combine/gate/consume choices.

**Test:** Serde roundtrip for every variant. Unit test that a SubFieldSpec
without `accumulator_spec` is unchanged in behavior (None is the default).
**Acceptance:** CI green. No `AccumulatorRole` match arms in `simthing-sim`.

---

### PR E-9 — `ArenaRegistry` in `simthing-driver` with incremental refresh

**Model:** Composer 2.5
**Prerequisites:** E-7 + E-8
**Scope:** Implement `ArenaRegistry` per `docs/adr/resource_flow_substrate.md`
§"Substrate shape". Lives in `simthing-driver` as session-owned state.

```rust
pub struct ArenaRegistry {
    pub arenas:       Vec<GpuArenaDescriptor>,
    pub participants: Vec<(ArenaIdx, SlotId)>,
    pub couplings:    Vec<ArenaCoupling>,
    pub generation:   u64,
}

// + GpuArenaDescriptor, ArenaCoupling, CouplingDelay, CouplingTransform,
//   FissionPolicy { Inherit, Reevaluate, Reject } — no Custom in v1.
```

**Boundary refresh API:** `ArenaRegistry::refresh_for_structural_mutation(&mut
self, mutated_subtree: &SubtreeId)`. Refresh re-evaluates admission selectors
**only for the affected subtree**, not the global registry. Modeled on the
B2 Approach B append-only threshold rebuild pattern. Naive global refresh on
every fission is **forbidden** — it creates a boundary-time bloat vector. The
expansion report updates correspondingly.

The driver compiles registry → flat `AccumulatorOp` registrations through
existing `WorldGpuState::sync_accumulator_*_session` paths. `simthing-sim`
remains arena-ignorant.

**Test:** Three-arena synthetic fixture (food, research, suppression) with
three coupling edges. Verify registry construction; verify fission scenario
refreshes only the affected subtree (bump `generation` selectively, not
globally). Verify the driver emits the correct flat `AccumulatorOp` set.
**Acceptance:** All tests pass. `simthing-sim` does not gain any
`ArenaRegistry` import.

---

### PR E-10 — `simthing-spec` admission framework

**Status:** Done (#153)
**Model:** Composer 2.5
**Prerequisites:** E-9
**Scope:** Implement the draconian content guardrail framework per
`docs/adr/resource_flow_substrate.md` §"Draconian content guardrail".

Spec compiler enforces at session build time (rejection, not warning):

1. Explicit participation only (property possession ≠ admission)
2. Hard caps per arena (`max_participants`, `max_coupling_fanout`,
   `max_orderband_depth`)
3. Wildcard discipline (declared upper bound; compiler computes expansion)
4. `FissionPolicy` declared per arena (from
   `{Inherit, Reevaluate, Reject}` — no `Custom` in v1)
5. Cycle-with-delay check (no cycle whose edges are all `Algebraic`)
6. OrderBand budget verified against declared `max_orderband_depth`
7. No hidden fanout exceeding declared budget

**Expansion report:** the compiler produces a per-build report listing
per-arena participant counts, per-coupling fanout, total registration count,
total OrderBand depth used, and any rejected-risk diagnostics.

**Test:** Fixture suite of intentionally-bad specs (implicit participation,
cap violation, cycle without delay, etc.), each must be rejected with a
specific diagnostic. Fixture suite of well-formed specs must compile and
produce expected expansion reports.
**Acceptance:** All test fixtures pass; expansion report format stable.

---

### ✅ PR E-11 — Hierarchical allocation kernel pattern + CPU oracle parity

**Model:** Opus (review and design pseudocode), Composer 2.5 (implementation)
**Status:** **Done (flat-star vertical slice)** — PR #159 (`8a628ca`). **E-11R** PR #160 landed. **Burn-in scaffold** landed. **Controlled burn-in scenario fixtures** landed. **Controlled opt-in CI soak** landed. Flat-star D=2 GPU path; nested GPU deferred pending E-11B readiness review. **E-11B readiness review landed** — see [`e11b_nested_hierarchy_gpu_readiness_review.md`](reviews/e11b_nested_hierarchy_gpu_readiness_review.md). Flag **default false**; soak active in opt-in tests only. No new WGSL; `simthing-sim` remains arena-ignorant.
**Why Opus:** E-11 is a real new GPU production capability. Although it reuses
the existing AccumulatorOp kernel, it is structured as a reverse-direction
OrderBand sweep with per-intermediate weight reductions and per-child share
computations. The composition is novel; verification needs its own parity
tests against a CPU oracle and stability tests under hierarchical fanout.

**Prerequisites:** E-9, E-10, **E-10R, E-8R, E-7R, E-10R2, E-10R3** (landed)
**Landed modules:** `arena_hierarchy`, `arena_allocation_oracle`, `arena_allocation_plan`, `child_share_eml` (EML formula registration), `arena_allocation_sync` (session flag wiring).
**Substrate:** `SourceSpec::SlotRange { start, count, col }` — explicit gather column for up-sweep into `intrinsic_flow_sum` / `weight_sum`.
**Tests:** `crates/simthing-driver/tests/e11_arena_allocation.rs` — 14 tests including CPU/GPU parity, zero-weight no-NaN, multi-level oracle, depth budget, fission gap, integration band ordering, no new WGSL, no simthing-sim arena imports.
**Constitution:** no new WGSL; no new `AccumulatorRole`; `simthing-sim` arena-ignorant; E-2B blocked unless enrollment compilation explicitly lands.
**Gate:** Readiness review complete ([`e11_readiness_review.md`](workshop/e11_readiness_review.md)). Implemented per [`e11_implementation_handoff.md`](workshop/e11_implementation_handoff.md).
**Scope:** Implement the allocation kernel pattern per
`docs/adr/resource_flow_substrate.md` §"Hierarchical allocation kernel
pattern". Per intermediate participant, the driver emits two AccumulatorOp
registrations:

```
1. Weight-sum reduction (upward sweep, alongside intrinsic Flow):
   source:  SlotRange { children }
   combine: Sum
   gate:    OrderBand(reduction_band)
   consume: ResetTarget
   target:  intermediate.weight_sum

2. Per-child disbursement (downward sweep):
   source:  SlotValue { intermediate, budget_col }
   combine: EvalEML { child_share_formula }
              where formula = select(weight_sum > 0,
                                      budget * child_weight / weight_sum,
                                      0)
   gate:    OrderBand(allocation_band)
   consume: AddToTarget
   target:  child.allocated_flow_col
```

`child_share_formula` is a fixed EML tree (well within the 32-node
`ExactDeterministic` class limit). One tree per arena, not per intermediate.
The EML `SELECT` op handles the `weight_sum == 0` case without kernel
modification.

OrderBand budget per arena: `2 × tree_depth` (reduction + allocation).

**Tests:**

1. **Parity test:** CPU oracle of the hierarchical allocation against the
   GPU implementation. Bit-exact for fixed tree topology under
   `ExactDeterministic` EML class.
2. **Stability test:** Hierarchical fanout under varying child counts
   (10/100/1000 children per intermediate). Conservation drift bounded by
   O(ε × n_children) per level as specified in the ADR. Replay bit-exact.
3. **Zero-weight test:** All children have zero demand. Verify
   `weight_sum == 0` produces zero disbursement and budget integrates to
   parent Balance via standard `governed_by`.
4. **Conservation test:** End-to-end arena (3 levels, 100 leaf participants)
   over 100 ticks: total intrinsic_flow + coupling_in = total leaf
   allocations + Balance changes. Verified within O(ε × n_levels ×
   n_children).

**Acceptance:** All four tests pass. CPU oracle parity is bit-exact for
`ExactDeterministic`. Replay is bit-exact under varying fission cascades.

---

## Phase T — Production transfer/emission registration ownership

**Design gate:** [`docs/reviews/transfer_emission_registration_ownership_opus_review.md`](reviews/transfer_emission_registration_ownership_opus_review.md)
(Opus, 2026-05-27, **Accepted**). No stop conditions triggered.

**T-1 status:** **Done** — PR #165 (`b2557e6`). `simthing-spec::spec::resource_economy` authoring types + `resource_economy_roundtrip` RON suite (12/12). No compile pass, driver materialization, session integration, or GPU changes yet. Transfer/emission flags remain default false.

**T-2 status:** **Done** — PR #166 (`986bc99`). `simthing-spec::compile::resource_economy` validation + `CompiledResourceEconomy` artifact + `ResourceEconomyExpansionReport`. Rejection suite (11) + positive/expansion report suite (8) + roundtrip (12) green. Transfer/emission flags remain default false.

**T-3 status:** **Done** — PR #167 (`05f8b10`). `simthing-driver::resource_economy_compile` materializes `CompiledResourceEconomy` into existing transfer/recipe/emission/threshold registration shapes. Stable emission `reg_idx` from authoring identity tested (11 driver tests). `ResourceEconomyRegistry` generation scaffold added. No WGSL changes. Transfer/emission flags remain default false.

**T-4 status:** **Done** — PR #168 (`92733c2`). Session integration + boundary refresh for materialized resource economy registrations. Uses existing `sync_accumulator_transfer_session` / `sync_accumulator_emission_session` paths via `resource_economy_sync` → `WorldGpuState::sync_{transfer,emission}_accumulator`. Flag-off populated-spec rejection enforced on boundary sync. Generation-keyed skip landed. Live slot resolution replaced T-3 property-id placeholder in session path. No WGSL changes. No CPU fallback. Transfer/emission flags remain default false.

**T-5 status:** **Done** — PR #169 (`91bdae3`). Boundary refresh / replay / 100-tick conservation burn-in for resource economy registrations. Uses existing transfer/emission accumulator sync paths. Replay determinism tested. Exact discrete transfer conservation tested. Recipe/emission oracle tests landed. No WGSL changes. No CPU fallback. Transfer/emission flags remain default false.

**T-6 status:** **Done** — direct commit `3294e6f`. Limited opt-in scenario flagging for resource economy transfer/emission execution. `ResourceEconomyOptInMode` is explicit and defaults disabled. Global transfer/emission flags remain default false; only explicitly opted-in scenarios enable the existing AccumulatorOp transfer/emission paths. T-5 burn-in remains green. No WGSL changes. No CPU fallback. `simthing-sim` remains spec-free and semantic-free. **Phase T complete.**

**D-1 status:** **Done** — [`docs/reviews/d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md). Discrete-transaction contention current-state audit and implementation recommendations. No production code changes. Phase T remains complete in default-off / explicit-opt-in posture. D-2 GPU allocator remains deferred.

**D-2a readiness status:** **Done** — [`docs/reviews/d2a_boundary_transaction_scheduling_readiness.md`](reviews/d2a_boundary_transaction_scheduling_readiness.md). Boundary transaction scheduling readiness review. No production code changes. Phase T remains complete. Phase T designer/RON smoke addendum remains landed. Hard-currency transfer remains exact discrete AccumulatorOp transfer/recipe/emission. Resource Flow remains separate. Bounded `FlatStarResourceFlow` posture unchanged. Global Resource Flow default-on remains deferred. **Recommendation: defer D-2a implementation** until a named multi-transaction product scenario requires sequential cross-band ordering; narrow driver-only ladder documented if approved later. No WGSL. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains spec-free and semantic-free.

**Next gate:** **Pause implementation (F)** per [`product_priority_vertical_slice_selection.md`](reviews/product_priority_vertical_slice_selection.md) — gather product requirements and name a scenario before authorizing D-2a, E-11B-5, spec/RON rebuild, new vertical slice, or additional soak. **E-11B remains paused.** Global default-on remains deferred.

**Product-priority vertical slice selection status:** **Done** — [`docs/reviews/product_priority_vertical_slice_selection.md`](reviews/product_priority_vertical_slice_selection.md). **No production code changes.** **Recommendation: F — pause and gather product requirements.** No named product scenario currently justifies D-2a, E-11B-5, simthing-spec/RON rebuild, new runtime vertical slice, or additional flat-star soak. Continued flat-star Resource Flow soak remains landed and green. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. E-11B remains paused; E-11B-5 requires a named nested dynamic Resource Flow scenario. D-2a remains deferred until a named hard-currency ordering scenario exists. simthing-spec/RON/Designer guardrail rebuild remains deferred until that track intentionally opens. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Resource Flow remains separate from Phase T hard-currency transfer/recipe/emission. Next implementation gate depends on the review recommendation.

**Continued flat-star soak status:** **Done** — [`resource_flow_flat_star_continued_soak.rs`](../crates/simthing-driver/tests/resource_flow_flat_star_continued_soak.rs) + [`FlatStarContinuedSoakSummary`](../crates/simthing-driver/src/resource_flow_flat_star_continued_soak.rs). Continued flat-star Resource Flow soak checkpoint landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. The checkpoint adds confidence/observability only and does not expand Resource Flow semantics. Static 512-participant, skewed-weight, Policy A dynamic fission, multi-arena, and replay determinism fixtures at 1000 ticks. E-11B remains paused; E-11B-5 is not authorized without a named nested dynamic Resource Flow scenario. D-2a remains deferred. Global flag default false. No WGSL. No CPU fallback. `simthing-sim` remains arena-ignorant.

**Workspace hygiene / paused-state consistency status:** **Done** — cosmetic export formatting fix in `simthing-driver` `lib.rs`; stale local `docs/tests/` artifacts removed. **No runtime behavior changes.** Implementation remains paused pending a named product scenario. The provisional mapping/location proposal should not trigger generic product scenario templates, broad simthing-spec/RON/Designer guardrail work, or runtime mapping implementation yet. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. E-11B remains paused; E-11B-5 requires a named nested dynamic Resource Flow scenario. D-2a remains deferred until a named hard-currency ordering scenario exists. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Resource Flow remains separate from Phase T hard-currency transfer/recipe/emission. No WGSL changes. No new AccumulatorRole variants. No CPU fallback. `simthing-sim` remains arena-ignorant and spec-free.

**E-11B-1 explicit nested participant materialization status:** **Done** — `ExplicitParticipantSpec.parent_subtree_root_id` optional field; `materialize_arena_participants` builds nested `ArenaParticipant` topology with depth-first allocation and per-parent child contiguity; `build_execution_plan` already dispatches to `build_nested_layout` when nested topology exists. E-11B explicit nested participant materialization landed. This is a narrow static materialization fix for future deep arena use cases, including provisional mapping/location hierarchy work. No mapping runtime behavior was implemented. No dynamic nested enrollment was implemented. Flat-star behavior remains backwards compatible when `parent_subtree_root_id` is `None`. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. No WGSL changes. No new AccumulatorRole variants. No CPU fallback. No slot compaction. No indirection-list SlotRange replacement. No Policy B Reevaluate. `simthing-sim` remains arena-ignorant and spec-free. E-11B-5 dynamic nested enrollment remains paused.

**E-11B static nested participant RON smoke status:** **Done** — [`resource_flow_nested_participant_roundtrip.rs`](../crates/simthing-spec/tests/resource_flow_nested_participant_roundtrip.rs) + [`e11b_nested_materialization_ron_session.rs`](../crates/simthing-driver/tests/e11b_nested_materialization_ron_session.rs). E-11B static nested participant RON smoke landed. `parent_subtree_root_id` remains an optional static authoring field for explicit Resource Flow participants. RON-authored D=3/D=4 explicit nested participant specs materialize into nested `ArenaParticipant` topology and reach `build_nested_layout`. Flat-star Resource Flow authoring remains backwards-compatible when `parent_subtree_root_id` is omitted. Pending mapping/location work may use static deep hierarchy materialization later, but no mapping runtime behavior was implemented. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. E-11B-5 dynamic nested enrollment remains deferred until a named scenario requires it. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. No WGSL changes. No new AccumulatorRole variants. No CPU fallback. No slot compaction. No Policy B Reevaluate. `simthing-sim` remains arena-ignorant and spec-free.

**RegionCell field-intelligence sandbox status:** **Reverted** — PR #197 removed after external concept validation. Implementation remains parked after E-11B-1 explicit nested participant materialization and E-11B static nested participant RON smoke. Static deep hierarchy authoring via `parent_subtree_root_id` remains landed. RON-authored D=3/D=4 explicit nested participant specs reach `build_nested_layout`. The sparse RegionCell field-intelligence sandbox was reverted after validating the concept externally; no sandbox test/prototype remains in the production repo. Mapping/location architecture remains provisional. Do not implement mapping/location runtime until the mapping doc is ready. Do not open generic scenario templates, simthing-spec/RON/Designer guardrail rebuild, E-11B-5, D-2a, Scatter/Gather, wavefront propagation, or new WGSL. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Resource Flow remains separate from Phase T hard-currency transfer/recipe/emission. `simthing-sim` remains arena-ignorant and spec-free.

**SEAD field-intelligence sandbox status:** **Reverted** — PR #200 probe merged then reverted to parked state (PR #201). SEAD field-intelligence sandbox completed and was reverted to parked state. The sandbox preserved its source at [`docs/workshop/archive/sead/sead_sandbox_code_preserve.rs`](workshop/archive/sead/sead_sandbox_code_preserve.rs) and its decision-gate results at [`docs/tests/sead_field_intelligence_sandbox_test_results.md`](tests/sead_field_intelligence_sandbox_test_results.md). No sandbox test/prototype remains in the production test suite. Overall probe verdict: **PARTIAL** — P1 later-band propagation, P2 GPU EvalEML urgency, P3 ScaleTarget dissipation substrate-real; velocity DEFERRED; corridor gradient PARTIAL. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, new WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD strategic horizon / velocity / PF-skip sandbox status:** **Reverted** — PR #202 probe merged then reverted to parked state. SEAD strategic horizon / velocity / PF-skip feasibility sandbox completed and was reverted to parked state. The sandbox source and decision-gate results are preserved at [`docs/workshop/archive/sead/sead_strategic_horizon_sandbox_code_preserve.rs`](workshop/archive/sead/sead_strategic_horizon_sandbox_code_preserve.rs) and [`docs/tests/sead_strategic_horizon_sandbox_test_results.md`](tests/sead_strategic_horizon_sandbox_test_results.md). No sandbox test/prototype remains in the production test suite. Overall probe verdict: **PARTIAL** — strategic horizon directional at H=8; explicit-column velocity PASS; PF skip PARTIAL. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD operator toolkit sandbox status:** **Reverted** — PR #204 probe merged then reverted to parked state. SEAD operator toolkit sandbox completed and was reverted to parked state. The sandbox source and decision-gate results are preserved in [`docs/workshop/archive/sead/sead_operator_toolkit_sandbox_code_preserve.rs`](workshop/archive/sead/sead_operator_toolkit_sandbox_code_preserve.rs) and [`docs/tests/sead_operator_toolkit_sandbox_test_results.md`](tests/sead_operator_toolkit_sandbox_test_results.md). No sandbox test/prototype remains in the production test suite. Overall probe verdict: **PARTIAL** — directed_decayed + hierarchy-first toolkit substrate-real; frontier cadence direction and scale cost remain constraints. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, new WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD tensor/stencil WGSL prototype sandbox status:** **Reverted** — PR #206 probe merged then reverted to parked state. SEAD tensor/stencil WGSL prototype sandbox completed and was reverted to parked state. The sandbox source, WGSL prototype(s), notes, and decision-gate results are preserved in [`docs/workshop/archive/sead/sead_tensor_stencil_wgsl_sandbox_code_preserve.rs`](workshop/archive/sead/sead_tensor_stencil_wgsl_sandbox_code_preserve.rs), [`docs/workshop/archive/sead/sead_tensor_stencil_prototype.wgsl`](workshop/archive/sead/sead_tensor_stencil_prototype.wgsl), and [`docs/tests/sead_tensor_stencil_wgsl_sandbox_test_results.md`](tests/sead_tensor_stencil_wgsl_sandbox_test_results.md). No sandbox test/prototype remains in the production runtime/test suite. Overall probe verdict: **PARTIAL** — general StructuredFieldStencilOp candidate confirmed; ~11× projected 30k speedup; normalized_stencil H=8 directional. Mapping/location architecture remains provisional. Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario. No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, production WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution.

**SEAD tensor/stencil WGSL refinement sandbox status:** **Reverted** — PR #208 probe merged then reverted to parked state. Workshop artifacts preserved; findings informed V7.6 promotion below.

**V7.6 StructuredFieldStencilOp status:** **Live, parked** — generic GPU primitive promoted from workshop evidence and guardrail-hardened. See [`docs/design_v7_6.md`](design_v7_6.md). Live code: [`structured_field_stencil.rs`](../crates/simthing-gpu/src/structured_field_stencil.rs), [`structured_field_stencil.wgsl`](../crates/simthing-gpu/src/shaders/structured_field_stencil.wgsl). Opt-in library API only — not wired into default production pass graph. **Implementation parked pending Mapping ADR.** Mapping optimization toolkit sandbox (PR #213, reverted) and remedial probe (PR #215, reverted) informed ADR: atlas batching adopt-provisional with G≥H gutter, cadence tiers adopt-now, dirty skip adopt-now, H-hop halo adopt-provisional, caller-managed source adopt-now, behavioral source defer pending source_mask API. See [`mapping_optimization_toolkit_sandbox_test_results.md`](tests/mapping_optimization_toolkit_sandbox_test_results.md) and [`mapping_optimization_remedial_sandbox_test_results.md`](tests/mapping_optimization_remedial_sandbox_test_results.md). No mapping runtime landed. `PipelineFlags::default().use_accumulator_resource_flow` remains false.

**Update (2026-05-28):** Mapping ADR **approved at the architecture level** — [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md), surfaced in [`design_v7_7.md`](design_v7_7.md). Approval unlocks **Phase M** generic natives (M-1–M-4; M-5 deferred) and names the first scenario-level slice. No mapping runtime is authorized: the compute already exists (StructuredFieldStencilOp kernel/WGSL/ping-pong/oracle + existing AccumulatorOp Layers 2–3), so "runtime" is only first-slice session-graph wiring, gated after the Phase M natives. `simthing-sim` remains semantic-free; no default changed.

**E-11B status:** **Paused** after kickoff + E-11B-4 fission/gap hardening + nested dynamic enrollment readiness review. E-11B paused after nested static GPU parity, fission/gap hardening, and nested dynamic enrollment readiness review. Nested D=3/D=4 static hierarchy materialization remains landed and GPU-parity covered. Nested reserved-gap children remain non-leaf unless explicitly admitted by a future nested enrollment gate. Nested dynamic enrollment is deferred until a named product scenario requires it. **E-11B-5 is not authorized** until product names a nested dynamic Resource Flow scenario. Future E-11B-5 must be narrow: explicit nested admission, contiguous extension or visible rejection, generation/sync reporting, replay/parity tests. FlatStarResourceFlow remains the accepted bounded production Resource Flow posture. E-11B remains an explicit nested extension and does not make Resource Flow global default-on. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Policy B Reevaluate remains deferred. D-2a remains deferred until a named hard-currency product scenario needs sequential cross-band ordering. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Resource Flow remains separate from hard-currency transfer. Designer-facing spec/RON guardrail rebuild remains deferred.

**E-11B readiness status:** **Done** — [`docs/reviews/e11b_nested_hierarchy_gpu_readiness_review.md`](reviews/e11b_nested_hierarchy_gpu_readiness_review.md). Nested hierarchy GPU execution/materialization current-state audit. No production code changes. Phase T remains complete. D-1 remains landed; D-2 GPU allocator remains deferred. `use_accumulator_resource_flow` remains default false.

**E-11B nested dynamic enrollment readiness status:** **Done** — [`docs/reviews/e11b_nested_dynamic_enrollment_readiness.md`](reviews/e11b_nested_dynamic_enrollment_readiness.md). Post–E-11B-4 audit for nested dynamic enrollment. **No production code changes.** **Recommendation: defer** nested dynamic enrollment until a named product scenario requires it; authorize narrow E-11B-5 ladder if product prioritizes. Policy B Reevaluate remains deferred unless Opus escalation triggered. Gap children do not become allocation leaves without explicit nested admission gate.

**E-11B implementation status (landed, track paused):** **Kickoff + fission/gap hardening landed.** Nested D=3/D=4 static Resource Flow hierarchy materialization has GPU parity coverage via existing AccumulatorOp v2 OrderBand reduction/allocation bands. **E-11B nested fission/gap hardening landed:** reserved-gap children preserve active child SlotRange and do not become allocation leaves unless explicitly admitted by a future nested enrollment gate; D=3/D=4 nested GPU parity remains green for active trees after safe gap claims; attaching gap fission children to nested topology rejects without slot compaction. **Track paused** — no further E-11B implementation until product names a scenario for E-11B-5 or another explicit nested extension.

**E-2B-5 status:** **Done (Policy A implementation + E-2B-5R hardening)** — [`react_to_fission_resource_flow_enrollment`](../crates/simthing-driver/src/resource_flow_fission_enrollment.rs) wired in `SimSession` boundary path. Fission children inherit parent arena membership and are admitted as arena-root sibling participants when capacity and contiguous-slot extension allow. **E-2B-5R landed:** dynamic fission enrollment atomicity and visible diagnostics hardening. Failed dynamic enrollment cannot leave partial tree/scaffold/registry state. Boundary-time dynamic enrollment reports are retained/inspectable via `last_resource_flow_dynamic_enrollment_report`. Policy B Reevaluate remains deferred (mapped to inherit-only). Gap-only enrollment reserved for future E-11B nested hierarchy semantics; not used for flat-star leaf disbursement. Soak: PR #178.

**E-2B-5R status:** **Done** — two-phase prepare/commit dynamic admission; registry failure rolls back allocator slot; conditional Resource Flow sync only after successful admissions when flag enabled.

**E-2B-5 soak status:** **Done** — [`resource_flow_dynamic_enrollment_soak.rs`](../crates/simthing-driver/src/resource_flow_dynamic_enrollment_soak.rs) + [`e2b5_dynamic_enrollment_soak.rs`](../crates/simthing-driver/tests/e2b5_dynamic_enrollment_soak.rs). Resource Flow dynamic enrollment soak landed (PR #178). E-2B-5R remained atomic under soak.

**Resource Flow default-on readiness status:** **Done** — [`resource_flow_default_on_readiness_review.md`](reviews/resource_flow_default_on_readiness_review.md). **Recommendation B:** limited scenario-class default-on readiness may proceed; **global default-on rejected (D)**. No production code changes. `use_accumulator_resource_flow` remains default false.

**RF-T1 status:** **Done** — limited scenario-class Resource Flow opt-in flagging. `ResourceFlowOptInMode` on `ResourceFlowSpec` enables `FlatStarOptIn` per authored scenario/game mode. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU Resource Flow execution. E-11 flat-star path, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment are reused. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.

**RF-T2 status:** **Done** — limited opt-in scenario burn-in expansion. [`resource_flow_opt_in_burn_in.rs`](../crates/simthing-driver/src/resource_flow_opt_in_burn_in.rs) + [`resource_flow_opt_in_burn_in.rs`](../crates/simthing-driver/tests/resource_flow_opt_in_burn_in.rs). Only explicitly authored `FlatStarOptIn` scenarios enable GPU Resource Flow execution via `open_from_spec`. Static 10/64-participant, skewed-weight, dynamic single/multi fission, two-arena, disabled-populated, wildcard-reject, resync stability, and replay determinism fixtures. Global flag remains default false. E-11B and Policy B remain deferred.

**RF-T3 status:** **Done** — product-like opt-in Resource Flow soak and telemetry surfacing. [`resource_flow_opt_in_telemetry.rs`](../crates/simthing-driver/src/resource_flow_opt_in_telemetry.rs), [`resource_flow_opt_in_product_soak.rs`](../crates/simthing-driver/src/resource_flow_opt_in_product_soak.rs), telemetry + product soak test suites. FlatStarOptIn scenarios emit/record flag-source, sync, arena, participant, generation, dynamic admission/rejection, and parity/replay metrics. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant.

**Resource Flow global/default-on re-review status:** **Done** — [`resource_flow_global_default_on_rereview.md`](reviews/resource_flow_global_default_on_rereview.md). No production code changes. **Recommendation B:** proceed to RF-T4 limited scenario-class default-on; global default-on rejected (D). RF-T1/T2/T3 evidence cited. Global flag remains default false.

**RF-T4 status:** **Done** — limited scenario-class Resource Flow default-on. `ResourceFlowExecutionProfile` on `GameModeSpec`; `FlatStarResourceFlow` profile enables flat-star GPU path at session open with `ScenarioClassDefaultOn` flag source. Spec `FlatStarOptIn` precedence preserved. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild deferred.

**RF-T5 status:** **Done** — scenario-class Resource Flow burn-in / telemetry soak. [`resource_flow_scenario_class_burn_in.rs`](../crates/simthing-driver/src/resource_flow_scenario_class_burn_in.rs) + [`resource_flow_scenario_class_burn_in.rs`](../crates/simthing-driver/tests/resource_flow_scenario_class_burn_in.rs). `FlatStarResourceFlow` profile scenarios soaked through product-like static (128/256), dynamic fission cadence, multi-arena, replay, rejection, and resync fixtures. Scenario-class enablement records `ScenarioClassDefaultOn` flag source and execution-profile attribution. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild deferred to the future simthing-spec rebuild track.

**Resource Flow limited scenario-class production posture status:** **Done** — [`resource_flow_limited_scenario_class_production_posture.md`](reviews/resource_flow_limited_scenario_class_production_posture.md). No production code changes. RF-T1 through RF-T5 remain landed. **Recommendation A:** limited scenario-class `FlatStarResourceFlow` is accepted as the current bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track.

**RF-T6 status:** **Done** — [`resource_flow_limited_scenario_class_posture.md`](resource_flow_limited_scenario_class_posture.md). Production docs / telemetry polish for bounded `FlatStarResourceFlow` posture. No production code changes. Limited scenario-class `FlatStarResourceFlow` remains the accepted bounded production Resource Flow posture. Global `PipelineFlags::default().use_accumulator_resource_flow` remains false. Presence of `ResourceFlowSpec` alone does not enable GPU execution. Spec `FlatStarOptIn` remains supported and takes precedence. E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A dynamic enrollment remain the only covered execution paths. E-11B remains deferred. Policy B Reevaluate remains deferred. No WGSL changes. No new AccumulatorRole variants. No CPU production fallback. `simthing-sim` remains arena-ignorant. Designer-facing spec/RON guardrail rebuild remains deferred to the future simthing-spec rebuild track.

**E-2B readiness status:** **Done** — [`docs/reviews/e2b_resource_flow_enrollment_compilation_readiness.md`](reviews/e2b_resource_flow_enrollment_compilation_readiness.md).

**Posture (preserves v7.5):** runtime substrate is unchanged; ownership of
transfer / recipe / emission / threshold-emit registrations moves to
`simthing-spec` (authoring) → `simthing-driver` (compilation) → existing
`simthing-core` builders and `simthing-gpu` planners. `simthing-sim` remains
spec-free and arena-ignorant. No new WGSL; no new `AccumulatorOp` primitive;
no CPU production fallback peer. Phase T is complete in a default-off /
explicit-opt-in production posture: transfer/emission flags remain globally
default false and are enabled only by scenario/session opt-in.

| PR | Model | Scope | Gate |
|----|-------|-------|------|
| **T-1** | Codex 5.5 | `simthing-spec` authoring types (`resource_economy.rs`) | **Done** — RON roundtrip suite |
| **T-2** | Composer 2.5 | `simthing-spec::compile::resource_economy` resolves keys / EML formulas / validation; extends expansion report | **Done** — rejection + expansion report suites (19/19) |
| **T-3** | Composer 2.5 | `simthing-driver::resource_economy_compile` → `ResourceEconomyRegistrations`; stable `reg_idx` from authoring identity; subtree-scoped refresh | **Done** — materialization + stable reg_idx suites (11/11) |
| **T-4** | Composer 2.5 | Session integration + boundary refresh via existing `sync_accumulator_{transfer,emission}_session` paths; generation-keyed skip; flag-off populated-spec rejection | **Done** — session open + flag-off reject + generation skip suites (8/8) |
| **T-5** | Composer 2.5 | Boundary refresh / replay determinism; 100-tick transfer/recipe/emission conservation burn-in vs CPU oracle; generation-keyed skip + reupload tests | **Done** — PR #169 (`91bdae3`) |
| **T-6** | Codex 5.5 | Limited opt-in scenario flagging / default-off production burn-in decision | **Done** — direct commit `3294e6f` |
| **RF-T1** | Codex 5.5 | Resource Flow execution opt-in (`ResourceFlowOptInMode` on `ResourceFlowSpec`; `SimSession::open_from_spec` applies flag) | **Done** — `resource_flow_opt_in` + roundtrip suites |
| **RF-T2** | Codex 5.5 | Opt-in scenario burn-in expansion (`resource_flow_opt_in_burn_in` fixtures via `open_from_spec`) | **Done** — `resource_flow_opt_in_burn_in` suite (15 tests) |
| **RF-T3** | Codex 5.5 | Product-like opt-in soak + telemetry (`ResourceFlowOptInTelemetryReport`, flag-source attribution, product fixtures) | **Done** — `resource_flow_opt_in_telemetry` (6) + `resource_flow_opt_in_product_soak` (13) |
| **RF re-review** | Codex 5.5 | Global/default-on readiness re-review post–RF-T3 | **Done** — [`resource_flow_global_default_on_rereview.md`](reviews/resource_flow_global_default_on_rereview.md); recommendation B |
| **RF-T4** | Codex 5.5 | Limited scenario-class default-on (`ResourceFlowExecutionProfile`, scenario-class flag source) | **Done** — `resource_flow_scenario_class_default_on` suite (16 tests) |
| **RF-T5** | Composer 2.5 | Scenario-class burn-in / telemetry soak (`FlatStarResourceFlow` profile product soak) | **Done** — `resource_flow_scenario_class_burn_in` suite (16 tests) |
| **RF posture** | — | Limited scenario-class production posture review | **Done** — Recommendation A; bounded `FlatStarResourceFlow` production posture |
| **RF-T6** | — | Production docs / telemetry polish for bounded `FlatStarResourceFlow` posture | **Done** — production-facing guide; no runtime behavior expansion |
| **Phase T RON smoke addendum** | — | Designer-authored `resource_economy` RON fixture through deserialize/compile/install/open path | **Done** — `resource_economy_designer_ron` + `resource_economy_designer_ron_session`; no runtime behavior expansion |
| **D-2a readiness** | — | Boundary transaction scheduling readiness review (post–D-1) | **Done** — [`d2a_boundary_transaction_scheduling_readiness.md`](reviews/d2a_boundary_transaction_scheduling_readiness.md); **defer implementation** |
| **E-11B kickoff** | Composer 2.5 | Nested D=3/D=4 static hierarchy materialization + GPU parity tests | **Done** — explicit nested extension; no default-on, WGSL, roles, CPU fallback, or `simthing-sim` arena awareness |
| **E-11B-4** | Composer 2.5 | Nested fission/gap preservation hardening + diagnostics | **Done** — `e11b_nested_fission_gap` suite (13 tests) |
| **E-11B nested dynamic enrollment readiness** | Composer 2.5 | Docs-first readiness review | **Done** — defer by default; narrow E-11B-5 ladder if product prioritizes |
| **E-11B pause checkpoint** | Composer 2.5 | Product-priority workspace checkpoint | **Done** — E-11B paused; E-11B-5 gated behind named scenario |
| **Continued flat-star soak** | Composer 2.5 | FlatStarResourceFlow confidence/observability checkpoint | **Done** — `resource_flow_flat_star_continued_soak` (12 tests; 512 static @ 1000 ticks) |
| **Product-priority vertical slice selection** | Composer 2.5 | Docs-first track selection review | **Done** — **Recommendation F:** pause; gather product requirements |

**Stop conditions (re-asserted; all unchanged from the v2 ADR):** no new WGSL,
no new `AccumulatorOp` primitive, no `simthing-sim` semantic ownership of
transfer/emission, no CPU production fallback, no weakening of exact transfer
conservation, no folding of hard-currency transfers into continuous Resource
Flow, no flipping of `use_accumulator_resource_flow` to default-on (that is a
separate gate, downstream of T-5 and E-2B).

**Phase T designer/RON smoke addendum landed:** a designer-authored
`resource_economy` RON fixture now exercises `deserialize_game_mode_ron` →
compile/install/`open_from_spec` and a short session run. Transfer, recipe, and
emission authoring remain explicit `ResourceEconomySpec` content.
`ResourceEconomyOptInMode` remains default disabled. Global transfer/emission
flags remain default false. No WGSL changes. No new `AccumulatorRole` variants.
No CPU production fallback. `simthing-sim` remains spec-free and semantic-free.
Resource Flow bounded `FlatStarResourceFlow` posture remains unchanged. Full
simthing-spec/RON/Designer guardrail rebuild remains deferred to its own future
track.

**Acceptance verification (post-T-6):**

```powershell
cargo test -p simthing-driver --test resource_economy_opt_in -- --nocapture
cargo test -p simthing-driver --test resource_economy_burn_in -- --nocapture
cargo test -p simthing-driver --test resource_economy_replay -- --nocapture
cargo test -p simthing-driver --test resource_economy_boundary_refresh -- --nocapture
cargo test -p simthing-driver --test resource_economy_session_open -- --nocapture
cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture
cargo test -p simthing-driver --test resource_economy_flag_off_rejects -- --nocapture
cargo test -p simthing-driver --test resource_economy_compile -- --nocapture
cargo test -p simthing-driver --test resource_economy_stable_reg_idx -- --nocapture
cargo test -p simthing-spec --test resource_economy_roundtrip -- --nocapture
cargo test -p simthing-spec --test resource_economy_designer_ron -- --nocapture
cargo test -p simthing-spec --test resource_economy_compile_rejections -- --nocapture
cargo test -p simthing-spec --test resource_economy_expansion_report -- --nocapture
cargo test -p simthing-gpu accumulator_op -- --nocapture
cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture
cargo check --workspace
cargo test --workspace
```

---

## Phase F — Old pipeline sunset

Phase F begins after Phase C is fully complete. Each sunset PR is Codex 5.5,
mechanical, gated on CI passing with the feature flag set to default-on.

### PR S-1 — Sunset intent fold (after C-2)

**Status:** Done locally. Legacy intent shader and pipeline wiring deleted;
AccumulatorOp intent is the default production path and is mandatory when
player or AI intents are pending.

**Deleted:**
- `crates/simthing-gpu/src/shaders/intent_delta.wgsl`
- Legacy intent pipeline, bind group layout, and dispatch branch in `passes.rs`

**Kept:** `PlayerIntent`/`IntentDelta` intake semantics, CPU-side intent folding,
and AccumulatorOp `COMBINE_AFFINE_INTENT` registration/dispatch.

**Tests:** `s1_intent_sunset.rs`, rewritten `c2_intent_accumulator_parity.rs`;
intent still composes with AccumulatorOp overlay/threshold ordering.
### PR S-2 — Sunset intensity update (after C-8) — **Landed (#138)**

**Status:** Merged. Legacy Pass 2 deleted; EvalEML intensity is the only production path.

**Deleted:**
- `crates/simthing-gpu/src/shaders/intensity_update.wgsl`
- Legacy intensity pipeline, bind group layout, and dispatch branches in `passes.rs`
- `IntensityParams`, `build_intensity_params`, `WorldGpuState::intensity_params`, legacy dispatch counter

**Kept:** `IntensityBehavior`, `compile_intensity_behavior_to_eml`, `intensity_accumulator.rs`, EML `Intensity` consumer.

**Flag posture:**
- `use_accumulator_eml` + `use_accumulator_intensity` default **true**
- `PipelineFlags::validate_intensity_enabled_for_registry` panics when intensity is disabled but the registry has `IntensityBehavior`

**Tests:** `s2_legacy_intensity_sunset.rs`, rewritten `c8b_intensity_eml_parity.rs` (CPU/EML golden), C-8 full integration still green.

**Inventory:** [`docs/workshop/s2_legacy_intensity_sunset_inventory.md`](workshop/s2_legacy_intensity_sunset_inventory.md)

### PR S-4 — Sunset reduction passes 4–6 (after C-5 + C-6)

**Gate:** S-4 can begin only after the readiness checklist in PR C-6 is satisfied
(default-on candidates, parity green, combined all-flags green, no legacy dispatch
when both reduction flags on, CI burn-in).

**S-4 deletion inventory:**
- `crates/simthing-gpu/src/shaders/reduction.wgsl`
- Reduction pipeline creation in `passes.rs`
- Reduction bind group layout if no longer used
- Legacy reduction topology upload branches only if not needed by Accumulator planner
- Legacy reduction standalone `run_reduction_passes` test helper, unless kept as oracle fixture
- Any `skip_soft_columns` plumbing

**Do not delete with legacy shader:** `child_starts`, `child_indices`, `depth_slots`,
column rules, `plan_reduction_orderband`, or topology upload paths still used by
AccumulatorOp reduction planner/tests.
### PR S-5 — Sunset velocity integration (after C-7)

**Status:** Done locally. Legacy velocity shader and pipeline wiring deleted;
AccumulatorOp velocity integration is the default production path and is
mandatory when governed Amount/Velocity pairs exist.

**Deleted:**
- `crates/simthing-gpu/src/shaders/velocity_integration.wgsl`
- Legacy velocity pipeline, bind group layout, and dispatch branch in `passes.rs`

**Kept:** C-7 `IntegrateWithClamp`, governed Amount/Velocity planning, and
bit-exact clamp semantics. E-7 generalized arbitrary governed pairs remain
future work.

**Tests:** `s5_velocity_sunset.rs`, `c7_velocity_accumulator_parity.rs`.

### PR S-6 — Sunset threshold scan / Pass 7 (after C-1)

**Status:** Done locally. Legacy threshold shader and Pass 7 pipeline wiring
deleted; AccumulatorOp threshold scan is the default production path and is
mandatory when threshold registrations exist.

**Deleted:**
- `crates/simthing-gpu/src/shaders/threshold_scan.wgsl`
- Legacy threshold pipeline, bind group layout, and dispatch branch in `passes.rs`

**Kept:** threshold registrations, compact event readback, and AccumulatorOp
`Threshold` + `EmitEvent` dispatch/readback.

**Tests:** `s6_threshold_sunset.rs`, rewritten `c1_threshold_scan_parity.rs`.

After S-1/S-2/S-3/S-4/S-5/S-6, the only retained old operation is snapshot
(`copy_buffer_to_buffer`).

Each sunset PR checklist:
1. Set feature flag default to `true` (AccumulatorOp path)
2. Run `cargo test --all` — must be fully green
3. Delete old WGSL shader file(s)
4. Delete old Rust pass module(s)
5. Remove fallback dispatch branches; compatibility flags must reject real workloads when disabled
6. Update `design_v7.md` §4 to remove the old pass entry
7. Add `SUPERSEDED` annotation to `design_v6.md` §10 entry for this pass

**Model for all sunset PRs:** Codex 5.5  
**Gate:** CI green at step 2 before any deletion proceeds. If step 2 fails,
block sunset and file an issue against the corresponding migration PR.

---

## Phase G — Design document finalization

### PR G-1 — Annotate design_v6.md §10 as superseded

**Model:** Codex 5.5  
**Scope:** Add to the top of `design_v6.md` §10:

```markdown
> ⚠️ SUPERSEDED — The GPU pipeline specification in this section is superseded
> by `docs/adr_accumulator_op_v2.md` and `docs/design_v7.md` §4.
> This section is retained for historical reference only.
> Do not implement from this section.
```

**Acceptance:** One commit. CI green.

### PR G-2 — design_v7.md §4 final review pass

**Model:** Opus  
**Scope:** After all Phase C and Phase F PRs are merged, Opus reads
`design_v7.md` §4 and confirms: (a) every operation family is described
correctly, (b) no old-pass descriptions remain, (c) the pipeline section is
consistent with the invariants doc and the ADR. Produces any needed corrections
as a doc-only PR.

**Gate:** Human + Opus sign-off before G-2 merges.

---

## Phase M — Mapping (Sparse RegionCell) natives

> **Gate:** **Satisfied — `docs/adr/mapping_sparse_regioncell.md` approved
> (architecture) 2026-05-28.** Phase M is unblocked. **Docs cleanup (pre-Phase M)
> completed:** superseded sandbox preserves, revert reports, and full logs archived;
> active guidance consolidated in the ADR, V7.7, invariants, and
> `docs/workshop/mapping_current_guidance.md`. It implements **only the few
> generic natives the ADR needs**; it does **not** implement a mapping runtime,
> does **not** wire any production pass graph to mapping behavior, and changes
> **no default**. Every native is a generic toolkit/driver/spec capability —
> `simthing-sim` stays map-free. The mapping ADR is built on existing primitives
> (`StructuredFieldStencilOp`, AccumulatorOp `SlotRange` Sum reduction, `EvalEML`,
> `Threshold`+`EmitEvent`), so the new-native surface is intentionally small.

The ADR's three-layer model already runs on shipped primitives. What is missing
is the thin generic plumbing that lets a scheduler drive them efficiently and a
designer declare a bounded field safely. That plumbing is Phase M.

### PR M-1 — RegionField execution API on `StructuredFieldStencilOp` (generic) — **Done**

**Model:** Composer 2.5  
**Status:** **Landed** — generic execution API + column-aware reduction convenience; no mapping runtime.

**Update (M-1.1):** **Landed** — configured execution now supports a no-readback dispatch/report path (`readback_values: false` default). Readback explicit for tests/diagnostics; stats remain readback-derived.
- `StructuredFieldStencilOp::execute_configured` with `StructuredFieldExecutionOptions` / `StructuredFieldStencilDebugReport`
- `ColumnAwareReductionSpec` + `column_aware_reduction_op` over existing `SlotRange` Sum (`simthing-core`)
- Tests: `structured_field_stencil.rs`, `structured_field_region_execution.rs`; existing stencil parity unchanged

**Scope (original):** Harden the V7.6 primitive's caller-facing execution surface so a
scheduler can drive it without re-implementing buffer management. Generic only —
no RegionCell/map names in the primitive.
- Confirm/strengthen ping-pong buffer binding and `run_configured_horizon` as the
  safe execution helper (V7.6 hardening already landed the horizon guards;
  M-1 only adds the batch-friendly entry points).
- Add a generic `column_aware_reduce_into(parent_slot, parent_col)` convenience
  over the existing `SlotRange` Sum so Layer-2 reduction is one call.
- Expose the debug surface fields the ADR requires: dispatch count, field max/L1,
  configured vs executed horizon, source policy in effect, active-mask ratio.

**Test:** Generic stencil parity unchanged (ping-pong GPU=CPU bit-exact retained);
new reduce-into convenience matches a manual `SlotRange` Sum bit-for-bit.  
**Acceptance:** CI green. `simthing-gpu` only. No new WGSL. No defaults changed.

### PR M-2 — Cadence scheduler + dirty macro-region skip (driver, adopted opts) — **Done**

**Model:** Composer 2.5  
**Status:** **Landed** — generic `FieldScheduler`, cadence tiers, dirty skip. Driver-only; no pass graph wiring.

**Update (M-2.1):** **Landed** — region identity `(FieldId, FieldRegionId)`; visitor-based scheduled execution orchestration; single-op guard prevents repeated same-buffer dispatch.

**Scope (original):** A generic field scheduler in `simthing-driver` implementing the two
**adopted** optimizations:
- Cadence tiers (`EveryTick | EveryN | OnEvent`) per registered field; the
  scheduler skips non-due fields before command-buffer construction. Deterministic
  and replay-safe (toolkit Test 3).
- Dirty macro-region skipping: skip clean macro-regions before dispatch;
  conservative false-schedules allowed, **false-skips forbidden** (toolkit Test 5,
  false_skips=0). The scheduler is field-generic; it does not know what a field
  *means*.

**Test:** Determinism/replay test on cadence skip; a dirty-skip correctness test
asserting zero false-skips against a full-grid oracle.  
**Acceptance:** CI green. Driver-only. No defaults changed.

### PR M-3 — `RegionFieldSpec` RON + mapping admission framework (spec, designer-layer) — **Done**

**Model:** Composer 2.5  
**Scope:** The designer-facing guardrail layer — the load-bearing safety surface.
In `simthing-spec`:
- `RegionFieldSpec` RON: grid bounds, field columns, operator
  (`normalized_stencil` / `source_capped_normalized`), horizon cap, source cap,
  cadence tier, source policy, optional perception-filter class.
- Field formula-class admission for `field_pressure`/`field_urgency`/`field_decay`/
  `bounded_field_update` at the **designer/spec policy layer** (C-8
  `register_formula` is runtime-sufficient; this removes wrong-layer whitelist
  rejection). Mirrors the V7.6 §1.2 EML guardrail relaxation.
- Admission rejections at **import/session-build**: horizon over cap without
  `allow_extended_horizon`+stability contract; `ActiveOnlyExperimentalNoHalo`
  requested for production; atlas requested without gutter≥H + VRAM reporting;
  behavioral source policy requested without a source-identity buffer; grid over
  declared bounds. These rejections are the mechanism that keeps blowout out of
  the simulation.
- Opt-in execution profile (`MappingExecutionProfile`), default-off; spec presence
  is structure, execution requires explicit opt-in (Resource Flow precedent).

**Landed:** Phase M-3 RegionFieldSpec RON + mapping admission framework.
RegionFieldSpec is designer/spec structure only and compiles/previews to generic
substrate configs. Grid size is designer-addressable as square N; admission rejects
N=0 and over-cap sizes. Square-only is enforced at spec admission, not in
StructuredFieldStencilOp or simthing-sim. Source policy v1 remains
CallerManagedOneShotSeedThenZero. Cadence maps to generic FieldCadence.
Reduction bindings compile to existing ColumnAwareReductionSpec / SlotRange Sum
semantics. Field formula classes field_pressure / field_urgency / field_decay /
bounded_field_update are admitted at designer/spec policy layer without new EML
opcodes. MappingExecutionProfile remains default Disabled; spec presence alone
does not enable execution. No production mapping runtime landed. No production
pass graph wiring landed. No map/faction/AI semantics entered simthing-sim or
WGSL. The follow-on Option 3 product-facing first-slice scenario fixture landed
(single grid, no atlas), and the full first-slice vertical SEAD slice is now
**ACCEPTED (Opus/product 2026-05-28, PASS WITH CONDITIONS)**. Next implementation
handoff is **queue-write scale hardening or broader map residency**.
The M-4 atlas packer (Option A/Option 4) is **not** next; it waits for a named
multi-theater scenario, an approved VRAM budget, and a §11-gate-passing M-4 PR.

**Test:** A rejection suite (one case per banned/over-cap condition) plus a
round-trip that a valid bounded field compiles to generic stencil + reduction +
EvalEML registrations with `simthing-sim` seeing only opaque ops.  
**Acceptance:** CI green. Spec/driver only. `PipelineFlags::default()` mapping
execution remains false.

### PR M-first-slice — First-slice mapping runtime + boundary/budget probe — **Done**

**Status:** **Landed** — opt-in `FirstSliceMappingSession` in `simthing-driver`; RegionField budget estimator in `simthing-spec`. **Not** wired into default `SimSession` pass graph.

Phase M-first-slice landed behind explicit `MappingExecutionProfile` opt-in.
It exercises one bounded RegionField grid with source_capped_normalized, H≤8,
caller-managed source protocol, dirty skip, SlotRange Sum reduction, and parent
field_urgency EvalEML. Single-grid edge-boundary parity confirms invalid neighbors
are nullified by generic boundary semantics, not semantic map code. RegionField
budget preview estimates designer-facing VRAM footprint and rejects over-budget
specs before runtime. No atlas batching landed. No M-4A algebraic atlas masking
landed. No active mask, perception, map residency, or behavioral source policy
landed. simthing-sim remains map-free. Defaults unchanged.

**Test:** [`phase_m_first_slice_runtime_test_results.md`](tests/phase_m_first_slice_runtime_test_results.md) — 11/11 PASS.

### PR M-first-slice-R1 — GPU-state ownership + no-readback correctness hardening — **Done**

**Status:** **Landed** — remedial fix for no-readback hot path in opt-in `FirstSliceMappingSession`. Generic GPU buffer helpers on `StructuredFieldStencilOp`. **Not** wired into default `SimSession` pass graph.

M-first-slice-R1 landed: GPU-state ownership/no-readback correctness hardening.
The first-slice hot path now preserves caller-managed source propagation without CPU readback.
Seed-only clearing is performed without discarding first-hop output.
Hot-path reports no longer return placeholder parent/EML values.
Invalid seeds reject cleanly.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, or behavioral source policy landed.
simthing-sim remains map-free.
Defaults unchanged.

**Test:** [`phase_m_first_slice_runtime_r1_no_readback_correctness_test_results.md`](tests/phase_m_first_slice_runtime_r1_no_readback_correctness_test_results.md) — 20/20 PASS (11 original + 9 R1).

### PR M-first-slice-R2 — GPU-resident Layer 1→2→3 bridge — **Done**

**Status:** **Landed** — removes hidden GPU→CPU→GPU staging before Layer 2/3 on the hot path. Generic `AccumulatorOpSession` GPU bridge helpers. **Not** wired into default `SimSession` pass graph.

M-first-slice-R2 landed: GPU-resident Layer 1→2→3 bridge.
The first-slice hot path now keeps stencil field state on GPU, copies canonical field data into AccumulatorOpSession values buffer without CPU readback, and executes SlotRange Sum + field_urgency EvalEML without hidden GPU→CPU→GPU staging.
Debug/diagnostic readback remains explicit and test-only.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Test:** [`phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md`](tests/phase_m_first_slice_runtime_r2_gpu_bridge_test_results.md) — 24/24 PASS.

### PR M-first-slice-R3 — GPU-resident readiness/observability parking — **Done**

**Status:** **Landed** — readiness/observability pass for Opus/product review. No new mapping runtime behavior. **Not** wired into default `SimSession` pass graph.

M-first-slice-R3 landed: GPU-resident first-slice readiness/observability parking pass.
The first-slice hot path remains GPU-resident through stencil, SlotRange Sum reduction, and field_urgency EvalEML. R3 adds readiness/cost-shape reporting and locks the no-hidden-readback invariant for Opus/product review.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and parent weights (acceptable for 10×10 first slice).

**Test:** [`phase_m_first_slice_runtime_r3_readiness_test_results.md`](tests/phase_m_first_slice_runtime_r3_readiness_test_results.md) — 28/28 PASS.

### PR M-product-fixture — Product-facing first-slice scenario fixture — **Done**

**Status:** **Landed** — small product-style RegionFieldSpec/RON fixture over the accepted
GPU-resident first-slice runtime. **Not** wired into default `SimSession` pass graph.

Phase M product-facing first-slice scenario fixture landed.
It drives the accepted GPU-resident first-slice runtime from a small product-style
RegionFieldSpec/RON fixture: one grid, source_capped_normalized, H<=8,
caller-managed seed-only clear, dirty scheduling, SlotRange Sum reduction, and parent
field_urgency EvalEML.
The fixture proves default-off behavior, explicit SparseRegionFieldV1 opt-in,
GPU-resident hot path with reduction_stencil_readbacks=0, finite propagated field
values, and personality/weight-sensitive urgency.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic
preinitialized resource column, fill helper, or GPU fill kernel after a separate measured
design step.

**Test:** [`phase_m_first_slice_product_fixture_test_results.md`](tests/phase_m_first_slice_product_fixture_test_results.md) — PASS.

### PR M-product-commitment-fixture — Threshold event over first-slice urgency — **Done**

**Status:** **Landed** — narrow product-facing SEAD commitment fixture over the accepted
first-slice product fixture. **Not** wired into default `SimSession` pass graph.

Phase M product commitment fixture landed.
It extends the product-facing first-slice fixture by using the existing threshold/event
substrate over parent field_urgency, proving the SEAD commitment path: GPU-resident field
propagation -> parent reduction -> EvalEML urgency -> threshold event.
Low-weight profile stays below threshold; high-weight profile crosses and emits the
expected event.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice and commitment fixtures.
Future multi-field/atlas scale must replace per-slot resource writes with a generic
preinitialized resource column, fill helper, or GPU fill kernel after a separate measured
design step.

**Test:** [`phase_m_first_slice_product_commitment_fixture_test_results.md`](tests/phase_m_first_slice_product_commitment_fixture_test_results.md) — PASS.

### PR M-commitment-spec — Designer-facing threshold binding — **Done**

**Status:** **Landed** — first-slice commitment threshold/event binding moved into
designer/spec-facing RON admission. **Not** wired into default `SimSession` pass graph.

Phase M CommitmentSpec fixture landed.
It moves the first-slice commitment threshold/event binding into a designer/spec-facing
RON-admitted configuration while preserving the existing GPU-resident SEAD path: field
propagation -> parent reduction -> field_urgency EvalEML -> Threshold + EmitEvent.
Low-weight profile remains below the authored threshold; high-weight profile crosses and
emits the authored event.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice and commitment fixtures.
Future multi-field/atlas scale must replace per-slot resource writes with a generic
preinitialized resource column, fill helper, or GPU fill kernel after a separate measured
design step.

**Test:** [`phase_m_first_slice_commitment_spec_test_results.md`](tests/phase_m_first_slice_commitment_spec_test_results.md) — PASS.

### PR M-first-slice-scenario-spec — Scenario-level RON authoring wrapper — **Done**

**Status:** **Landed** — narrow scenario-level RON wrapper for the first-slice mapping +
CommitmentSpec path with explicit `MappingExecutionProfile`. **Not** wired into default
`SimSession` pass graph. **Not** a general scenario engine.

Phase M FirstSliceScenarioSpec fixture landed.
It wraps the accepted first-slice RegionFieldSpec + CommitmentSpec in a scenario-level RON
authoring shape that includes explicit MappingExecutionProfile.
Disabled scenarios admit as structure but do not execute. SparseRegionFieldV1 scenarios
execute the GPU-resident first-slice path and emit the authored commitment event only when
field_urgency crosses the authored threshold.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

**Test:** [`phase_m_first_slice_scenario_spec_test_results.md`](tests/phase_m_first_slice_scenario_spec_test_results.md) — PASS.

### PR M-first-slice-scenario-spec-R1 — Hygiene / test boundary / budget hardening — **Done**

**Status:** **Landed** — remedial hygiene after FirstSliceScenarioSpec landing. **Not** a scope
expansion.

Phase M FirstSliceScenarioSpec-R1 hygiene landed.
The scenario-level RON wrapper remains opt-in and GPU-resident. The public/test-only boundary
was clarified, scenario budget estimate handling was hardened, and the prior build/test crash
history was documented with a final clean verification run.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

**Test:** [`phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md`](tests/phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md) — PASS.

### PR M-first-slice-vertical-proof-parking — Review packet + parking pass — **Done**

**Status:** **Landed** — docs/review packaging only. **No** additional runtime behavior.

Phase M first-slice vertical proof parked for Opus/product review.
The landed chain now covers scenario-level RON authoring with explicit MappingExecutionProfile,
RegionFieldSpec, CommitmentSpec, GPU-resident field propagation, parent reduction, field_urgency
EvalEML, and Threshold + EmitEvent commitment.
No additional runtime behavior landed in this parking pass.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future
multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized
resource column, fill helper, or GPU fill kernel after a separate measured design step.

**Review:** [`reviews/phase_m_first_slice_vertical_proof_review_packet.md`](reviews/phase_m_first_slice_vertical_proof_review_packet.md)

**Test:** [`phase_m_first_slice_vertical_proof_parking_test_results.md`](tests/phase_m_first_slice_vertical_proof_parking_test_results.md) — PASS.

### PR M-first-slice-summary-validity — Summary validity V1 — **Done**

**Status:** **Landed** — opt-in first-slice summary validity policy/status for clean/skipped RegionFields.

Phase M SummaryValidity V1 landed.
It adds a bounded first-slice summary validity policy/status so a clean or skipped RegionField can report whether its strategic parent summary is fresh, cached, zero-initial, or unavailable without rerunning dense field propagation or rederiving gameplay state on CPU.
The hot path remains GPU-resident; cached summaries retain GPU-resident parent summary values and report metadata only.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency system, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**V1-R1 hygiene + parking verification:** Runtime summary status moved from simthing-spec into simthing-driver as `FirstSliceSummaryStatus`. Designer-facing policy remains in spec admission. Full targeted first-slice verification + workspace check passed. Behavior unchanged. All V7.7 guardrails preserved.

**Design:** `RegionFieldSpec.summary_policy` (default `CachedUntilDirtyWithZeroInitial`); runtime `FirstSliceSummaryReport` with `FreshThisTick` / `Cached { age_ticks }` / `ZeroInitial` / `InvalidOrUnavailable`. Cached commitment threshold scan deferred.

**Known scale caveat:** First-slice bridge uses queue writes for child resource values and
parent weights. Before any multi-field, multi-map, atlas, or broader production scaling, replace
per-slot resource/weight queue writes with a measured GPU-resident mechanism such as a
preinitialized resource column, generic fill helper, or GPU fill kernel.

**Test:** [`phase_m_first_slice_summary_validity_test_results.md`](tests/phase_m_first_slice_summary_validity_test_results.md) — PASS.

### PR M-first-slice-queue-write-hardening — Queue-write scale hardening V1 — **Done**

**Status:** **Landed** — generic bulk child resource fill on first-slice GPU bridge.

Phase M Queue-Write Scale Hardening V1 landed.
The first-slice GPU bridge no longer uses per-child resource queue writes for the child resource column. It uses a generic bounded bulk/preinitialized fill path while preserving the GPU-resident stencil → accumulator → reduction → EML → threshold event flow.
Parent scalar weight writes remain constant-size and acceptable for the single-grid first-slice path.
No SummaryValidity behavior changed.
No CPU-side gameplay cache was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency expansion, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Design:** `AccumulatorOpSession::fill_slot_range_col`; readiness counters `gpu_bridge_bulk_col_fills`, `gpu_bridge_bulk_fill_values`, `gpu_bridge_parent_scalar_writes`.

**Remaining caveat:** V1 uses a generic GPU fill dispatch for strided column fills when count > 1. Parent weight/personality columns remain constant-size queue writes.

**Test:** [`phase_m_queue_write_scale_hardening_test_results.md`](tests/phase_m_queue_write_scale_hardening_test_results.md) — PASS.

### PR M-first-slice-map-residency — Map Residency V1 — **Done**

**Status:** **Landed** — first-slice residency status/reporting (metadata only).

Phase M Map Residency V1 landed.
It adds first-slice residency status/reporting over the accepted GPU-resident path: HotExecutedThisTick, ResidentCached, ColdSkipped, and DisabledUnavailable.
Residency status is metadata only. CPU does not recompute threat/urgency, emit commitment events, or mutate true field values for cached/skipped maps.
ResidentCached preserves visibility of prior GPU parent summaries through metadata while cached commitment scans remain deferred in V1.
No SummaryValidity behavior changed.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception/fog, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Design:** `FirstSliceResidencyStatus` / `FirstSliceResidencyReport` on `FirstSliceMappingReport.residency`; derived from existing SummaryValidity GPU-parent-summary state; no new RON field.

**Test:** [`phase_m_first_slice_map_residency_test_results.md`](tests/phase_m_first_slice_map_residency_test_results.md) — PASS.

### PR M-boundary-cadence-doctrine — Boundary Resolution Doctrine audit — **Done**

**Status:** **Landed** — docs+test audit confirming abstract deterministic tick/boundary cadence is expressible through existing substrate machinery.

Phase M Boundary Resolution Doctrine audit landed.
The substrate exposes abstract deterministic tick/boundary cadence through `ticks_per_day`, `boundary_reached`, `day_index`, boundary handlers, persistent GPU values, discrete resource-economy transfers, and summary-tier readback.
Despite the names, `day_index` and `ticks_per_day` do not make day/calendar semantics part of simthing-sim. A host may interpret `day_index` as a day, turn, frame, season, orbital step, market close, learning epoch, or other unit.
No DailyResolutionBoundary runtime primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
Daily meaning remains only one possible host/spec interpretation of `day_index`.
Pause/speed remain host/UI orchestration concerns: the deterministic sim advances only when the host requests ticks.
Example discrete boundary banking may use the discrete resource economy substrate, not the continuous Resource Flow substrate by default.
The CPU boundary consumes resolved summaries/events/metadata at the boundary; it must not scan dense RegionCell grids by default, recompute gameplay state, or emit AI commitments via CPU planner logic.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

Queue-write child resource scale caveat addressed for first-slice by generic bulk fill.
Parent scalar writes remain O(1).
Multi-field, multi-map, atlas, perception, source identity, and broader production scaling remain separately gated.

**Mechanisms confirmed:** `DispatchCoordinator` (`ticks_per_day`, `tick_in_day`, `boundary_reached`, `day_index`); `SimSession::run(max_days)` boundary loop; `BoundaryProtocol` + `can_skip_empty_boundary`; discrete `ResourceEconomySpec` / `CompiledResourceTransfer`; summary-tier readback at boundary.

**Test:** [`phase_m_boundary_cadence_doctrine_audit.md`](tests/phase_m_boundary_cadence_doctrine_audit.md) — PASS.

### PR M-daily-economy-fixture — Daily Economy Fixture V1 — **Done**

**Status:** **Landed** — opt-in product/example fixture demonstrating discrete boundary banking when a host interprets one boundary as one day.

Phase M Daily Economy Fixture V1 landed as a product/example fixture.
It proves that a game can interpret one abstract boundary as one day and run daily banking through existing discrete ResourceEconomySpec authoring: ticks_per_day=1, boundary_reached/day_index, ResourceEconomySpec production, discrete transfers into storage, upkeep transfers out, and threshold/event checks over resolved storage.
This does not make daily cadence canonical for SimThing.
Other simulations may interpret the same boundary machinery as turns, frames, months, seasons, orbital steps, or other semantic units.
No DailyResolutionBoundary runtime primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
The CPU boundary consumes resolved storage/events/metadata; it does not recompute economy state or emit planner decisions.
Resource Flow E-11 remains continuous/high-frequency oriented and default-off, not the default discrete boundary-banking substrate.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Fixture design:** Surplus RON uses `TransferOnly` opt-in with `daily_income` conjunctive recipe (food/ore → producer), `bank_daily_income` producer→treasury transfer, and `daily_upkeep` treasury→upkeep_sink transfer at `ticks_per_day=1`. Deficit variant uses producer→treasury/upkeep transfers plus `EmitOnThreshold` low-storage event. Emission registrations are intentionally absent — C-8d emission is EmitEvent-shaped, not hard-currency banking.

**Test:** [`phase_m_daily_economy_fixture_test_results.md`](tests/phase_m_daily_economy_fixture_test_results.md) — PASS.

### PR M-boundary-resolution-doctrine-r1 — Boundary Resolution Doctrine R1 — **Done**

**Status:** **Landed** — narrow docs/test-report terminology correction; no runtime behavior changes.

Phase M Boundary Resolution Doctrine R1 landed.
Active docs now frame tick/boundary cadence as abstract substrate machinery, with daily/Clausewitz semantics treated only as one host/spec example fixture rather than canonical SimThing semantics.
SimThing exposes abstract deterministic tick/boundary resolution; a boundary is the synchronization point at which resolved summaries, events, and metadata may be consumed by host/spec/boundary-handler code.
No DailyResolutionBoundary runtime primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
Daily Economy Fixture V1 remains valid as a product/example fixture only.
Resource Flow E-11 remains default-off.
No default SimSession mapping wiring was introduced.
simthing-sim remains map-free.
Defaults unchanged.

**Test:** [`phase_m_boundary_resolution_doctrine_r1_test_results.md`](tests/phase_m_boundary_resolution_doctrine_r1_test_results.md) — PASS.

### PR M-boundary-resolution-review-packet — Abstract Boundary + Example Economy Parking Packet — **Done**

**Status:** **Landed** — Opus/product review packet parking abstract boundary doctrine and example daily economy fixture; docs/review packaging only.

Phase M abstract boundary-resolution + example economy review packet landed.
The repo now distinguishes abstract substrate tick/boundary cadence from game-level daily interpretation. `ticks_per_day` and `day_index` remain the legible API names; despite the names, day/calendar semantics are not part of simthing-sim.
Daily Economy Fixture V1 remains a valid product/example fixture showing one game-level interpretation: one boundary as one day, with discrete ResourceEconomySpec banking.
No runtime behavior changed.
No DailyResolutionBoundary primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

**Review packet:** [`reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md`](reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md)

**Test:** [`phase_m_boundary_resolution_review_packet_test_results.md`](tests/phase_m_boundary_resolution_review_packet_test_results.md) — PASS.

### PR M-boundary-resolution-doctrine-r2 — Boundary Resolution Doctrine R2 — **Done**

**Status:** **Landed** — narrow docs-only terminology correction restoring legible tick/boundary/day_index/ticks_per_day vocabulary; no runtime behavior changes.

Phase M Boundary Resolution Doctrine R2 landed.
Active docs now use the clearer tick/boundary/day_index/ticks_per_day vocabulary while preserving the constitutional guardrail that day/calendar meaning is host/spec interpretation, not a hardcoded simthing-sim semantic.
Despite the names, `day_index` and `ticks_per_day` do not make day/calendar semantics part of simthing-sim.
Daily Economy Fixture V1 remains a valid product/example fixture only.
No runtime behavior changed. No public API renames. No DailyResolutionBoundary primitive. No Day/Calendar/Pause in simthing-sim.
Resource Flow E-11 remains default-off. simthing-sim remains map-free. Defaults unchanged.

**Test:** [`phase_m_boundary_resolution_doctrine_r2_terminology_test_results.md`](tests/phase_m_boundary_resolution_doctrine_r2_terminology_test_results.md) — PASS.

### PR M-boundary-resolution-acceptance — Opus/product acceptance — **Done (PASS WITH CONDITIONS)**

**Status:** **ACCEPTED** (Opus/product 2026-05-29) — [`reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`](reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md).
Abstract boundary doctrine accepted; Daily Economy Fixture V1 accepted as an example/product
fixture only; `ResourceEconomySpec` (discrete banking) vs Resource Flow E-11 (continuous,
default-off) distinction accepted; the eleven future-agent guardrails made **binding** in
`docs/invariants.md` ("Boundary resolution (abstract cadence)"). Condition C-1: the "no day
semantics in `simthing-sim`" guardrail is about *semantics* (no calendar arithmetic /
`Calendar`/`Pause` types / `DailyResolutionBoundary`), not the legible `day`/`day_index`/"day
boundary" naming that already pervades `simthing-sim`. **Next implementation step:** resource-economy
authoring ergonomics, or an economy+SEAD product fixture — a generic boundary-output packet only if
tightly bounded (never `DailyResolutionBoundary` by another name), and **not** the M-4 atlas packer.
Re-verified on GPU: boundary cadence 7/7, daily economy 7/7, admission 11/11. Docs-only acceptance.

### PR M-resource-economy-authoring-ergonomics-v1 — Authoring preview + diagnostics — **Done**

**Status:** **Done (PASS)** — Phase M Resource Economy Authoring Ergonomics V1 landed.
**EML-GADGET-2D Hysteresis + 2D R1 landed (conditional explicit-column Tier-2 in simthing-spec only)** — finite on/off threshold admission (on > off for high-activates), stateful CPU oracle, and **exact CMP/SELECT compiler parity** over existing EvalEML primitives (2D R1 truth correction; initial 2D stub superseded). Spec/admission/compiler/oracle only. No runtime gadget execution, no chained scheduling, no WGSL/GPU/simthing-sim changes, no economy→mapping bridge. Test: [`tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md`](tests/phase_m_eml_gadget_2d_hysteresis_r1_test_results.md).

**EML-GADGET-2E Acceleration landed (explicit velocity-column Tier-2 in simthing-spec only)** — `(current_velocity_col - previous_velocity_col) [/ dt]` over existing EvalEML SUB/DIV arithmetic only; no position-history acceleration, no previous_previous_col inference, no dense per-cell temporal memory. Spec/admission/compiler/oracle only. Test: [`tests/phase_m_eml_gadget_2e_acceleration_test_results.md`](tests/phase_m_eml_gadget_2e_acceleration_test_results.md).
It adds authoring preview/diagnostics for discrete `ResourceEconomySpec` fixtures so designers can inspect transfers, recipes, order bands, bindings, Resource Flow posture, and simple static net effects before runtime.
**R2 (narrow ergonomics, spec/admission/preview only):** Added `schedule_lines` helper to `ResourceEconomyPreviewReport` — compact designer-readable one-liners exposing the intended transfer/recipe/threshold schedule directly from the admitted authoring data. Pure preview; no runtime, no mapping coupling, no GPU changes.
No runtime economy behavior changed.
No `DailyResolutionBoundary` primitive was introduced.
No day/calendar/pause semantics were added to `simthing-sim`.
Legible `day_index`/`ticks_per_day` naming remains unchanged.
No Resource Flow default changed; Resource Flow E-11 remains continuous/high-frequency and default-off.
No CPU-side economy executor or planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
`simthing-sim` remains map-free.
Defaults unchanged.

**Implementation:** `compile_resource_economy_authoring_preview`, `compile_game_mode_resource_economy_authoring_preview`, `ResourceEconomyAuthoringPreview` / `ResourceEconomyPreviewReport` in `simthing-spec` admission layer; simple static transfer-only net per property/role for diagnostic metadata.

**Test:** [`phase_m_resource_economy_authoring_ergonomics_test_results.md`](tests/phase_m_resource_economy_authoring_ergonomics_test_results.md) — PASS.

### PR M-economy-sead-product-fixture-v1 — Economy + SEAD product fixture — **Done**

**Status:** **Done (PASS)** — Phase M Economy + SEAD Product Fixture V1 landed.
It demonstrates that an accepted discrete resource-economy boundary result can influence an opt-in first-slice SEAD commitment fixture without adding runtime semantic coupling: economy remains `ResourceEconomySpec`/discrete/opt-in, SEAD remains GPU-resident field/reduction/EML/Threshold+EmitEvent, and the CPU does not compute urgency or emit commitments.
This remains a product/acceptance fixture. It does not authorize a production economy→mapping runtime bridge or general scenario engine.
No runtime economy behavior changed.
No `DailyResolutionBoundary` primitive was introduced.
No day/calendar/pause semantics were added to `simthing-sim`.
Legible `day_index`/`ticks_per_day` naming remains unchanged.
No Resource Flow default changed; Resource Flow E-11 remains continuous/high-frequency and default-off.
No CPU-side economy executor or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
`simthing-sim` remains map-free.
Defaults unchanged.

**Integration strategy:** Option A — test-level orchestration. Run daily economy at boundary, read treasury, map stress to authored EML weight profiles `(0.2,0.1)` / `(0.9,0.1)`, drive existing `FirstSliceScenarioFixtureSession` commitment path.

**Test:** [`phase_m_economy_sead_product_fixture_test_results.md`](tests/phase_m_economy_sead_product_fixture_test_results.md) — PASS.

### PR M-product-fixture-chain-parking — Product-fixture chain review/parking packet — **Done; ACCEPTED 2026-05-29**

**Acceptance:** **ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`](reviews/phase_m_product_fixture_chain_acceptance_opus_review.md). Accepted as a fixture-level product proof; the economy→SEAD link stays in `tests/support` orchestration (CPU selects authored EML weight profiles, never computes urgency or emits commitments; both stay GPU-resident). Binding row added to `docs/invariants.md` Mapping section. Next implementation step: authoring ergonomics R2 or another tiny non-map-substrate + SEAD product fixture — not a generic boundary-output packet (D), not the M-4 atlas packer (E). No production economy→mapping bridge authorized.

**Status:** **Done (PASS)** — Phase M product-fixture chain parking packet landed.
The repo now has a reviewed chain from abstract tick/boundary doctrine through discrete `ResourceEconomySpec` boundary banking, resource-economy authoring preview, and an Economy + SEAD product fixture where resolved treasury stress selects authored EML weights and the existing GPU-resident first-slice path emits or does not emit a SEAD commitment through Threshold + EmitEvent.
This remains fixture orchestration only.
No production economy→mapping runtime bridge was introduced.
No generic boundary-output packet was introduced.
No `DailyResolutionBoundary` primitive was introduced.
No day/calendar/pause semantics were added to `simthing-sim`.
No Resource Flow default changed.
No CPU-side economy executor or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
`simthing-sim` remains map-free.
Defaults unchanged.

**Review packet:** [`reviews/phase_m_product_fixture_chain_review_packet.md`](reviews/phase_m_product_fixture_chain_review_packet.md)

**Test:** [`phase_m_product_fixture_chain_parking_test_results.md`](tests/phase_m_product_fixture_chain_parking_test_results.md) — PASS.

### PR M-eml-gadget-library — EML Gadget Library — **EML-GADGET-1 ACCEPTED; EML-GADGET-2 design ACCEPTED as a gate (Opus 2026-05-29); next impl = EML-GADGET-2A**

**Acceptance:** **EML-GADGET-1 ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md`](reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md). Tier-1 stateless gadgets (`FieldSampler`, `WeightedAccumulator`, algebraic `SoftStep`) accepted as `simthing-spec` node-template macros over existing `EvalEML` opcodes with CPU-oracle parity; R1 composition (per-gadget executable; multi-gadget `PerGadgetOnly`; preview ≠ runtime) and R2 node-cap (per executable tree) accepted. Binding rows added to `docs/invariants.md` ("EML Gadget Library"). Conditions: preview ≠ runtime (no driver/gpu/sim consumes the flatten preview); `PerGadgetOnly` is the only multi-gadget composition until intermediate wiring is separately gated; oracle-per-gadget binding for all future gadgets. Re-verified: eml_gadget_tier1 14/14, admission 11/11, authoring preview 8/8. **EML-GADGET-2 temporal-memory design ACCEPTED as a gate (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — [`reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md). Explicit-column temporal memory; Layer-3 default; authored snapshot/copy bands (`Identity`+`ResetTarget` — existing substrate, **no new opcode**); bounded-feedback admission contract (default `0 ≤ decay < 1`; clamp required when feeding a hard threshold; analytically-bounded escape must be admission-checkable; stateful-sequence oracle parity). Candidates: VelocityMonitor / Decay/EMA / BoundedFeedback accepted; Hysteresis conditional (2D); Acceleration + dense per-cell deferred. **Approved ladder: 2A (snapshot/copy fixture proof) → 2B (VelocityMonitor + Decay/EMA) → 2C (BoundedFeedback) → 2D (Hysteresis).** Implementation unauthorized until the separate 2A handoff. *(The #262 parking packet was reverted off master; the acceptance memos are authoritative.)*

**Status:** **Phase M EML-GADGET-1 + R1 + R2 landed.** Tier-1 stateless EML gadgets compile in `simthing-spec` to existing EvalEML node templates with mandatory CPU-oracle parity. R1 clarifies stack composition semantics; R2 corrects node-cap enforcement to per-gadget/single-tree only. Design note:
[`workshop/eml_gadget_library_design_note.md`](workshop/eml_gadget_library_design_note.md).
Designer-facing, RON-authored EML **gadgets** = named postfix node-template macros over the existing
`EvalEML` opcode set (NOT new WGSL kernels), composed into the GPU-resident `EvalEML` path. Lives in
`simthing-spec` (authoring/compiler); `simthing-gpu` stays generic; `simthing-sim` stays map-free.

Phase M EML-GADGET-1 landed.
It adds the Tier-1 stateless EML gadget library in simthing-spec: FieldSampler, WeightedAccumulator, and algebraic SoftStep as RON-authored node-template macros over existing EvalEML opcodes.
Each Tier-1 gadget has CPU-oracle parity tests.
SoftStep uses the ExactDeterministic algebraic form, not exp/logistic.
No new EML opcode was added.
No WGSL or GPU kernel was added.
No runtime economy behavior changed.
No production economy→mapping bridge was introduced.
No generic boundary-output packet was introduced.
No DailyResolutionBoundary primitive was introduced.
No day/calendar/pause semantics were added to simthing-sim.
No Resource Flow default changed.
No CPU-side economy executor, urgency computation, or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
simthing-sim remains map-free.
Defaults unchanged.
Chained multi-gadget order-band execution is deferred; V1 supports inline flatten and/or compile-plan preview only.

**EML-GADGET-1 R1 (2026-05-29):** Replaced ambiguous `flattened_nodes` with `EmlGadgetCompositionPlan`. Multi-gadget `output_col`/`input_col` chained stacks emit `PerGadgetOnly` + `chained_runtime_deferred` diagnostic; single-gadget stacks may expose executable `InlineFlattenPreview`. No runtime gadget execution introduced. Test: [`tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md`](tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md).

Phase M EML-GADGET-1 R1 landed.
It clarifies gadget stack compilation semantics: Tier-1 per-gadget node templates are executable and CPU-oracle proven; multi-gadget stack composition is proven through manual/orchestrated per-gadget chaining; executable flattened multi-gadget runtime output is not claimed in V1 and remains deferred unless a later gated compiler proves true intermediate wiring.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No temporal memory was introduced.
No new EML opcode was added.
No WGSL or GPU kernel was added.
No runtime economy behavior changed.
No production economy→mapping bridge was introduced.
No generic boundary-output packet was introduced.
No DailyResolutionBoundary primitive was introduced.
No day/calendar/pause semantics were added to simthing-sim.
No Resource Flow default changed.
No CPU-side economy executor, urgency computation, or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
simthing-sim remains map-free.
Defaults unchanged.

**EML-GADGET-1 R2 (2026-05-29):** Node-cap semantics corrected — `MAX_EML_TREE_NODES` enforced per executable gadget/single-tree only; multi-gadget `PerGadgetOnly` stacks may exceed single-tree total with `stack_total_exceeds_inline_cap` diagnostic. Test: [`tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md`](tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md).

Phase M EML-GADGET-1 R2 landed.
It corrects node-cap semantics after R1: the EvalEML node cap applies to each executable gadget/tree, not to the informational total node count of a PerGadgetOnly multi-gadget stack. Multi-gadget stacks whose total node count exceeds the single-tree cap now admit as PerGadgetOnly with diagnostics, while chained runtime scheduling remains deferred.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No temporal memory was introduced.
No new EML opcode was added.
No WGSL or GPU kernel was added.
No runtime economy behavior changed.
No production economy→mapping bridge was introduced.
No generic boundary-output packet was introduced.
No DailyResolutionBoundary primitive was introduced.
No day/calendar/pause semantics were added to simthing-sim.
No Resource Flow default changed.
No CPU-side economy executor, urgency computation, or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
simthing-sim remains map-free.
Defaults unchanged.

**Test:** [`tests/phase_m_eml_gadget_tier1_test_results.md`](tests/phase_m_eml_gadget_tier1_test_results.md) — PASS.

**Sequencing (binding):** EML-GADGET-1 landed **before Phase M Resource Economy Authoring Ergonomics
R2.** R2's designer-facing authoring must be able to expose and leverage the gadget library.

**PR ladder:**

- **EML-GADGET-1 — Tier-1 stateless gadgets — DONE.** Gadget descriptor + registry + RON
  authoring + flatten compiler + `FieldSampler`, `WeightedAccumulator`, `SoftStep` (bit-exact
  algebraic sigmoid). **Mandatory CPU-oracle parity per gadget.** `simthing-spec` only; no GPU/WGSL
  change; default-off; `ExactDeterministic`.
- **EML-GADGET-2 — Tier-2 temporal-memory slice (Composer).** **Landed through 2E** — 2A snapshot/copy, 2B VelocityMonitor + Decay/EMA, 2C BoundedFeedback, 2D Hysteresis (+ 2D R1 CMP/SELECT parity), **2E explicit velocity-column Acceleration**. Dense per-cell temporal memory and position-history acceleration remain separately gated. **Bounded-feedback admission guardrail** binding. Spec/admission/compiler/oracle only — no runtime gadget execution or chained scheduling.

Phase M EML-GADGET-2 temporal-memory design review packet landed.
It reviews Tier-2 temporal gadget candidates — VelocityMonitor, Decay/EMA, Acceleration, Hysteresis, and BoundedFeedback — without implementing them.
The review preserves EML-GADGET-1 acceptance conditions: gadgets remain simthing-spec node-template macros over existing EvalEML opcodes; preview is not runtime; PerGadgetOnly remains the only multi-gadget composition until separately gated; every gadget requires CPU-oracle parity.
Temporal memory is explicit-column state with authored snapshot/copy bands, not hidden runtime memory.
Temporal memory defaults to Layer-3 / parent/personality scope; dense per-cell temporal memory remains separately gated.
Bounded feedback requires decay < 1 and/or explicit clamp, otherwise admission must reject.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No temporal memory implementation landed.
No new EML opcode was added.
No WGSL or GPU kernel was added.
No simthing-sim semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.

**Test:** [`tests/phase_m_eml_gadget_tier2_design_review_test_results.md`](tests/phase_m_eml_gadget_tier2_design_review_test_results.md) — PASS.

**Stop conditions:** no per-gadget WGSL; no new opcode (incl. transcendental) without a separate
gate; no transcendental in `ExactDeterministic` gadgets; temporal memory stays Layer-3 scoped by
default; feedback must be bounded; default-off; `simthing-sim` map-free. Every gadget ships with a
CPU oracle.

### PR M-resource-economy-authoring-ergonomics-r2 — Authoring Ergonomics R2 — **Sequenced AFTER the EML Gadget Library**

**Status:** **Queued behind EML-GADGET-1/2.** R2 should expose and leverage the gadget library in the
designer-facing resource-economy authoring/preview surface rather than re-inventing composition.
Begin only after the gadget library is in place.

### PR M-4 — Opus design: atlas batching isolation + VRAM accounting (provisional) — **Design note Done; isolation policy ratified 2026-05-28; implementation still gated**

**Status:** Phase M-4 isolation policy is **ratified** (Opus, 2026-05-28, under human delegation — [`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md)): algebraic tile-local mask G=0 preferred for homogeneous square batches; physical gutter fallback; local-bounds metadata deferred; §11 checklist is a **binding acceptance gate**. **Atlas batching itself remains Provisional and unimplemented** — ratifying the isolation policy is **not** implementation authorization. `request_atlas_batching` stays rejected at admission until a §11-gate-passing M-4 PR. The first-slice product scenario fixture (Option 3, single grid, no atlas) has landed; **the atlas packer is still not next**.

**Model:** Opus (design), Composer 2.5 (implementation)  
**Why Opus:** Atlas batching is **provisional** in the ADR. The short-term policy
(gutter ≥ H, per-tile seed clearing) carries a 6.76× VRAM multiplier on 10×10 H=8
(remedial Test 2), and the long-term preferred policy (local-bounds tile-rect
metadata) needs API design. The decision space — when to pay the gutter tax vs
wait for local-bounds — is genuinely open and the cost of a wrong layout is
architectural.

**Opus task:** Specify the atlas isolation contract (gutter≥H now; local-bounds
metadata path), the mandatory VRAM-multiplier accounting/reporting, the per-tile
seed-clearing protocol, and the **production acceptance gate: bit-exact parity
against an exact per-tile-protocol CPU oracle** that replays the same seed-clear
+ gutter + boundary protocol the GPU atlas uses — **not** corridor-t44 agreement
alone (t44 was the sandbox tactical-signal metric; full-tile parity is only
meaningful against a protocol-faithful oracle). Produce a design note.  
**Design note landed:** [`workshop/mapping_atlas_batching_isolation_design_note.md`](workshop/mapping_atlas_batching_isolation_design_note.md).

Phase M-4 isolation policy is ratified (Opus 2026-05-28); atlas implementation remains gated.
Atlas batching remains provisional and unimplemented.
The design note defines the contract: algebraic tile-local mask G=0 (preferred) **or**
gutter >= effective horizon (fallback) for homogeneous square batches (M-4A evidence),
mandatory VRAM accounting, per-tile seed clearing, full-tile protocol-oracle
parity, and t44/corridor agreement is insufficient for production acceptance.
**M-4A (2026-05-19):** Algebraic tile-local masking sandbox completed and reverted.
G=0 mask is preferred candidate over physical G>=H (VRAM 1.0× vs 6.76×); physical gutter remains fallback.
See [`tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](tests/mapping_atlas_algebraic_mask_sandbox_test_results.md).
**M-4A architectural implication captured:** algebraic tile-local masking supports the general SimThing pattern of dense flat buffers + designer-authored relationships compiled into generic algebraic masks/gates. This implication is documented for Opus review in [`mapping_atlas_batching_isolation_design_note.md`](workshop/mapping_atlas_batching_isolation_design_note.md) §4; it does **not** ratify implementation or change atlas provisional status.
No production mapping runtime landed. No production pass graph wiring landed.
No map/faction/AI semantics entered simthing-sim or WGSL.

**Decision gate (resolved 2026-05-28):** Option B was taken — the first-slice runtime
landed and was hardened through R1/R2/R3 and **accepted by Opus as a stable base**
([`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md)).
Option 3 — a product-facing first-slice scenario fixture (single grid, no atlas) — also
landed, and the **full first-slice vertical SEAD slice is now ACCEPTED (Opus/product
2026-05-28, PASS WITH CONDITIONS)**: RON → opt-in profile → GPU field → reduction →
`field_urgency` EvalEML → Threshold + EmitEvent commitment
([`reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`](reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md)).
**Next implementation handoff: map residency / summary validity, or queue-write scale
hardening — NOT the atlas packer.** The per-slot queue-write caveat must be resolved before
any multi-field/atlas scaling. **Option A (atlas packer) is NOT next**; it is admissible only
after (a) a named multi-theater scenario that needs batching, (b) an approved VRAM budget, and
(c) a PR satisfying the §11 binding gate.

- **Option A (deferred):** Implement the generic M-4 atlas packer — only under the conditions above.
- **Option B (done):** First-slice runtime wiring — landed, hardened (R1/R2/R3), accepted.

**Implementation (Composer 2.5):** generic atlas packer in the driver behind the
mapping profile; VRAM-multiplier reporting wired into the debug surface; refuses
to pack unless exactly one admitted isolation policy is configured. **Blocked** — the
isolation-policy sign-off is done (Opus 2026-05-28), but atlas code stays blocked until a
named multi-theater scenario needs batching, an approved VRAM budget exists, and a PR
satisfies the §11 binding gate. Atlas remains **not** the next mapping step even after
the Option 3 fixture has landed.
**Gate:** Opus design note committed and isolation policy ratified; a §11-gate-passing M-4 PR
required before any atlas code. Atlas stays provisional, unimplemented, and opt-in;
`request_atlas_batching` stays rejected at admission until that PR.  
**Acceptance (future implementation PR only):** Design note committed; packer + VRAM reporting tests pass; refuses
unsafe packing; **full-tile bit-exact parity against the exact per-tile-protocol
CPU oracle** (t44-only acceptance is explicitly insufficient for production).

### PR M-5 — Generic source-identity buffer (deferred — unblocks behavioral source policy)

**Model:** Composer 2.5 (after a short Opus API note)  
**Scope:** **Deferred** per ADR. When a named scenario requires behavioral source
policy, add a generic `source_mask` / separate seed buffer to the primitive so
sources have explicit identity. Column-wide `source_col` zeroing remains **banned**.
Until then, caller-managed one-shot-seed-then-zero (v1) is the only source policy.  
**Acceptance:** Not scheduled until a scenario names the need.

---

### PR M-5-gradient — Gradient operator (single-target) + L3 Strategic Pressure Composition Pattern — **Approved candidate track (Opus 2026-05-29; remedially tightened)**

**Status:** **M-5A-gradient landed (2026-05-29, PASS).** Generic per-direction stencil weights
(`weight_north/south/east/west`) in `StructuredFieldStencilOp`; single-target
`RegionFieldOperatorSpec::Gradient { axis, output_col }` admission/compiler in `simthing-spec`;
CPU/GPU parity for GradientX/GradientY; Normalized and SourceCappedNormalized behavior preserved.
**M-5B-gradient landed (2026-05-29, PASS).** L3 composition RON fixture over M-5A substrate.
**M-5B-gradient R1 (2026-05-29, PASS):** integrated CPU-oracle test ties scalar + GradientX +
GradientY parent reductions into L3 EMA + WeightedAccumulator; no new substrate or runtime wiring.
**M-5C-gradient landed (2026-05-29, PASS).** Product-facing need/routing signal RON fixture
(unmet demand + price/labor gradients → routing composite); CPU-oracle integrated test; no production
bridge or ResourceEconomySpec→mapping coupling.
**M-5D-gradient landed (2026-05-29, PASS).** Frame/scenario-level gradient strict-sink admission
(`validate_region_field_frame_gradient_sinks`); rejects same-frame derivative feedback into diffusion
`source_col`. Dual-output
`GradientXY` remains deferred. No semantic WGSL; no default mapping wiring; no `simthing-sim`
changes; no atlas/M-4A; no source-mask/source-identity; no L1 coupling; no `sqrt`; no production
economy→mapping bridge. Report:
[`tests/phase_m_m5a_gradient_single_target_test_results.md`](tests/phase_m_m5a_gradient_single_target_test_results.md).

M-5-gradient is approved as a **generic stencil-extension track, not a semantic WGSL track.**
M-5A-gradient should first implement per-direction weights plus a **single-target** `Gradient` axis
admission/oracle. **Dual-output `GradientXY` is deferred** as a possible optimization gate. No
atlas/M-4A, source-mask/source-identity work, L1 field coupling, `sqrt` opcode, default mapping
wiring, `simthing-sim` awareness, or production economy→mapping bridge is authorized.

These are **generic field-calculus primitives** with lateral benefit beyond AI: the same `Gradient`
op routes resources down a scarcity gradient or dispatches migrants up an opportunity gradient
(meaning authored at the spec layer; shader sees floats only).

**Revised WGSL guardrail (Opus 2026-05-29):** the ban is on **semantic** WGSL only. Generic kernel
extensions — new parameters carrying no map/faction/AI semantics, meaning pinned at the
designer/spec admission layer, with CPU-oracle parity — are admissible. See `docs/invariants.md`
("Mapping (Sparse RegionCell)" row) and the design note §1.

**PR ladder:**

- **M-5A-gradient — single-target Gradient operator + per-direction stencil weights — **Done (PASS).**
  Generic `weight_north/south/east/west` in `FieldStencilParamsGpu` and WGSL; single `target_col`
  output unchanged. `RegionFieldOperatorSpec::Gradient { axis, output_col }` in `simthing-spec`;
  CPU oracle `GradientX=(east−west)/2`, `GradientY=(south−north)/2`; GPU parity on small grids.
  Normalized/SourceCappedNormalized preserved via isotropic weight mapping. Report:
  `docs/tests/phase_m_m5a_gradient_single_target_test_results.md`.
- **M-5B-gradient — L3 Strategic Pressure Composition Pattern RON fixture — **Done (PASS).**
  Reference RON fixture + test: scalar + Gradient X/Y fields, SlotRange Sum reductions, 3× Ema +
  WeightedAccumulator composite signal; optional GPU-resident threshold commitment. No new substrate.
  Report: `docs/tests/phase_m_m5b_gradient_l3_composition_test_results.md`.
- **M-5B-gradient R1 — integrated fixture evidence — **Done (PASS).**
  `m5b_integrated_parent_columns_feed_l3_composite` ties L1 field CPU oracles → parent reductions →
  L3 gadget stack in one test; no production multi-field runtime wiring added.
  Report: `docs/tests/phase_m_m5b_gradient_l3_composition_r1_test_results.md`.
- **M-5C-gradient — product-facing need/routing signal fixture — **Done (PASS).**
  RON fixtures + test: unmet-demand scalar + price/labor Gradient X/Y → SlotRange Sum → EMA +
  WeightedAccumulator `routing_signal`; CPU-oracle integrated; no production bridge.
  Report: `docs/tests/phase_m_m5c_gradient_need_signal_test_results.md`.
- **M-5D-gradient — Input Validation Rule: gradient-sink / no-within-frame-feedback admission — **Done (PASS).**
  `validate_region_field_frame_gradient_sinks` rejects gradient `output_col` used as same-frame
  `source_col`; `compile_region_field_frame_preview` validates then compiles frame groups; M-5B/M-5C
  valid-sink fixtures green. No runtime change.
  Report: `docs/tests/phase_m_m5d_gradient_sink_admission_test_results.md`; R1 grouped-helper evidence:
  `docs/tests/phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md`.
- **M-4A Atlas Readiness Gate — **Done (DEFER).** Tier-2 readiness pass: no named multi-theater
  product scenario requires atlas batching; M-5-gradient substrate satisfies current paths;
  source-mask (`M-5`), single-grid sparse mask, and L1 coupling remain separate gates. If a named
  scenario is approved later, next handoff is **Phase M-4 — Algebraic Tile-Local Atlas Packer
  (Homogeneous Square Batches)** behind §11 gate + VRAM budget. No implementation in this pass.
  Report: `docs/tests/phase_m_m4a_atlas_readiness_gate_results.md`.
- **M-6A Single-Grid Active Mask Readiness Gate — **Done (DEFER).** Tier-2 readiness pass:
  generic GPU `active_mask` hook exists but `ActiveOnlyExperimentalNoHalo` is not
  production-authorized per `invariants.md`; missing halo contract, CPU oracle, and GPU parity
  for masked ping-pong/boundary/source-cap/gradient. No named product scenario today. If authorized
  later, next handoff is **Phase M-6B — RegionField ActiveOnly Mask Admission** (halo-contract
  required). No implementation in this pass.
  Report: `docs/tests/phase_m_m6a_single_grid_active_mask_readiness_results.md`.
- **Product Scenario Selection Gate — **Done (SELECT → M-5E).** Tier-2 product gate: no named
  scenario for atlas (M-4A), active mask (M-6A), or source-mask; selected **Candidate D** —
  full-grid scarcity/opportunity/logistics composite on existing M-5-gradient substrate. Next
  handoff: **Phase M-5E-gradient — Scarcity/Opportunity Composite Product Fixture** (Tier-1;
  no new WGSL/substrate). M-4A, M-6A, source-mask remain deferred.
  Report: `docs/tests/phase_m_product_scenario_selection_gate_results.md`.

**Deferred (separate gates):** dual-output `GradientXY` (one-pass, widened output contract); `sqrt`
magnitude opcode; L1 cross-field coupling; dense per-cell gradient columns.

**Stop conditions:** shader must not name "gradient"/"threat"/any semantic; CPU/GPU oracle parity
must be exact; single-target path only (no dual-output kernel changes in M-5A-gradient); no
`simthing-sim` change; no source-mask/source-identity, atlas/M-4A, L1 coupling, `sqrt`, default
mapping wiring, CPU planner/urgency, or economy→mapping bridge. SEAD commitment stays GPU-resident.

---

## Updated PR ladder summary

| PR | Phase | Model | Description | Gate |
|---|---|---|---|---|
| A-1 | A | Codex 5.5 | Merge ADR + update invariants | Human review |
| A-2 | A | Codex 5.5 | CombineFn + AccumulatorOp types | None |
| A-3 | A | Composer 2.5 | EmlExpressionRegistry + whitelist | None |
| **A-4** | **A** | **Opus + Codex** | **Soft-aggregate tolerance audit** | **Human + Opus** |
| B-1 | B | Composer 2.5 | AccumulatorOpSession buffers | None |
| B-2 | B | Composer 2.5 | Pass B kernel bootstrap | Conservation test |
| B-3 | B | Codex 5.5 | Timestamp queries | None |
| **B-4** | **B** | **Opus + Composer** | **Summary readback design** | **Opus analysis** |
| C-1 | C | Composer 2.5 | Threshold scan migration | 5× readback speedup |
| C-2 | C | Codex 5.5 | Intent delta migration | Bit-exact parity |
| C-3 | C | Composer 2.5 | Overlay Add migration | Bit-exact parity |
| C-4 | C | Opus + Codex 5.5 | Multiply/Set OrderBand compiler | Landed behind flag |
| **C-5** | **C** | **Opus + Composer** | **WeightedMean tolerance audit + soft reductions** | **Landed (#121, #122)** |
| C-6 | C | Composer 2.5 | Sum/Max/Min/First exact reductions | **Landed (#124)** |
| C-7 | C | Composer 2.5 | Velocity integration | **Landed (#127)** |
| **C-8** | **C** | **Opus + Composer** | **EML + transfer + intensity + emission** | **Landed + S-2 sunset** |
| **D-1** | **D** | **Opus (memo only)** | **Discrete-transaction contention memo — see ADR `resource_flow_substrate.md`** | **Done** — [`d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md) |
| D-2 | D | — | **DEFERRED INDEFINITELY** — no concrete scope until D-1 memo motivates a revival | n/a |
| D-3 | D | Composer 2.5 | Changed-only logs + replay | Replay test |
| D-4 | D | Composer 2.5 + Opus | Cross-pool contention gate | Pass or ADR amendment |
| E-1 | E | Composer 2.5 | EmitOnThreshold builder | `e1_emit_on_threshold_builder` |
| E-2A | E | Codex 5.5 | resource_transfer_discrete builder | `e2a_resource_transfer_discrete_builder` |
| **E-2** | **E** | **Codex 5.5** | **SPLIT: discrete + flow-participant builders** | **Conservation + enrollment tests** |
| E-3 | E | Composer 2.5 | conjunctive_recipe builder + lift N≤4 cap | **Done (#147)** — `e3_conjunctive_recipe_builder` |
| E-3R | E | Composer 2.5 | throttle_hint_max_per_tick metadata hardening | **Done (#148)** — `e3_max_per_tick_is_metadata_not_gpu_cap` |
| E-7 | E | Composer 2.5 | governed_by planner generalization to arbitrary Named pairs | **Done (#149)** — `e7_governed_by_planner_generalization` |
| E-8 | E | Codex 5.5 | accumulator_spec on SubFieldSpec | **Done (#150)** — `accumulator_spec` serde/defaults tests |
| E-9 | E | Composer 2.5 | ArenaRegistry in simthing-driver | **Done (#151)** — `arena_registry` driver tests |
| E-9R | E | Composer 2.5 | participant_range contiguity hardening | **Done (#152)** — interleaved admission slice tests |
| E-4 | E | Composer 2.5 | Economic V1 RON + session integration | RON→session→conservation |
| E-5 | E | Composer 2.5 | Economic compact log integration | Replay test |
| **E-6** | **E** | **Codex 5.5** | **design_v7.md docs (mostly landed by v7.5 bump)** | **Doc consistency** |
| **E-7** | **E** | **Composer 2.5** | **`governed_by` planner generalization to arbitrary `(Named, Named)` pairs** | **Existing C-7 + new pair test** |
| **E-8** | **E** | **Codex 5.5** | **`accumulator_spec` on `SubFieldSpec`** | **Serde + invariant: no runtime branching** |
| **E-9** | **E** | **Composer 2.5** | **`ArenaRegistry` in `simthing-driver` with subtree-incremental refresh** | **3-arena fixture + refresh scope test** |
| **E-10** | **E** | **Composer 2.5** | **`simthing-spec` admission framework (caps, fission policy, cycle-with-delay, expansion report)** | **Done (#153)** — 13-case `e10_*` rejection + expansion report suite |
| **E-10R** | **E** | **Composer 2.5** | **Driver participant identity preflight + reserved-gap admission** | **Done** — `e10r_*` suite |
| **E-10R2** | **E** | **Composer 2.5** | **ArenaParticipant SimThing scaffold + contiguity/gap tests** | **Done** — `e10r2_*` suite |
| **E-10R3** | **E** | **Composer 2.5** | **Arena-local gap block reservation + capacity hardening** | **Done** — `e10r3_*` suite |
| **E-8R** | **E** | **Composer 2.5** | **Arena-internal plumbing columns at compile** | **Done** — `e8r_*` suite |
| **E-7R** | **E** | **Composer 2.5** | **`plan_governed_integration_at_band` ordering API** | **Done** — `e7r_*` suite |
| **E-11 design** | **E** | **Opus** | **Hierarchical allocation v2 design memo** | **Accepted** |
| **E-11 review** | **E** | **Composer 2.5** | **Final readiness review + narrowed handoff** | **Done** |
| **E-11** | **E** | **Opus + Composer 2.5** | **Flat-star D=2 allocation kernel + CPU oracle** | **Done — flat-star slice; E-11R landed; burn-in scaffold; flag default false** |
| **E-11R** | **E** | **Composer 2.5** | **Sync error propagation + scope honesty + session-path test** | **Done** |
| **E-11 burn-in** | **E** | **Composer 2.5** | **Controlled flat-star burn-in tests + report struct** | **Done** |
| **E-11 burn-in scenarios** | **E** | **Composer 2.5** | **Named flat-star scenario fixtures + `ResourceFlowScenarioBurnInReport`** | **Done** |
| **E-11 CI soak** | **E** | **Composer 2.5** | **Opt-in flat-star CI soak + `ResourceFlowSoakSummaryReport`** | **Done** |
| **E-11B readiness** | **E** | **Opus (memo only)** | **Nested hierarchy GPU execution/materialization audit** | **Done** — [`e11b_nested_hierarchy_gpu_readiness_review.md`](reviews/e11b_nested_hierarchy_gpu_readiness_review.md) |
| **E-11B kickoff** | **E** | **Composer 2.5** | **Nested D=3/D=4 static hierarchy materialization + GPU parity** | **Done** — explicit nested extension; no default-on |
| **E-11B nested dynamic enrollment readiness** | **E** | **Composer 2.5 (memo only)** | **Post–E-11B-4 nested dynamic enrollment audit** | **Done** — [`e11b_nested_dynamic_enrollment_readiness.md`](reviews/e11b_nested_dynamic_enrollment_readiness.md); defer by default |
| **E-11B pause checkpoint** | **E** | **Composer 2.5 (docs only)** | **Product-priority workspace checkpoint** | **Done** — E-11B paused; E-11B-5 gated behind named scenario |
| **Continued flat-star soak** | **E** | **Composer 2.5** | **FlatStarResourceFlow confidence checkpoint** | **Done** — 512 static @ 1000 ticks; no semantics expansion |
| **Product-priority vertical slice selection** | **E** | **Composer 2.5 (memo only)** | **Track selection after continued soak** | **Done** — [`product_priority_vertical_slice_selection.md`](reviews/product_priority_vertical_slice_selection.md); **Recommendation F** |
| **E-2B readiness** | **E** | **Opus (memo only)** | **Resource Flow enrollment compilation audit** | **Done** — [`e2b_resource_flow_enrollment_compilation_readiness.md`](reviews/e2b_resource_flow_enrollment_compilation_readiness.md) |
| **E-2B static enrollment** | **E** | **Composer 2.5** | **Selector → ExplicitParticipantSpec → E-11 flat-star** | **Done** — `resource_flow_enrollment_*` tests |
| **E-2B-5** | **E** | **Cursor** | **Policy A dynamic fission enrollment** | **Done** — [`resource_flow_fission_enrollment.rs`](../crates/simthing-driver/src/resource_flow_fission_enrollment.rs) |
| **E-2B-5R** | **E** | **Cursor** | **Atomicity + visible boundary diagnostics** | **Done** — prepare/commit admission; session report retention |
| **E-2B-5 soak** | **E** | **Cursor** | **Dynamic enrollment opt-in soak / burn-in** | **Done** — [`resource_flow_dynamic_enrollment_soak.rs`](../crates/simthing-driver/src/resource_flow_dynamic_enrollment_soak.rs) |
| **RF default-on readiness** | **E** | **Cursor (memo only)** | **Default-on posture audit** | **Done** — [`resource_flow_default_on_readiness_review.md`](reviews/resource_flow_default_on_readiness_review.md); **Recommendation B** |
| **E-2B-5 readiness** | **E** | **Opus (memo only)** | **Dynamic fission enrollment audit** | **Done** — [`e2b5_dynamic_fission_enrollment_readiness.md`](reviews/e2b5_dynamic_fission_enrollment_readiness.md) |
| S-1 | F | Codex 5.5 | Sunset intent fold | **Done locally** |
| S-2 | F | Codex 5.5 | Sunset intensity update | **Landed (#138)** |
| S-3 | F | Codex 5.5 | Sunset overlay prep | CI green at flag=on |
| S-4 | F | Codex 5.5 | Sunset reduction passes 4–6 | **Landed** |
| S-5 | F | Codex 5.5 | Sunset velocity integration | **Done locally** |
| S-6 | F | Codex 5.5 | Sunset threshold scan | **Done locally** |
| G-1 | G | Codex 5.5 | Annotate design_v6.md §10 superseded | One commit |
| **G-2** | **G** | **Opus** | **design_v7.md §4 final review** | **Human + Opus** |
| M-1 | M | Composer 2.5 | RegionField execution API on StructuredFieldStencilOp (generic) | **Done** — stencil parity + reduce-into bit-exact |
| M-2 | M | Composer 2.5 | Cadence scheduler + dirty macro-region skip (driver) | **Done** — determinism + zero false-skips |
| M-3 | M | Composer 2.5 | `RegionFieldSpec` RON + mapping admission framework (designer-layer) | **Done** — rejection suite + RON roundtrip; default-off |
| **M-product-fixture** | **M** | **Codex 5.5** | **Product-facing first-slice scenario fixture** | **Done** — product-style RegionFieldSpec/RON fixture over accepted GPU-resident first-slice runtime; default-off + explicit opt-in + no-readback hot path |
| **M-4** | **M** | **Opus + Composer** | **Atlas batching isolation + VRAM accounting (provisional)** | **Design note Done; isolation ratified 2026-05-28** (algebraic G=0 mask preferred, gutter fallback, §11 binding gate). Atlas still unimplemented; implementation gated on a §11-passing PR. Option 3 fixture landed; atlas packer still not next. |
| **M-4A** | **M** | **Composer 2.5** | **Algebraic tile-local atlas masking sandbox** | **Done; reverted** — [`mapping_atlas_algebraic_mask_sandbox_test_results.md`](tests/mapping_atlas_algebraic_mask_sandbox_test_results.md) |
| M-5 | M | Composer 2.5 | Generic source-identity buffer (behavioral source policy) | **DEFERRED** — not scheduled until a scenario names the need |

**Total: 43 active PRs** (38 prior + M-1..M-4 + M-4A; M-5 deferred). Remaining
Opus-gated: A-4, B-4, D-1 (memo), E-11, G-2, **M-4** — six. D-2 deferred
indefinitely; M-5 deferred pending a named scenario. Resource Flow Substrate
landing spans E-7 through E-11 per
`docs/adr/resource_flow_substrate.md`. Phase M lands the few generic natives for
`docs/adr/mapping_sparse_regioncell.md`; it authorizes no mapping runtime and
changes no default.
