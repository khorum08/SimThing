# NVIDIA FP temporary battery 11 — feeder / workspace sweep

**Temporary file:** `docs/tests/nvidia_fp_temp_11_feeder_workspace_sweep.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `11 - feeder / workspace sweep`
**Status:** PARTIAL / KNOWN TRIAGE

## Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture
cargo test -p simthing-feeder --test integration -- --nocapture
cargo test --workspace -- --nocapture
```

## Adapter evidence

```text
requested_adapter_substring: RTX
require_adapter_match: true
adapter_inventory: Intel(R) RaptorLake-S Mobile Graphics Controller; NVIDIA GeForce RTX 4080 Laptop GPU; Intel(R) UHD Graphics; Microsoft Basic Render Driver (see raw log)
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
selected_adapter_is_intel: false
gpu_tier_ran: true
```

## Results summary

| Target | Tests passed | Failed | Ignored | Notes |
|---|---:|---:|---:|---|
| adapter gate | 1 | 0 | 0 | |
| simthing-feeder integration | 5 | 0 | 0 | GPU patch/dispatch tests |
| cargo test --workspace | — | — | — | **stopped at compile** (see below) |

**Feeder sub-battery:** PASS (5/0/0).

**Workspace smoke:** PARTIAL / KNOWN TRIAGE — `cargo test --workspace` did not complete; compile failed on `phase_m_jit_desc0_kernel_descriptor` (`KernelDescriptor` field errors). Workspace did not reach a full test summary. Known Battery 07/08 triage tests were not re-run to completion in this workspace invocation.

## Performance/timing capture

| Command block | Cargo build | Test runtime |
|---|---|---|
| adapter gate | 0.26s | 0.90s |
| simthing-feeder integration | 1.17s | 1.45s |
| workspace smoke | stopped at compile (~31s wall) | n/a |

Diagnostic wall-clock only.

## Tolerance/parity standard

Feeder integration uses GPU patch/coalesce paths (`GpuVerified` discipline). No tolerance changes.

## Support fixture coverage check

| Fixture | Prior battery coverage |
|---|---|
| `tests/support/e11_flat_star` | covered by Battery 09 (`e11_arena_allocation`, E11B nested tests use E11 support) |
| `tests/support/e11_nested` | covered by Battery 09 (`e11b_nested_fission_gap`, `e11b_nested_hierarchy_gpu`) |
| `tests/support/gpu_exec0_fixture` | covered by Battery 07 (`phase_m_jit_exec0_production_candidate_fixture` passed) |
| `tests/support/mobility_gpu_kernel0/5_fixture` | not found as dedicated RTX battery `--test` run; referenced in Battery 03 compile graph only |
| `tests/support/daily_economy_session` | not found in committed nvidia_fp_temp battery runs |
| `tests/support/resource_economy_session` | covered by Battery 09 (`resource_economy_session_open`) |
| `tests/support/sead_v1_live_pipeline` | covered by Battery 08 (`phase_m_sead_pipe0_observer_event_pipeline`); compile deps in multiple batteries |

## Intel baseline comparison

| Target | Prior Intel result | NVIDIA RTX result | Notes |
|---|---|---|---|
| simthing-feeder integration | not found in committed logs for this target | 5/0/0 | inventory lists feeder |
| cargo test --workspace | not found in committed logs for this target | compile incomplete | not a single-pass PASS/FAIL count |
| Cargo timings | not found in committed logs for this target | see table | |

## Open triage items for Opus

1. Battery 07 / `jit_grad0_mag2_not_overclaimed_if_approximate`:
   stale doc-hygiene guard likely; reads closed/archive accumulator_op_v2 production plan.
   Not native sqrt; shader path uses mag2 and forbids sqrt.

2. Battery 07 / `jit_exec1_distinct_graphs_remain_separate_entries`:
   admission-ordering / harness bug likely; mixed cohort reached GPU helper before rejection.
   Not native sqrt; not NVIDIA FP tolerance drift.

3. Battery 08 / `phase_m_boundary_cadence_doctrine`:
   stale/missing doc-hygiene dependency.
   Test includes missing `docs/workshop/workshop_current_state.md`.
   Not NVIDIA FP drift; not SEAD runtime failure.

**Workspace note:** Full workspace confirms driver test compile failures in the JIT descriptor family (`phase_m_jit_desc0_kernel_descriptor`) before a workspace-wide pass/fail tally. No new NVIDIA FP drift detected in **feeder** integration. Known Battery 07/08 runtime/doc issues remain open; this workspace run did not launder them into PASS.

## Raw decisive excerpts

```text
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
test result: ok. 5 passed; 0 failed; 0 ignored (simthing-feeder integration)
error: could not compile `simthing-driver` (test "phase_m_jit_desc0_kernel_descriptor") due to 20 previous errors
error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
```

## Failures / blocked reason

- Workspace smoke: compile stop (not feeder failure).
- No new test failures beyond known triage family identified in this battery run.

## Interpretation

`simthing-feeder` GPU integration passes on RTX 4080. Full workspace smoke is **PARTIAL / KNOWN TRIAGE**: compile blocked on JIT descriptor test skew; does not complete sweep or resolve Batteries 07–08. Full NVIDIA sweep remains incomplete until Opus triage on 07/08 (and workspace compile health) is closed.

## §0.5 check

Evidence-only NVIDIA validation and temporary ladder bookkeeping; no shader/math/tolerance/source changes, no gameplay resource-flow behavior, no simthing-sim semantic expansion, no default session wiring.

---

## Raw cargo log

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-ce721f94-6209-4439-8ac0-6320a0fdbb97.ps1:87 char:1
+ cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_st ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unused...mlConsumerKind`:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:41:85
   |
41 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
   |                                                                                     ^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:124:6
    |
124 | impl EmlTreeMeta {
    |      ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:144:11
    |
144 | impl From<EmlTreeMeta> for EmlFormulaMeta {
    |           ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:674:41
    |
674 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:223:15
    |
223 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:535:65
    |
535 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:536:45
    |
536 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:126:12
    |
126 |         if self.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:129:12
    |
129 |         if self.node_count == 0 {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:132:12
    |
132 |         if self.node_count > MAX_EML_TREE_NODES {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:55
    |
133 |             return Err(EmlRegistryError::TooManyNodes(self.node_count));
    |                                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:135:51
    |
135 |         if !WHITELISTED_FORMULA_CLASSES.contains(&self.formula_class.as_str()) {
    |                                                   ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:137:17
    |
137 |                 self.formula_class.clone(),
    |                 ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:147:12
    |
147 |         if legacy.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:155:29
    |
155 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:159:31
    |
159 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:169:29
    |
169 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:173:31
    |
173 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:537:13
    |
537 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:538:13
    |
538 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:539:13
    |
539 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:675:8
    |
675 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:15
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:39
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:679:45
    |
679 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 27 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: unused import: `RF_CONTINUED_STATIC_512`
  --> crates\simthing-driver\src\resource_flow_flat_star_continued_soak.rs:13:5
   |
13 |     RF_CONTINUED_STATIC_512, RF_CONTINUED_STATIC_SKEWED,
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (lib) generated 1 warning (run `cargo fix --lib -p simthing-driver` to apply 1 suggestion)
warning: unused import: `channel_set_has_kind`
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:19:5
   |
19 |     channel_set_has_kind, ChannelKind, ChannelSet, LocationId,
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `ChannelSet`
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:14:34
   |
14 |     AtlasBatchPlan, ChannelKind, ChannelSet, LocationId, LocationMaterialization, LocationRole,
   |                                  ^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:6:11
  |
6 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_ID: &str = "ATLAS-BATCH-0-STORE";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:7:11
  |
7 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `canonical_pack_plan` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:64:8
   |
64 | pub fn canonical_pack_plan() -> AtlasBatchPlan {
   |        ^^^^^^^^^^^^^^^^^^^

warning: function `store_oracle_with_additional_sources` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:258:8
    |
258 | pub fn store_oracle_with_additional_sources(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `class_id_for_location_role` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:349:8
    |
349 | pub fn class_id_for_location_role(role: LocationRole) -> &'static str {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `pack_round_trip_cell` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:357:8
    |
357 | pub fn pack_round_trip_cell(
    |        ^^^^^^^^^^^^^^^^^^^^

warning: function `pirate_fleet_source_ids` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:406:8
    |
406 | pub fn pirate_fleet_source_ids(materialization: &LocationMaterialization) -> Vec<String> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID: &str = "ATLAS-BATCH-0-PACK";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `V78_ATLAS_VRAM_BUDGET_BYTES` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:6:11
  |
6 | pub const V78_ATLAS_VRAM_BUDGET_BYTES: u64 = 1_610_612_736;
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `PACKING_STRATEGY` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:8:11
  |
8 | pub const PACKING_STRATEGY: &str =
  |           ^^^^^^^^^^^^^^^^

warning: constant `CLASS_GALACTIC_20X20` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:11:11
   |
11 | pub const CLASS_GALACTIC_20X20: &str = "Galactic20x20";
   |           ^^^^^^^^^^^^^^^^^^^^

warning: constant `CLASS_STAR_SYSTEM_10X10` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:12:11
   |
12 | pub const CLASS_STAR_SYSTEM_10X10: &str = "StarSystem10x10";
   |           ^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `CLASS_PLANET_SURFACE_10X10` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:13:11
   |
13 | pub const CLASS_PLANET_SURFACE_10X10: &str = "PlanetSurface10x10";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `GALAXY_SIDE` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:23:7
   |
23 | const GALAXY_SIDE: u32 = 20;
   |       ^^^^^^^^^^^

warning: constant `SYSTEM_SIDE` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:24:7
   |
24 | const SYSTEM_SIDE: u32 = 10;
   |       ^^^^^^^^^^^

warning: constant `PLANET_SURFACE_SIDE` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:25:7
   |
25 | const PLANET_SURFACE_SIDE: u32 = 10;
   |       ^^^^^^^^^^^^^^^^^^^

warning: struct `TileClassDescriptor` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:28:12
   |
28 | pub struct TileClassDescriptor {
   |            ^^^^^^^^^^^^^^^^^^^

warning: struct `PackedTile` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:40:12
   |
40 | pub struct PackedTile {
   |            ^^^^^^^^^^

warning: struct `TileMaskBounds` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:49:12
   |
49 | pub struct TileMaskBounds {
   |            ^^^^^^^^^^^^^^

warning: struct `GZeroMaskDescriptor` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:57:12
   |
57 | pub struct GZeroMaskDescriptor {
   |            ^^^^^^^^^^^^^^^^^^^

warning: struct `VramReport` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:63:12
   |
63 | pub struct VramReport {
   |            ^^^^^^^^^^

warning: struct `AtlasBatchPlan` is never constructed
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:76:12
   |
76 | pub struct AtlasBatchPlan {
   |            ^^^^^^^^^^^^^^

warning: associated items `canonical`, `from_materialization`, `class`, and `tiles_in_class` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:85:12
    |
 84 | impl AtlasBatchPlan {
    | ------------------- associated items in this implementation
 85 |     pub fn canonical() -> Self {
    |            ^^^^^^^^^
...
 89 |     pub fn from_materialization(materialization: &LocationMaterialization) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^
...
153 |     pub fn class(&self, class_id: &str) -> Option<&TileClassDescriptor> {
    |            ^^^^^
...
157 |     pub fn tiles_in_class(&self, class_id: &str) -> Vec<&PackedTile> {
    |            ^^^^^^^^^^^^^^

warning: function `pack_class_row_major` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:165:4
    |
165 | fn pack_class_row_major(
    |    ^^^^^^^^^^^^^^^^^^^^

warning: function `build_g_zero_mask` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:208:4
    |
208 | fn build_g_zero_mask(
    |    ^^^^^^^^^^^^^^^^^

warning: function `build_vram_report` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:229:4
    |
229 | fn build_vram_report(
    |    ^^^^^^^^^^^^^^^^^

warning: function `bytes_per_cell_for_channels` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:273:4
    |
273 | fn bytes_per_cell_for_channels(channels: &ChannelSet) -> u64 {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `pack_coord` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:277:8
    |
277 | pub fn pack_coord(plan: &AtlasBatchPlan, location_id: LocationId, x: u32, y: u32) -> Option<(u32, u32)> {
    |        ^^^^^^^^^^

warning: function `unpack_coord` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:288:8
    |
288 | pub fn unpack_coord(
    |        ^^^^^^^^^^^^

warning: function `tile_source_at_atlas` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:308:4
    |
308 | fn tile_source_at_atlas(plan: &AtlasBatchPlan, class_id: &str, ax: u32, ay: u32) -> Option<LocationId> {
    |    ^^^^^^^^^^^^^^^^^^^^

warning: function `g_zero_sample` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:324:8
    |
324 | pub fn g_zero_sample(
    |        ^^^^^^^^^^^^^

warning: function `atlas_linear_index` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:351:4
    |
351 | fn atlas_linear_index(atlas_width: u32, ax: u32, ay: u32) -> Option<usize> {
    |    ^^^^^^^^^^^^^^^^^^

warning: function `channel_set_matches` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:355:8
    |
355 | pub fn channel_set_matches(lhs: &ChannelSet, rhs: &ChannelSet) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^

warning: function `channel_set_has_owner_indexed` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:359:8
    |
359 | pub fn channel_set_has_owner_indexed(set: &ChannelSet) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID: &str = "ATLAS-BATCH-0-LOC";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `EXPECTED_TOTAL_CELL_SLOTS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:7:11
  |
7 | pub const EXPECTED_TOTAL_CELL_SLOTS: u32 = 3000;
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `occupants_at` and `locations_by_role` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:242:12
    |
 93 | impl LocationMaterialization {
    | ---------------------------- methods in this implementation
...
242 |     pub fn occupants_at(&self, location_id: LocationId, cell: GridCell) -> Vec<&OccupantPlacement> {
    |            ^^^^^^^^^^^^
...
249 |     pub fn locations_by_role(&self, role: LocationRole) -> Vec<&LocationGridDescriptor> {
    |            ^^^^^^^^^^^^^^^^^

warning: function `cell_index` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:257:8
    |
257 | pub fn cell_index(map_base: u32, width: u32, x: u32, y: u32) -> u32 {
    |        ^^^^^^^^^^

warning: function `channel_set_has_kind` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:317:8
    |
317 | pub fn channel_set_has_kind(set: &ChannelSet, expected: ChannelKind) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_ID: &str = "ATLAS-BATCH-0-GEN";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `PIRATE_STARPORT_COUNT` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:13:11
   |
13 | pub const PIRATE_STARPORT_COUNT: usize = 1;
   |           ^^^^^^^^^^^^^^^^^^^^^

warning: methods `in_bounds`, `chebyshev_distance`, and `empty_cells_between` are never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:54:12
   |
49 | impl GridCell {
   | ------------- methods in this implementation
...
54 |     pub fn in_bounds(self, side: u32) -> bool {
   |            ^^^^^^^^^
...
58 |     pub fn chebyshev_distance(self, other: Self) -> u32 {
   |            ^^^^^^^^^^^^^^^^^^
...
62 |     pub fn empty_cells_between(self, other: Self) -> u32 {
   |            ^^^^^^^^^^^^^^^^^^^

warning: method `cell_count` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:81:18
   |
73 | impl GridDims {
   | ------------- method in this implementation
...
81 |     pub const fn cell_count(&self) -> u32 {
   |                  ^^^^^^^^^^

warning: methods `terran_systems`, `pirate_systems`, `starports`, `fleets_by_owner`, `minimum_terran_empty_spacing`, 
and `pirate_within_one_empty_cell_of_terran` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:182:12
    |
137 | impl DressRehearsalMap {
    | ---------------------- methods in this implementation
...
182 |     pub fn terran_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
    |            ^^^^^^^^^^^^^^
...
188 |     pub fn pirate_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
    |            ^^^^^^^^^^^^^^
...
194 |     pub fn starports(&self) -> impl Iterator<Item = &BuildingPlacement> {
    |            ^^^^^^^^^
...
200 |     pub fn fleets_by_owner(&self, owner: Owner) -> impl Iterator<Item = &FleetPlacement> {
    |            ^^^^^^^^^^^^^^^
...
204 |     pub fn minimum_terran_empty_spacing(&self) -> Option<u32> {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
218 |     pub fn pirate_within_one_empty_cell_of_terran(&self, pirate: &SystemDescriptor) -> bool {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_store_gpu") generated 49 warnings (run `cargo fix 
--test "dress_rehearsal_atlas_batch_0_store_gpu" -p simthing-driver` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.26s
     Running tests\dress_rehearsal_atlas_batch_0_store_gpu.rs 
(target\debug\deps\dress_rehearsal_atlas_batch_0_store_gpu-831a199d6239664e.exe)

running 1 test
adapter_inventory: [Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) UHD Graphics, Microsoft Basic Render Driver, Intel(R) UHD Graphics]
requested_adapter_substring: RTX
require_adapter_match: true
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
gpu_tier_ran: true
test gpu_adapter_is_discrete_rtx_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.90s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-ce721f94-6209-4439-8ac0-6320a0fdbb97.ps1:90 char:1
+ cargo test -p simthing-feeder --test integration -- --nocapture 2>&1  ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unused...mlConsumerKind`:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:41:85
   |
41 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
   |                                                                                     ^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:124:6
    |
124 | impl EmlTreeMeta {
    |      ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:144:11
    |
144 | impl From<EmlTreeMeta> for EmlFormulaMeta {
    |           ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:674:41
    |
674 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:223:15
    |
223 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:535:65
    |
535 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:536:45
    |
536 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:126:12
    |
126 |         if self.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:129:12
    |
129 |         if self.node_count == 0 {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:132:12
    |
132 |         if self.node_count > MAX_EML_TREE_NODES {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:55
    |
133 |             return Err(EmlRegistryError::TooManyNodes(self.node_count));
    |                                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:135:51
    |
135 |         if !WHITELISTED_FORMULA_CLASSES.contains(&self.formula_class.as_str()) {
    |                                                   ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:137:17
    |
137 |                 self.formula_class.clone(),
    |                 ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:147:12
    |
147 |         if legacy.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:155:29
    |
155 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:159:31
    |
159 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:169:29
    |
169 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:173:31
    |
173 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:537:13
    |
537 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:538:13
    |
538 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:539:13
    |
539 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:675:8
    |
675 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:15
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:39
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:679:45
    |
679 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 27 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
   Compiling simthing-feeder v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-feeder)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.17s
     Running tests\integration.rs (target\debug\deps\integration-7b18036ccc8cb411.exe)

running 5 tests
test boundary_requests_reach_tree_maintainer ... ok
test day_boundary_fires_on_ticks_per_day ... ok
test patch_through_channel_lands_on_gpu_after_one_tick ... ok
test add_and_multiply_patches_apply_on_gpu_without_rmw_readback ... ok
test many_patches_same_cell_coalesce_to_one_intent_delta ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.45s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-e5d9e50a-fff5-45c6-a549-3ced378218e3.ps1:88 char:1
+ cargo test --workspace -- --nocapture 2>&1 | Tee-Object -Append docs/ ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unused...mlConsumerKind`:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:41:85
   |
41 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
   |                                                                                     ^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:124:6
    |
124 | impl EmlTreeMeta {
    |      ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:144:11
    |
144 | impl From<EmlTreeMeta> for EmlFormulaMeta {
    |           ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:674:41
    |
674 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:223:15
    |
223 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:535:65
    |
535 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:536:45
    |
536 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:126:12
    |
126 |         if self.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:129:12
    |
129 |         if self.node_count == 0 {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:132:12
    |
132 |         if self.node_count > MAX_EML_TREE_NODES {
    |            ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:55
    |
133 |             return Err(EmlRegistryError::TooManyNodes(self.node_count));
    |                                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:135:51
    |
135 |         if !WHITELISTED_FORMULA_CLASSES.contains(&self.formula_class.as_str()) {
    |                                                   ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:137:17
    |
137 |                 self.formula_class.clone(),
    |                 ^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:147:12
    |
147 |         if legacy.has_transcendental {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:155:29
    |
155 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:159:31
    |
159 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:169:29
    |
169 |                 node_count: legacy.node_count,
    |                             ^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:173:31
    |
173 |                 display_name: legacy.formula_class,
    |                               ^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:537:13
    |
537 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:538:13
    |
538 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:539:13
    |
539 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:675:8
    |
675 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:15
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:677:39
    |
677 |     } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
    |                                       ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:679:45
    |
679 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

   Compiling simthing-core v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-core)
warning: `simthing-core` (lib) generated 27 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
   Compiling simthing-gpu v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-gpu)
   Compiling simthing-feeder v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-feeder)
   Compiling simthing-sim v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-sim)
warning: unused import: `RF_CONTINUED_STATIC_512`
  --> crates\simthing-driver\src\resource_flow_flat_star_continued_soak.rs:13:5
   |
13 |     RF_CONTINUED_STATIC_512, RF_CONTINUED_STATIC_SKEWED,
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (lib) generated 1 warning (run `cargo fix --lib -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (lib test) generated 1 warning (1 duplicate)
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
   Compiling simthing-spec v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-spec)
warning: variable does not need to be mutable
   --> crates\simthing-sim\tests\s2_legacy_intensity_sunset.rs:122:9
    |
122 |     let mut state = WorldGpuState::new(ctx, &proto.registry, 1);
    |         ----^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `n_dims`
   --> crates\simthing-sim\tests\c8b_intensity_eml_parity.rs:199:9
    |
199 |     let n_dims = reg.total_columns;
    |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_n_dims`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:417:9
    |
417 |     let mut resource_flow_syncs = if enrollment_report.any_admissions() && resource_flow_enabled {
    |         ----^^^^^^^^^^^^^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: constant `SMALL_FLAT_STAR_EQUAL_WEIGHTS` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:14:11
   |
14 | pub const SMALL_FLAT_STAR_EQUAL_WEIGHTS: &str = "small_flat_star_equal_weights";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `SMALL_FLAT_STAR_SKEWED_WEIGHTS` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:15:11
   |
15 | pub const SMALL_FLAT_STAR_SKEWED_WEIGHTS: &str = "small_flat_star_skewed_weights";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `SMALL_FLAT_STAR_ZERO_WEIGHTS` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:16:11
   |
16 | pub const SMALL_FLAT_STAR_ZERO_WEIGHTS: &str = "small_flat_star_zero_weights";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `SMALL_FLAT_STAR_REPEATED_BOUNDARY_SYNC` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:17:11
   |
17 | pub const SMALL_FLAT_STAR_REPEATED_BOUNDARY_SYNC: &str = "small_flat_star_repeated_boundary_sync";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `BurnInScenarioFixture` is never constructed
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:20:12
   |
20 | pub struct BurnInScenarioFixture {
   |            ^^^^^^^^^^^^^^^^^^^^^

warning: function `small_flat_star_equal_weights` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:28:8
   |
28 | pub fn small_flat_star_equal_weights() -> BurnInScenarioFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `small_flat_star_skewed_weights` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:38:8
   |
38 | pub fn small_flat_star_skewed_weights() -> BurnInScenarioFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `small_flat_star_zero_weights` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:48:8
   |
48 | pub fn small_flat_star_zero_weights() -> BurnInScenarioFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `small_flat_star_repeated_boundary_sync` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:58:8
   |
58 | pub fn small_flat_star_repeated_boundary_sync() -> BurnInScenarioFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `open_scenario_session` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:68:8
   |
68 | pub fn open_scenario_session(fixture: &BurnInScenarioFixture) -> FlatStarSession {
   |        ^^^^^^^^^^^^^^^^^^^^^

warning: function `scenario_cell_inputs` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:79:8
   |
79 | pub fn scenario_cell_inputs(
   |        ^^^^^^^^^^^^^^^^^^^^

warning: function `run_scenario_burn_in` is never used
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:93:8
   |
93 | pub fn run_scenario_burn_in(
   |        ^^^^^^^^^^^^^^^^^^^^

warning: function `assert_flat_star_only_no_nested_claims` is never used
   --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:132:8
    |
132 | pub fn assert_flat_star_only_no_nested_claims(fx: &FlatStarSession) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `assert_no_nan_in_leaf_allocated` is never used
   --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:184:8
    |
184 | pub fn assert_no_nan_in_leaf_allocated(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `flat_star_game_mode` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:90:8
   |
90 | pub fn flat_star_game_mode(max_orderband_depth: u32) -> GameModeSpec {
   |        ^^^^^^^^^^^^^^^^^^^

warning: function `fill_explicit_participants` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:128:8
    |
128 | pub fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FlatStarSession` is never constructed
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:140:12
    |
140 | pub struct FlatStarSession {
    |            ^^^^^^^^^^^^^^^

warning: function `open_flat_star_session` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:146:8
    |
146 | pub fn open_flat_star_session(hosted_count: usize, flag_enabled: bool) -> FlatStarSession {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `root_slot` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:194:8
    |
194 | pub fn root_slot(layout: &ArenaTreeLayout) -> u32 {
    |        ^^^^^^^^^

warning: function `leaf_slots` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:198:8
    |
198 | pub fn leaf_slots(layout: &ArenaTreeLayout) -> Vec<u32> {
    |        ^^^^^^^^^^

warning: function `standard_flat_star_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:221:8
    |
221 | pub fn standard_flat_star_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `ResourceFlowSoakMode` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:15:10
   |
15 | pub enum ResourceFlowSoakMode {
   |          ^^^^^^^^^^^^^^^^^^^^

warning: struct `ResourceFlowSoakFixture` is never constructed
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:21:12
   |
21 | pub struct ResourceFlowSoakFixture {
   |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `soak_equal_weights_1000` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:31:8
   |
31 | pub fn soak_equal_weights_1000() -> ResourceFlowSoakFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `soak_skewed_weights_1000` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:43:8
   |
43 | pub fn soak_skewed_weights_1000() -> ResourceFlowSoakFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `soak_zero_weights_1000` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:55:8
   |
55 | pub fn soak_zero_weights_1000() -> ResourceFlowSoakFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `soak_repeated_resync_100` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:67:8
   |
67 | pub fn soak_repeated_resync_100() -> ResourceFlowSoakFixture {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `assert_soak_opt_in` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:79:8
   |
79 | pub fn assert_soak_opt_in(soak: &ResourceFlowSoakFixture) {
   |        ^^^^^^^^^^^^^^^^^^

warning: function `run_flat_star_soak` is never used
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:88:8
   |
88 | pub fn run_flat_star_soak(fx: &mut FlatStarSession, soak: &ResourceFlowSoakFixture) -> 
ResourceFlowSoakSummaryReport {
   |        ^^^^^^^^^^^^^^^^^^

warning: function `assert_reject_no_partial_mutation` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:742:8
    |
742 | pub fn assert_reject_no_partial_mutation(fx: &EnrolledSoakSession, child_id: SimThingId) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `assert_contiguity_unchanged_on_reject` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:753:8
    |
753 | pub fn assert_contiguity_unchanged_on_reject(setup: &EnrollmentSoakSetup, fx: &EnrolledSoakSession) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `child_id_for_reject_fixture` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:806:8
    |
806 | pub fn child_id_for_reject_fixture() -> SimThingId {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `child_id_for_contiguity_fixture` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:811:8
    |
811 | pub fn child_id_for_contiguity_fixture() -> SimThingId {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reserved_gap_slots_unchanged` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:816:8
    |
816 | pub fn reserved_gap_slots_unchanged(setup: &EnrollmentSoakSetup, fx: &EnrolledSoakSession) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_PROFILE_NAME` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:10:11
   |
10 | pub const FRONTIER_V1_PROFILE_NAME: &str = "FrontierV1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_SKELETON_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:11:11
   |
11 | pub const FRONTIER_V1_SKELETON_ID: &str = "frontier_v1_0_scenario_skeleton_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:12:11
   |
12 | pub const FRONTIER_V1_FIXTURE_ID: &str = "frontier_v1_1_opt_in_fixture_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_GPU_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:13:11
   |
13 | pub const FRONTIER_V1_GPU_FIXTURE_ID: &str = "frontier_v1_2_gpu_replay_acceptance_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_GPU_RF_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:14:11
   |
14 | pub const FRONTIER_V1_GPU_RF_FIXTURE_ID: &str = "frontier_v1_3_gpu_resource_flow_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_SEAD_ROUTE_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:15:11
   |
15 | pub const FRONTIER_V1_SEAD_ROUTE_FIXTURE_ID: &str = "frontier_v1_4_sead_route_replay_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_LIVE_SELF_AI_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:16:11
   |
16 | pub const FRONTIER_V1_LIVE_SELF_AI_FIXTURE_ID: &str = "frontier_v1_5_live_self_ai_route_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_PROFILE_NAME` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:19:11
   |
19 | pub const FRONTIER_V2_PROFILE_NAME: &str = "FrontierV2";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_RESOURCE_PROPOSAL_CODE` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:22:11
   |
22 | pub const FRONTIER_V1_RESOURCE_PROPOSAL_CODE: u32 = 1001;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_RESOURCE_EVENT_CODE` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:24:11
   |
24 | pub const FRONTIER_V1_RESOURCE_EVENT_CODE: u32 = 1;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_ALLOCATOR_ROUTE_CODE` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:27:11
   |
27 | pub const FRONTIER_V1_ALLOCATOR_ROUTE_CODE: u32 = 1;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_STRUCTURAL_ROUTE_CODE` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:29:11
   |
29 | pub const FRONTIER_V1_STRUCTURAL_ROUTE_CODE: u32 = 2;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_MOVEMENT_ROUTE_CODE` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:31:11
   |
31 | pub const FRONTIER_V1_MOVEMENT_ROUTE_CODE: u32 = 3;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `SeadPipelineVersion` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:34:10
   |
34 | pub enum SeadPipelineVersion {
   |          ^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierTheaterSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:40:12
   |
40 | pub struct FrontierTheaterSpec {
   |            ^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierFactionSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:55:12
   |
55 | pub struct FrontierFactionSpec {
   |            ^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierFlatStarResourceFlowSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:60:12
   |
60 | pub struct FrontierFlatStarResourceFlowSpec {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierSeadSelfAiSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:73:12
   |
73 | pub struct FrontierSeadSelfAiSpec {
   |            ^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierEconomyFieldCouplingSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:86:12
   |
86 | pub struct FrontierEconomyFieldCouplingSpec {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1ScenarioSkeleton` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:93:12
   |
93 | pub struct FrontierV1ScenarioSkeleton {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1AdmissionReport` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:107:12
    |
107 | pub struct FrontierV1AdmissionReport {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `ProposalKind` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:118:10
    |
118 | pub enum ProposalKind {
    |          ^^^^^^^^^^^^

warning: enum `ProposalRoute` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:125:10
    |
125 | pub enum ProposalRoute {
    |          ^^^^^^^^^^^^^

warning: struct `FrontierV1FixtureConfig` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:133:12
    |
133 | pub struct FrontierV1FixtureConfig {
    |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MappingSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:144:12
    |
144 | pub struct MappingSummary {
    |            ^^^^^^^^^^^^^^

warning: struct `ResourceFlowSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:151:12
    |
151 | pub struct ResourceFlowSummary {
    |            ^^^^^^^^^^^^^^^^^^^

warning: struct `RouteSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:158:12
    |
158 | pub struct RouteSummary {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1FixtureFingerprint` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:166:12
    |
166 | pub struct FrontierV1FixtureFingerprint {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `FrontierV1FieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:174:10
    |
174 | pub enum FrontierV1FieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^

warning: enum `FrontierV2Status` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:183:10
    |
183 | pub enum FrontierV2Status {
    |          ^^^^^^^^^^^^^^^^

warning: enum `FrontierV1LiveSelfAiFieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:189:10
    |
189 | pub enum FrontierV1LiveSelfAiFieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveSelfAiFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveSelfAiFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveSelfAiSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:215:12
    |
215 | pub struct FrontierV1LiveSelfAiSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:233:12
    |
232 | impl FrontierV1LiveSelfAiSummary {
    | -------------------------------- method in this implementation
233 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1LiveSelfAiOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveSelfAiOracleOutput {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1GpuReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:257:12
    |
257 | pub struct FrontierV1GpuReplaySummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:274:12
    |
273 | impl FrontierV1GpuReplaySummary {
    | ------------------------------- method in this implementation
274 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1RouteReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:294:12
    |
294 | pub struct FrontierV1RouteReplaySummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1SeadReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:306:12
    |
306 | pub struct FrontierV1SeadReplaySummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GpuResourceFlowAllocationSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:314:12
    |
314 | pub struct GpuResourceFlowAllocationSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `combined` and `hex` are never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:323:12
    |
322 | impl FrontierV1FixtureFingerprint {
    | --------------------------------- methods in this implementation
323 |     pub fn combined(&self) -> u64 {
    |            ^^^^^^^^
...
330 |     pub fn hex(&self) -> String {
    |            ^^^

warning: struct `FrontierV1FixtureOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:336:12
    |
336 | pub struct FrontierV1FixtureOutput {
    |            ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `frontier_v1_happy_path_skeleton` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:346:8
    |
346 | pub fn frontier_v1_happy_path_skeleton() -> FrontierV1ScenarioSkeleton {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `frontier_v1_1_smoke_skeleton` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:400:8
    |
400 | pub fn frontier_v1_1_smoke_skeleton() -> FrontierV1ScenarioSkeleton {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `frontier_v1_1_fixture_config` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:407:8
    |
407 | pub fn frontier_v1_1_fixture_config() -> FrontierV1FixtureConfig {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `frontier_v1_mapping_field_spec` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:424:8
    |
424 | pub fn frontier_v1_mapping_field_spec() -> RegionFieldSpec {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_gpu_field_values` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:459:8
    |
459 | pub fn hash_gpu_field_values(values: &[f32]) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_gpu_resource_flow` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:467:8
    |
467 | pub fn hash_gpu_resource_flow(summary: GpuResourceFlowAllocationSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `frontier_v1_flat_star_weights` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:477:8
    |
477 | pub fn frontier_v1_flat_star_weights() -> (f32, f32) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `proposal_route_to_code` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:481:8
    |
481 | pub fn proposal_route_to_code(route: ProposalRoute) -> Option<u32> {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_route_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:490:8
    |
490 | pub fn build_route_replay_summary(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_sead_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:518:8
    |
518 | pub fn build_sead_replay_summary(cpu_output: &FrontierV1FixtureOutput) -> FrontierV1SeadReplaySummary {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_route_replay_detail` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:527:8
    |
527 | pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_self_ai_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_self_ai_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_self_ai_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:558:8
    |
558 | pub fn hash_live_self_ai_feedback_candidate(c: FrontierV1LiveSelfAiFeedbackCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_self_ai_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:573:8
    |
573 | pub fn hash_live_self_ai_summary(summary: FrontierV1LiveSelfAiSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:597:8
    |
597 | pub fn build_feedback_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_live_self_ai_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:621:8
    |
621 | pub fn cpu_live_self_ai_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_sead_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:663:8
    |
663 | pub fn hash_sead_replay_summary(summary: FrontierV1SeadReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_gpu_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:672:8
    |
672 | pub fn build_gpu_replay_summary(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_gpu_replay_summary_with_rf` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:696:8
    |
696 | pub fn build_gpu_replay_summary_with_rf(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_frontier_v1_4_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:722:8
    |
722 | pub fn build_frontier_v1_4_replay_summary(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `gpu_resource_flow_from_cpu_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:757:8
    |
757 | pub fn gpu_resource_flow_from_cpu_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_frontier_v1_admission` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:770:8
    |
770 | pub fn validate_frontier_v1_admission(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `classify_proposal_route` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:794:8
    |
794 | pub fn classify_proposal_route(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_mapping_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:827:8
    |
827 | pub fn cpu_mapping_oracle(
    |        ^^^^^^^^^^^^^^^^^^

warning: function `cpu_resource_flow_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:862:8
    |
862 | pub fn cpu_resource_flow_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_route_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:885:8
    |
885 | pub fn cpu_route_oracle(
    |        ^^^^^^^^^^^^^^^^

warning: function `fingerprint_from_parts` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:906:8
    |
906 | pub fn fingerprint_from_parts(
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `run_frontier_v1_fixture` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:920:8
    |
920 | pub fn run_frontier_v1_fixture(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_default_off` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:942:4
    |
942 | fn validate_default_off(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    |    ^^^^^^^^^^^^^^^^^^^^

warning: function `validate_mapping` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:975:4
    |
975 | fn validate_mapping(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    |    ^^^^^^^^^^^^^^^^

warning: function `validate_flat_star` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1028:4
     |
1028 | fn validate_flat_star(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
     |    ^^^^^^^^^^^^^^^^^^

warning: function `validate_sead_routing` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1078:4
     |
1078 | fn validate_sead_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_coupling` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1120:4
     |
1120 | fn validate_coupling(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
     |    ^^^^^^^^^^^^^^^^^

warning: function `hash_mapping` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1149:4
     |
1149 | fn hash_mapping(m: MappingSummary) -> u64 {
     |    ^^^^^^^^^^^^

warning: function `hash_resource_flow` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1157:4
     |
1157 | fn hash_resource_flow(r: ResourceFlowSummary) -> u64 {
     |    ^^^^^^^^^^^^^^^^^^

warning: function `hash_routes` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1165:4
     |
1165 | fn hash_routes(r: RouteSummary) -> u64 {
     |    ^^^^^^^^^^^

warning: function `hash_u32` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1174:4
     |
1174 | fn hash_u32(v: u32) -> u64 {
     |    ^^^^^^^^

warning: function `fnv64` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1178:4
     |
1178 | fn fnv64(seed: &[u8]) -> u64 {
     |    ^^^^^

warning: function `fnv_append_u32` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1187:4
     |
1187 | fn fnv_append_u32(mut hash: u64, v: u32) -> u64 {
     |    ^^^^^^^^^^^^^^

warning: function `fnv_mix` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1195:4
     |
1195 | fn fnv_mix(v: u64) -> u64 {
     |    ^^^^^^^

warning: function `live_self_ai_field_status_code` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1199:4
     |
1199 | fn live_self_ai_field_status_code(s: FrontierV1LiveSelfAiFieldStatus) -> u32 {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FirstSliceScenarioFixtureSession` is never constructed
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:14:12
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated items `open`, `queue_seeds`, `tick_mapping`, `tick_with_scenario_commitment`, and 
`diagnostic_readback_reduction_eml` are never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:22:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- associated items in this implementation
21 |     /// Open from an admitted scenario compile preview. Commitment binding is taken from the preview only.
22 |     pub fn open(
   |            ^^^^
...
34 |     pub fn queue_seeds(&mut self, seeds: &[FirstSliceSeed]) -> Result<(), FirstSliceMappingError> {
   |            ^^^^^^^^^^^
...
38 |     pub fn tick_mapping(
   |            ^^^^^^^^^^^^
...
47 |     pub fn tick_with_scenario_commitment(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
61 |     pub fn diagnostic_readback_reduction_eml(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:774:21
    |
774 |                     EmlTreeMeta {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:791:17
    |
791 |                 EmlTreeMeta {
    |                 ^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:775:25
    |
775 |                         node_count: 1,
    |                         ^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:776:25
    |
776 |                         has_transcendental: false,
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:777:25
    |
777 |                         formula_class: class.to_string(),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:792:21
    |
792 |                     node_count: 8,
    |                     ^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:793:21
    |
793 |                     has_transcendental: true,
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:794:21
    |
794 |                     formula_class: "intensity_update".to_string(),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused variable: `pipelines`
   --> crates\simthing-gpu\src\passes.rs:632:45
    |
632 |     fn run_velocity_integration_test_helper(pipelines: &Pipelines, state: &WorldGpuState, dt: f32) {
    |                                             ^^^^^^^^^ help: if this is intentional, prefix it with an 
underscore: `_pipelines`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-core` (lib test) generated 34 warnings (26 duplicates)
warning: unused imports: `ExactSqrtAuthorityClass`, `MAG_F_FROM_DXDY_PROBE_LABEL`, `MAG_F_FROM_MAG2_LABEL`, 
`SQRT_F_ARTIFACT_PATH`, and `SQRT_F_ENTRYPOINT`
  --> crates\simthing-driver\tests\phase_m_jit_sqrt_mag0_f_exact_magnitude.rs:15:5
   |
15 |     ExactSqrtAuthorityClass, KernelDescriptorSpec, MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^
16 |     MAG_F_FROM_DXDY_PROBE_LABEL, MAG_F_FROM_MAG2_DESCRIPTOR_ID, MAG_F_FROM_MAG2_LABEL,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^                                 ^^^^^^^^^^^^^^^^^^^^^
17 |     MappingExecutionProfile, NativeMathClass, OutputAuthority, SQRT_F_ARTIFACT_HASH,
18 |     SQRT_F_ARTIFACT_PATH, SQRT_F_DESCRIPTOR_ID, SQRT_F_ENTRYPOINT, SQRT_F_PROOF_REPORT, SpecError,
   |     ^^^^^^^^^^^^^^^^^^^^                        ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `mag2_in`
   --> crates\simthing-driver\tests\phase_m_jit_sqrt_mag0_f_exact_magnitude.rs:327:19
    |
327 |         .map(|(i, mag2_in)| (out[i * 2], out[i * 2 + 1]))
    |                   ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mag2_in`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (test "phase_m_jit_sqrt_mag0_f_exact_magnitude") generated 2 warnings (run `cargo fix 
--test "phase_m_jit_sqrt_mag0_f_exact_magnitude" -p simthing-driver` to apply 2 suggestions)
warning: variant `Mag2OnlyQ12` is never constructed
  --> crates\simthing-driver\tests\phase_m_jit_sqrt_mag2_perf0_fixed_hotpath.rs:22:5
   |
18 | enum BenchPath {
   |      --------- variant in this enum
...
22 |     Mag2OnlyQ12,
   |     ^^^^^^^^^^^
   |
   = note: `BenchPath` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored 
during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (test "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath") generated 1 warning
warning: unused import: `install_atomic`
  --> crates\simthing-driver\tests\phase_m_frontier_v1_3_gpu_resource_flow.rs:19:59
   |
19 |     build_execution_plan, compiled_stencil_to_gpu_config, install_atomic, resolve_node_columns,
   |                                                           ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\phase_m_frontier_v1_3_gpu_resource_flow.rs:122:9
    |
122 |     let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    |         ----^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `ctx`
   --> crates\simthing-driver\tests\phase_m_frontier_v1_3_gpu_resource_flow.rs:400:15
    |
400 |     with_gpu(|ctx| {
    |               ^^^ help: if this is intentional, prefix it with an underscore: `_ctx`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V1_SKELETON_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:11:11
   |
11 | pub const FRONTIER_V1_SKELETON_ID: &str = "frontier_v1_0_scenario_skeleton_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: variants `ReplayAccepted` and `PendingGpu` are never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:177:5
    |
174 | pub enum FrontierV1FieldStatus {
    |          --------------------- variants in this enum
...
177 |     ReplayAccepted,
    |     ^^^^^^^^^^^^^^
178 |     PendingGpu,
    |     ^^^^^^^^^^
    |
    = note: `FrontierV1FieldStatus` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: `simthing-driver` (test "phase_m_frontier_v1_3_gpu_resource_flow") generated 37 warnings (32 duplicates) (run 
`cargo fix --test "phase_m_frontier_v1_3_gpu_resource_flow" -p simthing-driver` to apply 3 suggestions)
warning: unused import: `channel_set_has_kind`
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:19:5
   |
19 |     channel_set_has_kind, ChannelKind, ChannelSet, LocationId,
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `ChannelSet`
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:14:34
   |
14 |     AtlasBatchPlan, ChannelKind, ChannelSet, LocationId, LocationMaterialization, LocationRole,
   |                                  ^^^^^^^^^^

warning: function `store_oracle_with_additional_sources` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_store.rs:258:8
    |
258 | pub fn store_oracle_with_additional_sources(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID: &str = "ATLAS-BATCH-0-PACK";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `PACKING_STRATEGY` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:8:11
  |
8 | pub const PACKING_STRATEGY: &str =
  |           ^^^^^^^^^^^^^^^^

warning: function `tile_source_at_atlas` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:308:4
    |
308 | fn tile_source_at_atlas(plan: &AtlasBatchPlan, class_id: &str, ax: u32, ay: u32) -> Option<LocationId> {
    |    ^^^^^^^^^^^^^^^^^^^^

warning: function `g_zero_sample` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:324:8
    |
324 | pub fn g_zero_sample(
    |        ^^^^^^^^^^^^^

warning: function `atlas_linear_index` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:351:4
    |
351 | fn atlas_linear_index(atlas_width: u32, ax: u32, ay: u32) -> Option<usize> {
    |    ^^^^^^^^^^^^^^^^^^

warning: function `channel_set_matches` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:355:8
    |
355 | pub fn channel_set_matches(lhs: &ChannelSet, rhs: &ChannelSet) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^

warning: function `channel_set_has_owner_indexed` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:359:8
    |
359 | pub fn channel_set_has_owner_indexed(set: &ChannelSet) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID: &str = "ATLAS-BATCH-0-LOC";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `EXPECTED_TOTAL_CELL_SLOTS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:7:11
  |
7 | pub const EXPECTED_TOTAL_CELL_SLOTS: u32 = 3000;
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `occupants_at` and `locations_by_role` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:242:12
    |
 93 | impl LocationMaterialization {
    | ---------------------------- methods in this implementation
...
242 |     pub fn occupants_at(&self, location_id: LocationId, cell: GridCell) -> Vec<&OccupantPlacement> {
    |            ^^^^^^^^^^^^
...
249 |     pub fn locations_by_role(&self, role: LocationRole) -> Vec<&LocationGridDescriptor> {
    |            ^^^^^^^^^^^^^^^^^

warning: function `cell_index` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:257:8
    |
257 | pub fn cell_index(map_base: u32, width: u32, x: u32, y: u32) -> u32 {
    |        ^^^^^^^^^^

warning: function `channel_set_has_kind` is never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:317:8
    |
317 | pub fn channel_set_has_kind(set: &ChannelSet, expected: ChannelKind) -> bool {
    |        ^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_ID: &str = "ATLAS-BATCH-0-GEN";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:2:11
  |
2 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS: &str =
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `PIRATE_STARPORT_COUNT` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:13:11
   |
13 | pub const PIRATE_STARPORT_COUNT: usize = 1;
   |           ^^^^^^^^^^^^^^^^^^^^^

warning: methods `in_bounds`, `chebyshev_distance`, and `empty_cells_between` are never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:54:12
   |
49 | impl GridCell {
   | ------------- methods in this implementation
...
54 |     pub fn in_bounds(self, side: u32) -> bool {
   |            ^^^^^^^^^
...
58 |     pub fn chebyshev_distance(self, other: Self) -> u32 {
   |            ^^^^^^^^^^^^^^^^^^
...
62 |     pub fn empty_cells_between(self, other: Self) -> u32 {
   |            ^^^^^^^^^^^^^^^^^^^

warning: method `cell_count` is never used
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:81:18
   |
73 | impl GridDims {
   | ------------- method in this implementation
...
81 |     pub const fn cell_count(&self) -> u32 {
   |                  ^^^^^^^^^^

warning: methods `terran_systems`, `pirate_systems`, `starports`, `fleets_by_owner`, `minimum_terran_empty_spacing`, 
and `pirate_within_one_empty_cell_of_terran` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_gen.rs:182:12
    |
137 | impl DressRehearsalMap {
    | ---------------------- methods in this implementation
...
182 |     pub fn terran_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
    |            ^^^^^^^^^^^^^^
...
188 |     pub fn pirate_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
    |            ^^^^^^^^^^^^^^
...
194 |     pub fn starports(&self) -> impl Iterator<Item = &BuildingPlacement> {
    |            ^^^^^^^^^
...
200 |     pub fn fleets_by_owner(&self, owner: Owner) -> impl Iterator<Item = &FleetPlacement> {
    |            ^^^^^^^^^^^^^^^
...
204 |     pub fn minimum_terran_empty_spacing(&self) -> Option<u32> {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
218 |     pub fn pirate_within_one_empty_cell_of_terran(&self, pirate: &SystemDescriptor) -> bool {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_store") generated 23 warnings (run `cargo fix --test 
"dress_rehearsal_atlas_batch_0_store" -p simthing-driver` to apply 2 suggestions)
warning: unused variable: `prop_words`
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:778:9
    |
778 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
    |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:100:8
    |
100 | struct ProposalOutcome {
    |        ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `proposal_count`, `proposal_overflow`, and `elapsed` are never read
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:128:5
    |
126 | struct ConsumerOutcome {
    |        --------------- fields in this struct
127 |     summary: ProposalSummary,
128 |     proposal_count: u32,
    |     ^^^^^^^^^^^^^^
129 |     proposal_overflow: u32,
    |     ^^^^^^^^^^^^^^^^^
130 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: field `reductions` is never read
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:134:5
    |
133 | struct FullChainOutcome {
    |        ---------------- field in this struct
134 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^

warning: function `pack_reductions` is never used
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:647:4
    |
647 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
    |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:740:4
    |
740 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
    |    ^^^^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
   --> crates\simthing-driver\tests\phase_m_sead_act1_phase_e_proposal_consumer.rs:877:4
    |
877 | fn run_proposals_gpu(
    |    ^^^^^^^^^^^^^^^^^

warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\e11b_nested_fission_gap.rs:28:5
   |
28 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `cols` is never read
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:143:9
    |
140 | pub struct FlatStarSession {
    |            --------------- field in this struct
...
143 |     pub cols: NodeColumnRefs,
    |         ^^^^

warning: function `flat_star_cell_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:206:8
    |
206 | pub fn flat_star_cell_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_sead_act1_phase_e_proposal_consumer") generated 7 warnings (run `cargo fix 
--test "phase_m_sead_act1_phase_e_proposal_consumer" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "e11b_nested_fission_gap") generated 7 warnings (3 duplicates) (run `cargo fix --test 
"e11b_nested_fission_gap" -p simthing-driver` to apply 1 suggestion)
warning: function `flow_subfield` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:21:4
   |
21 | fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
   |    ^^^^^^^^^^^^^

warning: function `register_food_flow` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:40:8
   |
40 | pub fn register_food_flow(reg: &mut DimensionRegistry) {
   |        ^^^^^^^^^^^^^^^^^^

warning: function `flat_star_scenario` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:66:8
   |
66 | pub fn flat_star_scenario(hosted_count: usize, n_slots: u32) -> Scenario {
   |        ^^^^^^^^^^^^^^^^^^

warning: type alias `CellKey` is never used
  --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:25:6
   |
25 | type CellKey = (u32, u32);
   |      ^^^^^^^

warning: function `flow_subfield` is never used
  --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:27:4
   |
27 | fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
   |    ^^^^^^^^^^^^^

warning: function `register_food_flow_with_allocation` is never used
  --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:46:8
   |
46 | pub fn register_food_flow_with_allocation(reg: &mut DimensionRegistry) {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `register_research_flow` is never used
  --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:72:4
   |
72 | fn register_research_flow(reg: &mut DimensionRegistry) {
   |    ^^^^^^^^^^^^^^^^^^^^^^

warning: struct `EnrollmentSoakSetup` is never constructed
  --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:98:12
   |
98 | pub struct EnrollmentSoakSetup {
   |            ^^^^^^^^^^^^^^^^^^^

warning: struct `EnrolledSoakSession` is never constructed
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:107:12
    |
107 | pub struct EnrolledSoakSession {
    |            ^^^^^^^^^^^^^^^^^^^

warning: struct `DynamicEnrollmentSoakFixture` is never constructed
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:118:12
    |
118 | pub struct DynamicEnrollmentSoakFixture {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `fission_outcome` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:130:4
    |
130 | fn fission_outcome(pairs: Vec<(SimThingId, SimThingId)>) -> FissionOutcome {
    |    ^^^^^^^^^^^^^^^

warning: function `open_single_fission_setup` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:138:8
    |
138 | pub fn open_single_fission_setup(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `open_multi_fission_setup` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:204:4
    |
204 | fn open_multi_fission_setup(parent_count: usize, max_participants: u32) -> EnrollmentSoakSetup {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `open_two_arena_setup` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:270:4
    |
270 | fn open_two_arena_setup() -> EnrollmentSoakSetup {
    |    ^^^^^^^^^^^^^^^^^^^^

warning: function `open_enrolled_soak_session` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:376:8
    |
376 | pub fn open_enrolled_soak_session(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `run_enrollment_only_soak` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:473:8
    |
473 | pub fn run_enrollment_only_soak(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `run_dynamic_enrollment_soak` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:537:8
    |
537 | pub fn run_dynamic_enrollment_soak(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_single_fission_inherit` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:610:8
    |
610 | pub fn dynamic_enrollment_single_fission_inherit() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_multiple_fissions_same_arena` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:624:8
    |
624 | pub fn dynamic_enrollment_multiple_fissions_same_arena() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_two_arenas_inherit` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:638:8
    |
638 | pub fn dynamic_enrollment_two_arenas_inherit() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_reject_when_cap_full` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:652:8
    |
652 | pub fn dynamic_enrollment_reject_when_cap_full() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_contiguity_blocked_no_compaction` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:666:8
    |
666 | pub fn dynamic_enrollment_contiguity_blocked_no_compaction() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_flag_off_no_gpu_sync` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:680:8
    |
680 | pub fn dynamic_enrollment_flag_off_no_gpu_sync() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `dynamic_enrollment_repeated_resync` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:694:8
    |
694 | pub fn dynamic_enrollment_repeated_resync() -> DynamicEnrollmentSoakFixture {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `open_fixture_session` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:708:8
    |
708 | pub fn open_fixture_session(
    |        ^^^^^^^^^^^^^^^^^^^^

warning: function `clone_enrolled_for_replay` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:763:8
    |
763 | pub fn clone_enrolled_for_replay(fx: &EnrolledSoakSession) -> EnrolledSoakSession {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `run_replay_burn_in` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:789:8
    |
789 | pub fn run_replay_burn_in(fx: &mut EnrolledSoakSession, ticks: u32) -> DynamicEnrollmentSoakReport {
    |        ^^^^^^^^^^^^^^^^^^

warning: function `assert_sibling_contiguity_after_admission` is never used
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:796:8
    |
796 | pub fn assert_sibling_contiguity_after_admission(fx: &EnrolledSoakSession) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-sim` (test "c8b_intensity_eml_parity") generated 1 warning (run `cargo fix --test 
"c8b_intensity_eml_parity" -p simthing-sim` to apply 1 suggestion)
warning: `simthing-driver` (test "resource_flow_scenario_class_burn_in") generated 148 warnings (120 duplicates)
warning: unused import: `install_atomic`
  --> crates\simthing-driver\tests\phase_m_frontier_v1_4_sead_route_replay.rs:22:59
   |
22 |     build_execution_plan, compiled_stencil_to_gpu_config, install_atomic, resolve_node_columns,
   |                                                           ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\phase_m_frontier_v1_4_sead_route_replay.rs:124:9
    |
124 |     let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    |         ----^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: field `gpu_rf` is never read
   --> crates\simthing-driver\tests\phase_m_frontier_v1_4_sead_route_replay.rs:230:5
    |
227 | struct FrontierV1SeadRouteRun {
    |        ---------------------- field in this struct
...
230 |     gpu_rf: GpuResourceFlowAllocationSummary,
    |     ^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: variants `CpuOracleOnly` and `PendingGpu` are never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:176:5
    |
174 | pub enum FrontierV1FieldStatus {
    |          --------------------- variants in this enum
175 |     GpuVerified,
176 |     CpuOracleOnly,
    |     ^^^^^^^^^^^^^
177 |     ReplayAccepted,
178 |     PendingGpu,
    |     ^^^^^^^^^^
    |
    = note: `FrontierV1FieldStatus` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: `simthing-driver` (test "phase_m_frontier_v1_4_sead_route_replay") generated 31 warnings (27 duplicates) (run 
`cargo fix --test "phase_m_frontier_v1_4_sead_route_replay" -p simthing-driver` to apply 2 suggestions)
warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\e11_arena_allocation.rs:22:5
   |
22 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `elapsed` is never read
   --> crates\simthing-driver\tests\phase_m_sead_act0_numeric_proposals.rs:100:5
    |
 96 | struct ProposalOutcome {
    |        --------------- field in this struct
...
100 |     elapsed: std::time::Duration,
    |     ^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `dispatch_count` is never read
   --> crates\simthing-driver\tests\phase_m_sead_act0_numeric_proposals.rs:666:5
    |
660 | struct ChainOutcome {
    |        ------------ field in this struct
...
666 |     dispatch_count: u32,
    |     ^^^^^^^^^^^^^^

warning: `simthing-driver` (test "e11_arena_allocation") generated 1 warning (run `cargo fix --test 
"e11_arena_allocation" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "phase_m_sead_act0_numeric_proposals") generated 2 warnings
warning: `simthing-sim` (test "s2_legacy_intensity_sunset") generated 1 warning (run `cargo fix --test 
"s2_legacy_intensity_sunset" -p simthing-sim` to apply 1 suggestion)
warning: field `flags` is never read
  --> crates\simthing-driver\tests\phase_m_sead_obs0_mobile_overlay_score.rs:73:5
   |
68 | struct ObsOutput {
   |        --------- field in this struct
...
73 |     flags: u32,
   |     ^^^^^
   |
   = note: `ObsOutput` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored 
during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `dispatches` and `includes_readback` are never read
   --> crates\simthing-driver\tests\phase_m_sead_obs0_mobile_overlay_score.rs:353:5
    |
350 | struct WarmRunOutcome {
    |        -------------- fields in this struct
...
353 |     dispatches: u32,
    |     ^^^^^^^^^^
354 |     includes_readback: bool,
    |     ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_sead_obs0_mobile_overlay_score") generated 2 warnings
warning: `simthing-driver` (test "phase_m_frontier_v1_2_gpu_replay_acceptance") generated 36 warnings (36 duplicates)
warning: function `standard_flat_star_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:221:8
    |
221 | pub fn standard_flat_star_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (test "e11_burn_in_scenarios") generated 125 warnings (124 duplicates)
warning: unused import: `MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:24:5
   |
24 |     MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `MobilityGpuKernel0OracleOutput`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel4_34k_projection_fixture.rs:20:5
   |
20 |     MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated functions `registration_only` and `registration_and_dispatch` are never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel3_projection_fixture.rs:46:12
   |
45 | impl MobilityGpuKernel3Gate {
   | --------------------------- associated functions in this implementation
46 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^
...
54 |     pub fn registration_and_dispatch() -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `default_projection_probe` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel3_projection_fixture.rs:78:12
   |
77 | impl MobilityGpuKernel3FixtureInput {
   | ----------------------------------- associated function in this implementation
78 |     pub fn default_projection_probe(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated functions `registration_only` and `registration_and_dispatch` are never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:39:12
   |
38 | impl MobilityGpuKernel1Gate {
   | --------------------------- associated functions in this implementation
39 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^
...
47 |     pub fn registration_and_dispatch() -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `default_dispatch_probe` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:72:12
   |
71 | impl MobilityGpuKernel1FixtureInput {
   | ----------------------------------- associated function in this implementation
72 |     pub fn default_dispatch_probe(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `default_probe` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel0_fixture.rs:119:12
    |
118 | impl MobilityGpuKernel0FixtureInput {
    | ----------------------------------- associated function in this implementation
119 |     pub fn default_probe() -> Self {
    |            ^^^^^^^^^^^^^

warning: constant `MOBILITY_RUNTIME1B_DISPATCH_GATE` is never used
  --> crates\simthing-driver\tests\support\mobility_runtime1b_fixture.rs:25:11
   |
25 | pub const MOBILITY_RUNTIME1B_DISPATCH_GATE: &str = "mobility_runtime1b_dispatch_closed";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type alias `MobilityRuntime1bForbiddenPathRequests` is never used
   --> crates\simthing-driver\tests\support\mobility_runtime1b_fixture.rs:190:10
    |
190 | pub type MobilityRuntime1bForbiddenPathRequests = MobilityRuntime1aForbiddenPathRequests;
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `default_disabled` is never used
  --> crates\simthing-driver\tests\support\mobility_runtime1a_fixture.rs:42:12
   |
41 | impl MobilityRuntime1aDriverFixtureSession {
   | ------------------------------------------ associated function in this implementation
42 |     pub fn default_disabled() -> Self {
   |            ^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "mobility_gpu_kernel4_34k_projection_fixture") generated 10 warnings (run `cargo fix 
--test "mobility_gpu_kernel4_34k_projection_fixture" -p simthing-driver` to apply 2 suggestions)
warning: variants `CorrectlyRoundedIntegerOnly` and `CorrectlyRoundedHwBitmaskNormalized` are never constructed
  --> crates\simthing-driver\tests\phase_m_jit_sqrt_exact_candidate_battery.rs:49:5
   |
45 | enum ExactSqrtCandidate {
   |      ------------------ variants in this enum
...
49 |     CorrectlyRoundedIntegerOnly,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
50 |     CorrectlyRoundedHwBitmaskNormalized,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `ExactSqrtCandidate` has derived impls for the traits `Clone` and `Debug`, but these are intentionally 
ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `name`, `gpu`, and `cpu` are never read
   --> crates\simthing-driver\tests\phase_m_jit_sqrt_exact_candidate_battery.rs:882:5
    |
880 | struct EdgeRowResult {
    |        ------------- fields in this struct
881 |     candidate: ExactSqrtCandidate,
882 |     name: &'static str,
    |     ^^^^
883 |     x: f32,
884 |     gpu: f32,
    |     ^^^
885 |     cpu: f32,
    |     ^^^
    |
    = note: `EdgeRowResult` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored 
during dead code analysis

warning: `simthing-driver` (test "phase_m_jit_sqrt_exact_candidate_battery") generated 2 warnings
warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\e11b_nested_hierarchy_gpu.rs:25:5
   |
25 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (test "e11b_nested_hierarchy_gpu") generated 7 warnings (6 duplicates) (run `cargo fix 
--test "e11b_nested_hierarchy_gpu" -p simthing-driver` to apply 1 suggestion)
warning: unnecessary parentheses around index expression
  --> crates\simthing-driver\tests\resource_economy_burn_in.rs:37:10
   |
37 |     flat[((cohort_slot * n_dims + food_col) as usize)] = 200.0;
   |          ^                                          ^
   |
   = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
   |
37 -     flat[((cohort_slot * n_dims + food_col) as usize)] = 200.0;
37 +     flat[(cohort_slot * n_dims + food_col) as usize] = 200.0;
   |

warning: unnecessary parentheses around index expression
  --> crates\simthing-driver\tests\resource_economy_burn_in.rs:38:10
   |
38 |     flat[((0 * n_dims + store_col) as usize)] = 5.0;
   |          ^                                 ^
   |
help: remove these parentheses
   |
38 -     flat[((0 * n_dims + store_col) as usize)] = 5.0;
38 +     flat[(0 * n_dims + store_col) as usize] = 5.0;
   |

warning: unnecessary parentheses around index expression
   --> crates\simthing-driver\tests\resource_economy_burn_in.rs:129:10
    |
129 |     flat[((cohort_slot * n_dims + food_col) as usize)] = 7.75;
    |          ^                                          ^
    |
help: remove these parentheses
    |
129 -     flat[((cohort_slot * n_dims + food_col) as usize)] = 7.75;
129 +     flat[(cohort_slot * n_dims + food_col) as usize] = 7.75;
    |

warning: `simthing-driver` (test "resource_economy_burn_in") generated 3 warnings (run `cargo fix --test 
"resource_economy_burn_in" -p simthing-driver` to apply 3 suggestions)
warning: `simthing-driver` (test "e2b5_dynamic_enrollment_soak") generated 120 warnings (run `cargo fix --test 
"e2b5_dynamic_enrollment_soak" -p simthing-driver` to apply 1 suggestion)
warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\e2b5_dynamic_fission_enrollment.rs:450:9
    |
450 |     let mut fx = open_enrollment_fixture(1, 16, FissionPolicySpec::Inherit, 0);
    |         ----^^
    |         |
    |         help: remove this `mut`

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\e2b5_dynamic_fission_enrollment.rs:482:9
    |
482 |     let mut fx = open_enrollment_fixture(1, 1, FissionPolicySpec::Inherit, 0);
    |         ----^^
    |         |
    |         help: remove this `mut`

warning: `simthing-driver` (test "e2b5_dynamic_fission_enrollment") generated 146 warnings (144 duplicates) (run 
`cargo fix --test "e2b5_dynamic_fission_enrollment" -p simthing-driver` to apply 2 suggestions)
warning: unused import: `FieldDispatchSchedule`
 --> crates\simthing-driver\tests\phase_m_first_slice_runtime.rs:6:70
  |
6 |     FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions, FieldDispatchSchedule,
  |                                                                      ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `deserialize_region_field_ron`
  --> crates\simthing-driver\tests\phase_m_first_slice_runtime.rs:15:35
   |
15 |     compile_region_field_preview, deserialize_region_field_ron, estimate_region_field_budget,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_first_slice_runtime") generated 2 warnings (run `cargo fix --test 
"phase_m_first_slice_runtime" -p simthing-driver` to apply 2 suggestions)
warning: unused variable: `layer`
   --> crates\simthing-driver\tests\phase_m_sead_obs2_multilayer_overlay_score.rs:386:43
    |
386 |         let layers = std::array::from_fn(|layer| {
    |                                           ^^^^^ help: if this is intentional, prefix it with an underscore: 
`_layer`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `sum`
   --> crates\simthing-driver\tests\phase_m_sead_obs2_multilayer_overlay_score.rs:444:21
    |
444 |                 let sum = cpu_mag2_bits(inp.gx, inp.gy);
    |                     ^^^ help: if this is intentional, prefix it with an underscore: `_sum`

warning: field `flags` is never read
  --> crates\simthing-driver\tests\phase_m_sead_obs2_multilayer_overlay_score.rs:55:5
   |
52 | struct MultiLayerOutput {
   |        ---------------- field in this struct
...
55 |     flags: u32,
   |     ^^^^^
   |
   = note: `MultiLayerOutput` has derived impls for the traits `Clone` and `Debug`, but these are intentionally 
ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `dispatches` and `includes_readback` are never read
   --> crates\simthing-driver\tests\phase_m_sead_obs2_multilayer_overlay_score.rs:210:5
    |
207 | struct WarmRunOutcome {
    |        -------------- fields in this struct
...
210 |     dispatches: u32,
    |     ^^^^^^^^^^
211 |     includes_readback: bool,
    |     ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_sead_obs2_multilayer_overlay_score") generated 4 warnings (run `cargo fix 
--test "phase_m_sead_obs2_multilayer_overlay_score" -p simthing-driver` to apply 2 suggestions)
warning: unused import: `MobilityGpuKernel0OracleOutput`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:18:5
   |
18 |     MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:24:5
   |
24 |     MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `MobilityOwner0OwnerChange` and `MobilityRuntime1aFixtureGate`
  --> crates\simthing-spec\tests\mobility_runtime1_production_fixture.rs:10:5
   |
10 |     MobilityOwner0OwnerChange, MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
...
13 |     MobilityRuntime0HarnessConfig, MobilityRuntime1aFixtureGate,
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: associated functions `registration_only` and `registration_and_dispatch` are never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:39:12
   |
38 | impl MobilityGpuKernel1Gate {
   | --------------------------- associated functions in this implementation
39 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^
...
47 |     pub fn registration_and_dispatch() -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `V78AtlasVramBudget`
 --> crates\simthing-driver\tests\phase_m_c1_atlas_scale_model.rs:1:21
  |
1 | use simthing_spec::{V78AtlasVramBudget, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES};
  |                     ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unnecessary parentheses around index expression
  --> crates\simthing-driver\tests\resource_economy_replay.rs:29:14
   |
29 |         flat[((cohort_slot * n_dims + food_col) as usize)] = 12.0;
   |              ^                                          ^
   |
   = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
   |
29 -         flat[((cohort_slot * n_dims + food_col) as usize)] = 12.0;
29 +         flat[(cohort_slot * n_dims + food_col) as usize] = 12.0;
   |

warning: unnecessary parentheses around index expression
  --> crates\simthing-driver\tests\resource_economy_replay.rs:30:14
   |
30 |         flat[((0 * n_dims + store_col) as usize)] = 3.0;
   |              ^                                 ^
   |
help: remove these parentheses
   |
30 -         flat[((0 * n_dims + store_col) as usize)] = 3.0;
30 +         flat[(0 * n_dims + store_col) as usize] = 3.0;
   |

warning: multiple fields are never read
  --> crates\simthing-driver\tests\phase_m_c1_atlas_scale_model.rs:6:5
   |
 4 | struct C1AtlasScaleModel {
   |        ----------------- fields in this struct
 5 |     starmap_width: u32, starmap_height: u32, star_count: u32,
 6 |     star_grid_width: u32, star_grid_height: u32,
   |     ^^^^^^^^^^^^^^^       ^^^^^^^^^^^^^^^^
 7 |     avg_planet_systems_per_star: u32,
 8 |     planet_system_grid_width: u32, planet_system_grid_height: u32,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^       ^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     avg_satellites_per_planet_system: u32,
10 |     body_surface_width: u32, body_surface_height: u32,
   |     ^^^^^^^^^^^^^^^^^^       ^^^^^^^^^^^^^^^^^^^
11 |     n_dims: u32,
   |     ^^^^^^
   |
   = note: `C1AtlasScaleModel` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:109:9
    |
109 |         exact_sqrt_artifact: None,
    |         ^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `pre_sqrt_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:110:9
    |
110 |         pre_sqrt_contract: None,
    |         ^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `mag2_source_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:111:9
    |
111 |         mag2_source_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `score_authority_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:112:9
    |
112 |         score_authority_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:124:9
    |
124 |         exact_sqrt_artifact: None,
    |         ^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `pre_sqrt_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:125:9
    |
125 |         pre_sqrt_contract: None,
    |         ^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `mag2_source_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:126:9
    |
126 |         mag2_source_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `score_authority_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:127:9
    |
127 |         score_authority_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:142:9
    |
142 |         exact_sqrt_artifact: None,
    |         ^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `pre_sqrt_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:143:9
    |
143 |         pre_sqrt_contract: None,
    |         ^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `mag2_source_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:144:9
    |
144 |         mag2_source_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `score_authority_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:145:9
    |
145 |         score_authority_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:163:9
    |
163 |         exact_sqrt_artifact: None,
    |         ^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `pre_sqrt_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:164:9
    |
164 |         pre_sqrt_contract: None,
    |         ^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `mag2_source_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:165:9
    |
165 |         mag2_source_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `score_authority_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:166:9
    |
166 |         score_authority_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `exact_sqrt_artifact`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:184:9
    |
184 |         exact_sqrt_artifact: None,
    |         ^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `pre_sqrt_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:185:9
    |
185 |         pre_sqrt_contract: None,
    |         ^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `mag2_source_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:186:9
    |
186 |         mag2_source_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `KernelDescriptor` has no field named `score_authority_contract`
   --> crates\simthing-driver\tests\phase_m_jit_desc0_kernel_descriptor.rs:187:9
    |
187 |         score_authority_contract: None,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `KernelDescriptor` does not have this field
    |
    = note: all struct fields are already assigned

For more information about this error, try `rustc --explain E0560`.
error: could not compile `simthing-driver` (test "phase_m_jit_desc0_kernel_descriptor") due to 20 previous errors
warning: build failed, waiting for other jobs to finish...
warning: unused variable: `prop_words`
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1715:9
     |
1715 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
     |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V2_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:8:11
  |
8 | pub const FRONTIER_V2_FIXTURE_ID: &str = "frontier_v2_0_closed_loop_consumer_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V2_1_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:9:11
  |
9 | pub const FRONTIER_V2_1_FIXTURE_ID: &str = "frontier_v2_1_candidate_evolution_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_2_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:10:11
   |
10 | pub const FRONTIER_V2_2_FIXTURE_ID: &str = "frontier_v2_2_movement_feedback_application_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_4_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:12:11
   |
12 | pub const FRONTIER_V2_4_FIXTURE_ID: &str = "frontier_v2_4_combined_feedback_loop_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_4_COMBINED_FEEDBACK_TICKS` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:13:11
   |
13 | pub const FRONTIER_V2_4_COMBINED_FEEDBACK_TICKS: u32 = 4;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_CLOSED_LOOP_TICKS` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:14:11
   |
14 | pub const FRONTIER_V2_CLOSED_LOOP_TICKS: u32 = 2;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_2_MOVEMENT_FEEDBACK_TICKS` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:15:11
   |
15 | pub const FRONTIER_V2_2_MOVEMENT_FEEDBACK_TICKS: u32 = 3;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variants `ReplayAccepted`, `NotImplemented`, and `Pending` are never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:21:5
   |
19 | pub enum FrontierV2FieldStatus {
   |          --------------------- variants in this enum
20 |     GpuVerified,
21 |     ReplayAccepted,
   |     ^^^^^^^^^^^^^^
...
24 |     NotImplemented,
   |     ^^^^^^^^^^^^^^
25 |     Pending,
   |     ^^^^^^^
   |
   = note: `FrontierV2FieldStatus` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: variants `OwnColumnShadowWrite` and `RejectedCrossEntity` are never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:41:5
   |
40 | pub enum FrontierV2WriteClassification {
   |          ----------------------------- variants in this enum
41 |     OwnColumnShadowWrite,
   |     ^^^^^^^^^^^^^^^^^^^^
42 |     BoundaryRequestShadowWrite,
43 |     RejectedCrossEntity,
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `FrontierV2WriteClassification` has derived impls for the traits `Debug` and `Clone`, but these are 
intentionally ignored during dead code analysis

warning: enum `FrontierV2MovementWriteError` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:56:10
   |
56 | pub enum FrontierV2MovementWriteError {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2ClosedLoopSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:112:12
    |
112 | pub struct FrontierV2ClosedLoopSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:131:12
    |
130 | impl FrontierV2ClosedLoopSummary {
    | -------------------------------- method in this implementation
131 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `build_movement_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:186:8
    |
186 | pub fn build_movement_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_structural_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:202:8
    |
202 | pub fn build_structural_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2CandidateEvolutionSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:260:12
    |
260 | pub struct FrontierV2CandidateEvolutionSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:278:12
    |
277 | impl FrontierV2CandidateEvolutionSummary {
    | ---------------------------------------- method in this implementation
278 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `hash_candidate_pair_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:290:8
    |
290 | pub fn hash_candidate_pair_delta(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_movement_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:320:8
    |
320 | pub fn hash_movement_candidate(c: FrontierV2MovementCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_closed_loop_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:339:8
    |
339 | pub fn hash_closed_loop_delta(tick0: &FrontierV2TickRun, tick1: &FrontierV2TickRun) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `initial_own_column_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:379:8
    |
379 | pub fn initial_own_column_shadow(unit_id: u32) -> FrontierV2OwnColumnShadow {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_movement_write_target` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:388:8
    |
388 | pub fn validate_movement_write_target(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `clamp_grid_coord` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:401:8
    |
401 | pub fn clamp_grid_coord(value: i32, grid_size: u32) -> u32 {
    |        ^^^^^^^^^^^^^^^^

warning: function `apply_movement_to_own_column_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:407:8
    |
407 | pub fn apply_movement_to_own_column_shadow(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_own_column_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:434:8
    |
434 | pub fn hash_own_column_shadow(shadow: FrontierV2OwnColumnShadow) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2MovementFeedbackSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:444:12
    |
444 | pub struct FrontierV2MovementFeedbackSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:467:12
    |
466 | impl FrontierV2MovementFeedbackSummary {
    | -------------------------------------- method in this implementation
467 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `hash_movement_feedback_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:480:8
    |
480 | pub fn hash_movement_feedback_delta(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_combined_feedback_to_config` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:630:8
    |
630 | pub fn apply_combined_feedback_to_config(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2CombinedFeedbackSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:654:12
    |
654 | pub struct FrontierV2CombinedFeedbackSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:683:12
    |
682 | impl FrontierV2CombinedFeedbackSummary {
    | -------------------------------------- method in this implementation
683 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `hash_combined_feedback_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:695:8
    |
695 | pub fn hash_combined_feedback_delta(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

warning: static `GPU_MUTEX` is never used
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:14:12
   |
14 | pub static GPU_MUTEX: Mutex<()> = Mutex::new(());
   |            ^^^^^^^^^

warning: function `with_gpu` is never used
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:21:8
   |
21 | pub fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
   |        ^^^^^^^^

warning: constant `PIPE0_ORDERING_CLASS` is never used
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:41:11
   |
41 | pub const PIPE0_ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";
   |           ^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_SEMANTIC_TERMS` is never used
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:43:7
   |
43 | const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
   |       ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_EXACT_TERMS` is never used
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:49:7
   |
49 | const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];
   |       ^^^^^^^^^^^^^^^^^^^^^

warning: fields `event_rows`, `elapsed`, and `dispatch_count` are never read
  --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:93:5
   |
92 | pub struct Pipe0Outcome {
   |            ------------ fields in this struct
93 |     event_rows: Vec<EventRow>,
   |     ^^^^^^^^^^
...
97 |     elapsed: std::time::Duration,
   |     ^^^^^^^
98 |     dispatch_count: u32,
   |     ^^^^^^^^^^^^^^

warning: method `summary` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:761:12
    |
748 | impl Act2ChainOutcome {
    | --------------------- method in this implementation
...
761 |     pub fn summary(&self) -> ProposalSummary {
    |            ^^^^^^^

warning: method `proposal_code` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:767:12
    |
766 | impl ProposalRecord {
    | ------------------- method in this implementation
767 |     pub fn proposal_code(&self) -> u32 {
    |            ^^^^^^^^^^^^^

warning: method `accepted_count` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:773:12
    |
772 | impl ProposalSummary {
    | -------------------- method in this implementation
773 |     pub fn accepted_count(&self) -> u32 {
    |            ^^^^^^^^^^^^^^

warning: method `admitted` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:787:12
    |
778 | impl AdmissionRecord {
    | -------------------- method in this implementation
...
787 |     pub fn admitted(&self) -> bool {
    |            ^^^^^^^^

warning: constant `ACT2_ORDERING_CLASS` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:813:11
    |
813 | pub const ACT2_ORDERING_CLASS: &str = "OrderInvariantExact";
    |           ^^^^^^^^^^^^^^^^^^^

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:871:12
    |
871 | pub struct ProposalOutcome {
    |            ^^^^^^^^^^^^^^^

warning: struct `ConsumerOutcome` is never constructed
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:897:12
    |
897 | pub struct ConsumerOutcome {
    |            ^^^^^^^^^^^^^^^

warning: fields `reductions` and `elapsed` are never read
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:905:5
    |
904 | pub struct Act2ChainOutcome {
    |            ---------------- fields in this struct
905 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^
...
910 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: struct `AdmitOutcome` is never constructed
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:934:12
    |
934 | pub struct AdmitOutcome {
    |            ^^^^^^^^^^^^

warning: function `default_admission_rules` is never used
   --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:999:8
    |
999 | pub fn default_admission_rules() -> AdmissionRulesGpu {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `act2_event_rec` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1179:8
     |
1179 | pub fn act2_event_rec(index: u32, code: u32, state: u32, score: i32) -> Act2EventRecord {
     |        ^^^^^^^^^^^^^^

warning: function `pack_proposals` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1472:4
     |
1472 | fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^

warning: function `pack_summary` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1512:4
     |
1512 | fn pack_summary(summary: ProposalSummary) -> [u32; 7] {
     |    ^^^^^^^^^^^^

warning: function `pack_reductions` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1584:4
     |
1584 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1677:4
     |
1677 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
     |    ^^^^^^^^^^^^^^^^

warning: function `run_consume_gpu` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1692:4
     |
1692 | fn run_consume_gpu(
     |    ^^^^^^^^^^^^^^^

warning: function `run_admit_gpu` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1814:4
     |
1814 | fn run_admit_gpu(
     |    ^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
    --> crates\simthing-driver\tests\support\sead_v1_live_pipeline.rs:1903:4
     |
1903 | fn run_proposals_gpu(
     |    ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_c1_atlas_scale_model") generated 2 warnings (run `cargo fix --test 
"phase_m_c1_atlas_scale_model" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-spec` (test "mobility_runtime1_production_fixture") generated 1 warning (run `cargo fix --test 
"mobility_runtime1_production_fixture" -p simthing-spec` to apply 1 suggestion)
warning: `simthing-driver` (test "resource_flow_flat_star_continued_soak") generated 148 warnings (148 duplicates)
warning: `simthing-driver` (test "mobility_gpu_kernel2_34k_fixture") generated 8 warnings (5 duplicates) (run `cargo 
fix --test "mobility_gpu_kernel2_34k_fixture" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "e11r_arena_allocation") generated 140 warnings (140 duplicates)
warning: `simthing-driver` (test "resource_economy_replay") generated 2 warnings (run `cargo fix --test 
"resource_economy_replay" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-gpu` (lib test) generated 1 warning (run `cargo fix --lib -p simthing-gpu --tests` to apply 1 
suggestion)
warning: `simthing-driver` (test "resource_flow_scenario_class_default_on") generated 143 warnings (143 duplicates)
warning: `simthing-driver` (test "phase_m_frontier_v2_3_structural_feedback_application") generated 88 warnings (31 
duplicates) (run `cargo fix --test "phase_m_frontier_v2_3_structural_feedback_application" -p simthing-driver` to 
apply 1 suggestion)
