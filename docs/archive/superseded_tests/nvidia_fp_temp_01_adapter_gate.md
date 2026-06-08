# NVIDIA FP temporary battery 01 — adapter gate + STORE-GPU smoke

**Temporary file:** `docs/tests/nvidia_fp_temp_01_adapter_gate.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `01 - adapter gate + STORE-GPU smoke`
**Status:** PASS

## 1. Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_01_adapter_gate.md

$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_01_adapter_gate.md
```

## 2. Adapter evidence

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

## 3. Results

```text
Battery A (gpu_adapter_is_discrete_rtx_target): 1 passed; 0 failed; 0 ignored
Battery B (full STORE-GPU): 10 passed; 0 failed; 0 ignored
STORE-GPU exact parity: 38/38 entries bit-exact (f32::to_bits)
```

Final Cargo result line (full suite): `test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.54s`

Cargo build timing (full suite): `Finished test profile [optimized + debuginfo] target(s) in 0.23s`

## 4. Tolerance / parity standard

```text
standard: ExactDeterministic bit-exact (STORE-GPU EC-A3-gpu)
threshold: n/a (integer masked sums)
max_error: 0 mismatches
bit_exact_entries: 38/38
```

## 5. Warnings summary

Pre-existing `simthing-core` EML deprecations and dead-code warnings from private `#[path]` GEN/LOC/PACK chain in test binary; unrelated to this battery.

## 6. Failures / blocked reason

```text
none
```

## 7. Interpretation

- Replaces prior Intel-only evidence: partial (STORE-GPU RTX evidence already on master; this battery re-confirms)
- Durable conclusion to fold into permanent docs: discrete RTX adapter gate + STORE-GPU exact parity confirmed on NVIDIA GeForce RTX 4080 Laptop GPU

## 8. §0.5 check

Evidence-only temporary battery. No gameplay resource-flow behavior, no recursive allocation change, no CPU planner behavior, no `simthing-sim` semantic expansion, no default session wiring.

---

## Raw cargo log (below)
cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-c9478f2d-8d6d-477d-906a-4abfb4bf7524.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atla ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
    Finished `test` profile [optimized + debuginfo] target(s) in 1.59s
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.87s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-99402bf8-f509-4884-a456-d72a781ebc7a.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atla ...
+                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.23s
     Running tests\dress_rehearsal_atlas_batch_0_store_gpu.rs 
(target\debug\deps\dress_rehearsal_atlas_batch_0_store_gpu-831a199d6239664e.exe)

running 10 tests
test no_r1_r2_r3_r4_behavior ... ok
test no_semantic_shader_or_gameplay_inputs ... ok
test store_gpu_consumes_accepted_store_oracle ... ok
test store_gpu_status_matches_gate ... ok
adapter_inventory: [Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) UHD Graphics, Microsoft Basic Render Driver, Intel(R) UHD Graphics]
requested_adapter_substring: RTX
require_adapter_match: true
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
gpu_tier_ran: true
test gpu_adapter_is_discrete_rtx_target ... ok
adapter_inventory: [Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) UHD Graphics, Microsoft Basic Render Driver, Intel(R) UHD Graphics]
requested_adapter_substring: RTX
require_adapter_match: true
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
gpu_tier_ran: true
requested_adapter_substring: RTX
require_adapter_match: true
adapter_inventory: [Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) RaptorLake-S Mobile Graphics Controller, NVIDIA GeForce RTX 4080 Laptop GPU, Intel(R) UHD Graphics, Microsoft Basic Render Driver, Intel(R) UHD Graphics]
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
adapter/device: NVIDIA GeForce RTX 4080 Laptop GPU
device_name: simthing-gpu device
gpu_tier_ran: true
cpu_oracle_entry_count: 38
gpu_output_entry_count: 38
co-location cases tested:
  ten_pirate_shared_cell: true
  constructed_planet_patrol_pirate: true
parity_standard: ExactDeterministic bit-exact (f32::to_bits)
exact_match: 38/38 entries bit-exact; mismatches=0
owner_channel_leakage_checks: blind_sum_guard=true
EC-A3-gpu_closed: true
skipped_gpu_tests: none (tier ran)
test gpu_parity_full_store_table ... ok
test gpu_preserves_constructed_planet_patrol_pirate_distinction ... ok
test gpu_channel_entries_do_not_blind_sum_by_position ... ok
test gpu_preserves_10_pirate_shared_cell_channels ... ok
test gpu_owner_indexed_entries_do_not_blind_sum_by_position ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.54s

