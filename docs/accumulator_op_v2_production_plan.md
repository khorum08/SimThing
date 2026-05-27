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
(spec/builder integration); shared-input cross-pool contention (D-1); Soft/Fast
EML classes remain future-gated.

---

## Phase D — Contention hardening and performance gates

Phase D is conditional: it begins only if Phase C's contention scenario
benchmarks show the v1 allocator is a production bottleneck.

### ⚠️ PR D-1 — RESCOPED to discrete-transaction contention analysis memo

**Status (2026-05-26):** Original hot-pool allocator v2 design scope is
**dissolved for the continuous-flow case** by the Resource Flow Substrate ADR
(`docs/adr/resource_flow_substrate.md`). Continuous flow eliminates per-tick
shared-pool contention architecturally — the workshop's 16-pool / 100k
requester regime cannot arise under the Resource Flow Substrate because no
shared pool slot is written at tick time. Hierarchical fanout distributes
contention across tree depth.

**D-1 rescoped to a short Opus memo** evaluating whether *discrete*
transactions (construction commits, treaty payments, emergency spend) reach
contention scales that justify a GPU allocator at all. Likely outcome:
CPU-side priority queue with `SubtractFromSource` ops at boundary time is
sufficient at realistic scales (O(10²) discrete decisions per faction per
boundary, vs the workshop's O(10⁵)).

**Model:** Opus (memo only)
**Gate:** Memo committed to `docs/workshop/`. No implementation PR.
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
| **D-1** | **D** | **Opus (memo only)** | **Discrete-transaction contention analysis memo** | **Memo committed** |
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

#### PR E-2B — `resource_flow_participant(...)`

```rust
pub fn resource_flow_participant(
    slot:     SlotId,
    arena:    ArenaName,
    role:     AccumulatorRole,  // IntrinsicFlow | AllocatedFlow | AllocatorWeight
) -> AccumulatorOpSet
```

Produces the registrations that enroll a slot in an arena's continuous-flow
substrate. Used by E-9 `ArenaRegistry` compilation. Returns a set (not a single
op) because enrollment may produce reduction + allocation registrations.

**Test:** Enrollment test for the flow participant builder (arena participant set
is well-formed; reduction + allocation ops are emitted as expected).
**Acceptance:** E-2A and E-2B tests pass independently. The two-overlay transfer hack is removed.

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
**Status:** **Done** — PR #159 (`8a628ca`); allocation execution + `e11_*` suite (14/14). `use_accumulator_resource_flow` default **false** pending burn-in.
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
| **D-1** | **D** | **Opus (memo only)** | **RESCOPED to discrete-transaction memo — see ADR `resource_flow_substrate.md`** | **Memo committed** |
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
| **E-11** | **E** | **Opus + Composer 2.5** | **Hierarchical allocation kernel + CPU oracle parity + stability tests** | **Done** |
| S-1 | F | Codex 5.5 | Sunset intent fold | **Done locally** |
| S-2 | F | Codex 5.5 | Sunset intensity update | **Landed (#138)** |
| S-3 | F | Codex 5.5 | Sunset overlay prep | CI green at flag=on |
| S-4 | F | Codex 5.5 | Sunset reduction passes 4–6 | **Landed** |
| S-5 | F | Codex 5.5 | Sunset velocity integration | **Done locally** |
| S-6 | F | Codex 5.5 | Sunset threshold scan | **Done locally** |
| G-1 | G | Codex 5.5 | Annotate design_v6.md §10 superseded | One commit |
| **G-2** | **G** | **Opus** | **design_v7.md §4 final review** | **Human + Opus** |

**Total: 38 PRs.** Remaining Opus-gated: A-4, B-4, D-1 (memo), E-11, G-2 —
five of thirty-eight. D-2 deferred indefinitely. Resource Flow Substrate
landing spans E-7 through E-11 per
`docs/adr/resource_flow_substrate.md`.
