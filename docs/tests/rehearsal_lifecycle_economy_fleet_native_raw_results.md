# Native fleet / RF upkeep raw execution

Code head `2e98c82e07189dea350ec4fd2d430d288f8d4788`, base `da1d7c1535f8953cde4ac70703a4a5f8313a0734`, tree `f09ae7babe65a3842b4304ae5a5360c839c08474`.

## conjunction-first.txt

```text
   Compiling simthing-gpu v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-gpu)
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-kernel` (lib) generated 15 warnings
warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: `simthing-tools` (lib) generated 5 warnings
   Compiling simthing-feeder v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-feeder)
   Compiling simthing-spec v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-spec)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

   Compiling simthing-sim v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-sim)
   Compiling simthing-clausething v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-clausething)
warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-driver)
warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

   Compiling simthing-workshop v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop)
warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-sim` (lib) generated 2 warnings
warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

   Compiling simthing-mapeditor v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-mapeditor)
warning: `simthing-clausething` (lib) generated 10 warnings
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 39s
     Running tests\rehearsal_lifecycle_economy_fleet.rs (C:/Users/mvorm/SimThing-0088-construction-local-2068/target\debug\deps\rehearsal_lifecycle_economy_fleet-babbad5ba47d144d.exe)

running 1 test
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpIy04PU\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:9a31fb39f0fc2262:21795 targets={"E1": [SimThingId(218)], "terran_mine": [SimThingId(211)], "terran_refinery": [SimThingId(212)], "pirate_generator_1": [SimThingId(214)], "stellaristhing_base": [SimThingId(231)], "pirate_mine": [SimThingId(216)], "pirate_refinery": [SimThingId(217)], "terran": [SimThingId(207)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "pirate_generator_2": [SimThingId(215)], "A1": [SimThingId(213)], "pirate": [SimThingId(208)]}
CELL case=minerals-then-energy generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [72, 187, 116, 246, 78, 229, 172, 35, 162, 78, 154, 200, 157, 3, 80, 58], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [72, 187, 116, 246, 78, 229, 172, 35, 162, 78, 154, 200, 157, 3, 80, 58], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpRZitvb\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:df74c97a1e9b5329:21795 targets={"pirate": [SimThingId(259)], "terran_generator_1": [SimThingId(260)], "terran_mine": [SimThingId(262)], "pirate_refinery": [SimThingId(268)], "terran_generator_2": [SimThingId(261)], "A1": [SimThingId(264)], "E1": [SimThingId(269)], "pirate_generator_2": [SimThingId(266)], "terran_refinery": [SimThingId(263)], "stellaristhing_base": [SimThingId(282)], "pirate_generator_1": [SimThingId(265)], "terran": [SimThingId(258)], "pirate_mine": [SimThingId(267)]}
CELL case=energy-then-minerals generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [163, 65, 219, 6, 199, 125, 96, 141, 103, 163, 191, 16, 75, 132, 33, 226], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [163, 65, 219, 6, 199, 125, 96, 141, 103, 163, 191, 16, 75, 132, 33, 226], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 3.54s


```

## capped-stock-second.txt

```text
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-kernel` (lib) generated 15 warnings
warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

warning: `simthing-clausething` (lib) generated 10 warnings
warning: `simthing-sim` (lib) generated 2 warnings
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

warning: `simthing-tools` (lib) generated 5 warnings
warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.93s
     Running tests\rehearsal_lifecycle_economy_fleet.rs (C:/Users/mvorm/SimThing-0088-construction-local-2068/target\debug\deps\rehearsal_lifecycle_economy_fleet-babbad5ba47d144d.exe)

running 1 test
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpc5uJHS\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:b236b448052e45b9:21574 targets={"pirate_generator_1": [SimThingId(214)], "terran_refinery": [SimThingId(212)], "pirate_mine": [SimThingId(216)], "pirate_refinery": [SimThingId(217)], "terran_generator_2": [SimThingId(210)], "pirate_generator_2": [SimThingId(215)], "E1": [SimThingId(218)], "terran_generator_1": [SimThingId(209)], "A1": [SimThingId(213)], "terran": [SimThingId(207)], "pirate": [SimThingId(208)], "stellaristhing_base": [SimThingId(231)], "terran_mine": [SimThingId(211)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpYrgLwR\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:bdd39841a4669fd7:21573 targets={"terran_generator_2": [SimThingId(261)], "E1": [SimThingId(269)], "pirate_generator_2": [SimThingId(266)], "pirate_mine": [SimThingId(267)], "stellaristhing_base": [SimThingId(282)], "terran": [SimThingId(258)], "terran_generator_1": [SimThingId(260)], "pirate_generator_1": [SimThingId(265)], "pirate": [SimThingId(259)], "terran_refinery": [SimThingId(263)], "A1": [SimThingId(264)], "pirate_refinery": [SimThingId(268)], "terran_mine": [SimThingId(262)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 2.74s


```

## native-birth-third-v4.txt

```text
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-kernel` (lib) generated 15 warnings
warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

warning: `simthing-clausething` (lib) generated 10 warnings
warning: `simthing-sim` (lib) generated 2 warnings
warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: `simthing-tools` (lib) generated 5 warnings
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
   Compiling simthing-workshop v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop)
    Finished `test` profile [optimized + debuginfo] target(s) in 10.72s
     Running tests\rehearsal_lifecycle_economy_fleet.rs (C:/Users/mvorm/SimThing-0088-construction-local-2068/target\debug\deps\rehearsal_lifecycle_economy_fleet-babbad5ba47d144d.exe)

running 1 test
test rehearsal_economy_fleet_native_funded_output_must_birth_fleets ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpcI2iT7\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:082f7f148364f203:26980 targets={"pirate_shipyard": [SimThingId(215)], "E1": [SimThingId(220)], "terran": [SimThingId(207)], "pirate": [SimThingId(208)], "terran_generator_2": [SimThingId(211)], "pirate_mine": [SimThingId(218)], "terran_generator_1": [SimThingId(210)], "pirate_refinery": [SimThingId(219)], "terran_shipyard": [SimThingId(209)], "pirate_generator_2": [SimThingId(217)], "terran_refinery": [SimThingId(213)], "A1": [SimThingId(214)], "stellaristhing_base": [SimThingId(235)], "terran_mine": [SimThingId(212)], "pirate_generator_1": [SimThingId(216)]}
STOCK_CELL case=native-funded-birth generation=0 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=0 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=0 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=native-funded-birth generation=0 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=1 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=1 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=1 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=native-funded-birth generation=1 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=209 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=1 owner=pirate shipyard_id=215 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=2 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=2 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=2 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=native-funded-birth generation=2 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=2 owner=pirate shipyard_id=215 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=3 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=3 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=native-funded-birth generation=3 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=native-funded-birth generation=3 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=3 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=4 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=4 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=native-funded-birth generation=4 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=native-funded-birth generation=4 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=4 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=5 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=5 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=5 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=native-funded-birth generation=5 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
FUNDING_FLOW generation=5 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
NATIVE_BIRTH generation=6 parent=214 id=262 faction=0
NATIVE_BIRTH generation=6 parent=220 id=268 faction=1
STOCK_CELL case=native-funded-birth generation=6 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=6 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=6 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=6 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=6 owner=pirate shipyard_id=215 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=7 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=7 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=7 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=7 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=7 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=8 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=8 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=8 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=8 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=8 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=9 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=9 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=9 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=9 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(2)
FUNDING_FLOW generation=9 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(2)
NATIVE_BIRTH generation=10 parent=214 id=265 faction=0
STOCK_CELL case=native-funded-birth generation=10 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=10 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=10 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=10 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=10 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(3)
NATIVE_BIRTH generation=11 parent=220 id=271 faction=1
STOCK_CELL case=native-funded-birth generation=11 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=11 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=11 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=11 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=11 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=12 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=12 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=12 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=12 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=12 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=13 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=13 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=13 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=13 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=13 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=14 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=14 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=14 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=14 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=14 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=15 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=15 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=15 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=15 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=15 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=16 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=16 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=16 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=16 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=16 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=17 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=17 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=17 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=17 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=17 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=18 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=18 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=18 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=18 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=18 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=19 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=19 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=19 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=19 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=19 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=20 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=20 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=20 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=20 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=20 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=21 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=21 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=21 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=21 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=21 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=22 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=22 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=22 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=22 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=22 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=23 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=23 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=23 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=23 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=23 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=24 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=24 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=24 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=24 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=24 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=25 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=25 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=25 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=25 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=25 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=26 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=26 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=26 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=26 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=26 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=27 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=27 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=27 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=27 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=27 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=28 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=28 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=28 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=28 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=28 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=29 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=29 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=29 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=29 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=29 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=30 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=30 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=30 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=30 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=30 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=31 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=31 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=31 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=31 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=31 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=32 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=32 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=32 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=32 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=32 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=33 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=33 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=33 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=33 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=209 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=33 owner=pirate shipyard_id=215 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=34 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=34 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=34 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=34 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=34 owner=pirate shipyard_id=215 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=35 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=native-funded-birth generation=35 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=35 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=35 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=35 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=36 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=native-funded-birth generation=36 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=36 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=36 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=36 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=37 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=native-funded-birth generation=37 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=37 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=37 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=209 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=37 owner=pirate shipyard_id=215 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=38 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=native-funded-birth generation=38 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=38 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=38 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=209 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=38 owner=pirate shipyard_id=215 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=39 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=native-funded-birth generation=39 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=39 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=39 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=209 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=39 owner=pirate shipyard_id=215 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=40 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=native-funded-birth generation=40 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=40 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=40 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=40 owner=pirate shipyard_id=215 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
BIRTH_SHAPE owner=terran id=262 children=[SimThingId(263), SimThingId(264)] hull=17179870000 extent=3 parent=214 generation=6
BIRTH_SHAPE owner=terran id=265 children=[SimThingId(266), SimThingId(267)] hull=1073741800 extent=3 parent=214 generation=10
BIRTH_SHAPE owner=pirate id=268 children=[SimThingId(269), SimThingId(270)] hull=17179870000 extent=3 parent=220 generation=6
BIRTH_SHAPE owner=pirate id=271 children=[SimThingId(272), SimThingId(273)] hull=536870900 extent=3 parent=220 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 234, 235, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259} fresh_ids=[262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273] capacity=76
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 1.98s


```

## born-upkeep.txt

```text
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-kernel` (lib) generated 15 warnings
warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

warning: `simthing-clausething` (lib) generated 10 warnings
warning: `simthing-sim` (lib) generated 2 warnings
warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: `simthing-tools` (lib) generated 5 warnings
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
   Compiling simthing-workshop v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop)
    Finished `test` profile [optimized + debuginfo] target(s) in 11.89s
     Running tests\rehearsal_lifecycle_economy_fleet.rs (C:/Users/mvorm/SimThing-0088-construction-local-2068/target\debug\deps\rehearsal_lifecycle_economy_fleet-babbad5ba47d144d.exe)

running 1 test
test rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmplld8Jq\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:8ed8685493bf35dd:27258 targets={"pirate_generator_2": [SimThingId(217)], "E1": [SimThingId(220)], "terran_mine": [SimThingId(212)], "A1": [SimThingId(214)], "pirate_shipyard": [SimThingId(215)], "terran_generator_1": [SimThingId(210)], "terran_refinery": [SimThingId(213)], "pirate_refinery": [SimThingId(219)], "pirate_mine": [SimThingId(218)], "pirate": [SimThingId(208)], "terran_shipyard": [SimThingId(209)], "pirate_generator_1": [SimThingId(216)], "terran_generator_2": [SimThingId(211)], "terran": [SimThingId(207)], "stellaristhing_base": [SimThingId(235)]}
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=214 id=262
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=220 id=268
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=262 parent=214 slot=Some(SlotIndex(38)) authored_observed_flow=0 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=268 parent=220 slot=Some(SlotIndex(41)) authored_observed_flow=0 energy_arena=1 members=[]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmph5MfdM\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:275de25fa57f794c:27260 targets={"pirate_refinery": [SimThingId(286)], "A1": [SimThingId(281)], "pirate_generator_1": [SimThingId(283)], "terran_refinery": [SimThingId(280)], "pirate_shipyard": [SimThingId(282)], "pirate": [SimThingId(275)], "terran": [SimThingId(274)], "E1": [SimThingId(287)], "terran_generator_1": [SimThingId(277)], "terran_shipyard": [SimThingId(276)], "pirate_mine": [SimThingId(285)], "terran_mine": [SimThingId(279)], "terran_generator_2": [SimThingId(278)], "stellaristhing_base": [SimThingId(302)], "pirate_generator_2": [SimThingId(284)]}
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=281 id=329
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=287 id=335
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[]

thread 'rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow' (34824) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:1052:5:
2.2 STOP: native born properties do not enter ongoing RF upkeep: [
    "zero-upkeep-control/262: born energy property is absent from parent RF arena",
    "zero-upkeep-control/268: born energy property is absent from parent RF arena",
    "one-energy-upkeep/terran: G8 net spendable energy 2, expected 1 after one born fleet",
    "one-energy-upkeep/pirate: G8 net spendable energy 2, expected 1 after one born fleet",
    "one-energy-upkeep/329: born energy property is absent from parent RF arena",
    "one-energy-upkeep/335: born energy property is absent from parent RF arena",
]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.87s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`

```

## code-head-suite.txt

```text
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-kernel` (lib) generated 15 warnings
warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-sim` (lib) generated 2 warnings
warning: `simthing-clausething` (lib) generated 10 warnings
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

warning: `simthing-tools` (lib) generated 5 warnings
warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
   Compiling simthing-workshop v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop)
    Finished `test` profile [optimized + debuginfo] target(s) in 12.80s
     Running tests\rehearsal_lifecycle_economy_fleet.rs (C:/Users/mvorm/SimThing-0088-construction-local-2068/target\debug\deps\rehearsal_lifecycle_economy_fleet-babbad5ba47d144d.exe)

running 5 tests
test rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow ... NATIVE_SOURCE_BEGIN case=zero-upkeep-control
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp4XpUoX\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:05a911bdd3ffe691:27258 targets={"pirate_generator_2": [SimThingId(217)], "E1": [SimThingId(220)], "pirate": [SimThingId(208)], "pirate_refinery": [SimThingId(219)], "terran_generator_2": [SimThingId(211)], "terran_mine": [SimThingId(212)], "A1": [SimThingId(214)], "terran": [SimThingId(207)], "pirate_mine": [SimThingId(218)], "pirate_shipyard": [SimThingId(215)], "terran_generator_1": [SimThingId(210)], "stellaristhing_base": [SimThingId(235)], "terran_shipyard": [SimThingId(209)], "pirate_generator_1": [SimThingId(216)], "terran_refinery": [SimThingId(213)]}
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=214 id=262
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=220 id=268
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=262 parent=214 slot=Some(SlotIndex(38)) authored_observed_flow=0 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=268 parent=220 slot=Some(SlotIndex(41)) authored_observed_flow=0 energy_arena=1 members=[]
NATIVE_SOURCE_BEGIN case=one-energy-upkeep
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpmn0ma6\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:672908515368d45c:27260 targets={"A1": [SimThingId(281)], "E1": [SimThingId(287)], "terran_mine": [SimThingId(279)], "terran_refinery": [SimThingId(280)], "terran_generator_2": [SimThingId(278)], "pirate_generator_2": [SimThingId(284)], "stellaristhing_base": [SimThingId(302)], "pirate_shipyard": [SimThingId(282)], "terran_generator_1": [SimThingId(277)], "terran_shipyard": [SimThingId(276)], "pirate": [SimThingId(275)], "pirate_generator_1": [SimThingId(283)], "pirate_mine": [SimThingId(285)], "pirate_refinery": [SimThingId(286)], "terran": [SimThingId(274)]}
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=281 id=329
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=287 id=335
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[]

thread 'rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow' (45364) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:1057:5:
2.2 STOP: native born properties do not enter ongoing RF upkeep: [
    "zero-upkeep-control/262: born energy property is absent from parent RF arena",
    "zero-upkeep-control/268: born energy property is absent from parent RF arena",
    "one-energy-upkeep/terran: G8 net spendable energy 2, expected 1 after one born fleet",
    "one-energy-upkeep/pirate: G8 net spendable energy 2, expected 1 after one born fleet",
    "one-energy-upkeep/329: born energy property is absent from parent RF arena",
    "one-energy-upkeep/335: born energy property is absent from parent RF arena",
]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test rehearsal_economy_fleet_existing_asset_executes_without_profile_replacement ... SOURCE path=C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop\../../scenarios/stellaristhing_base.clause identity=fnv1a64:ee4e4df9e8c9fbd9:5798 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=canonical: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=canonical identity=fnv1a64:3eac70da4b9dd41f:21555 targets={"pirate_refinery": [SimThingId(351)], "A1": [SimThingId(347)], "pirate_mine": [SimThingId(350)], "terran_generator_2": [SimThingId(344)], "terran_mine": [SimThingId(345)], "terran": [SimThingId(341)], "pirate_generator_2": [SimThingId(349)], "pirate": [SimThingId(342)], "pirate_generator_1": [SimThingId(348)], "stellaristhing_base": [SimThingId(365)], "terran_generator_1": [SimThingId(343)], "E1": [SimThingId(352)], "terran_refinery": [SimThingId(346)]}
N0 case=canonical root=364 existing_ids={341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 364, 365, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389} targets={"pirate_refinery": [SimThingId(351)], "A1": [SimThingId(347)], "pirate_mine": [SimThingId(350)], "terran_generator_2": [SimThingId(344)], "terran_mine": [SimThingId(345)], "terran": [SimThingId(341)], "pirate_generator_2": [SimThingId(349)], "pirate": [SimThingId(342)], "pirate_generator_1": [SimThingId(348)], "stellaristhing_base": [SimThingId(365)], "terran_generator_1": [SimThingId(343)], "E1": [SimThingId(352)], "terran_refinery": [SimThingId(346)]}
CELL case=canonical generation=0 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [107, 77, 252, 51, 86, 69, 197, 133, 138, 127, 37, 184, 18, 241, 208, 77], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [107, 77, 252, 51, 86, 69, 197, 133, 138, 127, 37, 184, 18, 241, 208, 77], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [107, 77, 252, 51, 86, 69, 197, 133, 138, 127, 37, 184, 18, 241, 208, 77], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [107, 77, 252, 51, 86, 69, 197, 133, 138, 127, 37, 184, 18, 241, 208, 77], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp6CAfbg\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=energy-withheld: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=energy-withheld identity=fnv1a64:7b9538142b5c4c18:21554 targets={"pirate_generator_1": [SimThingId(399)], "pirate": [SimThingId(393)], "stellaristhing_base": [SimThingId(416)], "terran_mine": [SimThingId(396)], "terran": [SimThingId(392)], "terran_generator_1": [SimThingId(394)], "terran_refinery": [SimThingId(397)], "E1": [SimThingId(403)], "A1": [SimThingId(398)], "pirate_refinery": [SimThingId(402)], "terran_generator_2": [SimThingId(395)], "pirate_mine": [SimThingId(401)], "pirate_generator_2": [SimThingId(400)]}
N0 case=energy-withheld root=415 existing_ids={392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 415, 416, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440} targets={"pirate_generator_1": [SimThingId(399)], "pirate": [SimThingId(393)], "stellaristhing_base": [SimThingId(416)], "terran_mine": [SimThingId(396)], "terran": [SimThingId(392)], "terran_generator_1": [SimThingId(394)], "terran_refinery": [SimThingId(397)], "E1": [SimThingId(403)], "A1": [SimThingId(398)], "pirate_refinery": [SimThingId(402)], "terran_generator_2": [SimThingId(395)], "pirate_mine": [SimThingId(401)], "pirate_generator_2": [SimThingId(400)]}
CELL case=energy-withheld generation=0 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [133, 201, 102, 213, 18, 29, 43, 148, 248, 0, 201, 94, 63, 101, 201, 141], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [133, 201, 102, 213, 18, 29, 43, 148, 248, 0, 201, 94, 63, 101, 201, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [133, 201, 102, 213, 18, 29, 43, 148, 248, 0, 201, 94, 63, 101, 201, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [133, 201, 102, 213, 18, 29, 43, 148, 248, 0, 201, 94, 63, 101, 201, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpA6PWLO\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:1c452a8577e0e5e6:21574 targets={"pirate": [SimThingId(444)], "stellaristhing_base": [SimThingId(467)], "A1": [SimThingId(449)], "pirate_refinery": [SimThingId(453)], "terran_refinery": [SimThingId(448)], "pirate_generator_1": [SimThingId(450)], "pirate_mine": [SimThingId(452)], "terran": [SimThingId(443)], "terran_generator_1": [SimThingId(445)], "terran_generator_2": [SimThingId(446)], "E1": [SimThingId(454)], "terran_mine": [SimThingId(447)], "pirate_generator_2": [SimThingId(451)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpRTblFL\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:78cbc9021536d4fc:21573 targets={"terran_mine": [SimThingId(498)], "pirate": [SimThingId(495)], "pirate_generator_1": [SimThingId(501)], "pirate_mine": [SimThingId(503)], "A1": [SimThingId(500)], "pirate_generator_2": [SimThingId(502)], "pirate_refinery": [SimThingId(504)], "stellaristhing_base": [SimThingId(518)], "terran": [SimThingId(494)], "terran_refinery": [SimThingId(499)], "terran_generator_2": [SimThingId(497)], "terran_generator_1": [SimThingId(496)], "E1": [SimThingId(505)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
ok
test rehearsal_economy_fleet_native_funded_output_must_birth_fleets ... NATIVE_SOURCE_BEGIN case=native-funded-birth
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpJ6QjP5\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:3e8307e4e7032278:26980 targets={"terran_shipyard": [SimThingId(547)], "pirate_generator_2": [SimThingId(555)], "pirate_refinery": [SimThingId(557)], "terran": [SimThingId(545)], "pirate_generator_1": [SimThingId(554)], "pirate_mine": [SimThingId(556)], "pirate_shipyard": [SimThingId(553)], "stellaristhing_base": [SimThingId(573)], "terran_generator_1": [SimThingId(548)], "A1": [SimThingId(552)], "E1": [SimThingId(558)], "terran_generator_2": [SimThingId(549)], "terran_mine": [SimThingId(550)], "pirate": [SimThingId(546)], "terran_refinery": [SimThingId(551)]}
STOCK_CELL case=native-funded-birth generation=0 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=0 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=0 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=native-funded-birth generation=0 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=1 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=1 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=1 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=native-funded-birth generation=1 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=547 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=1 owner=pirate shipyard_id=553 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=2 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=2 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=2 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=native-funded-birth generation=2 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=2 owner=pirate shipyard_id=553 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=3 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=3 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=native-funded-birth generation=3 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=native-funded-birth generation=3 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=3 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=4 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=4 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=native-funded-birth generation=4 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=native-funded-birth generation=4 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=4 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=5 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=5 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=5 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=native-funded-birth generation=5 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
FUNDING_FLOW generation=5 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
NATIVE_BIRTH generation=6 parent=552 id=600 faction=0
NATIVE_BIRTH generation=6 parent=558 id=606 faction=1
STOCK_CELL case=native-funded-birth generation=6 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=6 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=6 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=6 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=6 owner=pirate shipyard_id=553 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=7 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=7 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=7 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=7 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=7 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=8 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=8 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=8 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=8 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=8 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=9 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=9 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=9 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=9 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(2)
FUNDING_FLOW generation=9 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(2)
NATIVE_BIRTH generation=10 parent=552 id=603 faction=0
STOCK_CELL case=native-funded-birth generation=10 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=10 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=10 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=10 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=10 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(3)
NATIVE_BIRTH generation=11 parent=558 id=609 faction=1
STOCK_CELL case=native-funded-birth generation=11 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=11 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=11 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=11 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=11 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=12 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=12 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=12 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=12 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=12 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=13 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=13 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=13 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=13 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=13 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=14 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=14 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=14 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=14 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=14 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=15 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=15 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=15 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=15 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=15 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=16 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=16 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=16 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=16 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=16 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=17 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=17 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=17 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=17 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=17 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=18 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=18 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=18 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=18 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=18 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=19 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=19 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=19 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=19 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=19 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=20 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=20 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=20 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=20 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=20 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=21 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=21 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=21 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=21 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=21 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=22 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=22 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=22 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=22 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=22 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=23 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=23 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=23 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=23 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=23 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=24 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=24 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=24 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=24 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=24 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=25 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=25 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=25 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=25 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=25 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=26 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=26 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=26 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=26 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=26 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=27 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=27 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=27 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=27 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=27 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=28 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=28 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=28 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=28 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=28 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=29 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=29 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=29 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=29 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=29 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=30 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=30 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=30 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=30 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=30 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=31 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=31 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=31 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=31 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=31 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=32 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=32 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=32 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=32 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=32 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=33 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=33 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=33 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=33 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=547 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=33 owner=pirate shipyard_id=553 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=34 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=34 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=34 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=34 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=34 owner=pirate shipyard_id=553 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=35 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=native-funded-birth generation=35 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=35 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=35 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=35 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=36 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=native-funded-birth generation=36 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=36 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=36 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=36 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=37 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=native-funded-birth generation=37 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=37 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=37 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=547 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=37 owner=pirate shipyard_id=553 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=38 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=native-funded-birth generation=38 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=38 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=38 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=547 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=38 owner=pirate shipyard_id=553 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=39 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=native-funded-birth generation=39 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=39 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=39 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=547 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=39 owner=pirate shipyard_id=553 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=40 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=native-funded-birth generation=40 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=40 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=40 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=40 owner=pirate shipyard_id=553 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
BIRTH_SHAPE owner=terran id=600 children=[SimThingId(601), SimThingId(602)] hull=17179870000 extent=3 parent=552 generation=6
BIRTH_SHAPE owner=terran id=603 children=[SimThingId(604), SimThingId(605)] hull=1073741800 extent=3 parent=552 generation=10
BIRTH_SHAPE owner=pirate id=606 children=[SimThingId(607), SimThingId(608)] hull=17179870000 extent=3 parent=558 generation=6
BIRTH_SHAPE owner=pirate id=609 children=[SimThingId(610), SimThingId(611)] hull=536870900 extent=3 parent=558 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 572, 573, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597} fresh_ids=[600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611] capacity=76
ok
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpaSsTNf\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:8b655a79d8b7a188:21795 targets={"pirate": [SimThingId(613)], "E1": [SimThingId(623)], "terran_generator_1": [SimThingId(614)], "A1": [SimThingId(618)], "stellaristhing_base": [SimThingId(636)], "terran": [SimThingId(612)], "terran_generator_2": [SimThingId(615)], "terran_mine": [SimThingId(616)], "terran_refinery": [SimThingId(617)], "pirate_generator_2": [SimThingId(620)], "pirate_mine": [SimThingId(621)], "pirate_generator_1": [SimThingId(619)], "pirate_refinery": [SimThingId(622)]}
CELL case=minerals-then-energy generation=0 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [249, 185, 253, 118, 199, 254, 28, 109, 214, 82, 205, 107, 140, 90, 119, 38], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [249, 185, 253, 118, 199, 254, 28, 109, 214, 82, 205, 107, 140, 90, 119, 38], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpr0HaNE\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:2c8fd904ddb0825f:21795 targets={"A1": [SimThingId(669)], "pirate_mine": [SimThingId(672)], "terran": [SimThingId(663)], "terran_mine": [SimThingId(667)], "stellaristhing_base": [SimThingId(687)], "pirate_generator_1": [SimThingId(670)], "terran_generator_2": [SimThingId(666)], "terran_refinery": [SimThingId(668)], "terran_generator_1": [SimThingId(665)], "pirate_refinery": [SimThingId(673)], "E1": [SimThingId(674)], "pirate": [SimThingId(664)], "pirate_generator_2": [SimThingId(671)]}
CELL case=energy-then-minerals generation=0 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [101, 78, 111, 72, 52, 233, 169, 77, 231, 20, 8, 32, 176, 148, 88, 132], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [101, 78, 111, 72, 52, 233, 169, 77, 231, 20, 8, 32, 176, 148, 88, 132], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

failures:

failures:
    rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.81s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`

```

## code-head-check.txt

```text
warning: unused import: `EmlConsumerKind`
 --> crates\simthing-core\src\intensity_eml.rs:5:5
  |
5 |     EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
  --> crates\simthing-core\src\lib.rs:96:85
   |
96 |     EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
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
   --> crates\simthing-core\src\eml_registry.rs:743:41
    |
743 | pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    |                                         ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:145:21
    |
145 |     fn from(legacy: EmlTreeMeta) -> Self {
    |                     ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:224:15
    |
224 |         meta: EmlTreeMeta,
    |               ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:540:65
    |
540 |     pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
    |                                                                 ^^^^^^^^^^^

warning: use of deprecated struct `eml_registry::EmlTreeMeta`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:541:45
    |
541 |         self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
    |                                             ^^^^^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
  --> crates\simthing-core\src\fission_child_spawn.rs:54:51
   |
54 |         SimThingKindTag::Faction => SimThingKind::Faction,
   |                                                   ^^^^^^^

warning: use of deprecated unit variant `simthing::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-core\src\simthing.rs:252:23
    |
252 |         SimThingKind::Faction => authored == "Faction" || authored == "Owner",
    |                       ^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:132:47
    |
132 |         if EmlResourceClass::smallest_fitting(self.node_count, 1).is_none() {
    |                                               ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:133:45
    |
133 |             return Err(resource_class_error(self.node_count, 1));
    |                                             ^^^^^^^^^^^^^^^

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
   --> crates\simthing-core\src\eml_registry.rs:542:13
    |
542 |             node_count: f.meta.node_count,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:543:13
    |
543 |             has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:544:13
    |
544 |             formula_class: f.meta.display_name.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::has_transcendental`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:744:8
    |
744 |     if meta.has_transcendental {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::node_count`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:746:50
    |
746 |     } else if EmlResourceClass::smallest_fitting(meta.node_count, 1).is_none() {
    |                                                  ^^^^^^^^^^^^^^^

warning: use of deprecated field `eml_registry::EmlTreeMeta::formula_class`: use EmlFormulaMeta (C-8a)
   --> crates\simthing-core\src\eml_registry.rs:748:45
    |
748 |     } else if is_whitelisted_formula_class(&meta.formula_class) {
    |                                             ^^^^^^^^^^^^^^^^^^

warning: `simthing-core` (lib) generated 28 warnings (run `cargo fix --lib -p simthing-core` to apply 1 suggestion)
warning: methods `drop_dense_materialization` and `rebuild_dense_materialization` are never used
   --> crates\simthing-kernel\src\accumulator_op\runtime.rs:148:19
    |
145 | impl OverlayCompileCache {
    | ------------------------ methods in this implementation
...
148 |     pub(crate) fn drop_dense_materialization(&mut self) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
155 |     pub(crate) fn rebuild_dense_materialization(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `dispatch_world_summaries` is never used
    --> crates\simthing-kernel\src\accumulator_op\session.rs:2803:19
     |
 213 | impl AccumulatorOpSession {
     | ------------------------- method in this implementation
...
2803 |     pub(crate) fn dispatch_world_summaries(&self, ctx: &GpuContext, values: &Buffer) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `narrowed` and `narrowing` are never used
  --> crates\simthing-kernel\src\derived_span_projection.rs:45:19
   |
35 | impl ChangedLocus {
   | ----------------- methods in this implementation
...
45 |     pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
   |                   ^^^^^^^^
...
62 |     pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
   |                   ^^^^^^^^^

warning: variants `Stead`, `Palma`, and `GuYang` are never constructed
  --> crates\simthing-kernel\src\derived_span_projection.rs:86:5
   |
85 | pub(crate) enum FieldRegistrationAuthority {
   |                 -------------------------- variants in this enum
86 |     Stead,
   |     ^^^^^
87 |     Palma,
   |     ^^^^^
88 |     GuYang,
   |     ^^^^^^
   |
   = note: `FieldRegistrationAuthority` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: associated items `new`, `authority`, and `registration_id` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:98:12
    |
 97 | impl FieldRegistrationRef {
    | ------------------------- associated items in this implementation
 98 |     pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
    |            ^^^
...
105 |     pub fn authority(self) -> FieldRegistrationAuthority {
    |            ^^^^^^^^^
...
109 |     pub fn registration_id(self) -> u32 {
    |            ^^^^^^^^^^^^^^^

warning: associated items `new` and `raw` are never used
   --> crates\simthing-kernel\src\derived_span_projection.rs:118:12
    |
117 | impl DerivedWorkId {
    | ------------------ associated items in this implementation
118 |     pub fn new(raw: u32) -> Self {
    |            ^^^
...
122 |     pub fn raw(self) -> u32 {
    |            ^^^

warning: variants `FieldRegistration` and `Work` are never constructed
   --> crates\simthing-kernel\src\derived_span_projection.rs:133:5
    |
130 | pub(crate) enum DerivedDependencyTarget {
    |                 ----------------------- variants in this enum
...
133 |     FieldRegistration(FieldRegistrationRef),
    |     ^^^^^^^^^^^^^^^^^
134 |     Work(DerivedWorkId),
    |     ^^^^
    |
    = note: `DerivedDependencyTarget` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `field_law_proof` and `canonical_order_proof` are never read
   --> crates\simthing-kernel\src\field_sweep.rs:732:5
    |
724 | pub struct FieldSweepRegistration {
    |            ---------------------- fields in this struct
...
732 |     field_law_proof: FieldLawProof,
    |     ^^^^^^^^^^^^^^^
733 |     transient_read_proof: Option<FieldTransientCertificate>,
734 |     canonical_order_proof: CanonicalOrderProof,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `FieldSweepRegistration` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: function `push` is never used
    --> crates\simthing-kernel\src\field_sweep.rs:1519:4
     |
1519 | fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
     |    ^^^^

warning: method `write_gpu_records` is never used
   --> crates\simthing-kernel\src\gpu_readback.rs:123:19
    |
 78 | impl EmissionRecordReadback {
    | --------------------------- method in this implementation
...
123 |     pub(crate) fn write_gpu_records(&self, queue: &Queue, records: &[EmissionRecordGpu]) {
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `candidates_binding` and `count_binding` are never used
   --> crates\simthing-kernel\src\gpu_readback.rs:354:19
    |
316 | impl ThresholdEventCandidatesReadback {
    | ------------------------------------- methods in this implementation
...
354 |     pub(crate) fn candidates_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^^^^^^
...
359 |     pub(crate) fn count_binding(&self) -> &Buffer {
    |                   ^^^^^^^^^^^^^

warning: methods `dependency_index` and `profile_digest_by_logical_identity` are never used
   --> crates\simthing-kernel\src\overlay_prep.rs:227:8
    |
133 | impl OverlaySpanProjection {
    | -------------------------- methods in this implementation
...
227 |     fn dependency_index(&self) -> &DerivedDependencyIndex {
    |        ^^^^^^^^^^^^^^^^
...
231 |     fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `band_crossing_updates_from_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:128:15
    |
128 | pub(crate) fn band_crossing_updates_from_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_sealed_band_crossings_to_anchor_table` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:154:15
    |
154 | pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `oracle_anchor_table_after_deltas` is never used
   --> crates\simthing-kernel\src\sealed\anchor_table.rs:164:15
    |
164 | pub(crate) fn oracle_anchor_table_after_deltas(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: value assigned to `rejected_out_of_radius` is never read
   --> crates\simthing-mapgenerator\src\cluster.rs:149:13
    |
149 |             rejected_out_of_radius += 1;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `fixture_lattice_edge`
   --> crates\simthing-mapgenerator\src\cluster.rs:204:5
    |
204 |     fixture_lattice_edge: u32,
    |     ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fixture_lattice_edge`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `chebyshev_distance` is never used
  --> crates\simthing-mapgenerator\src\strategies\common.rs:21:8
   |
21 | pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
   |        ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-kernel` (lib) generated 15 warnings
warning: `simthing-mapgenerator` (lib) generated 3 warnings (run `cargo fix --lib -p simthing-mapgenerator` to apply 1 suggestion)
    Checking simthing-gpu v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-gpu)
warning: use of deprecated associated function `bevy::prelude::Handle::<A>::weak_from_u128`: use the `weak_handle!` macro with a UUID string instead
  --> crates\simthing-tools\src\bevy.rs:54:13
   |
54 |     Handle::weak_from_u128(0x5459_5045_4c52_3300_0000_0000_0000_0001);
   |             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: field `bind_group` is never read
  --> crates\simthing-tools\src\text_render.rs:90:9
   |
89 | pub struct TextAtlasGpuResource {
   |            -------------------- field in this struct
90 |     pub bind_group: BindGroup,
   |         ^^^^^^^^^^
   |
   = note: `TextAtlasGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:145:9
    |
143 | pub struct TextDeformGpuResource {
    |            --------------------- field in this struct
144 |     pub rows_buffer: Buffer,
145 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextDeformGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:173:9
    |
171 | pub struct TextPathGpuResource {
    |            ------------------- field in this struct
172 |     pub rows_buffer: Buffer,
173 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextPathGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: field `bind_group` is never read
   --> crates\simthing-tools\src\text_render.rs:181:9
    |
179 | pub struct TextWarpGpuResource {
    |            ------------------- field in this struct
180 |     pub rows_buffer: Buffer,
181 |     pub bind_group: BindGroup,
    |         ^^^^^^^^^^
    |
    = note: `TextWarpGpuResource` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: `simthing-tools` (lib) generated 5 warnings
    Checking simthing-feeder v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-feeder)
    Checking simthing-spec v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-spec)
warning: unused imports: `GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID`, `STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS`, and `STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS`
  --> crates\simthing-spec\src\spec\planet_child_location.rs:15:27
   |
15 |     SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
20 |     PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PlanetChildLocationAdmissionClassification`
  --> crates\simthing-spec\src\spec\scenario_ingestion.rs:21:38
   |
21 |     evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
   --> crates\simthing-spec\src\spec\scenario.rs:680:56
    |
680 |     matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
    |                                                        ^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: unused variable: `value`
   --> crates\simthing-spec\src\spec\owner_silo_runtime_writeback.rs:100:18
    |
100 |             Some(value) => Some(read_required_silo_amount(
    |                  ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `report` is never read
    --> crates\simthing-spec\src\spec\planet_child_location.rs:1384:13
     |
1384 |             report.rejected_count = 1;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-spec` (lib) generated 5 warnings (run `cargo fix --lib -p simthing-spec` to apply 3 suggestions)
    Checking simthing-clausething v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-clausething)
    Checking simthing-sim v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-sim)
warning: use of deprecated unit variant `simthing_core::SimThingKind::Faction`: Use Owner. Retained only for legacy serialized data compatibility.
    --> crates\simthing-clausething\src\hydrate_scenario.rs:3190:39
     |
3190 |         "Faction" => Ok(SimThingKind::Faction),
     |                                       ^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    |
 98 | impl SimRuntimeTree {
    | ------------------- methods in this implementation
...
118 |     pub(crate) fn replace(&mut self, tree: SimThing) -> SimThing {
    |                   ^^^^^^^
...
307 |     pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
    |                   ^^^^^^
...
311 |     pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
    |                   ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `detach_at_path` is never used
  --> crates\simthing-sim\src\tree_index.rs:45:8
   |
45 | pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
   |        ^^^^^^^^^^^^^^

warning: `simthing-sim` (lib) generated 2 warnings
    Checking simthing-driver v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-driver)
warning: unused import: `GpuContext`
  --> crates\simthing-driver\src\simulation_fabric.rs:44:20
   |
44 | use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
   |                    ^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: associated function `deserialize_msg` is never used
  --> crates\simthing-clausething\src\jomini\errors.rs:37:19
   |
11 | impl Error {
   | ---------- associated function in this implementation
...
37 |     pub(crate) fn deserialize_msg(msg: impl Into<Box<str>>) -> Self {
   |                   ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `new` is never used
   --> crates\simthing-clausething\src\jomini\errors.rs:181:19
    |
179 | impl ReaderError {
    | ---------------- associated function in this implementation
180 |     #[inline]
181 |     pub(crate) fn new(position: usize, kind: ReaderErrorKind) -> Self {
    |                   ^^^

warning: function `get_split` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:11:15
   |
11 | pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<(&[u8; N], &[u8])> {
   |               ^^^^^^^^^

warning: function `bytewise_equal` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:52:10
   |
52 | const fn bytewise_equal(lhs: u64, rhs: u64) -> u64 {
   |          ^^^^^^^^^^^^^^

warning: function `sum_usize` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:61:10
   |
61 | const fn sum_usize(values: u64) -> u64 {
   |          ^^^^^^^^^

warning: function `count_chunk` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:73:21
   |
73 | pub(crate) const fn count_chunk(value: u64, byte: u8) -> u64 {
   |                     ^^^^^^^^^^^

warning: function `leading_whitespace` is never used
  --> crates\simthing-clausething\src\jomini\util.rs:78:15
   |
78 | pub(crate) fn leading_whitespace(value: u64) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^

warning: struct `ResourceAmount` is never constructed
   --> crates\simthing-clausething\src\hydrate_field_economy.rs:230:8
    |
230 | struct ResourceAmount {
    |        ^^^^^^^^^^^^^^

warning: function `parse_resource_amount` is never used
    --> crates\simthing-clausething\src\hydrate_field_economy.rs:1053:4
     |
1053 | fn parse_resource_amount(
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-clausething` (lib) generated 10 warnings
    Checking simthing-workshop v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop)
warning: unused variable: `registry`
   --> crates\simthing-driver\src\arena_allocation_sync.rs:377:5
    |
377 |     registry: &DimensionRegistry,
    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_registry`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `packed_cpu` is never read
   --> crates\simthing-driver\src\min_plus_traversal_field.rs:406:52
    |
406 |             let mut packed_cpu: Option<Vec<f32>> = None;
    |                                                    ^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:273:5
    |
273 |     alloc.install_initial_tree(&scenario.root);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
    |
273 |     let _ = alloc.install_initial_tree(&scenario.root);
    |     +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:352:13
    |
352 |             alloc.install_initial_tree(&scenario.root);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
352 |             let _ = alloc.install_initial_tree(&scenario.root);
    |             +++++++

warning: unused `std::result::Result` that must be used
   --> crates\simthing-driver\src\resource_flow_convergence_burn_in.rs:406:17
    |
406 |                 alloc.install_initial_tree(&scenario.root);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
    |
406 |                 let _ = alloc.install_initial_tree(&scenario.root);
    |                 +++++++

warning: `simthing-driver` (lib) generated 6 warnings (run `cargo fix --lib -p simthing-driver` to apply 2 suggestions)
    Checking simthing-mapeditor v0.1.0 (C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-mapeditor)
warning: unused imports: `apply_gridcell_property_edit` and `structural_property_value_u32`
  --> crates\simthing-mapeditor\src\hydration.rs:8:5
   |
 8 |     apply_gridcell_property_edit, apply_star_system_display_name_metadata,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 9 |     load_scenario_spec_from_json_str, resolve_map_container, serialize_scenario_authority,
10 |     star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `planet_non_grid_child_owner_ref`
  --> crates\simthing-mapeditor\src\studio_scenario_document.rs:15:5
   |
15 |     planet_non_grid_child_owner_ref, planet_owner_ref, resolve_map_container,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `lerp` is never used
   --> crates\simthing-mapeditor\src\hyperlane_buckets.rs:255:4
    |
255 | fn lerp(a: f32, b: f32, t: f32) -> f32 {
    |    ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:91:24
   |
91 |     source_texel_size: Vec2,
   |                        ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:92:22
   |
92 |     blur_texel_size: Vec2,
   |                      ^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:93:18
   |
93 |     panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:94:18
   |
94 |     panel_count: u32,
   |                  ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:95:14
   |
95 |     enabled: u32,
   |              ^^^

warning: function `check` is never used
  --> crates\simthing-mapeditor\src\studio_frosted_glass.rs:96:15
   |
96 |     _padding: Vec2,
   |               ^^^^

warning: function `collect_field_accretion_sample` is never used
    --> crates\simthing-mapeditor\src\studio_live_session_bridge.rs:1111:4
     |
1111 | fn collect_field_accretion_sample(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field `phase` is never read
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:188:16
    |
181 | pub(super) struct BatchedGalaxySceneBuild {
    |                   ----------------------- field in this struct
...
188 |     pub(super) phase: SceneAdoptionVisibilityPhase,
    |                ^^^^^

warning: function `format_simthing_nameplate_id` is never used
   --> crates\simthing-mapeditor\src\app\galaxy_render.rs:631:8
    |
631 | pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sync_star_visuals_system` is never used
   --> crates\simthing-mapeditor\src\app\picking.rs:163:8
    |
163 | pub fn sync_star_visuals_system(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-mapeditor` (lib) generated 13 warnings (run `cargo fix --lib -p simthing-mapeditor` to apply 2 suggestions)
    Finished `dev` profile [optimized + debuginfo] target(s) in 18.04s

```

## code-head-scan.txt

```text
DOCTRINE SCAN REPORT  (commit 2e98c82e, 2026-09-19T02:31:09Z)
  scanner self-test: SKIPPED
  scan mode: PR delta (da1d7c15..2e98c82e)
  reliable scope: whole-tree
  heuristic scope: changed files / changed lines
  --- results ---
  FIELD-SWEEP-SINGLE-PATH-ALGEBRA  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; algebra is authored EML data and never an enum/tag/operator match in the sweep path
  FIELD-SWEEP-SINGLE-PATH-SHADERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; no eighth bespoke field shader in either production shader home beside the exact canonical generic interpreter
  FIELD-SWEEP-LEGACY-CALLERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; the seven retiring operators are migration referees only and have zero compiled production callers
  FIELD-SWEEP-DENSE-CAP-CROSSING  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; generic and sparse LinkGraph adjacency cannot inherit dense REGION_FIELD theater caps
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  TEST-BUDGET  PASS  0  design §0.9.5 test admission budget
  SPEC-LOWERER-KIND-READ  PASS  0  ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but closed-lowerer hits are higher suspicion because lowerers are constitutionally closed unless a DA-authorized amendment names them
  GUARD-KABUKI-TRIPWIRE  PASS  0  handoff_template section H + ci_screening_surface section 4; HEURISTIC tripwire for bespoke source-scanning guards and test-side include_str source greps; HC-6: symbol with well-formed FRESH HORIZON-ENTRY(iso-date): consumer/ref is EXEMPT (dated+assessable; unmarked/stale stay FLAGGED; never bare-token forever-pass); HC-8 accepted evasion residue: PRIVATE fn source-scanner or var-bound include_str! evades the pub-fn-anchored arms (DA review is the backstop; regex intentionally NOT widened — would false-fire on legit parsers); legitimate cases route to INSPECT triage, never FAIL
  EXECUTION-STATUS-UNCLASSIFIED  PASS  0  design 0.0.8.7 §3 Phase 0 EXECUTION-STATUS-TAXONOMY-0; HEURISTIC: execution-flavored driver/kernel surface missing from scripts/ci/execution_status_taxonomy.tsv (delta-scoped on PR)
  CELL-STORAGE-POLYMORPHISM  PASS  0  design 0.0.8.7 §2 P0(e) fence (i) CELL-STORAGE-POLYMORPHISM; HEURISTIC reach detector for tagged/templated/heterogeneous matrix-cell storage across production crates (workshop excluded)
  BESPOKE-PATHFINDER  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 PALMA BESPOKE-PATHFINDER; HEURISTIC: A* (BinaryHeap+came_from/g_score/open_set) OR ordinary Dijkstra (dist/distance+prev/predecessor with dijkstra/shortest_path/relax_edge) in production crates
  BORDER-SERVICE  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 Gu-Yang BORDER-SERVICE; HEURISTIC: border/frontline semantic service machinery (not mere presentation polyline projection/cache); mapeditor included for service-layer reaches
  OWNER-POLICY-WEIGHT-AUTHORITY-MINT  PASS  0  design 0.0.8.7 §3 Phase 3 FIRST-CITIZEN-SPECIALISTS-0; HEURISTIC: authored clause/scenario-JSON sources minting OWNER_POLICY_WEIGHT_AUTHORITY property id 8_300_318 outside hydration field-economy derivation (hydration-derived dumps excluded)
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 4 entries
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=48s

```

