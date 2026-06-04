# NVIDIA FP temporary battery 02 — PACK-GPU f32 GpuVerified on NVIDIA

**Temporary file:** `docs/tests/nvidia_fp_temp_02_pack_gpu.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `02 - PACK-GPU f32 GpuVerified`
**Status:** PASS

## 1. Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_02_pack_gpu.md
```

## 2. Adapter evidence

```text
requested_adapter_substring: RTX (env set; PACK-GPU uses GpuContext discrete-first selection)
require_adapter_match: true (STORE-GPU gate; PACK-GPU logs selected adapter in parity output)
adapter_inventory: see Battery 01 log
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true (name contains RTX)
selected_adapter_is_discrete_rtx: true
selected_adapter_is_intel: false
gpu_tier_ran: true
```

## 3. Results

```text
passed: 8
failed: 0
ignored: 0
filtered out: 0
```

Final Cargo result line: `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s`

Cargo build timing: `Finished test profile [optimized + debuginfo] target(s) in 1.23s`

## 4. Tolerance / parity standard

```text
standard: EC-A2b GpuVerified full-tile Linf <= 1e-4
threshold: 1e-4
max_error: 0.000031 (PlanetSurface / StarSystem); 0.000015 (Galactic)
EC-A2b-exact: deferred (not bit-exact / not to_bits)
```

## 5. Intel vs NVIDIA comparison

| Metric | Prior Intel iGPU | NVIDIA RTX run |
|---|---:|---:|
| Adapter | Intel(R) RaptorLake-S Mobile Graphics Controller | NVIDIA GeForce RTX 4080 Laptop GPU |
| Galactic20x20 L8 | 0.000004 | 0.000015 |
| StarSystem10x10 L8 | 0.000031 | 0.000031 |
| PlanetSurface10x10 L8 | 0.000031 | 0.000031 |
| EC-A2b_GpuVerified_closed | true | true |
| EC-A2b-exact | deferred | deferred |
| Cargo target/build time | 1.21s | 1.23s |
| Test runtime | 0.52s | 0.55s |
| Passed/failed/ignored | 8/0/0 | 8/0/0 |

## 6. Per-class L8 (NVIDIA)

```text
Galactic20x20: 0.000015258789 (passed_Linf_le_1e-4=true)
StarSystem10x10: 0.000031 (passed_Linf_le_1e-4=true)
PlanetSurface10x10: 0.000031 (passed_Linf_le_1e-4=true)
EC-A2b_GpuVerified_closed: true
```

## 7. Interpretation

- Replaces prior Intel-only evidence for PACK-GPU: **yes** (adds NVIDIA RTX f32 GpuVerified parity; prior `scenario_0080_2_atlas_batch_0_pack_gpu_*` Intel logs remain as Intel-only baseline)
- Galactic L8 slightly higher on RTX (1.5e-5 vs 4e-6) but still within 1e-4 tolerance
- Durable conclusion: PACK-GPU EC-A2b passes on discrete RTX 4080 with same GpuVerified standard; no shader/math/tolerance changes

## 8. Warnings summary

Pre-existing EML deprecations and pack/loc/gen dead-code warnings in test binary; unrelated.

## 9. Failures / blocked reason

```text
none
```

## 10. §0.5 check

Evidence-only temporary battery. No shader/math/tolerance changes.

---

## Raw cargo log (below)
cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-825fdbe9-d78f-4da1-8e27-dc251f84990c.ps1:82 char:147
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
warning: unused imports: `Owner` and `channel_set_has_kind`
  --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:19:5
   |
19 |     channel_set_has_kind, ChannelKind, ChannelSet, LocationId,
   |     ^^^^^^^^^^^^^^^^^^^^
20 |     LocationMaterialization, LocationRole, Owner,
   |                                            ^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: constant `DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID` is never used
 --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_pack.rs:1:11
  |
1 | pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID: &str = "ATLAS-BATCH-0-PACK";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

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

warning: methods `location`, `occupants_at`, and `locations_by_role` are never used
   --> crates\simthing-driver\tests\..\src\dress_rehearsal_atlas_batch_0_loc.rs:238:12
    |
 93 | impl LocationMaterialization {
    | ---------------------------- methods in this implementation
...
238 |     pub fn location(&self, id: LocationId) -> Option<&LocationGridDescriptor> {
    |            ^^^^^^^^
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

warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_pack_gpu") generated 20 warnings (run `cargo fix 
--test "dress_rehearsal_atlas_batch_0_pack_gpu" -p simthing-driver` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.23s
     Running tests\dress_rehearsal_atlas_batch_0_pack_gpu.rs 
(target\debug\deps\dress_rehearsal_atlas_batch_0_pack_gpu-f31480333ce36f12.exe)

running 8 tests
test channel_metadata_survives ... ok
test gpu_fixture_uses_accepted_pack_plan ... ok
test no_semantic_shader_inputs ... ok
test pack_gpu_status_matches_gate ... ok
PACK-GPU galactic: Linf=0.000015258789 tiles=1 adapter=NVIDIA GeForce RTX 4080 Laptop GPU
adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
device_name: simthing-gpu device
gpu_tier_ran: true
tile_classes_tested: Galactic20x20, StarSystem10x10, PlanetSurface10x10
class=Galactic20x20 tile_count=1 atlas=20x20 output_elements=1600 full_tile_Linf=0.000015 passed_Linf_le_1e-4=true
class=StarSystem10x10 tile_count=13 atlas=130x10 output_elements=5200 full_tile_Linf=0.000031 passed_Linf_le_1e-4=true
class=PlanetSurface10x10 tile_count=13 atlas=130x10 output_elements=5200 full_tile_Linf=0.000031 passed_Linf_le_1e-4=true
EC-A2b_GpuVerified_closed: true
EC-A2b-exact: deferred (not bit-exact / not to_bits)
PACK-GPU star-system batch: Linf=0.000030517578 atlas=130x10
PACK-GPU planet-surface batch: Linf=0.000030517578
test gpu_oracle_parity_galactic_20x20 ... ok
test g_zero_blocks_cross_tile_and_out_of_atlas ... ok
test gpu_oracle_parity_star_system_10x10_batch ... ok
test gpu_oracle_parity_planet_surface_10x10_batch ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s

