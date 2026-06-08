# NVIDIA FP temporary battery 08 — FIELD_POLICY / boundary / scheduler GPU-touching batch

**Temporary file:** `docs/tests/nvidia_fp_temp_08_field_policy_boundary_scheduler.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `08 - FIELD_POLICY / boundary / scheduler GPU-touching batch`
**Status:** PARTIAL

## Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture
# FIELD_POLICY/boundary/scheduler: --test integration binaries (handoff broad filters skipped)
cargo test -p simthing-driver --test phase_m_field_policy_obs0_mobile_overlay_score -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_obs2_multilayer_overlay_score -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_obs3_fixed_point_score -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_obs4_threshold_event -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_event0_compaction -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_event1_code_bucketing -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_event2_bucket_reductions -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_act0_numeric_proposals -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_act1_phase_e_proposal_consumer -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_act2_proposal_admission_records -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_act3_economic_fixture_records -- --nocapture
cargo test -p simthing-driver --test phase_m_field_policy_pipe0_observer_event_pipeline -- --nocapture
cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture
cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture  # BLOCKED: compile
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

| Suite | Tests passed | Failed | Ignored | Notes |
|---|---:|---:|---:|---|
| adapter gate | 1 | 0 | 0 | |
| phase_m_field_policy_obs0_mobile_overlay_score | 9 | 0 | 0 | |
| phase_m_field_policy_obs2_multilayer_overlay_score | 6 | 0 | 0 | |
| phase_m_field_policy_obs3_fixed_point_score | 6 | 0 | 0 | |
| phase_m_field_policy_obs4_threshold_event | 7 | 0 | 0 | |
| phase_m_field_policy_event0_compaction | 7 | 0 | 0 | |
| phase_m_field_policy_event1_code_bucketing | 7 | 0 | 0 | |
| phase_m_field_policy_event2_bucket_reductions | 8 | 0 | 0 | |
| phase_m_field_policy_act0_numeric_proposals | 8 | 0 | 0 | |
| phase_m_field_policy_act1_phase_e_proposal_consumer | 8 | 0 | 0 | |
| phase_m_field_policy_act2_proposal_admission_records | 8 | 0 | 0 | |
| phase_m_field_policy_act3_economic_fixture_records | 8 | 0 | 0 | |
| phase_m_field_policy_pipe0_observer_event_pipeline | 7 | 0 | 0 | |
| phase_m2_field_scheduler | 12 | 0 | 0 | |
| phase_m_boundary_cadence_doctrine | — | — | — | **BLOCKED** (compile) |
| **Totals (executed)** | **105** | **0** | **0** | 1 target not run |

## Performance/timing capture

| Command block | Cargo build | Test runtime |
|---|---|---|
| adapter gate | 0.24s | 0.93s |
| field_policy_obs0 | 0.12s | 2.05s |
| field_policy_obs2 | 1.59s | 1.37s |
| field_policy_obs3 | 1.57s | 1.68s |
| field_policy_obs4 | 1.71s | 1.81s |
| field_policy_event0 | 1.62s | 2.04s |
| field_policy_event1 | 0.13s | 2.14s |
| field_policy_event2 | 0.13s | 2.57s |
| field_policy_act0 | 1.83s | 2.58s |
| field_policy_act1 | 0.16s | 2.61s |
| field_policy_act2 | 1.69s | 2.78s |
| field_policy_act3 | 1.74s | 2.61s |
| field_policy_pipe0 | 1.75s | 2.14s |
| phase_m2_field_scheduler | 1.57s | 0.83s |
| boundary_cadence_doctrine | compile failed | n/a |

Diagnostic wall-clock only.

## Tolerance/parity standard

Existing FIELD_POLICY GPU-touching f32/exact discipline per `docs/invariants.md` and FIELD_POLICY track. No tolerance changes.

## Intel baseline comparison

| Target | Prior Intel result | NVIDIA RTX result | Notes |
|---|---|---|---|
| phase_m_field_policy_obs0 | not found in committed logs for this target | 9/0/0 | phase docs cite 7–9 pass (adapter unlogged) |
| phase_m_field_policy_obs2/3/4 | not found in committed logs for this target | 6/6/7 pass | |
| phase_m_field_policy_event0/1/2 | not found in committed logs for this target | 7/7/8 pass | |
| phase_m_field_policy_act0–3 | not found in committed logs for this target | 8 each | |
| phase_m_field_policy_pipe0 | not found in committed logs for this target | 7/0/0 | |
| phase_m2_field_scheduler | not found in committed logs for this target | 12/0/0 | |
| phase_m_boundary_cadence_doctrine | phase docs cite 7/7 pass | not run | compile: missing `workshop_current_state.md` |
| Cargo timings | not found in committed logs for this target | see table | |

## Battery 07 open triage items for Opus

1. `jit_grad0_mag2_not_overclaimed_if_approximate`:
   stale doc-hygiene guard likely; reads closed/archive accumulator_op_v2 production plan.
   Not native sqrt; shader path uses mag2 and forbids sqrt.

2. `jit_exec1_distinct_graphs_remain_separate_entries`:
   admission-ordering / harness bug likely; mixed cohort reached GPU helper before rejection.
   Not native sqrt; not NVIDIA FP tolerance drift.

## Raw decisive excerpts

```text
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
test result: ok. 9 passed; 0 failed; 0 ignored (field_policy_obs0)
test result: ok. 12 passed; 0 failed; 0 ignored (field_scheduler)
error: couldn't read docs/workshop/workshop_current_state.md (boundary_cadence_doctrine compile)
```

## Failures / blocked reason

- `phase_m_boundary_cadence_doctrine`: **BLOCKED** — test binary does not compile; `include_str!("../../../docs/workshop/workshop_current_state.md")` missing from repo. Not remediated per handoff (no source edits).

## Interpretation

RTX adapter gate passes. All 13 executed FIELD_POLICY/scheduler integration binaries pass on NVIDIA (105 tests, 0 failed). Boundary cadence doctrine could not be run on this checkout; battery is PARTIAL until that compile dependency is restored or triaged separately. Battery 07 failures remain open for Opus.

## §0.5 check

Evidence-only NVIDIA validation; no shader/math/tolerance/source changes, no gameplay resource-flow behavior, no simthing-sim semantic expansion, no default session wiring.

---

## Raw cargo log

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:87 char:1
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.24s
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.93s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
warning: field `flags` is never read
  --> crates\simthing-driver\tests\phase_m_field_policy_obs0_mobile_overlay_score.rs:73:5
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
   --> crates\simthing-driver\tests\phase_m_field_policy_obs0_mobile_overlay_score.rs:353:5
    |
350 | struct WarmRunOutcome {
    |        -------------- fields in this struct
...
353 |     dispatches: u32,
    |     ^^^^^^^^^^
354 |     includes_readback: bool,
    |     ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_obs0_mobile_overlay_score") generated 2 warnings
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running tests\phase_m_field_policy_obs0_mobile_overlay_score.rs 
(target\debug\deps\phase_m_field_policy_obs0_mobile_overlay_score-a8c8916f849f710f.exe)

running 9 tests
field_policy_obs0_wgsl: semantic_free=true F_hash=e2e9e27601ee2e13 mag_path=fixed_q16_plus_F
field_policy_obs0_score_authority: mag=ExactAuthoritative(Q16.16+F) score=ApproximateDiagnosticF32 no_planner_no_bridge
field_policy_obs0_wiring: default_off=true production_wiring=false descriptor=landed
field_policy_obs1_wiring: descriptor=landed no_scheduler_no_bridge
test field_policy_obs0_wgsl_semantic_free ... ok
test field_policy_obs0_score_authority_matches_arithmetic ... ok
test field_policy_obs0_no_default_runtime_wiring ... ok
test field_policy_obs1_no_default_runtime_wiring ... ok
field_policy_obs0_dense: tested=50176 mag2_exact=50176 mag_max_ulp=0 score_max_ulp=0 overflow=0
test field_policy_obs0_dense_overlay_corpus ... ok
field_policy_obs0_edge: tested=6 mag_max_ulp=0
test field_policy_obs0_edge_rows_correctness ... ok
field_policy_obs0_spot: rows=512 mag2_exact=512/512 mag_max_ulp=0 overflow=0
test field_policy_obs0_exact_magnitude_spot_check ... ok
field_policy_obs0_perf_34k: rows=34000 dispatches=1 includes_readback=true elapsed_ms=8.532 per_row_us=0.2509 spot_mag_max_ulp=0 path=q16_mag2_F_sqrt_f32_score
field_policy_obs0_perf_compare: SQRT-MAG2-PERF-0 combined Q16.16 ~1.7 ms; overlay adds f32 score multiply/add
test field_policy_obs0_perf_34k_mobile_overlay_score ... ok
field_policy_obs1_warm_34k: rows=34000 dispatches=32 includes_readback=true total_ms=3.492 per_dispatch_ms=0.109 per_row_us=0.0032 spot_mag_max_ulp=0 score_authority=ApproximateDiagnosticF32
field_policy_obs1_warm_compare: FIELD_POLICY-OBS-0 cold ~15.6 ms; SQRT-MAG2-PERF-0 combined ~1.7 ms
test field_policy_obs1_perf_34k_warm_repeated_dispatch ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.05s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: unused variable: `layer`
   --> crates\simthing-driver\tests\phase_m_field_policy_obs2_multilayer_overlay_score.rs:386:43
    |
386 |         let layers = std::array::from_fn(|layer| {
    |                                           ^^^^^ help: if this is intentional, prefix it with an underscore: 
`_layer`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `sum`
   --> crates\simthing-driver\tests\phase_m_field_policy_obs2_multilayer_overlay_score.rs:444:21
    |
444 |                 let sum = cpu_mag2_bits(inp.gx, inp.gy);
    |                     ^^^ help: if this is intentional, prefix it with an underscore: `_sum`

warning: field `flags` is never read
  --> crates\simthing-driver\tests\phase_m_field_policy_obs2_multilayer_overlay_score.rs:55:5
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
   --> crates\simthing-driver\tests\phase_m_field_policy_obs2_multilayer_overlay_score.rs:210:5
    |
207 | struct WarmRunOutcome {
    |        -------------- fields in this struct
...
210 |     dispatches: u32,
    |     ^^^^^^^^^^
211 |     includes_readback: bool,
    |     ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_obs2_multilayer_overlay_score") generated 4 warnings (run `cargo fix 
--test "phase_m_field_policy_obs2_multilayer_overlay_score" -p simthing-driver` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.59s
     Running tests\phase_m_field_policy_obs2_multilayer_overlay_score.rs 
(target\debug\deps\phase_m_field_policy_obs2_multilayer_overlay_score-44b5a22863921516.exe)

running 6 tests
field_policy_obs2_wgsl: semantic_free=true layers=4 F_hash=e2e9e27601ee2e13
field_policy_obs2_score_authority: ApproximateDiagnosticF32 per_layer_mag=Exact
test field_policy_obs2_wgsl_semantic_free ... field_policy_obs2_wiring: descriptor=landed no_scheduler_no_bridge
ok
test field_policy_obs2_score_authority_is_approximate ... ok
test field_policy_obs2_no_default_runtime_wiring ... ok
field_policy_obs2_dense: rows=6272 layer_mag_slots=25088 mag2_exact=25088 mag_max_ulp=0 score_max_ulp=0 overflow=0 score_authority=ApproximateDiagnosticF32
test field_policy_obs2_dense_multilayer_correctness ... ok
field_policy_obs2_perf_34k: rows=34000 layers=4 dispatches=1 includes_readback=true elapsed_ms=23.600 per_row_us=0.6941 layer_mags=136000 spot_mag_max_ulp=0 score_authority=ApproximateDiagnosticF32
test field_policy_obs2_perf_34k_multilayer_overlay_score ... ok
field_policy_obs2_warm_34k: rows=34000 layers=4 dispatches=32 includes_readback=true total_ms=3.325 per_dispatch_ms=0.104 per_row_us=0.0031 per_layer_mag_us=0.0008 spot_mag_max_ulp=0
field_policy_obs2_warm_compare: FIELD_POLICY-OBS-1 warm ~0.129 ms/dispatch single-layer
test field_policy_obs2_perf_34k_warm_repeated_dispatch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.37s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: unused variable: `layer`
   --> crates\simthing-driver\tests\phase_m_field_policy_obs3_fixed_point_score.rs:417:43
    |
417 |         let layers = std::array::from_fn(|layer| {
    |                                           ^^^^^ help: if this is intentional, prefix it with an underscore: 
`_layer`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `flags` is never read
  --> crates\simthing-driver\tests\phase_m_field_policy_obs3_fixed_point_score.rs:55:5
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

warning: `simthing-driver` (test "phase_m_field_policy_obs3_fixed_point_score") generated 2 warnings (run `cargo fix --test 
"phase_m_field_policy_obs3_fixed_point_score" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.57s
     Running tests\phase_m_field_policy_obs3_fixed_point_score.rs 
(target\debug\deps\phase_m_field_policy_obs3_fixed_point_score-25ab01438231aecf.exe)

running 6 tests
field_policy_obs3_score_authority: score_fixed=ExactQ16WeightedSum obs2_score_bits=ApproximateDiagnosticF32
field_policy_obs3_wiring: descriptor=landed no_scheduler_no_bridge semantic_free=true
test field_policy_obs3_score_authority_fixed_point ... ok
test field_policy_obs3_no_default_runtime_wiring ... ok
field_policy_obs3_dense: rows=6272 layer_mag_slots=25088 mag_exact=25088 mag_max_ulp=0 score_exact=6272 overflow=0 score_authority=ExactQ16WeightedSum
test field_policy_obs3_fixed_score_dense_corpus ... ok
field_policy_obs3_edge: cases=6 mag2_exact=24 mag_max_ulp=0 score_exact=6/6 overflow=0 worst=None
test field_policy_obs3_fixed_score_edge_rows ... ok
field_policy_obs3_perf_34k: rows=34000 layers=4 dispatches=1 includes_readback=true elapsed_ms=25.808 per_row_us=0.7590 spot_mag_max_ulp=0 spot_score_exact=512/512 overflow=0 score_authority=ExactQ16WeightedSum
test field_policy_obs3_perf_34k_fixed_score ... ok
field_policy_obs3_warm_34k: rows=34000 layers=4 dispatches=32 includes_readback=true total_ms=2.837 per_dispatch_ms=0.089 per_row_us=0.0026 per_layer_mag_us=0.0007 spot_mag_max_ulp=0 score_authority=ExactQ16WeightedSum
field_policy_obs3_warm_compare: FIELD_POLICY-OBS-2 warm ~0.238 ms/dispatch f32 score
test field_policy_obs3_perf_34k_warm_repeated_dispatch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.68s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.71s
     Running tests\phase_m_field_policy_obs4_threshold_event.rs 
(target\debug\deps\phase_m_field_policy_obs4_threshold_event-e21e86653b1440cb.exe)

running 7 tests
field_policy_obs4_wiring: descriptor=landed no_scheduler_no_bridge
field_policy_obs4_event_authority: state=Exact event_code=Exact gpu_resident=true no_cpu_planner=true
test field_policy_obs4_no_default_runtime_wiring ... ok
field_policy_obs4_wgsl: semantic_free=true F_hash=e2e9e27601ee2e13
test field_policy_obs4_event_authority_is_exact_deterministic ... ok
test field_policy_obs4_wgsl_semantic_free ... ok
field_policy_obs4_perf_34k: rows=34000 dispatches=1 includes_readback=true elapsed_ms=37.360 per_row_us=1.0988 spot_score_exact=512/512 spot_state_exact=512/512 spot_event_exact=512/512 overflow=0 total_events=17429 events_up_sample=152
test field_policy_obs4_perf_34k_threshold_event ... ok
field_policy_obs4_warm_34k: rows=34000 dispatches=32 includes_readback=true total_ms=3.180 per_dispatch_ms=0.099 per_row_us=0.0029 spot_exact=512/512 events_up_sample=152 event_authority=ExactDeterministicEventFlag
field_policy_obs4_warm_compare: FIELD_POLICY-OBS-3 warm ~0.278 ms/dispatch fixed score
test field_policy_obs4_perf_34k_warm_repeated_dispatch ... ok
field_policy_obs4_dense: rows=47040 score_exact=47040 state_exact=47040 event_exact=47040 overflow=0 events_up=12586
test field_policy_obs4_threshold_dense_corpus ... ok
field_policy_obs4_edge: cases=8 score_exact=8/8 state_exact=8/8 event_exact=8/8 overflow=0
test field_policy_obs4_threshold_edge_rows ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.81s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.62s
     Running tests\phase_m_field_policy_event0_compaction.rs 
(target\debug\deps\phase_m_field_policy_event0_compaction-5418fbd511b6a493.exe)

running 7 tests
field_policy_event0_wgsl: semantic_free=true ordering=UnspecifiedAtomicOrder
test field_policy_event0_wgsl_semantic_free ... okfield_policy_event0_wiring: descriptor=landed no_cpu_planner no_bridge

test field_policy_event0_no_default_runtime_wiring ... ok
field_policy_event0_edge[no_events]: inputs=2 nonzero=0 capacity=8 event_count=0 written=0 overflow=0 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[single_event]: inputs=1 nonzero=1 capacity=4 event_count=1 written=1 overflow=0 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[all_events]: inputs=6 nonzero=6 capacity=8 event_count=6 written=6 overflow=0 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[mixed_codes]: inputs=4 nonzero=3 capacity=8 event_count=3 written=3 overflow=0 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[capacity_exact_full]: inputs=4 nonzero=4 capacity=4 event_count=4 written=4 overflow=0 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[capacity_overflow]: inputs=6 nonzero=6 capacity=4 event_count=6 written=4 overflow=1 ordering=UnspecifiedAtomicOrder
field_policy_event0_edge[zero_capacity]: inputs=1 nonzero=1 capacity=0 event_count=1 written=0 overflow=1 ordering=UnspecifiedAtomicOrder
test field_policy_event0_compaction_edge_rows ... ok
field_policy_event0_dense: rows=4096 nonzero=2340 event_count=2340 membership=exact_unordered ordering=UnspecifiedAtomicOrder
test field_policy_event0_compaction_dense_corpus ... ok
field_policy_event0_obs4_smoke: input_rows=34000 nonzero=17142 compact_count=17142 overflow=0 membership=exact_unordered
test field_policy_event0_obs4_to_compaction_smoke ... ok
field_policy_event0_warm_34k: rows=34000 density_pct=50 dispatches=32 total_ms=4.019 per_dispatch_ms=0.126 per_row_us=0.0037 event_count=16805 overflow=0 ordering=UnspecifiedAtomicOrder
test field_policy_event0_perf_34k_warm_repeated_dispatch ... ok
field_policy_event0_perf_density_0: rows=34000 density_pct=0 elapsed_ms=5.280 per_row_us=0.1553 event_count=0 per_event_us=0.0000 capacity=128 overflow=0
field_policy_event0_perf_density_1: rows=34000 density_pct=1 elapsed_ms=1.214 per_row_us=0.0357 event_count=337 per_event_us=3.6033 capacity=465 overflow=0
field_policy_event0_perf_density_10: rows=34000 density_pct=10 elapsed_ms=1.086 per_row_us=0.0320 event_count=3317 per_event_us=0.3275 capacity=3445 overflow=0
field_policy_event0_perf_density_50: rows=34000 density_pct=50 elapsed_ms=1.193 per_row_us=0.0351 event_count=16805 per_event_us=0.0710 capacity=16933 overflow=0
field_policy_event0_perf_density_100: rows=34000 density_pct=100 elapsed_ms=1.561 per_row_us=0.0459 event_count=34000 per_event_us=0.0459 capacity=34128 overflow=0
test field_policy_event0_perf_34k_compaction ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.04s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests\phase_m_field_policy_event1_code_bucketing.rs 
(target\debug\deps\phase_m_field_policy_event1_code_bucketing-6bf4a05c6bd585bb.exe)

running 7 tests
field_policy_event1_wiring: default_off=true descriptor=m_jit_field_policy_event1_code_bucketing
field_policy_event1_wgsl: semantic_free=true ordering=UnspecifiedAtomicOrder
test field_policy_event1_no_default_runtime_wiring ... ok
test field_policy_event1_wgsl_semantic_free ... ok
field_policy_event1_edge[no_events]: records=0 capacity=8 counts=[0, 0, 0, 0] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[only_code_1]: records=1 capacity=8 counts=[0, 1, 0, 0] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[only_code_2]: records=1 capacity=8 counts=[0, 0, 1, 0] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[mixed_codes]: records=4 capacity=8 counts=[0, 2, 1, 1] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[code_0_ignored]: records=2 capacity=8 counts=[0, 1, 0, 0] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[invalid_code]: records=2 capacity=8 counts=[0, 1, 0, 0] overflow=[0, 0, 0, 0] invalid=1 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[capacity_exact_full]: records=4 capacity=4 counts=[0, 4, 0, 0] overflow=[0, 0, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[capacity_overflow]: records=6 capacity=4 counts=[0, 0, 6, 0] overflow=[0, 0, 1, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_event1_edge[zero_capacity]: records=1 capacity=0 counts=[0, 1, 0, 0] overflow=[0, 1, 0, 0] invalid=0 membership_ok=true ordering=UnspecifiedAtomicOrder
test field_policy_event1_bucket_edge_rows ... ok
field_policy_event1_34k[all_code_1]: elapsed_ms=8.730 per_record_us=0.2568 per_bucket_us=0.2568 counts=[0, 34000, 0, 0] overflow=[0, 1, 0, 0] invalid=0 capacity=20000
field_policy_event1_34k[balanced_12]: elapsed_ms=2.378 per_record_us=0.0700 per_bucket_us=0.0700 counts=[0, 17000, 17000, 0] overflow=[0, 0, 0, 0] invalid=0 capacity=20000
field_policy_event1_34k[balanced_123]: elapsed_ms=2.140 per_record_us=0.0629 per_bucket_us=0.0629 counts=[0, 11334, 11333, 11333] overflow=[0, 0, 0, 0] invalid=0 capacity=20000
field_policy_event1_34k[skewed_90_10]: elapsed_ms=2.840 per_record_us=0.0835 per_bucket_us=0.0835 counts=[0, 30529, 3471, 0] overflow=[0, 1, 0, 0] invalid=0 capacity=20000
field_policy_event1_34k[invalid_mix]: elapsed_ms=1.904 per_record_us=0.0560 per_bucket_us=0.0595 counts=[0, 10667, 10667, 10666] overflow=[0, 0, 0, 0] invalid=2000 capacity=20000
test field_policy_event1_perf_34k_bucketing ... ok
field_policy_event1_34k_warm: repeats=32 total_ms=8.599 per_dispatch_ms=0.2687 per_record_us=0.0079 counts=[0, 17000, 17000, 0] overflow=[0, 0, 0, 0] invalid=0 ordering=UnspecifiedAtomicOrder
test field_policy_event1_perf_34k_warm_repeated_dispatch ... ok
field_policy_event1_dense: records=8192 invalid=910 counts=[0, 2731, 1820, 1820] ordering=UnspecifiedAtomicOrder
test field_policy_event1_bucket_dense_corpus ... ok
field_policy_event1_pipe0_smoke: rows=512 dispatches=3 event_count=272 compact_overflow=0 counts=[0, 152, 120, 0] invalid=0 membership_ok=true
test field_policy_event1_pipe0_to_bucket_smoke ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.14s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests\phase_m_field_policy_event2_bucket_reductions.rs 
(target\debug\deps\phase_m_field_policy_event2_bucket_reductions-7b0354006edffba9.exe)

running 8 tests
field_policy_event2_wgsl: semantic_free=true ordering=UnspecifiedAtomicOrder
field_policy_event2_wiring: default_off=true descriptor=m_jit_field_policy_event2_bucket_reductions
test field_policy_event2_wgsl_semantic_free ... ok
test field_policy_event2_no_default_runtime_wiring ... ok
field_policy_event2_34k: elapsed_ms=31.864 per_record_us=0.9372 counts=[0, 17000, 17000, 0] per_code=[(0, 1), (17000, 0), (17000, 0), (0, 1)] ordering=UnspecifiedAtomicOrder
test field_policy_event2_perf_34k_bucket_reductions ... ok
field_policy_event2_event1_smoke: records=256 counts=[0, 86, 85, 85] ordering=UnspecifiedAtomicOrder
test field_policy_event2_event1_to_reductions_smoke ... ok
field_policy_event2_pipe0_smoke: compact=341 event_count=341 counts=[0, 170, 171, 0] ordering=UnspecifiedAtomicOrder
test field_policy_event2_pipe0_to_bucket_reductions_smoke ... ok
field_policy_event2_dense: counts=[0, 1366, 1365, 1365] sums_ok ordering=UnspecifiedAtomicOrder
test field_policy_event2_reduction_dense_corpus ... ok
field_policy_event2_34k_warm: repeats=32 total_ms=7.808 per_dispatch_ms=0.2440 per_record_us=0.0072 counts=[0, 17000, 17000, 0] ordering=UnspecifiedAtomicOrder
test field_policy_event2_perf_34k_warm_repeated_dispatch ... ok
field_policy_event2_edge[empty]: counts=[0, 0, 0, 0] ok=true flags=[1, 1, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[single]: counts=[0, 1, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[all_positive]: counts=[0, 2, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[all_negative]: counts=[0, 0, 2, 0] ok=true flags=[1, 1, 0, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[mixed_signs]: counts=[0, 2, 2, 0] ok=true flags=[1, 0, 0, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[min_max_ties]: counts=[0, 2, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[capacity_full]: counts=[0, 4, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[overflowed_input]: counts=[0, 6, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
field_policy_event2_edge[sum_overflow]: counts=[0, 2, 0, 0] ok=true flags=[1, 0, 1, 1] ordering=UnspecifiedAtomicOrder
test field_policy_event2_reduction_edge_rows ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.57s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: field `elapsed` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act0_numeric_proposals.rs:100:5
    |
 96 | struct ProposalOutcome {
    |        --------------- field in this struct
...
100 |     elapsed: std::time::Duration,
    |     ^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `dispatch_count` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act0_numeric_proposals.rs:666:5
    |
660 | struct ChainOutcome {
    |        ------------ field in this struct
...
666 |     dispatch_count: u32,
    |     ^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_act0_numeric_proposals") generated 2 warnings
    Finished `test` profile [optimized + debuginfo] target(s) in 1.83s
     Running tests\phase_m_field_policy_act0_numeric_proposals.rs 
(target\debug\deps\phase_m_field_policy_act0_numeric_proposals-619f9a70428d5ad5.exe)

running 8 tests
field_policy_act0_wgsl: semantic_free=true ordering=UnspecifiedAtomicOrder
field_policy_act0_wiring: default_off=true descriptor=m_jit_field_policy_act0_numeric_proposals
test field_policy_act0_wgsl_semantic_free ... ok
test field_policy_act0_no_default_runtime_wiring ... ok
field_policy_act0_dense: count=4 overflow=0 membership=exact ordering=UnspecifiedAtomicOrder
test field_policy_act0_dense_proposal_corpus ... ok
field_policy_act0_34k_warm: repeats=32 total_ms=15.552 per_pipeline_ms=0.4860 per_record_us=0.0143 proposal_count=3 overflow=0 ordering=UnspecifiedAtomicOrder
test field_policy_act0_perf_34k_warm_repeated_dispatch ... ok
field_policy_act0_edge[empty_reductions]: count=0 overflow=0 written=0 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[count_below]: count=0 overflow=0 written=0 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[count_equal]: count=1 overflow=0 written=1 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[count_above]: count=1 overflow=0 written=1 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[score_below]: count=0 overflow=0 written=0 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[score_equal]: count=1 overflow=0 written=1 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[score_above]: count=1 overflow=0 written=1 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[multiple_codes]: count=2 overflow=0 written=2 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[zero_capacity]: count=1 overflow=1 written=0 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[capacity_exact]: count=2 overflow=0 written=2 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[capacity_overflow]: count=2 overflow=1 written=1 ordering=UnspecifiedAtomicOrder
field_policy_act0_edge[input_sum_overflow]: count=1 overflow=0 written=1 ordering=UnspecifiedAtomicOrder
test field_policy_act0_proposal_edge_rows ... ok
field_policy_act0_event2_smoke: records=256 proposal_count=4 overflow=0 dispatches=3 ordering=UnspecifiedAtomicOrder
test field_policy_act0_event2_to_proposal_smoke ... ok
field_policy_act0_pipe_smoke: compact=341 event_count=341 bucket_counts=[0, 170, 171, 0] proposal_count=3 overflow=0 ordering=UnspecifiedAtomicOrder
test field_policy_act0_pipe_to_proposal_smoke ... ok
field_policy_act0_34k: dispatches=3 elapsed_ms=10.584 readback=true event_count=34000 proposal_count=3 overflow=0 per_record_us=0.3113 ordering=UnspecifiedAtomicOrder
test field_policy_act0_perf_34k_numeric_proposals ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.58s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
warning: unused variable: `prop_words`
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:778:9
    |
778 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
    |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:100:8
    |
100 | struct ProposalOutcome {
    |        ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `proposal_count`, `proposal_overflow`, and `elapsed` are never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:128:5
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
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:134:5
    |
133 | struct FullChainOutcome {
    |        ---------------- field in this struct
134 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^

warning: function `pack_reductions` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:647:4
    |
647 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
    |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:740:4
    |
740 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
    |    ^^^^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs:877:4
    |
877 | fn run_proposals_gpu(
    |    ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_act1_phase_e_proposal_consumer") generated 7 warnings (run `cargo fix 
--test "phase_m_field_policy_act1_phase_e_proposal_consumer" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests\phase_m_field_policy_act1_phase_e_proposal_consumer.rs 
(target\debug\deps\phase_m_field_policy_act1_phase_e_proposal_consumer-c580809873d5c6ea.exe)

running 8 tests
field_policy_act1_wiring: default_off=true descriptor=m_jit_field_policy_act1_phase_e_proposal_consumer
field_policy_act1_wgsl: semantic_free=true ordering=OrderInvariantExact
test field_policy_act1_no_default_runtime_wiring ... ok
test field_policy_act1_wgsl_semantic_free ... ok
field_policy_act1_edge[zero_proposals]: accepted=0 ignored=0 invalid=0 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[one_admitted]: accepted=1 ignored=0 invalid=0 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[one_invalid]: accepted=0 ignored=0 invalid=1 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[mixed_admitted_invalid]: accepted=1 ignored=0 invalid=1 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[proposal_count_gt_capacity]: accepted=1 ignored=2 invalid=0 flags=2 ordering=OrderInvariantExact
field_policy_act1_edge[proposal_overflow_set]: accepted=2 ignored=0 invalid=0 flags=2 ordering=OrderInvariantExact
field_policy_act1_edge[negative_score]: accepted=1 ignored=0 invalid=0 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[large_positive_score]: accepted=1 ignored=0 invalid=0 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[sum_overflow_boundary]: accepted=2 ignored=0 invalid=0 flags=0 ordering=OrderInvariantExact
field_policy_act1_edge[empty_capacity]: accepted=0 ignored=1 invalid=0 flags=0 ordering=OrderInvariantExact
test field_policy_act1_consumer_edge_rows ... ok
field_policy_act1_34k: dispatches=4 elapsed_ms=41.832 readback=true event_count=34000 proposal_count=3 accepted=3 overflow=0 per_record_us=1.2303 ordering=OrderInvariantExact
test field_policy_act1_perf_34k_phase_e_consumer ... ok
field_policy_act1_dense: accepted=102 invalid=26 max=4699 ordering=OrderInvariantExact
test field_policy_act1_dense_consumer_corpus ... ok
field_policy_act1_full_chain: compact=341 event_count=341 bucket_counts=[0, 170, 171, 0] proposal_count=3 accepted=3 overflow=0 ordering=OrderInvariantExact
test field_policy_act1_full_chain_phase_e_consumer_smoke ... ok
field_policy_act1_34k_warm: repeats=32 total_ms=14.740 per_pipeline_ms=0.4606 per_record_us=0.0135 accepted=3 overflow=0 ordering=OrderInvariantExact
test field_policy_act1_perf_34k_warm_repeated_dispatch ... ok
field_policy_act1_act0_smoke: proposal_count=4 accepted=4 invalid=0 overflow=0 dispatches=4 ordering=OrderInvariantExact
test field_policy_act1_act0_to_phase_e_consumer_smoke ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.61s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: unused variable: `prop_words`
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:956:9
    |
956 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
    |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
    --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:1755:9
     |
1755 |     let mut zero = ProposalSummary {
     |         ----^^^^
     |         |
     |         help: remove this `mut`
     |
     = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:107:8
    |
107 | struct ProposalOutcome {
    |        ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: struct `ConsumerOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:133:8
    |
133 | struct ConsumerOutcome {
    |        ^^^^^^^^^^^^^^^

warning: field `reductions` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:141:5
    |
140 | struct FullChainOutcome {
    |        ---------------- field in this struct
141 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^

warning: field `elapsed` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:172:5
    |
170 | struct AdmitOutcome {
    |        ------------ field in this struct
171 |     admission: AdmissionRecord,
172 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: function `pack_proposals` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:713:4
    |
713 | fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
    |    ^^^^^^^^^^^^^^

warning: function `pack_reductions` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:825:4
    |
825 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
    |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:918:4
    |
918 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
    |    ^^^^^^^^^^^^^^^^

warning: function `run_consume_gpu` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:933:4
    |
933 | fn run_consume_gpu(
    |    ^^^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act2_proposal_admission_records.rs:1144:4
     |
1144 | fn run_proposals_gpu(
     |    ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_act2_proposal_admission_records") generated 11 warnings (run `cargo fix 
--test "phase_m_field_policy_act2_proposal_admission_records" -p simthing-driver` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.69s
     Running tests\phase_m_field_policy_act2_proposal_admission_records.rs 
(target\debug\deps\phase_m_field_policy_act2_proposal_admission_records-b1ae6ff190289fed.exe)

running 8 tests
field_policy_act2_wgsl: semantic_free=true ordering=OrderInvariantExact
test field_policy_act2_wgsl_semantic_free ... ok
field_policy_act2_wiring: default_off=true descriptor=m_jit_field_policy_act2_proposal_admission_records
test field_policy_act2_no_default_runtime_wiring ... ok
field_policy_act2_act1_smoke: proposal_count=4 accepted=4 admission_code=5001 flags=1 dispatches=5 ordering=OrderInvariantExact
test field_policy_act2_act1_to_admission_smoke ... ok
field_policy_act2_dense: rows=64 ordering=OrderInvariantExact
test field_policy_act2_dense_admission_corpus ... ok
field_policy_act2_full_chain: compact=341 event_count=341 bucket_counts=[0, 170, 171, 0] proposal_count=3 accepted=3 admission_code=5001 flags=1 overflow=0 ordering=OrderInvariantExact
test field_policy_act2_full_chain_admission_smoke ... ok
field_policy_act2_34k: dispatches=5 elapsed_ms=12.758 readback=true event_count=34000 proposal_count=3 accepted=3 admission_code=5001 overflow=0 per_record_us=0.3752 ordering=OrderInvariantExact
test field_policy_act2_perf_34k_admission_records ... ok
field_policy_act2_edge[zero_accepted]: code=5001 flags=6 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[accepted_below]: code=5001 flags=2 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[accepted_equal]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
field_policy_act2_edge[accepted_above]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
field_policy_act2_edge[max_score_below]: code=5001 flags=4 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[max_score_equal]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
field_policy_act2_edge[max_score_above]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
field_policy_act2_edge[invalid_below_max]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
field_policy_act2_edge[invalid_above_max]: code=5001 flags=8 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[input_overflow]: code=5001 flags=16 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[summary_overflow]: code=5001 flags=32 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[negative_max_score]: code=5001 flags=4 admitted=false ordering=OrderInvariantExact
field_policy_act2_edge[large_positive_score]: code=5001 flags=1 admitted=true ordering=OrderInvariantExact
test field_policy_act2_admission_edge_rows ... ok
field_policy_act2_34k_warm: repeats=32 total_ms=16.308 per_pipeline_ms=0.5096 per_record_us=0.0150 admission_code=5001 flags=1 overflow=0 ordering=OrderInvariantExact
test field_policy_act2_perf_34k_warm_repeated_dispatch ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.78s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: unused variable: `prop_words`
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1198:9
     |
1198 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
     |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:114:8
    |
114 | struct ProposalOutcome {
    |        ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: struct `ConsumerOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:140:8
    |
140 | struct ConsumerOutcome {
    |        ^^^^^^^^^^^^^^^

warning: field `reductions` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:148:5
    |
147 | struct FullChainOutcome {
    |        ---------------- field in this struct
148 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^

warning: struct `AdmitOutcome` is never constructed
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:178:8
    |
178 | struct AdmitOutcome {
    |        ^^^^^^^^^^^^

warning: field `elapsed` is never read
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:215:5
    |
213 | struct FixtureOutcome {
    |        -------------- field in this struct
214 |     fixture: FixtureRecord,
215 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: function `default_admission_rules` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:283:4
    |
283 | fn default_admission_rules() -> AdmissionRulesGpu {
    |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `pack_proposals` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:870:4
    |
870 | fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
    |    ^^^^^^^^^^^^^^

warning: function `pack_summary` is never used
   --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:910:4
    |
910 | fn pack_summary(summary: ProposalSummary) -> [u32; 7] {
    |    ^^^^^^^^^^^^

warning: function `pack_reductions` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1067:4
     |
1067 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1160:4
     |
1160 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
     |    ^^^^^^^^^^^^^^^^

warning: function `run_consume_gpu` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1175:4
     |
1175 | fn run_consume_gpu(
     |    ^^^^^^^^^^^^^^^

warning: function `run_admit_gpu` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1297:4
     |
1297 | fn run_admit_gpu(
     |    ^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
    --> crates\simthing-driver\tests\phase_m_field_policy_act3_economic_fixture_records.rs:1487:4
     |
1487 | fn run_proposals_gpu(
     |    ^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_field_policy_act3_economic_fixture_records") generated 14 warnings (run `cargo fix 
--test "phase_m_field_policy_act3_economic_fixture_records" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.74s
     Running tests\phase_m_field_policy_act3_economic_fixture_records.rs 
(target\debug\deps\phase_m_field_policy_act3_economic_fixture_records-7df5714feaa7fb57.exe)

running 8 tests
field_policy_act3_wgsl: semantic_free=true ordering=OrderInvariantExact
test field_policy_act3_wgsl_semantic_free ... ok
field_policy_act3_wiring: default_off=true descriptor=m_jit_field_policy_act3_economic_fixture_records
test field_policy_act3_no_default_runtime_wiring ... ok
field_policy_act3_edge[admitted_known_code]: record_code=9001 flags=1 emitted=true ordering=OrderInvariantExact
field_policy_act3_edge[admitted_unknown_code]: record_code=0 flags=4 emitted=false ordering=OrderInvariantExact
field_policy_act3_edge[not_admitted]: record_code=0 flags=2 emitted=false ordering=OrderInvariantExact
field_policy_act3_edge[input_overflow]: record_code=0 flags=10 emitted=false ordering=OrderInvariantExact
field_policy_act3_edge[summary_overflow]: record_code=0 flags=18 emitted=false ordering=OrderInvariantExact
field_policy_act3_edge[zero_accepted]: record_code=9001 flags=1 emitted=true ordering=OrderInvariantExact
field_policy_act3_edge[large_accepted]: record_code=9001 flags=1 emitted=true ordering=OrderInvariantExact
field_policy_act3_edge[negative_max_score]: record_code=9001 flags=1 emitted=true ordering=OrderInvariantExact
field_policy_act3_edge[large_positive_score]: record_code=9001 flags=1 emitted=true ordering=OrderInvariantExact
field_policy_act3_edge[mapped_code_5002]: record_code=9002 flags=1 emitted=true ordering=OrderInvariantExact
test field_policy_act3_fixture_record_edge_rows ... ok
field_policy_act3_full_chain: compact=341 event_count=341 bucket_counts=[0, 170, 171, 0] proposal_count=3 accepted=3 admission_code=5001 record_code=9001 record_flags=1 overflow=0 ordering=OrderInvariantExact
test field_policy_act3_full_chain_fixture_record_smoke ... ok
field_policy_act3_34k_warm: repeats=32 total_ms=9.484 per_pipeline_ms=0.2964 per_record_us=0.0087 record_code=9001 overflow=0 ordering=OrderInvariantExact
test field_policy_act3_perf_34k_warm_repeated_dispatch ... ok
field_policy_act3_dense: rows=64 ordering=OrderInvariantExact
test field_policy_act3_dense_fixture_record_corpus ... ok
field_policy_act3_act2_smoke: admission_code=5001 admission_flags=1 record_code=9001 record_flags=1 priority=100 tier=1 dispatches=6 ordering=OrderInvariantExact
test field_policy_act3_act2_to_fixture_record_smoke ... ok
field_policy_act3_34k: dispatches=6 elapsed_ms=0.230 readback=true event_count=34000 proposal_count=3 accepted=3 admission_code=5001 record_code=9001 overflow=0 per_record_us=0.0068 ordering=OrderInvariantExact
test field_policy_act3_perf_34k_fixture_records ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.61s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.75s
     Running tests\phase_m_field_policy_pipe0_observer_event_pipeline.rs 
(target\debug\deps\phase_m_field_policy_pipe0_observer_event_pipeline-579ed8c798bce96b.exe)

running 7 tests
field_policy_pipe0_wiring: default_off=true production_wiring=false descriptor=m_jit_field_policy_pipe0_observer_event_pipeline
field_policy_pipe0_wgsl: semantic_free=true F_hash=e2e9e27601ee2e13 ordering=UnspecifiedAtomicOrder
test field_policy_pipe0_no_default_runtime_wiring ... ok
test field_policy_pipe0_wgsl_semantic_free ... ok
field_policy_pipe0_dense: rows=47040 event_count=25508 overflow=0 score_exact=47040 state_exact=47040 event_exact=47040 membership=25508/25508 ordering=UnspecifiedAtomicOrder
test field_policy_pipe0_dense_corpus ... ok
field_policy_pipe0_34k_integrated: rows=34000 dispatches=2 readback=true elapsed_ms=0.264 per_row_us=0.0078 event_count=17429 overflow=0 sample_score=512/512 sample_state=512/512 sample_event=512/512 membership=17429/17429 ordering=UnspecifiedAtomicOrder
test field_policy_pipe0_perf_34k_integrated_pipeline ... ok
field_policy_pipe0_34k_warm: repeats=32 total_ms=3.832 per_pipeline_ms=0.1197 per_row_us=0.0035 dispatches=64 event_count=17429 overflow=0 membership=17429/17429 ordering=UnspecifiedAtomicOrder
test field_policy_pipe0_perf_34k_warm_repeated_dispatch ... ok
field_policy_pipe0_edge[no_events]: rows=1 capacity=8 event_count=0 overflow=0 score_exact=1/1 state_exact=1/1 event_exact=1/1 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[single_event]: rows=1 capacity=8 event_count=1 overflow=0 score_exact=1/1 state_exact=1/1 event_exact=1/1 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[all_events]: rows=6 capacity=8 event_count=3 overflow=0 score_exact=6/6 state_exact=6/6 event_exact=6/6 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[up_down_hysteresis_hold]: rows=3 capacity=8 event_count=2 overflow=0 score_exact=3/3 state_exact=3/3 event_exact=3/3 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[capacity_exact_full]: rows=4 capacity=4 event_count=4 overflow=0 score_exact=4/4 state_exact=4/4 event_exact=4/4 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[capacity_overflow]: rows=6 capacity=4 event_count=6 overflow=1 score_exact=6/6 state_exact=6/6 event_exact=6/6 membership_ok=true ordering=UnspecifiedAtomicOrder
field_policy_pipe0_edge[zero_capacity]: rows=1 capacity=0 event_count=1 overflow=1 score_exact=1/1 state_exact=1/1 event_exact=1/1 membership_ok=true ordering=UnspecifiedAtomicOrder
test field_policy_pipe0_edge_rows ... ok
field_policy_pipe0_34k_cap[capacity_0]: capacity=0 event_count=17429 written=0 overflow=1 membership_ok=true
field_policy_pipe0_34k_cap[capacity_half]: capacity=8714 event_count=17429 written=8714 overflow=1 membership_ok=true
field_policy_pipe0_34k_cap[capacity_exact]: capacity=17429 event_count=17429 written=17429 overflow=0 membership_ok=true
field_policy_pipe0_34k_cap[capacity_34k]: capacity=34000 event_count=17429 written=17429 overflow=0 membership_ok=true
test field_policy_pipe0_perf_34k_capacity_variants ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.14s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.57s
     Running tests\phase_m2_field_scheduler.rs (target\debug\deps\phase_m2_field_scheduler-f158a0e4c8990be6.exe)

running 12 tests
test test_c_dirty_skip_correctness_zero_false_skips ... ok
test test_h_no_production_pass_graph_wiring ... ok
test test_m2_1_region_identity_includes_field_identity ... ok
test test_m2_1_same_field_region_replacement ... ok
test test_g_alternate_square_size_fixtures_size_agnostic ... ok
test test_m2_1_scheduled_visitor_does_not_execute_skipped ... ok
test test_b_invalid_cadence_rejected ... ok
test test_a_cadence_determinism_and_replay ... ok
test test_d_scheduler_report_metrics ... ok
test test_m2_1_single_op_guard_rejects_multiple_scheduled ... ok
test test_e_scheduled_execution_uses_no_readback_default ... ok
test test_f_readback_only_for_oracle_debug ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.83s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-2bfec782-fbe7-4cec-bd84-9d8f80b45f5b.ps1:108 char:3
+   cargo test -p simthing-driver --test $t -- --nocapture 2>&1 | Tee-O ...
+   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
error: couldn't read `crates\simthing-driver\tests\../../../docs/workshop/workshop_current_state.md`: The system 
cannot find the file specified. (os error 2)
   --> crates\simthing-driver\tests\phase_m_boundary_cadence_doctrine.rs:169:9
    |
169 |         include_str!("../../../docs/workshop/workshop_current_state.md"),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `simthing-driver` (test "phase_m_boundary_cadence_doctrine") due to 1 previous error
