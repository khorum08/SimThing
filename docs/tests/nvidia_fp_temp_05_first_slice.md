# NVIDIA FP temporary battery 05 — Phase M first-slice family

**Temporary file:** `docs/tests/nvidia_fp_temp_05_first_slice.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `05 - Phase M first-slice family`
**Status:** PASS

## Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_map_residency -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_queue_write_hardening -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture
```

(Handoff listed `--test phase_m_first_slice_product_commitment`; **substitution** to `phase_m_first_slice_product_commitment_fixture` — exact binary in repo.)

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

| Suite | Tests | Failed | Ignored |
|---|---:|---:|---:|
| adapter gate | 1 | 0 | 0 |
| phase_m_first_slice_runtime | 28 | 0 | 0 |
| phase_m_first_slice_product_fixture | 7 | 0 | 0 |
| phase_m_first_slice_product_commitment_fixture | 7 | 0 | 0 |
| phase_m_first_slice_map_residency | 7 | 0 | 0 |
| phase_m_first_slice_queue_write_hardening | 4 | 0 | 0 |
| phase_m_first_slice_scenario_spec | 9 | 0 | 0 |
| phase_m_first_slice_summary_validity | 11 | 0 | 0 |
| **Total (incl. gate)** | **74** | **0** | **0** |

## Performance/timing capture

| Command block | Cargo build | Test runtime |
|---|---|---|
| adapter gate | 0.27s | 0.97s |
| first_slice_runtime | 0.12s | 3.91s |
| product_fixture | 1.88s | 1.71s |
| product_commitment_fixture | 1.44s | 1.55s |
| map_residency | 0.26s | 2.30s |
| queue_write_hardening | 1.73s | 1.73s |
| scenario_spec | 1.96s | 1.96s |
| summary_validity | 1.77s | 2.70s |

Wall-clock timings are diagnostic only.

## Tolerance/parity standard

Existing first-slice GPU/f32 thresholds (`GpuVerified` per `docs/invariants.md`). No tolerance changes.

## Intel baseline comparison

| Target | Prior Intel result | NVIDIA RTX result | Notes |
|---|---|---|---|
| phase_m_first_slice_runtime | not found in committed logs for this target | 28/0/0 | phase docs cite 28/28 (adapter unlogged) |
| product_fixture | not found in committed logs for this target | 7/0/0 | phase docs cite 7/7 |
| product_commitment_fixture | not found in committed logs for this target | 7/0/0 | phase docs cite 7/7 |
| map_residency | not found in committed logs for this target | 7/0/0 | phase docs cite 7/7 |
| queue_write_hardening | not found in committed logs for this target | 4/0/0 | phase docs cite 4/4 |
| scenario_spec | not found in committed logs for this target | 9/0/0 | phase docs cite 9/9 |
| summary_validity | not found in committed logs for this target | 11/0/0 | phase docs cite 11/11 |
| Cargo timings | not found in committed logs for this target | see table above | |

## Raw decisive excerpts

```text
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
test gpu_adapter_is_discrete_rtx_target ... ok
test result: ok. 28 passed; 0 failed; 0 ignored (runtime)
test result: ok. 11 passed; 0 failed; 0 ignored (summary_validity)
```

## Failures / blocked reason

none

## Interpretation

Direct same-file adapter gate confirms RTX-targeted environment. First-slice runtime/product/map-residency/queue/scenario/summary families pass on discrete RTX 4080 with existing thresholds.

## §0.5 check

Evidence-only NVIDIA validation; no shader/math/tolerance changes, no gameplay resource-flow behavior, no simthing-sim semantic expansion, no default session wiring.

---

## Raw cargo log

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:87 char:1
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

running 1 test
204 |     pub fn minimum_terran_empty_spacing(&self) -> Option<u32> {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
218 |     pub fn pirate_within_one_empty_cell_of_terran(&self, pirate: &SystemDescriptor) -> bool {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_store_gpu") generated 49 warnings (run `cargo fix 
--test "dress_rehearsal_atlas_batch_0_store_gpu" -p simthing-driver` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.27s
     Running tests\dress_rehearsal_atlas_batch_0_store_gpu.rs 
(target\debug\deps\dress_rehearsal_atlas_batch_0_store_gpu-831a199d6239664e.exe)
adapter_inventory: [Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) UHD Graphics, Microsoft Basic Render Driver, Intel(R) UHD Graphics]
requested_adapter_substring: RTX
require_adapter_match: true
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
gpu_tier_ran: true
test gpu_adapter_is_discrete_rtx_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.97s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running tests\phase_m_first_slice_runtime.rs (target\debug\deps\phase_m_first_slice_runtime-58bb6ac71917f75b.exe)

running 28 tests
test test_10_region_field_budget_estimator ... ok
test test_0_guardrail_sanity ... ok
test test_1_ron_roundtrip_runtime_registration ... ok
test test_r1_j_posture_preserved ... ok
test test_r2_g_posture_preserved ... ok
test test_r3_c_budget_readiness_summary ... ok
test test_r3_d_no_accidental_feature_expansion ... ok
test test_r1_f_debug_readback_still_returns_values ... ok
test test_r2_b_gpu_bridge_matches_debug_semantics ... ok
test test_4_field_scheduler_integration ... ok
test test_r1_a_hot_path_matches_debug_with_diagnostic_readback ... ok
test test_5_layer2_sum_reduction ... ok
test test_r3_a_readiness_report_hot_path_shape ... ok
test test_9_deterministic_replay ... ok
test test_7_no_readback_hot_path ... ok
test test_r3_b_debug_readback_explicit ... ok
test test_r1_h_dispatch_count_honesty ... ok
test test_r2_a_hot_path_no_hidden_reduction_readback ... ok
test test_6_layer3_eval_eml ... ok
test test_r1_e_hot_path_report_no_fake_values ... ok
test test_3_edge_corner_algebraic_boundary_parity ... ok
test test_r1_c_no_readback_two_tick_persistence ... ok
test test_r1_g_invalid_seed_rejected ... ok
test test_r2_c_two_tick_gpu_bridge_persistence ... ok
test test_8_default_off_enforcement ... ok
test test_r1_d_seed_only_clear_gpu_resident ... ok
test test_2_single_tick_gpu_execution ... ok
test test_r1_b_no_readback_preserves_first_hop_propagation ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.91s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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
    Finished `test` profile [optimized + debuginfo] target(s) in 1.88s
     Running tests\phase_m_first_slice_product_fixture.rs 
(target\debug\deps\phase_m_first_slice_product_fixture-d8d813dd323dc736.exe)

running 7 tests
test product_fixture_rejects_atlas_request ... ok
test product_fixture_ron_admits_and_budget_passes ... ok
test product_fixture_posture_preserved ... ok
test product_fixture_sparse_profile_executes_gpu_resident_hot_path ... ok
test product_fixture_default_profile_does_not_execute ... ok
test product_fixture_edge_and_field_values_remain_finite ... ok
test product_fixture_same_field_high_weight_yields_higher_urgency ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.71s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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
    Finished `test` profile [optimized + debuginfo] target(s) in 1.44s
     Running tests\phase_m_first_slice_product_commitment_fixture.rs 
(target\debug\deps\phase_m_first_slice_product_commitment_fixture-6b19dfffd226628a.exe)

running 7 tests
test product_commitment_ron_admits ... ok
test product_commitment_posture_preserved ... ok
test product_commitment_invalid_specs_reject ... ok
test product_commitment_default_profile_emits_no_event ... ok
test product_commitment_high_urgency_event_is_deterministic ... ok
commitment_low low_threat=9965.211 low_urgency=2003.0422 threshold=5490.8657 event_count_low=0 dispatches=9 reduction_stencil_readbacks=0
test product_commitment_low_urgency_stays_below_threshold_without_event ... ok
commitment_high high_threat=9965.211 high_urgency=8978.689 threshold=5490.8657 event_count_high=1 dispatches=9 reduction_stencil_readbacks=0
test product_commitment_high_urgency_crosses_threshold_and_emits_event ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.55s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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
warning: unused import: `FirstSliceMappingSession`
 --> crates\simthing-driver\tests\phase_m_first_slice_map_residency.rs:6:5
  |
6 |     FirstSliceMappingSession, FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryStatus,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

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

warning: function `flat_star_cell_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:206:8
    |
206 | pub fn flat_star_cell_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^

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

warning: constant `FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:15:11
   |
15 | pub const FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID: &str = "frontier_v1_4_field_policy_route_replay_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:16:11
   |
16 | pub const FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID: &str = "frontier_v1_5_live_field_agent_route_v1";
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

warning: enum `FieldPolicyPipelineVersion` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:34:10
   |
34 | pub enum FieldPolicyPipelineVersion {
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

warning: struct `FrontierFieldPolicyFieldAgentSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:73:12
   |
73 | pub struct FrontierFieldPolicyFieldAgentSpec {
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

warning: enum `FrontierV1LiveFieldAgentFieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:189:10
    |
189 | pub enum FrontierV1LiveFieldAgentFieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:215:12
    |
215 | pub struct FrontierV1LiveFieldAgentSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:233:12
    |
232 | impl FrontierV1LiveFieldAgentSummary {
    | -------------------------------- method in this implementation
233 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveFieldAgentOracleOutput {
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

warning: struct `FrontierV1FieldPolicyReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:306:12
    |
306 | pub struct FrontierV1FieldPolicyReplaySummary {
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

warning: function `build_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:518:8
    |
518 | pub fn build_field_policy_replay_summary(cpu_output: &FrontierV1FixtureOutput) -> FrontierV1FieldPolicyReplaySummary {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_route_replay_detail` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:527:8
    |
527 | pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_field_agent_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:558:8
    |
558 | pub fn hash_live_field_agent_feedback_candidate(c: FrontierV1LiveFieldAgentFeedbackCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:573:8
    |
573 | pub fn hash_live_field_agent_summary(summary: FrontierV1LiveFieldAgentSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:597:8
    |
597 | pub fn build_feedback_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_live_field_agent_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:621:8
    |
621 | pub fn cpu_live_field_agent_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:663:8
    |
663 | pub fn hash_field_policy_replay_summary(summary: FrontierV1FieldPolicyReplaySummary) -> u64 {
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

warning: function `validate_field_policy_routing` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1078:4
     |
1078 | fn validate_field_policy_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
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

warning: function `live_field_agent_field_status_code` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1199:4
     |
1199 | fn live_field_agent_field_status_code(s: FrontierV1LiveFieldAgentFieldStatus) -> u32 {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `mapping_execution_profile` is never read
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:17:5
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            -------------------------------- field in this struct
...
17 |     mapping_execution_profile: MappingExecutionProfile,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `diagnostic_readback_reduction_eml` is never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:61:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- method in this implementation
...
61 |     pub fn diagnostic_readback_reduction_eml(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_first_slice_map_residency") generated 151 warnings (run `cargo fix --test 
"phase_m_first_slice_map_residency" -p simthing-driver` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.26s
     Running tests\phase_m_first_slice_map_residency.rs 
(target\debug\deps\phase_m_first_slice_map_residency-72afc7ce8e849976.exe)

running 7 tests
test map_residency_posture_preserved ... ok
test dirty_refresh_from_cached ... ok
test hot_executed_tick ... ok
test cold_skipped_before_execution ... ok
test map_residency_sequence_is_deterministic ... ok
test resident_cached_tick ... ok
test disabled_profile_unavailable ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.30s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

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

warning: function `flat_star_cell_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:206:8
    |
206 | pub fn flat_star_cell_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^

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

warning: constant `FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:15:11
   |
15 | pub const FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID: &str = "frontier_v1_4_field_policy_route_replay_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:16:11
   |
16 | pub const FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID: &str = "frontier_v1_5_live_field_agent_route_v1";
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

warning: enum `FieldPolicyPipelineVersion` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:34:10
   |
34 | pub enum FieldPolicyPipelineVersion {
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

warning: struct `FrontierFieldPolicyFieldAgentSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:73:12
   |
73 | pub struct FrontierFieldPolicyFieldAgentSpec {
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

warning: enum `FrontierV1LiveFieldAgentFieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:189:10
    |
189 | pub enum FrontierV1LiveFieldAgentFieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:215:12
    |
215 | pub struct FrontierV1LiveFieldAgentSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:233:12
    |
232 | impl FrontierV1LiveFieldAgentSummary {
    | -------------------------------- method in this implementation
233 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveFieldAgentOracleOutput {
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

warning: struct `FrontierV1FieldPolicyReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:306:12
    |
306 | pub struct FrontierV1FieldPolicyReplaySummary {
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

warning: function `build_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:518:8
    |
518 | pub fn build_field_policy_replay_summary(cpu_output: &FrontierV1FixtureOutput) -> FrontierV1FieldPolicyReplaySummary {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_route_replay_detail` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:527:8
    |
527 | pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_field_agent_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:558:8
    |
558 | pub fn hash_live_field_agent_feedback_candidate(c: FrontierV1LiveFieldAgentFeedbackCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:573:8
    |
573 | pub fn hash_live_field_agent_summary(summary: FrontierV1LiveFieldAgentSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:597:8
    |
597 | pub fn build_feedback_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_live_field_agent_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:621:8
    |
621 | pub fn cpu_live_field_agent_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:663:8
    |
663 | pub fn hash_field_policy_replay_summary(summary: FrontierV1FieldPolicyReplaySummary) -> u64 {
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

warning: function `validate_field_policy_routing` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1078:4
     |
1078 | fn validate_field_policy_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
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

warning: function `live_field_agent_field_status_code` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1199:4
     |
1199 | fn live_field_agent_field_status_code(s: FrontierV1LiveFieldAgentFieldStatus) -> u32 {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: fields `commitment` and `mapping_execution_profile` are never read
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:16:5
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            -------------------------------- fields in this struct
15 |     session: FirstSliceMappingSession,
16 |     commitment: Option<CompiledFirstSliceCommitmentThreshold>,
   |     ^^^^^^^^^^
17 |     mapping_execution_profile: MappingExecutionProfile,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `tick_with_scenario_commitment` and `diagnostic_readback_reduction_eml` are never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:47:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- methods in this implementation
...
47 |     pub fn tick_with_scenario_commitment(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
61 |     pub fn diagnostic_readback_reduction_eml(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_first_slice_queue_write_hardening") generated 150 warnings (run `cargo fix 
--test "phase_m_first_slice_queue_write_hardening" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.73s
     Running tests\phase_m_first_slice_queue_write_hardening.rs 
(target\debug\deps\phase_m_first_slice_queue_write_hardening-647cb9d14e6003a4.exe)

running 4 tests
test queue_write_hardening_posture_preserved ... ok
test bulk_fill_path_matches_prior_diagnostic_outputs ... ok
test first_slice_bridge_uses_bulk_child_resource_fill ... ok
test summary_validity_unaffected_by_bulk_fill ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.73s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

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

warning: function `flat_star_cell_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:206:8
    |
206 | pub fn flat_star_cell_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^

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

warning: constant `FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:15:11
   |
15 | pub const FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID: &str = "frontier_v1_4_field_policy_route_replay_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:16:11
   |
16 | pub const FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID: &str = "frontier_v1_5_live_field_agent_route_v1";
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

warning: enum `FieldPolicyPipelineVersion` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:34:10
   |
34 | pub enum FieldPolicyPipelineVersion {
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

warning: struct `FrontierFieldPolicyFieldAgentSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:73:12
   |
73 | pub struct FrontierFieldPolicyFieldAgentSpec {
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

warning: enum `FrontierV1LiveFieldAgentFieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:189:10
    |
189 | pub enum FrontierV1LiveFieldAgentFieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:215:12
    |
215 | pub struct FrontierV1LiveFieldAgentSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:233:12
    |
232 | impl FrontierV1LiveFieldAgentSummary {
    | -------------------------------- method in this implementation
233 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveFieldAgentOracleOutput {
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

warning: struct `FrontierV1FieldPolicyReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:306:12
    |
306 | pub struct FrontierV1FieldPolicyReplaySummary {
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

warning: function `build_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:518:8
    |
518 | pub fn build_field_policy_replay_summary(cpu_output: &FrontierV1FixtureOutput) -> FrontierV1FieldPolicyReplaySummary {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_route_replay_detail` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:527:8
    |
527 | pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_field_agent_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:558:8
    |
558 | pub fn hash_live_field_agent_feedback_candidate(c: FrontierV1LiveFieldAgentFeedbackCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:573:8
    |
573 | pub fn hash_live_field_agent_summary(summary: FrontierV1LiveFieldAgentSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:597:8
    |
597 | pub fn build_feedback_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_live_field_agent_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:621:8
    |
621 | pub fn cpu_live_field_agent_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:663:8
    |
663 | pub fn hash_field_policy_replay_summary(summary: FrontierV1FieldPolicyReplaySummary) -> u64 {
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

warning: function `validate_field_policy_routing` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1078:4
     |
1078 | fn validate_field_policy_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
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

warning: function `live_field_agent_field_status_code` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1199:4
     |
1199 | fn live_field_agent_field_status_code(s: FrontierV1LiveFieldAgentFieldStatus) -> u32 {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `mapping_execution_profile` is never read
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:17:5
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            -------------------------------- field in this struct
...
17 |     mapping_execution_profile: MappingExecutionProfile,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `tick_mapping` is never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:38:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- method in this implementation
...
38 |     pub fn tick_mapping(
   |            ^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_first_slice_scenario_spec") generated 150 warnings (run `cargo fix --test 
"phase_m_first_slice_scenario_spec" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.96s
     Running tests\phase_m_first_slice_scenario_spec.rs 
(target\debug\deps\phase_m_first_slice_scenario_spec-67eca9c8ff316f65.exe)

running 9 tests
test scenario_production_test_boundary ... ok
test invalid_scenario_specs_reject ... ok
test scenario_posture_preserved ... ok
test scenario_ron_admits ... ok
scenario_high high_threat=9965.211 high_urgency=8978.689 threshold=5490.8657 event_count=1 event_kind=0x53454144
test authored_scenario_high_profile_emits_expected_event ... ok
test disabled_scenario_admits_but_does_not_execute ... ok
scenario_low low_threat=9965.211 low_urgency=2003.0422 threshold=5490.8657 event_count=0
test authored_scenario_low_profile_emits_no_event ... ok
test authored_scenario_high_profile_event_is_deterministic ... ok
hot_path dispatches=9 reduction_stencil_readbacks=0 urgency=8978.689
test authored_scenario_executes_gpu_resident_hot_path ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.96s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-39bf9917-22e7-4426-8276-6e273e22f5f1.ps1:100 char:3
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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

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

warning: function `flat_star_cell_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:206:8
    |
206 | pub fn flat_star_cell_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^

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

warning: constant `FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:15:11
   |
15 | pub const FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID: &str = "frontier_v1_4_field_policy_route_replay_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:16:11
   |
16 | pub const FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID: &str = "frontier_v1_5_live_field_agent_route_v1";
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

warning: enum `FieldPolicyPipelineVersion` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:34:10
   |
34 | pub enum FieldPolicyPipelineVersion {
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

warning: struct `FrontierFieldPolicyFieldAgentSpec` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:73:12
   |
73 | pub struct FrontierFieldPolicyFieldAgentSpec {
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

warning: enum `FrontierV1LiveFieldAgentFieldStatus` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:189:10
    |
189 | pub enum FrontierV1LiveFieldAgentFieldStatus {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:215:12
    |
215 | pub struct FrontierV1LiveFieldAgentSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:233:12
    |
232 | impl FrontierV1LiveFieldAgentSummary {
    | -------------------------------- method in this implementation
233 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveFieldAgentOracleOutput {
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

warning: struct `FrontierV1FieldPolicyReplaySummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:306:12
    |
306 | pub struct FrontierV1FieldPolicyReplaySummary {
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

warning: function `build_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:518:8
    |
518 | pub fn build_field_policy_replay_summary(cpu_output: &FrontierV1FixtureOutput) -> FrontierV1FieldPolicyReplaySummary {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_route_replay_detail` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:527:8
    |
527 | pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_field_agent_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:558:8
    |
558 | pub fn hash_live_field_agent_feedback_candidate(c: FrontierV1LiveFieldAgentFeedbackCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:573:8
    |
573 | pub fn hash_live_field_agent_summary(summary: FrontierV1LiveFieldAgentSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `build_feedback_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:597:8
    |
597 | pub fn build_feedback_candidate(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `cpu_live_field_agent_oracle` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:621:8
    |
621 | pub fn cpu_live_field_agent_oracle(
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_field_policy_replay_summary` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:663:8
    |
663 | pub fn hash_field_policy_replay_summary(summary: FrontierV1FieldPolicyReplaySummary) -> u64 {
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

warning: function `validate_field_policy_routing` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1078:4
     |
1078 | fn validate_field_policy_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
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

warning: function `live_field_agent_field_status_code` is never used
    --> crates\simthing-driver\tests\support\frontier_v1.rs:1199:4
     |
1199 | fn live_field_agent_field_status_code(s: FrontierV1LiveFieldAgentFieldStatus) -> u32 {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `mapping_execution_profile` is never read
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:17:5
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            -------------------------------- field in this struct
...
17 |     mapping_execution_profile: MappingExecutionProfile,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `diagnostic_readback_reduction_eml` is never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:61:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- method in this implementation
...
61 |     pub fn diagnostic_readback_reduction_eml(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_first_slice_summary_validity") generated 150 warnings (run `cargo fix --test 
"phase_m_first_slice_summary_validity" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.77s
     Running tests\phase_m_first_slice_summary_validity.rs 
(target\debug\deps\phase_m_first_slice_summary_validity-6e22a0825c71cb36.exe)

running 11 tests
test summary_status_is_driver_owned ... ok
test summary_policy_ron_admits ... ok
test summary_validity_posture_preserved ... ok
test cached_summary_does_not_cpu_emit_event ... ok
test dirty_seed_invalidates_cached_and_refreshes ... ok
test disabled_summary_status_semantics ... ok
test fresh_summary_after_executed_tick ... ok
test cached_summary_on_skipped_clean_tick ... ok
test summary_validity_sequence_is_deterministic ... ok
test summary_policy_does_not_enable_execution ... ok
test zero_initial_skip_before_execution ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.70s

