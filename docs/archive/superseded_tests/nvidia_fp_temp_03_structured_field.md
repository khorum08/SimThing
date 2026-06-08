# NVIDIA FP temporary battery 03 — structured field f32 substrate

**Temporary file:** `docs/tests/nvidia_fp_temp_03_structured_field.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `03 - structured field f32 substrate`
**Status:** PASS

## Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture  # remedial 02A same-file adapter gate
cargo test -p simthing-gpu structured_field_stencil -- --nocapture
cargo test -p simthing-driver structured_field_region_execution -- --nocapture  # BLOCKED: compiles all driver tests; see deviation
cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture
cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture
cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture
```

## Adapter evidence

```text
requested_adapter_substring: RTX
require_adapter_match: true
adapter_inventory: Intel(R) RaptorLake-S Mobile Graphics Controller; NVIDIA GeForce RTX 4080 Laptop GPU; Intel(R) UHD Graphics; Microsoft Basic Render Driver (see same-file raw log — remedial 02A adapter gate append)
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
selected_adapter_is_intel: false
gpu_tier_ran: true
```

Direct proof: same-file `gpu_adapter_is_discrete_rtx_target` run (see raw log append at end of file). Structured-field battery commands do not print adapter lines.

## Results summary

| Suite | Tests | Failed | Ignored |
|---|---:|---:|---:|
| simthing-gpu `structured_field_stencil` (filter; lib+partial integration) | 12 | 0 | 0 |
| simthing-gpu `--test structured_field_stencil` (full integration) | 30 | 0 | 0 |
| simthing-driver `--test structured_field_region_execution` | 5 | 0 | 0 |
| simthing-driver `--test structured_field_stencil_parent_eml` | 2 | 0 | 0 |
| **Total executed** | **49** | **0** | **0** |

GPU CPU parity tests in integration binary include `structured_field_stencil_clamp_boundary_gpu_cpu_parity` and related — all passed.

## Performance/timing capture

| Command block | Cargo build | Test runtime |
|---|---|---|
| adapter gate (remedial 02A) | 0.29s | 0.90s |
| gpu filter (first) | Finished test profile ... in 2.10s | 0.81s (integration 6 tests) |
| driver region `--test` | 0.13s | 1.82s |
| driver parent_eml `--test` | 1.38s | 0.98s |
| gpu `--test structured_field_stencil` | 0.13s | 2.90s |

Wall-clock timings are diagnostic only (no timestamp-query perf oracle in these targets).

## Tolerance/parity standard

Existing structured-field GPU/CPU parity thresholds (f32 `GpuVerified` discipline per `docs/invariants.md`). No tolerance changes.

## Intel baseline comparison

| Target | Prior Intel result | NVIDIA RTX result | Notes |
|---|---|---|---|
| simthing-gpu `--test structured_field_stencil` | not found in committed logs for this target | 30/0/0 pass | Prior docs cite 16/16 or 25/25 on unspecified adapter |
| structured_field_region_execution | not found in committed logs for this target | 5/0/0 pass | phase_m1_1 cites 5/5 (adapter not logged) |
| structured_field_stencil_parent_eml | not found in committed logs for this target | 2/0/0 pass | phase docs cite 2/2 |
| Cargo timings | not found in committed logs for this target | see table above | |

## Raw decisive excerpts

- Adapter gate (remedial 02A): `selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU`, `adapter_target_matched: true`, `test gpu_adapter_is_discrete_rtx_target ... ok`
- `test result: ok. 30 passed; 0 failed; 0 ignored` (full gpu integration binary)
- `test result: ok. 5 passed; 0 failed; 0 ignored` (region execution)
- `test result: ok. 2 passed; 0 failed; 0 ignored` (parent EML)

## Failures / blocked reason

- Initial handoff command `cargo test -p simthing-driver structured_field_region_execution` failed at compile (unrelated `KernelDescriptor` field errors in other driver test binaries). **Substitution:** `--test structured_field_region_execution` per handoff §5.4.

## Interpretation

Direct same-file adapter gate confirms the battery was run under the RTX-targeted environment; structured-field commands themselves do not print adapter lines. Load-bearing structured-field f32 substrate passes on discrete RTX 4080 using existing tests and thresholds.

## §0.5 check

Evidence-only NVIDIA validation; no shader/math/tolerance changes, no gameplay resource-flow behavior, no simthing-sim semantic expansion, no default session wiring.

---

## Raw cargo log (below)

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-01e2246a-b9a3-440c-8c8a-c65da0accebb.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-gpu structured_field_stencil -- -- ...
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
   Compiling simthing-gpu v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-gpu)
warning: unused variable: `pipelines`
   --> crates\simthing-gpu\src\passes.rs:632:45
    |
632 |     fn run_velocity_integration_test_helper(pipelines: &Pipelines, state: &WorldGpuState, dt: f32) {
    |                                             ^^^^^^^^^ help: if this is intentional, prefix it with an 
underscore: `_pipelines`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-gpu` (lib test) generated 1 warning (run `cargo fix --lib -p simthing-gpu --tests` to apply 1 
suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 2.10s
     Running unittests src\lib.rs (target\debug\deps\simthing_gpu-0a0ab375d7f8dfe0.exe)

running 6 tests
test structured_field_stencil::unit_tests::debug_report_field_stats_from_values ... ok
test structured_field_stencil::unit_tests::extended_horizon_allows_h16_with_flag ... ok
test structured_field_stencil::unit_tests::execution_steps_reject_above_configured_horizon ... ok
test structured_field_stencil::unit_tests::debug_report_skips_stats_by_default ... ok
test structured_field_stencil::unit_tests::horizon_cap_default_rejects_h16 ... ok
test structured_field_stencil::unit_tests::source_capped_requires_cap ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 165 filtered out; finished in 0.00s

     Running tests\accumulator_op_session_gpu_bridge.rs 
(target\debug\deps\accumulator_op_session_gpu_bridge-0ae72449adb5bb5e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests\structured_field_stencil.rs (target\debug\deps\structured_field_stencil-ea3d47ff05ca6282.exe)

running 6 tests
test structured_field_stencil_active_mask_provisional ... ok
test structured_field_stencil_inert_by_default ... ok
test structured_field_stencil_source_cap_cluster_indices_correct ... ok
test structured_field_stencil_horizon_execution_rejects_steps_above_config ... ok
test structured_field_stencil_clamp_boundary_gpu_cpu_parity ... ok
test structured_field_stencil_source_policy_documented_or_enforced ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.81s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-231c0d29-4eee-4791-831a-562c2d771887.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-driver structured_field_region_exe ...
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

   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
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
warning: unused import: `MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:24:5
   |
24 |     MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `MobilityGpuKernel0OracleOutput`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel3_projection_fixture.rs:21:29
   |
21 |     MobilityGpuKernel0Gate, MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `run_opt_in_burn_in`
 --> crates\simthing-driver\tests\resource_flow_opt_in_telemetry.rs:9:5
  |
9 |     run_opt_in_burn_in, run_product_soak_with_telemetry, telemetry_for_open_session,
  |     ^^^^^^^^^^^^^^^^^^
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
error: couldn't read `crates\simthing-driver\tests\../../../docs/workshop/workshop_current_state.md`: The system 
cannot find the file specified. (os error 2)
   --> crates\simthing-driver\tests\phase_m_boundary_cadence_doctrine.rs:169:9
    |
169 |         include_str!("../../../docs/workshop/workshop_current_state.md"),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `simthing-driver` (test "phase_m_jit_desc0_kernel_descriptor") due to 20 previous errors
warning: build failed, waiting for other jobs to finish...
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

error: could not compile `simthing-driver` (test "phase_m_boundary_cadence_doctrine") due to 1 previous error
warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\support\e2b5_dynamic_enrollment_soak.rs:417:9
    |
417 |     let mut resource_flow_syncs = if enrollment_report.any_admissions() && resource_flow_enabled {
    |         ----^^^^^^^^^^^^^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `nested_hierarchy_materialization_report`
  --> crates\simthing-driver\tests\support\e11_nested.rs:11:5
   |
11 |     nested_hierarchy_materialization_report, plan_arena_allocation, register_child_share_formula,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\support\e11_nested.rs:22:5
   |
22 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^

warning: unused import: `slots_are_contiguous`
 --> crates\simthing-driver\tests\phase_e_a0_nested_resource_flow_static.rs:9:76
  |
9 |     refresh_fission_participant_child, reserve_gap_pools_for_parent_slots, slots_are_contiguous,
  |                                                                            ^^^^^^^^^^^^^^^^^^^^

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

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\support\e11_nested.rs:344:10
    |
344 |     let (mut root, hosted) = hosted_cohorts(hosted_count);
    |          ----^^^^
    |          |
    |          help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

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

warning: function `standard_flat_star_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:221:8
    |
221 | pub fn standard_flat_star_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `prop_words`
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1715:9
     |
1715 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
     |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `boundary_before` is never read
   --> crates\simthing-driver\tests\phase_m_frontier_v2_4_combined_feedback_loop.rs:249:5
    |
241 | struct FrontierV2CombinedFeedbackRun {
    |        ----------------------------- field in this struct
...
249 |     boundary_before: FrontierV2BoundaryRequestShadow,
    |     ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

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

warning: field `expect_bit_exact` is never read
  --> crates\simthing-driver\tests\support\e11_burn_in_scenarios.rs:25:9
   |
20 | pub struct BurnInScenarioFixture {
   |            --------------------- field in this struct
...
25 |     pub expect_bit_exact: bool,
   |         ^^^^^^^^^^^^^^^^
   |
   = note: `BurnInScenarioFixture` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V2_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:8:11
  |
8 | pub const FRONTIER_V2_FIXTURE_ID: &str = "frontier_v2_0_closed_loop_consumer_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V2_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:8:11
  |
8 | pub const FRONTIER_V2_FIXTURE_ID: &str = "frontier_v2_0_closed_loop_consumer_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_2_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:10:11
   |
10 | pub const FRONTIER_V2_2_FIXTURE_ID: &str = "frontier_v2_2_movement_feedback_application_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_1_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:9:11
  |
9 | pub const FRONTIER_V2_1_FIXTURE_ID: &str = "frontier_v2_1_candidate_evolution_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FRONTIER_V2_3_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:11:11
   |
11 | pub const FRONTIER_V2_3_FIXTURE_ID: &str = "frontier_v2_3_structural_feedback_application_v1";
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

warning: variants `ReplayAccepted`, `FixtureCandidate`, `NotImplemented`, and `Pending` are never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:21:5
   |
19 | pub enum FrontierV2FieldStatus {
   |          --------------------- variants in this enum
20 |     GpuVerified,
21 |     ReplayAccepted,
   |     ^^^^^^^^^^^^^^
22 |     FixtureCandidate,
   |     ^^^^^^^^^^^^^^^^
23 |     FixtureOnly,
24 |     NotImplemented,
   |     ^^^^^^^^^^^^^^
25 |     Pending,
   |     ^^^^^^^
   |
   = note: `FrontierV2FieldStatus` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: variant `Disabled` is never constructed
  --> crates\simthing-driver\tests\support\e11_resource_flow_soak.rs:16:5
   |
15 | pub enum ResourceFlowSoakMode {
   |          -------------------- variant in this enum
16 |     Disabled,
   |     ^^^^^^^^
   |
   = note: `ResourceFlowSoakMode` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: variant `RejectedCrossEntity` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:43:5
   |
40 | pub enum FrontierV2WriteClassification {
   |          ----------------------------- variant in this enum
...
43 |     RejectedCrossEntity,
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `FrontierV2WriteClassification` has derived impls for the traits `Debug` and `Clone`, but these are 
intentionally ignored during dead code analysis

warning: constant `FRONTIER_V2_4_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:12:11
   |
12 | pub const FRONTIER_V2_4_FIXTURE_ID: &str = "frontier_v2_4_combined_feedback_loop_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `hash_closed_loop_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:339:8
    |
339 | pub fn hash_closed_loop_delta(tick0: &FrontierV2TickRun, tick1: &FrontierV2TickRun) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2MovementFeedbackSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:444:12
    |
444 | pub struct FrontierV2MovementFeedbackSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `apply_structural_feedback_to_config` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:529:8
    |
529 | pub fn apply_structural_feedback_to_config(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `FrontierV2WriteClassification` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:40:10
   |
40 | pub enum FrontierV2WriteClassification {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2StructuralFeedbackSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:561:12
    |
561 | pub struct FrontierV2StructuralFeedbackSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2OwnColumnShadow` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:48:12
   |
48 | pub struct FrontierV2OwnColumnShadow {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `FrontierV2MovementWriteError` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:56:10
   |
56 | pub enum FrontierV2MovementWriteError {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:585:12
    |
584 | impl FrontierV2StructuralFeedbackSummary {
    | ---------------------------------------- method in this implementation
585 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `hash_structural_feedback_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:599:8
    |
599 | pub fn hash_structural_feedback_delta(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2BoundaryRequestShadow` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:62:12
   |
62 | pub struct FrontierV2BoundaryRequestShadow {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: enum `FrontierV2StructuralWriteError` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:73:10
   |
73 | pub enum FrontierV2StructuralWriteError {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `source_seed_placement` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:422:8
    |
422 | pub fn source_seed_placement(
    |        ^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_own_column_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:434:8
    |
434 | pub fn hash_own_column_shadow(shadow: FrontierV2OwnColumnShadow) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_structural_to_boundary_request_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:498:8
    |
498 | pub fn apply_structural_to_boundary_request_shadow(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `derive_next_tick_structural_feedback_code` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:519:8
    |
519 | pub fn derive_next_tick_structural_feedback_code(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

warning: function `hash_boundary_request_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:548:8
    |
548 | pub fn hash_boundary_request_shadow(shadow: FrontierV2BoundaryRequestShadow) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: static `GPU_MUTEX` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:14:12
   |
14 | pub static GPU_MUTEX: Mutex<()> = Mutex::new(());
   |            ^^^^^^^^^

warning: function `with_gpu` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:21:8
   |
21 | pub fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
   |        ^^^^^^^^

warning: constant `PIPE0_ORDERING_CLASS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:41:11
   |
41 | pub const PIPE0_ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";
   |           ^^^^^^^^^^^^^^^^^^^^

warning: function `empty_boundary_request_shadow` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:617:8
    |
617 | pub fn empty_boundary_request_shadow(source_unit_id: u32) -> FrontierV2BoundaryRequestShadow {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `apply_combined_feedback_to_config` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:630:8
    |
630 | pub fn apply_combined_feedback_to_config(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_SEMANTIC_TERMS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:43:7
   |
43 | const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
   |       ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_EXACT_TERMS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:49:7
   |
49 | const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];
   |       ^^^^^^^^^^^^^^^^^^^^^

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

warning: fields `event_rows`, `elapsed`, and `dispatch_count` are never read
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:93:5
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
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:761:12
    |
748 | impl Act2ChainOutcome {
    | --------------------- method in this implementation
...
761 |     pub fn summary(&self) -> ProposalSummary {
    |            ^^^^^^^

warning: method `proposal_code` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:767:12
    |
766 | impl ProposalRecord {
    | ------------------- method in this implementation
767 |     pub fn proposal_code(&self) -> u32 {
    |            ^^^^^^^^^^^^^

warning: method `accepted_count` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:773:12
    |
772 | impl ProposalSummary {
    | -------------------- method in this implementation
773 |     pub fn accepted_count(&self) -> u32 {
    |            ^^^^^^^^^^^^^^

warning: method `admitted` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:787:12
    |
778 | impl AdmissionRecord {
    | -------------------- method in this implementation
...
787 |     pub fn admitted(&self) -> bool {
    |            ^^^^^^^^

warning: constant `ACT2_ORDERING_CLASS` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:813:11
    |
813 | pub const ACT2_ORDERING_CLASS: &str = "OrderInvariantExact";
    |           ^^^^^^^^^^^^^^^^^^^

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:871:12
    |
871 | pub struct ProposalOutcome {
    |            ^^^^^^^^^^^^^^^

warning: struct `ConsumerOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:897:12
    |
897 | pub struct ConsumerOutcome {
    |            ^^^^^^^^^^^^^^^

warning: fields `reductions` and `elapsed` are never read
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:905:5
    |
904 | pub struct Act2ChainOutcome {
    |            ---------------- fields in this struct
905 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^
...
910 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: struct `AdmitOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:934:12
    |
934 | pub struct AdmitOutcome {
    |            ^^^^^^^^^^^^

warning: function `default_admission_rules` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:999:8
    |
999 | pub fn default_admission_rules() -> AdmissionRulesGpu {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `act2_event_rec` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1179:8
     |
1179 | pub fn act2_event_rec(index: u32, code: u32, state: u32, score: i32) -> Act2EventRecord {
     |        ^^^^^^^^^^^^^^

warning: function `pack_proposals` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1472:4
     |
1472 | fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^

warning: function `pack_summary` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1512:4
     |
1512 | fn pack_summary(summary: ProposalSummary) -> [u32; 7] {
     |    ^^^^^^^^^^^^

warning: function `pack_reductions` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1584:4
     |
1584 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1677:4
     |
1677 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
     |    ^^^^^^^^^^^^^^^^

warning: function `run_consume_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1692:4
     |
1692 | fn run_consume_gpu(
     |    ^^^^^^^^^^^^^^^

warning: function `run_admit_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1814:4
     |
1814 | fn run_admit_gpu(
     |    ^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1903:4
     |
1903 | fn run_proposals_gpu(
     |    ^^^^^^^^^^^^^^^^^

warning: variants `BoundaryRequestShadowWrite` and `RejectedCrossEntity` are never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:42:5
   |
40 | pub enum FrontierV2WriteClassification {
   |          ----------------------------- variants in this enum
41 |     OwnColumnShadowWrite,
42 |     BoundaryRequestShadowWrite,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
43 |     RejectedCrossEntity,
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `FrontierV2WriteClassification` has derived impls for the traits `Debug` and `Clone`, but these are 
intentionally ignored during dead code analysis

warning: function `hash_structural_candidate` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:330:8
    |
330 | pub fn hash_structural_candidate(c: FrontierV2StructuralCandidate) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "mobility_gpu_kernel3_projection_fixture") generated 8 warnings (run `cargo fix 
--test "mobility_gpu_kernel3_projection_fixture" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "resource_flow_opt_in_telemetry") generated 150 warnings (run `cargo fix --test 
"resource_flow_opt_in_telemetry" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "phase_m_field_policy_obs0_mobile_overlay_score") generated 2 warnings
warning: `simthing-driver` (test "phase_e_a0_nested_resource_flow_static") generated 4 warnings (run `cargo fix --test 
"phase_e_a0_nested_resource_flow_static" -p simthing-driver` to apply 4 suggestions)
warning: `simthing-driver` (test "e11_burn_in_scenarios") generated 125 warnings (124 duplicates)
warning: `simthing-driver` (test "phase_m_frontier_v1_2_gpu_replay_acceptance") generated 36 warnings (34 duplicates)
warning: `simthing-driver` (test "e11_resource_flow_soak") generated 120 warnings (118 duplicates)
warning: `simthing-driver` (test "e2b5_dynamic_fission_enrollment") generated 146 warnings (144 duplicates) (run 
`cargo fix --test "e2b5_dynamic_fission_enrollment" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "phase_m_frontier_v2_1_candidate_evolution") generated 95 warnings (71 duplicates) 
(run `cargo fix --test "phase_m_frontier_v2_1_candidate_evolution" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "phase_m_frontier_v2_4_combined_feedback_loop") generated 82 warnings (33 duplicates)
warning: `simthing-driver` (lib test) generated 1 warning (1 duplicate)
warning: `simthing-driver` (test "phase_m_frontier_v2_2_movement_feedback_application") generated 89 warnings (87 
duplicates)
cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-231c0d29-4eee-4791-831a-562c2d771887.ps1:82 char:302
+ ... d_field.md; cargo test -p simthing-driver structured_field_stencil_pa ...
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
warning: `simthing-driver` (lib test) generated 1 warning (1 duplicate)
   Compiling simthing-driver v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-driver)
warning: unused variable: `prop_words`
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1715:9
     |
1715 |     let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
     |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_prop_words`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `boundary_before` is never read
   --> crates\simthing-driver\tests\phase_m_frontier_v2_4_combined_feedback_loop.rs:249:5
    |
241 | struct FrontierV2CombinedFeedbackRun {
    |        ----------------------------- field in this struct
...
249 |     boundary_before: FrontierV2BoundaryRequestShadow,
    |     ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V2_FIXTURE_ID` is never used
 --> crates\simthing-driver\tests\support\frontier_v2.rs:8:11
  |
8 | pub const FRONTIER_V2_FIXTURE_ID: &str = "frontier_v2_0_closed_loop_consumer_v1";
  |           ^^^^^^^^^^^^^^^^^^^^^^

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

warning: constant `FRONTIER_V2_3_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v2.rs:11:11
   |
11 | pub const FRONTIER_V2_3_FIXTURE_ID: &str = "frontier_v2_3_structural_feedback_application_v1";
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

warning: variants `ReplayAccepted`, `FixtureCandidate`, `NotImplemented`, and `Pending` are never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:21:5
   |
19 | pub enum FrontierV2FieldStatus {
   |          --------------------- variants in this enum
20 |     GpuVerified,
21 |     ReplayAccepted,
   |     ^^^^^^^^^^^^^^
22 |     FixtureCandidate,
   |     ^^^^^^^^^^^^^^^^
23 |     FixtureOnly,
24 |     NotImplemented,
   |     ^^^^^^^^^^^^^^
25 |     Pending,
   |     ^^^^^^^
   |
   = note: `FrontierV2FieldStatus` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

warning: variant `RejectedCrossEntity` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v2.rs:43:5
   |
40 | pub enum FrontierV2WriteClassification {
   |          ----------------------------- variant in this enum
...
43 |     RejectedCrossEntity,
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `FrontierV2WriteClassification` has derived impls for the traits `Debug` and `Clone`, but these are 
intentionally ignored during dead code analysis

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

warning: function `hash_closed_loop_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:339:8
    |
339 | pub fn hash_closed_loop_delta(tick0: &FrontierV2TickRun, tick1: &FrontierV2TickRun) -> u64 {
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

warning: function `apply_structural_feedback_to_config` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:529:8
    |
529 | pub fn apply_structural_feedback_to_config(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV2StructuralFeedbackSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v2.rs:561:12
    |
561 | pub struct FrontierV2StructuralFeedbackSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `combined_hex` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:585:12
    |
584 | impl FrontierV2StructuralFeedbackSummary {
    | ---------------------------------------- method in this implementation
585 |     pub fn combined_hex(&self) -> String {
    |            ^^^^^^^^^^^^

warning: function `hash_structural_feedback_delta` is never used
   --> crates\simthing-driver\tests\support\frontier_v2.rs:599:8
    |
599 | pub fn hash_structural_feedback_delta(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `hash_gpu_resource_flow` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:467:8
    |
467 | pub fn hash_gpu_resource_flow(summary: GpuResourceFlowAllocationSummary) -> u64 {
    |        ^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^

warning: function `open_flat_star_session` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:146:8
    |
146 | pub fn open_flat_star_session(hosted_count: usize, flag_enabled: bool) -> FlatStarSession {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `standard_flat_star_inputs` is never used
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:221:8
    |
221 | pub fn standard_flat_star_inputs(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: static `GPU_MUTEX` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:14:12
   |
14 | pub static GPU_MUTEX: Mutex<()> = Mutex::new(());
   |            ^^^^^^^^^

warning: function `with_gpu` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:21:8
   |
21 | pub fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
   |        ^^^^^^^^

warning: constant `PIPE0_ORDERING_CLASS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:41:11
   |
41 | pub const PIPE0_ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";
   |           ^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_SEMANTIC_TERMS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:43:7
   |
43 | const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
   |       ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `FORBIDDEN_EXACT_TERMS` is never used
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:49:7
   |
49 | const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];
   |       ^^^^^^^^^^^^^^^^^^^^^

warning: fields `event_rows`, `elapsed`, and `dispatch_count` are never read
  --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:93:5
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
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:761:12
    |
748 | impl Act2ChainOutcome {
    | --------------------- method in this implementation
...
761 |     pub fn summary(&self) -> ProposalSummary {
    |            ^^^^^^^

warning: method `proposal_code` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:767:12
    |
766 | impl ProposalRecord {
    | ------------------- method in this implementation
767 |     pub fn proposal_code(&self) -> u32 {
    |            ^^^^^^^^^^^^^

warning: method `accepted_count` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:773:12
    |
772 | impl ProposalSummary {
    | -------------------- method in this implementation
773 |     pub fn accepted_count(&self) -> u32 {
    |            ^^^^^^^^^^^^^^

warning: method `admitted` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:787:12
    |
778 | impl AdmissionRecord {
    | -------------------- method in this implementation
...
787 |     pub fn admitted(&self) -> bool {
    |            ^^^^^^^^

warning: constant `ACT2_ORDERING_CLASS` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:813:11
    |
813 | pub const ACT2_ORDERING_CLASS: &str = "OrderInvariantExact";
    |           ^^^^^^^^^^^^^^^^^^^

warning: struct `ProposalOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:871:12
    |
871 | pub struct ProposalOutcome {
    |            ^^^^^^^^^^^^^^^

warning: struct `ConsumerOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:897:12
    |
897 | pub struct ConsumerOutcome {
    |            ^^^^^^^^^^^^^^^

warning: fields `reductions` and `elapsed` are never read
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:905:5
    |
904 | pub struct Act2ChainOutcome {
    |            ---------------- fields in this struct
905 |     reductions: [ReductionResult; CODE_COUNT],
    |     ^^^^^^^^^^
...
910 |     elapsed: std::time::Duration,
    |     ^^^^^^^

warning: struct `AdmitOutcome` is never constructed
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:934:12
    |
934 | pub struct AdmitOutcome {
    |            ^^^^^^^^^^^^

warning: function `default_admission_rules` is never used
   --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:999:8
    |
999 | pub fn default_admission_rules() -> AdmissionRulesGpu {
    |        ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `act2_event_rec` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1179:8
     |
1179 | pub fn act2_event_rec(index: u32, code: u32, state: u32, score: i32) -> Act2EventRecord {
     |        ^^^^^^^^^^^^^^

warning: function `pack_proposals` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1472:4
     |
1472 | fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^

warning: function `pack_summary` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1512:4
     |
1512 | fn pack_summary(summary: ProposalSummary) -> [u32; 7] {
     |    ^^^^^^^^^^^^

warning: function `pack_reductions` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1584:4
     |
1584 | fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
     |    ^^^^^^^^^^^^^^^

warning: function `decode_proposals` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1677:4
     |
1677 | fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
     |    ^^^^^^^^^^^^^^^^

warning: function `run_consume_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1692:4
     |
1692 | fn run_consume_gpu(
     |    ^^^^^^^^^^^^^^^

warning: function `run_admit_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1814:4
     |
1814 | fn run_admit_gpu(
     |    ^^^^^^^^^^^^^

warning: function `run_proposals_gpu` is never used
    --> crates\simthing-driver\tests\support\field_policy_v1_live_pipeline.rs:1903:4
     |
1903 | fn run_proposals_gpu(
     |    ^^^^^^^^^^^^^^^^^

warning: unused import: `MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel1_dispatch_fixture.rs:24:5
   |
24 |     MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `MobilityGpuKernel0OracleOutput`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel3_projection_fixture.rs:21:29
   |
21 |     MobilityGpuKernel0Gate, MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: `simthing-driver` (test "phase_m_frontier_v2_4_combined_feedback_loop") generated 82 warnings (run `cargo fix 
--test "phase_m_frontier_v2_4_combined_feedback_loop" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "mobility_gpu_kernel3_projection_fixture") generated 8 warnings (run `cargo fix 
--test "mobility_gpu_kernel3_projection_fixture" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_pack_gpu") generated 20 warnings (run `cargo fix 
--test "dress_rehearsal_atlas_batch_0_pack_gpu" -p simthing-driver` to apply 1 suggestion)
warning: unused import: `nested_hierarchy_materialization_report`
  --> crates\simthing-driver\tests\support\e11_nested.rs:11:5
   |
11 |     nested_hierarchy_materialization_report, plan_arena_allocation, register_child_share_formula,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\support\e11_nested.rs:22:5
   |
22 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^

warning: unused import: `slots_are_contiguous`
 --> crates\simthing-driver\tests\phase_e_a0_nested_resource_flow_static.rs:9:76
  |
9 |     refresh_fission_participant_child, reserve_gap_pools_for_parent_slots, slots_are_contiguous,
  |                                                                            ^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\support\e11_nested.rs:344:10
    |
344 |     let (mut root, hosted) = hosted_cohorts(hosted_count);
    |          ----^^^^
    |          |
    |          help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `simthing-driver` (test "phase_e_a0_nested_resource_flow_static") generated 4 warnings (run `cargo fix --test 
"phase_e_a0_nested_resource_flow_static" -p simthing-driver` to apply 4 suggestions)
error: couldn't read `crates\simthing-driver\tests\../../../docs/workshop/workshop_current_state.md`: The system 
cannot find the file specified. (os error 2)
   --> crates\simthing-driver\tests\phase_m_boundary_cadence_doctrine.rs:169:9
    |
169 |         include_str!("../../../docs/workshop/workshop_current_state.md"),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: unused import: `CompoundField0082Weights`
  --> crates\simthing-driver\tests\compound_field_0080_2.rs:12:30
   |
12 |     CompoundField0082Report, CompoundField0082Weights, BASE_DESIRABILITY, COMPOUND_FIELD_0080_2_ID,
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `FirstSliceMappingSession`
 --> crates\simthing-driver\tests\phase_m_first_slice_map_residency.rs:6:5
  |
6 |     FirstSliceMappingSession, FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryStatus,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error: could not compile `simthing-driver` (test "phase_m_boundary_cadence_doctrine") due to 1 previous error
warning: build failed, waiting for other jobs to finish...
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

warning: struct `FrontierV1LiveFieldAgentFeedbackCandidate` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:201:12
    |
201 | pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `FrontierV1LiveFieldAgentOracleOutput` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:245:12
    |
245 | pub struct FrontierV1LiveFieldAgentOracleOutput {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GpuResourceFlowAllocationSummary` is never constructed
   --> crates\simthing-driver\tests\support\frontier_v1.rs:314:12
    |
314 | pub struct GpuResourceFlowAllocationSummary {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: function `frontier_v1_flat_star_weights` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:477:8
    |
477 | pub fn frontier_v1_flat_star_weights() -> (f32, f32) {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `hash_live_field_agent_gpu_execution` is never used
   --> crates\simthing-driver\tests\support\frontier_v1.rs:540:8
    |
540 | pub fn hash_live_field_agent_gpu_execution(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: field `cols` is never read
   --> crates\simthing-driver\tests\support\e11_flat_star.rs:143:9
    |
140 | pub struct FlatStarSession {
    |            --------------- field in this struct
...
143 |     pub cols: NodeColumnRefs,
    |         ^^^^

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

warning: constant `FRONTIER_V1_SKELETON_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:11:11
   |
11 | pub const FRONTIER_V1_SKELETON_ID: &str = "frontier_v1_0_scenario_skeleton_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `FRONTIER_V1_FIXTURE_ID` is never used
  --> crates\simthing-driver\tests\support\frontier_v1.rs:12:11
   |
12 | pub const FRONTIER_V1_FIXTURE_ID: &str = "frontier_v1_1_opt_in_fixture_v1";
   |           ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: variant `Other` is never constructed
  --> crates\simthing-driver\tests\support\frontier_v1.rs:36:5
   |
34 | pub enum FieldPolicyPipelineVersion {
   |          ------------------- variant in this enum
35 |     ProposalPipelineV1,
36 |     Other,
   |     ^^^^^
   |
   = note: `FieldPolicyPipelineVersion` has derived impls for the traits `Debug` and `Clone`, but these are intentionally 
ignored during dead code analysis

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

warning: unused import: `std::path::Path`
  --> crates\simthing-driver\tests\e11b_nested_fission_gap.rs:28:5
   |
28 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

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

warning: function `try_gpu` is never used
  --> crates\simthing-driver\tests\support\e11_flat_star.rs:17:8
   |
17 | pub fn try_gpu() -> Option<GpuContext> {
   |        ^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

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

warning: unused import: `install_atomic`
  --> crates\simthing-driver\tests\phase_m_frontier_v1_3_gpu_resource_flow.rs:19:59
   |
19 |     build_execution_plan, compiled_stencil_to_gpu_config, install_atomic, resolve_node_columns,
   |                                                           ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `MOBILITY_RUNTIME1B_NAMED_GATE` and `MobilityRuntime1bForbiddenPathRequests`
  --> crates\simthing-driver\tests\support\mobility_runtime1b_dispatch_fixture.rs:20:42
   |
20 |     MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bForbiddenPathRequests,
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
23 |     MOBILITY_RUNTIME1B_NAMED_GATE, MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `install_atomic`
  --> crates\simthing-driver\tests\phase_m_frontier_v1_4_field_policy_route_replay.rs:22:59
   |
22 |     build_execution_plan, compiled_stencil_to_gpu_config, install_atomic, resolve_node_columns,
   |                                                           ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

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

warning: unused import: `MobilityGpuKernel0OracleOutput`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel4_34k_projection_fixture.rs:20:5
   |
20 |     MobilityGpuKernel0OracleOutput, MobilityGpuKernel0ParityClassification,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS` and `encode_parent_key_for_projection`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel4_34k_projection_fixture.rs:18:34
   |
18 |     cpu_column_transform_oracle, encode_parent_key_for_projection,
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
23 |     MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS,
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `cpu_chain_oracle`
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:12:37
   |
12 |     cpu_chain_checksum_for_columns, cpu_chain_oracle,
   |                                     ^^^^^^^^^^^^^^^^

warning: field `mapping_execution_profile` is never read
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:17:5
   |
14 | pub struct FirstSliceScenarioFixtureSession {
   |            -------------------------------- field in this struct
...
17 |     mapping_execution_profile: MappingExecutionProfile,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `tick_mapping` is never used
  --> crates\simthing-driver\tests\support\first_slice_scenario_fixture.rs:38:12
   |
20 | impl FirstSliceScenarioFixtureSession {
   | ------------------------------------- method in this implementation
...
38 |     pub fn tick_mapping(
   |            ^^^^^^^^^^^^

warning: constant `MOBILITY_RUNTIME1B_DISPATCH_GATE` is never used
  --> crates\simthing-driver\tests\support\gpu_exec0_fixture.rs:19:11
   |
19 | pub const MOBILITY_RUNTIME1B_DISPATCH_GATE: &str = "mobility_runtime1b_dispatch_closed";
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

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

warning: constant `MOBILITY_GPU_KERNEL8_NAMED_GATE` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:26:11
   |
26 | pub const MOBILITY_GPU_KERNEL8_NAMED_GATE: &str =
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: constant `MOBILITY_GPU_KERNEL8_VARIANT_COUNT` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:30:11
   |
30 | pub const MOBILITY_GPU_KERNEL8_VARIANT_COUNT: usize = 4;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:32:11
   |
32 | pub const MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED: bool = false;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8Gate` is never constructed
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:40:12
   |
40 | pub struct MobilityGpuKernel8Gate {
   |            ^^^^^^^^^^^^^^^^^^^^^^

warning: associated functions `registration_only` and `registration_and_dispatch` are never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:47:12
   |
46 | impl MobilityGpuKernel8Gate {
   | --------------------------- associated functions in this implementation
47 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^
...
55 |     pub fn registration_and_dispatch() -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8ForbiddenPathRequests` is never constructed
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:65:12
   |
65 | pub struct MobilityGpuKernel8ForbiddenPathRequests {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8FixtureInput` is never constructed
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:80:12
   |
80 | pub struct MobilityGpuKernel8FixtureInput {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `default_variant_batch` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:87:12
   |
86 | impl MobilityGpuKernel8FixtureInput {
   | ----------------------------------- associated function in this implementation
87 |     pub fn default_variant_batch() -> Self {
   |            ^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8ReplayReport` is never constructed
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:97:12
   |
97 | pub struct MobilityGpuKernel8ReplayReport {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8VariantReport` is never constructed
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:106:12
    |
106 | pub struct MobilityGpuKernel8VariantReport {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `MobilityGpuKernel8FixtureReport` is never constructed
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:117:12
    |
117 | pub struct MobilityGpuKernel8FixtureReport {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `run_mobility_gpu_kernel8_fixture` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:217:8
    |
217 | pub fn run_mobility_gpu_kernel8_fixture(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `classify_variant_parity` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:381:4
    |
381 | fn classify_variant_parity(
    |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_forbidden` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:398:4
    |
398 | fn validate_forbidden(
    |    ^^^^^^^^^^^^^^^^^^

warning: function `kernel6_input` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:442:4
    |
442 | fn kernel6_input(
    |    ^^^^^^^^^^^^^

warning: function `shell` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:474:4
    |
474 | fn shell(input: &MobilityGpuKernel8FixtureInput) -> MobilityGpuKernel8FixtureReport {
    |    ^^^^^

warning: function `disabled_no_op_report` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:519:4
    |
519 | fn disabled_no_op_report(input: &MobilityGpuKernel8FixtureInput) -> MobilityGpuKernel8FixtureReport {
    |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `rejected_report` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel8_variant_batch_fixture.rs:526:4
    |
526 | fn rejected_report(
    |    ^^^^^^^^^^^^^^^

warning: associated function `registration_only` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel6_chain_fixture.rs:49:12
   |
48 | impl MobilityGpuKernel6Gate {
   | --------------------------- associated function in this implementation
49 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^

warning: function `permuted_projected_34k_columns_for_kernel6` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel6_chain_fixture.rs:225:8
    |
225 | pub fn permuted_projected_34k_columns_for_kernel6() -> MobilityGpuKernel0ColumnProbe {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `registration_only` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel5_second_kernel_fixture.rs:82:12
   |
81 | impl MobilityGpuKernel5Gate {
   | --------------------------- associated function in this implementation
82 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^

warning: function `permuted_projected_34k_columns_for_kernel5` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel5_second_kernel_fixture.rs:257:8
    |
257 | pub fn permuted_projected_34k_columns_for_kernel5() -> MobilityGpuKernel0ColumnProbe {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `registration_only` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel4_34k_projection_fixture.rs:64:12
   |
63 | impl MobilityGpuKernel4Gate {
   | --------------------------- associated function in this implementation
64 |     pub fn registration_only() -> Self {
   |            ^^^^^^^^^^^^^^^^^

warning: function `generate_permuted_34k_runtime_composition_input` is never used
   --> crates\simthing-driver\tests\support\mobility_gpu_kernel4_34k_projection_fixture.rs:335:8
    |
335 | pub fn generate_permuted_34k_runtime_composition_input() -> MobilityRuntime0CompositionInput {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS` is never used
  --> crates\simthing-driver\tests\support\mobility_gpu_kernel3_projection_fixture.rs:34:11
   |
34 | pub const MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS: [&str; 6] = [
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

warning: variable does not need to be mutable
   --> crates\simthing-driver\tests\phase_m_frontier_v1_4_field_policy_route_replay.rs:124:9
    |
124 |     let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    |         ----^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: field `gpu_rf` is never read
   --> crates\simthing-driver\tests\phase_m_frontier_v1_4_field_policy_route_replay.rs:230:5
    |
227 | struct FrontierV1FieldPolicyRouteRun {
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

warning: `simthing-driver` (test "dress_rehearsal_atlas_batch_0_store") generated 23 warnings (15 duplicates) (run 
`cargo fix --test "dress_rehearsal_atlas_batch_0_store" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "phase_m_first_slice_map_residency") generated 151 warnings (32 duplicates) (run 
`cargo fix --test "phase_m_first_slice_map_residency" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "compound_field_0080_2") generated 1 warning (run `cargo fix --test 
"compound_field_0080_2" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "resource_flow_enrollment_session") generated 147 warnings (147 duplicates)
warning: `simthing-driver` (test "resource_economy_burn_in") generated 3 warnings (run `cargo fix --test 
"resource_economy_burn_in" -p simthing-driver` to apply 3 suggestions)
warning: `simthing-driver` (test "phase_m_frontier_v1_0_scenario_skeleton") generated 66 warnings (64 duplicates)
warning: `simthing-driver` (test "phase_m_frontier_v1_1_opt_in_fixture") generated 41 warnings (40 duplicates)
warning: `simthing-driver` (test "e11b_nested_fission_gap") generated 7 warnings (5 duplicates) (run `cargo fix --test 
"e11b_nested_fission_gap" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "phase_m_field_policy_act1_phase_e_proposal_consumer") generated 7 warnings (run `cargo fix 
--test "phase_m_field_policy_act1_phase_e_proposal_consumer" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "resource_flow_opt_in_burn_in") generated 148 warnings (145 duplicates)
warning: `simthing-driver` (test "resource_flow_scenario_class_burn_in") generated 148 warnings (148 duplicates)
warning: `simthing-driver` (test "mobility_runtime1b_dispatch_fixture") generated 4 warnings (2 duplicates) (run 
`cargo fix --test "mobility_runtime1b_dispatch_fixture" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath") generated 1 warning
warning: `simthing-driver` (test "mobility_gpu_kernel4_34k_projection_fixture") generated 10 warnings (6 duplicates) 
(run `cargo fix --test "mobility_gpu_kernel4_34k_projection_fixture" -p simthing-driver` to apply 1 suggestion)
warning: `simthing-driver` (test "mobility_gpu_kernel9_frame_stream_fixture") generated 36 warnings (8 duplicates) 
(run `cargo fix --test "mobility_gpu_kernel9_frame_stream_fixture" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "phase_m_first_slice_runtime") generated 2 warnings (run `cargo fix --test 
"phase_m_first_slice_runtime" -p simthing-driver` to apply 2 suggestions)
warning: `simthing-driver` (test "phase_m_jit_sqrt_exact_candidate_battery") generated 2 warnings
warning: `simthing-driver` (test "phase_m_economy_field_policy_product_fixture") generated 2 warnings
warning: `simthing-driver` (test "phase_m_frontier_v1_3_gpu_resource_flow") generated 37 warnings (33 duplicates) (run 
`cargo fix --test "phase_m_frontier_v1_3_gpu_resource_flow" -p simthing-driver` to apply 3 suggestions)
warning: `simthing-driver` (test "phase_m_frontier_v1_4_field_policy_route_replay") generated 31 warnings (27 duplicates) (run 
`cargo fix --test "phase_m_frontier_v1_4_field_policy_route_replay" -p simthing-driver` to apply 2 suggestions)
cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-1c1b60c7-327a-485e-b102-4dea378bb67d.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-driver --test structured_field_reg ...
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests\structured_field_region_execution.rs 
(target\debug\deps\structured_field_region_execution-ec47108e86639b7c.exe)

running 5 tests
test test_e_no_production_pass_graph_wiring ... ok
test test_b_readback_path_explicit ... ok
test test_a_execution_api_horizon_guard ... ok
test test_d_column_aware_reduction_matches_manual_slot_range_sum ... ok
test test_c_stats_path_readback_derived ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.82s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-1c1b60c7-327a-485e-b102-4dea378bb67d.ps1:82 char:309
+ ... d_field.md; cargo test -p simthing-driver --test structured_field_ste ...
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
    Finished `test` profile [optimized + debuginfo] target(s) in 1.38s
     Running tests\structured_field_stencil_parent_eml.rs 
(target\debug\deps\structured_field_stencil_parent_eml-df197ce34db50b58.exe)

running 2 tests
test test_g_production_defaults_unaffected ... ok
test test_e_column_aware_parent_eml ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.98s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-486c9ede-6a1b-4bfe-a0be-44e7c7245123.ps1:82 char:147
+ ... ER_MATCH=1; cargo test -p simthing-gpu --test structured_field_stenci ...
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


running 30 tests
test gradient_xy_aliased_output_columns_rejected ... ok
test gradient_xy_cpu_oracle_matches_two_single_axis_passes ... ok
test gradient_xy_target_y_out_of_range_rejected ... ok
test m5a_cpu_oracle_gradient_y_on_small_grid ... ok
test m5a_cpu_oracle_gradient_x_on_small_grid ... ok
test guard_no_production_pipeline_integration ... ok
test gradient_xy_cpu_oracle_writes_both_axes_one_pass ... ok
test m5a_cpu_oracle_isotropic_weights_match_legacy_gamma ... ok
test structured_field_stencil_active_mask_provisional ... ok
test structured_field_stencil_inert_by_default ... ok
test structured_field_stencil_source_cap_cluster_indices_correct ... ok
test test_d_source_cap_and_horizon_cap ... ok
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests\structured_field_stencil.rs (target\debug\deps\structured_field_stencil-ea3d47ff05ca6282.exe)
test test_m1_execute_configured_rejects_steps_above_horizon ... ok
test test_m1_1_horizon_guard_on_no_readback_and_readback_paths ... ok
test structured_field_stencil_source_policy_documented_or_enforced ... ok
test structured_field_stencil_clamp_boundary_gpu_cpu_parity ... ok
test structured_field_stencil_horizon_execution_rejects_steps_above_config ... ok
test test_m1_1_execute_configured_no_readback_default ... ok
test test_c_10x10_h8_tactical_horizon ... ok
test test_b_pingpong_correctness ... ok
test m5a_gpu_parity_gradient_y ... ok
test test_a_wgsl_compile_and_3x3_correctness ... ok
test test_m1_execute_configured_uses_horizon ... ok
test m5a_gpu_parity_gradient_x ... ok
test m5a_single_target_output_contract_preserved ... ok
test test_r1_gpu_buffer_copy_and_cell_write_helpers ... ok
test m5a_gpu_parity_source_capped_after_directional_weight_refactor ... ok
test gradient_xy_gpu_parity_both_axes ... ok
test test_m1_debug_report_with_stats_requires_readback ... ok
test m5a_gpu_parity_normalized_after_directional_weight_refactor ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.90s

c a r g o   :   w a r n i n g :   u n u s e d   i m p o r t :   ` E m l C o n s u m e r K i n d ` 
 
 A t   C : \ U s e r s \ m v o r m \ A p p D a t a \ L o c a l \ T e m p \ p s - s c r i p t - 3 e 1 f 8 6 5 f - a 5 c 3 - 4 5 b c - 8 f e 0 - d 3 4 9 8 c 3 a 6 1 b 9 . p s 1 : 8 2   c h a r : 1 4 7 
 
 +   . . .   E R _ M A T C H = 1 ;   c a r g o   t e s t   - p   s i m t h i n g - d r i v e r   - - t e s t   d r e s s _ r e h e a r s a l _ a t l a   . . . 
 
 +                                   ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ 
 
         +   C a t e g o r y I n f o                     :   N o t S p e c i f i e d :   ( w a r n i n g :   u n u s e d . . . m l C o n s u m e r K i n d ` : S t r i n g )   [ ] ,   R e m o t e E x c e p t i o n 
 
         +   F u l l y Q u a l i f i e d E r r o r I d   :   N a t i v e C o m m a n d E r r o r 
 
   
 
   - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ i n t e n s i t y _ e m l . r s : 5 : 5 
 
     | 
 
 5   |           E m l C o n s u m e r K i n d ,   E m l C o n s u m e r M a s k ,   E m l E x e c u t i o n C l a s s ,   E m l F o r m u l a M e t a ,   E m l T r e e I d , 
 
     |           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
     | 
 
     =   n o t e :   ` # [ w a r n ( u n u s e d _ i m p o r t s ) ] `   ( p a r t   o f   ` # [ w a r n ( u n u s e d ) ] ` )   o n   b y   d e f a u l t 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
     - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ l i b . r s : 4 1 : 8 5 
 
       | 
 
 4 1   |           E m l E x p r e s s i o n R e g i s t r y ,   E m l F o r m u l a M e t a ,   E m l N o d e G p u ,   E m l R e g i s t r y E r r o r ,   E m l T r e e I d ,   E m l T r e e M e t a , 
 
       |                                                                                                                                                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
       | 
 
       =   n o t e :   ` # [ w a r n ( d e p r e c a t e d ) ] `   o n   b y   d e f a u l t 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 2 4 : 6 
 
         | 
 
 1 2 4   |   i m p l   E m l T r e e M e t a   { 
 
         |             ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 4 4 : 1 1 
 
         | 
 
 1 4 4   |   i m p l   F r o m < E m l T r e e M e t a >   f o r   E m l F o r m u l a M e t a   { 
 
         |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 6 7 4 : 4 1 
 
         | 
 
 6 7 4   |   p u b   f n   c l a s s i f y _ l e g a c y _ t r e e _ m e t a ( m e t a :   & E m l T r e e M e t a )   - >   E m l E x e c u t i o n C l a s s   { 
 
         |                                                                                   ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 4 5 : 2 1 
 
         | 
 
 1 4 5   |           f n   f r o m ( l e g a c y :   E m l T r e e M e t a )   - >   S e l f   { 
 
         |                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 2 2 3 : 1 5 
 
         | 
 
 2 2 3   |                   m e t a :   E m l T r e e M e t a , 
 
         |                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 5 3 5 : 6 5 
 
         | 
 
 5 3 5   |           p u b   f n   g e t _ l e g a c y _ m e t a ( & s e l f ,   t r e e _ i d :   E m l T r e e I d )   - >   O p t i o n < E m l T r e e M e t a >   { 
 
         |                                                                                                                                   ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   s t r u c t   ` e m l _ r e g i s t r y : : E m l T r e e M e t a ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 5 3 6 : 4 5 
 
         | 
 
 5 3 6   |                   s e l f . f o r m u l a s . g e t ( & t r e e _ i d ) . m a p ( | f |   E m l T r e e M e t a   { 
 
         |                                                                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : h a s _ t r a n s c e n d e n t a l ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 2 6 : 1 2 
 
         | 
 
 1 2 6   |                   i f   s e l f . h a s _ t r a n s c e n d e n t a l   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 2 9 : 1 2 
 
         | 
 
 1 2 9   |                   i f   s e l f . n o d e _ c o u n t   = =   0   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 3 2 : 1 2 
 
         | 
 
 1 3 2   |                   i f   s e l f . n o d e _ c o u n t   >   M A X _ E M L _ T R E E _ N O D E S   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 3 3 : 5 5 
 
         | 
 
 1 3 3   |                           r e t u r n   E r r ( E m l R e g i s t r y E r r o r : : T o o M a n y N o d e s ( s e l f . n o d e _ c o u n t ) ) ; 
 
         |                                                                                                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 3 5 : 5 1 
 
         | 
 
 1 3 5   |                   i f   ! W H I T E L I S T E D _ F O R M U L A _ C L A S S E S . c o n t a i n s ( & s e l f . f o r m u l a _ c l a s s . a s _ s t r ( ) )   { 
 
         |                                                                                                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 3 7 : 1 7 
 
         | 
 
 1 3 7   |                                   s e l f . f o r m u l a _ c l a s s . c l o n e ( ) , 
 
         |                                   ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : h a s _ t r a n s c e n d e n t a l ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 4 7 : 1 2 
 
         | 
 
 1 4 7   |                   i f   l e g a c y . h a s _ t r a n s c e n d e n t a l   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 5 5 : 2 9 
 
         | 
 
 1 5 5   |                                   n o d e _ c o u n t :   l e g a c y . n o d e _ c o u n t , 
 
         |                                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 5 9 : 3 1 
 
         | 
 
 1 5 9   |                                   d i s p l a y _ n a m e :   l e g a c y . f o r m u l a _ c l a s s , 
 
         |                                                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 6 9 : 2 9 
 
         | 
 
 1 6 9   |                                   n o d e _ c o u n t :   l e g a c y . n o d e _ c o u n t , 
 
         |                                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 1 7 3 : 3 1 
 
         | 
 
 1 7 3   |                                   d i s p l a y _ n a m e :   l e g a c y . f o r m u l a _ c l a s s , 
 
         |                                                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 5 3 7 : 1 3 
 
         | 
 
 5 3 7   |                           n o d e _ c o u n t :   f . m e t a . n o d e _ c o u n t , 
 
         |                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : h a s _ t r a n s c e n d e n t a l ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 5 3 8 : 1 3 
 
         | 
 
 5 3 8   |                           h a s _ t r a n s c e n d e n t a l :   f . m e t a . e x e c u t i o n _ c l a s s   = =   E m l E x e c u t i o n C l a s s : : F a s t A p p r o x i m a t e , 
 
         |                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 5 3 9 : 1 3 
 
         | 
 
 5 3 9   |                           f o r m u l a _ c l a s s :   f . m e t a . d i s p l a y _ n a m e . c l o n e ( ) , 
 
         |                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : h a s _ t r a n s c e n d e n t a l ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 6 7 5 : 8 
 
         | 
 
 6 7 5   |           i f   m e t a . h a s _ t r a n s c e n d e n t a l   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 6 7 7 : 1 5 
 
         | 
 
 6 7 7   |           }   e l s e   i f   m e t a . n o d e _ c o u n t   = =   0   | |   m e t a . n o d e _ c o u n t   >   M A X _ E M L _ T R E E _ N O D E S   { 
 
         |                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : n o d e _ c o u n t ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 6 7 7 : 3 9 
 
         | 
 
 6 7 7   |           }   e l s e   i f   m e t a . n o d e _ c o u n t   = =   0   | |   m e t a . n o d e _ c o u n t   >   M A X _ E M L _ T R E E _ N O D E S   { 
 
         |                                                                               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   u s e   o f   d e p r e c a t e d   f i e l d   ` e m l _ r e g i s t r y : : E m l T r e e M e t a : : f o r m u l a _ c l a s s ` :   u s e   E m l F o r m u l a M e t a   ( C - 8 a ) 
 
       - - >   c r a t e s \ s i m t h i n g - c o r e \ s r c \ e m l _ r e g i s t r y . r s : 6 7 9 : 4 5 
 
         | 
 
 6 7 9   |           }   e l s e   i f   i s _ w h i t e l i s t e d _ f o r m u l a _ c l a s s ( & m e t a . f o r m u l a _ c l a s s )   { 
 
         |                                                                                           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   ` s i m t h i n g - c o r e `   ( l i b )   g e n e r a t e d   2 7   w a r n i n g s   ( r u n   ` c a r g o   f i x   - - l i b   - p   s i m t h i n g - c o r e `   t o   a p p l y   1   s u g g e s t i o n ) 
 
 w a r n i n g :   u n u s e d   i m p o r t :   ` R F _ C O N T I N U E D _ S T A T I C _ 5 1 2 ` 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ s r c \ r e s o u r c e _ f l o w _ f l a t _ s t a r _ c o n t i n u e d _ s o a k . r s : 1 3 : 5 
 
       | 
 
 1 3   |           R F _ C O N T I N U E D _ S T A T I C _ 5 1 2 ,   R F _ C O N T I N U E D _ S T A T I C _ S K E W E D , 
 
       |           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
       | 
 
       =   n o t e :   ` # [ w a r n ( u n u s e d _ i m p o r t s ) ] `   ( p a r t   o f   ` # [ w a r n ( u n u s e d ) ] ` )   o n   b y   d e f a u l t 
 
 
 
 w a r n i n g :   ` s i m t h i n g - d r i v e r `   ( l i b )   g e n e r a t e d   1   w a r n i n g   ( r u n   ` c a r g o   f i x   - - l i b   - p   s i m t h i n g - d r i v e r `   t o   a p p l y   1   s u g g e s t i o n ) 
 
 w a r n i n g :   u n u s e d   i m p o r t :   ` c h a n n e l _ s e t _ h a s _ k i n d ` 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 9 : 5 
 
       | 
 
 1 9   |           c h a n n e l _ s e t _ h a s _ k i n d ,   C h a n n e l K i n d ,   C h a n n e l S e t ,   L o c a t i o n I d , 
 
       |           ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
       | 
 
       =   n o t e :   ` # [ w a r n ( u n u s e d _ i m p o r t s ) ] `   ( p a r t   o f   ` # [ w a r n ( u n u s e d ) ] ` )   o n   b y   d e f a u l t 
 
 
 
 w a r n i n g :   u n u s e d   i m p o r t :   ` C h a n n e l S e t ` 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 1 4 : 3 4 
 
       | 
 
 1 4   |           A t l a s B a t c h P l a n ,   C h a n n e l K i n d ,   C h a n n e l S e t ,   L o c a t i o n I d ,   L o c a t i o n M a t e r i a l i z a t i o n ,   L o c a t i o n R o l e , 
 
       |                                                                     ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ S T O R E _ I D `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 6 : 1 1 
 
     | 
 
 6   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ S T O R E _ I D :   & s t r   =   " A T L A S - B A T C H - 0 - S T O R E " ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
     | 
 
     =   n o t e :   ` # [ w a r n ( d e a d _ c o d e ) ] `   ( p a r t   o f   ` # [ w a r n ( u n u s e d ) ] ` )   o n   b y   d e f a u l t 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ S T O R E _ S T A T U S _ P A S S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 7 : 1 1 
 
     | 
 
 7   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ S T O R E _ S T A T U S _ P A S S :   & s t r   = 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c a n o n i c a l _ p a c k _ p l a n `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 6 4 : 8 
 
       | 
 
 6 4   |   p u b   f n   c a n o n i c a l _ p a c k _ p l a n ( )   - >   A t l a s B a t c h P l a n   { 
 
       |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` s t o r e _ o r a c l e _ w i t h _ a d d i t i o n a l _ s o u r c e s `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 2 5 8 : 8 
 
         | 
 
 2 5 8   |   p u b   f n   s t o r e _ o r a c l e _ w i t h _ a d d i t i o n a l _ s o u r c e s ( 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c l a s s _ i d _ f o r _ l o c a t i o n _ r o l e `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 3 4 9 : 8 
 
         | 
 
 3 4 9   |   p u b   f n   c l a s s _ i d _ f o r _ l o c a t i o n _ r o l e ( r o l e :   L o c a t i o n R o l e )   - >   & ' s t a t i c   s t r   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` p a c k _ r o u n d _ t r i p _ c e l l `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 3 5 7 : 8 
 
         | 
 
 3 5 7   |   p u b   f n   p a c k _ r o u n d _ t r i p _ c e l l ( 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` p i r a t e _ f l e e t _ s o u r c e _ i d s `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e . r s : 4 0 6 : 8 
 
         | 
 
 4 0 6   |   p u b   f n   p i r a t e _ f l e e t _ s o u r c e _ i d s ( m a t e r i a l i z a t i o n :   & L o c a t i o n M a t e r i a l i z a t i o n )   - >   V e c < S t r i n g >   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ P A C K _ I D `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 : 1 1 
 
     | 
 
 1   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ P A C K _ I D :   & s t r   =   " A T L A S - B A T C H - 0 - P A C K " ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ P A C K _ S T A T U S _ P A S S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 : 1 1 
 
     | 
 
 2   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ P A C K _ S T A T U S _ P A S S :   & s t r   = 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` V 7 8 _ A T L A S _ V R A M _ B U D G E T _ B Y T E S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 6 : 1 1 
 
     | 
 
 6   |   p u b   c o n s t   V 7 8 _ A T L A S _ V R A M _ B U D G E T _ B Y T E S :   u 6 4   =   1 _ 6 1 0 _ 6 1 2 _ 7 3 6 ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` P A C K I N G _ S T R A T E G Y `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 8 : 1 1 
 
     | 
 
 8   |   p u b   c o n s t   P A C K I N G _ S T R A T E G Y :   & s t r   = 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` C L A S S _ G A L A C T I C _ 2 0 X 2 0 `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 1 : 1 1 
 
       | 
 
 1 1   |   p u b   c o n s t   C L A S S _ G A L A C T I C _ 2 0 X 2 0 :   & s t r   =   " G a l a c t i c 2 0 x 2 0 " ; 
 
       |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` C L A S S _ S T A R _ S Y S T E M _ 1 0 X 1 0 `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 2 : 1 1 
 
       | 
 
 1 2   |   p u b   c o n s t   C L A S S _ S T A R _ S Y S T E M _ 1 0 X 1 0 :   & s t r   =   " S t a r S y s t e m 1 0 x 1 0 " ; 
 
       |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` C L A S S _ P L A N E T _ S U R F A C E _ 1 0 X 1 0 `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 3 : 1 1 
 
       | 
 
 1 3   |   p u b   c o n s t   C L A S S _ P L A N E T _ S U R F A C E _ 1 0 X 1 0 :   & s t r   =   " P l a n e t S u r f a c e 1 0 x 1 0 " ; 
 
       |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` G A L A X Y _ S I D E `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 3 : 7 
 
       | 
 
 2 3   |   c o n s t   G A L A X Y _ S I D E :   u 3 2   =   2 0 ; 
 
       |               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` S Y S T E M _ S I D E `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 4 : 7 
 
       | 
 
 2 4   |   c o n s t   S Y S T E M _ S I D E :   u 3 2   =   1 0 ; 
 
       |               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` P L A N E T _ S U R F A C E _ S I D E `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 5 : 7 
 
       | 
 
 2 5   |   c o n s t   P L A N E T _ S U R F A C E _ S I D E :   u 3 2   =   1 0 ; 
 
       |               ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` T i l e C l a s s D e s c r i p t o r `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 8 : 1 2 
 
       | 
 
 2 8   |   p u b   s t r u c t   T i l e C l a s s D e s c r i p t o r   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` P a c k e d T i l e `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 4 0 : 1 2 
 
       | 
 
 4 0   |   p u b   s t r u c t   P a c k e d T i l e   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` T i l e M a s k B o u n d s `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 4 9 : 1 2 
 
       | 
 
 4 9   |   p u b   s t r u c t   T i l e M a s k B o u n d s   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` G Z e r o M a s k D e s c r i p t o r `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 5 7 : 1 2 
 
       | 
 
 5 7   |   p u b   s t r u c t   G Z e r o M a s k D e s c r i p t o r   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` V r a m R e p o r t `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 6 3 : 1 2 
 
       | 
 
 6 3   |   p u b   s t r u c t   V r a m R e p o r t   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   s t r u c t   ` A t l a s B a t c h P l a n `   i s   n e v e r   c o n s t r u c t e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 7 6 : 1 2 
 
       | 
 
 7 6   |   p u b   s t r u c t   A t l a s B a t c h P l a n   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   a s s o c i a t e d   i t e m s   ` c a n o n i c a l ` ,   ` f r o m _ m a t e r i a l i z a t i o n ` ,   ` c l a s s ` ,   a n d   ` t i l e s _ i n _ c l a s s `   a r e   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 8 5 : 1 2 
 
         | 
 
   8 4   |   i m p l   A t l a s B a t c h P l a n   { 
 
         |   - - - - - - - - - - - - - - - - - - -   a s s o c i a t e d   i t e m s   i n   t h i s   i m p l e m e n t a t i o n 
 
   8 5   |           p u b   f n   c a n o n i c a l ( )   - >   S e l f   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
   8 9   |           p u b   f n   f r o m _ m a t e r i a l i z a t i o n ( m a t e r i a l i z a t i o n :   & L o c a t i o n M a t e r i a l i z a t i o n )   - >   S e l f   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 1 5 3   |           p u b   f n   c l a s s ( & s e l f ,   c l a s s _ i d :   & s t r )   - >   O p t i o n < & T i l e C l a s s D e s c r i p t o r >   { 
 
         |                         ^ ^ ^ ^ ^ 
 
 . . . 
 
 1 5 7   |           p u b   f n   t i l e s _ i n _ c l a s s ( & s e l f ,   c l a s s _ i d :   & s t r )   - >   V e c < & P a c k e d T i l e >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` p a c k _ c l a s s _ r o w _ m a j o r `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 1 6 5 : 4 
 
         | 
 
 1 6 5   |   f n   p a c k _ c l a s s _ r o w _ m a j o r ( 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` b u i l d _ g _ z e r o _ m a s k `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 0 8 : 4 
 
         | 
 
 2 0 8   |   f n   b u i l d _ g _ z e r o _ m a s k ( 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` b u i l d _ v r a m _ r e p o r t `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 2 9 : 4 
 
         | 
 
 2 2 9   |   f n   b u i l d _ v r a m _ r e p o r t ( 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` b y t e s _ p e r _ c e l l _ f o r _ c h a n n e l s `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 7 3 : 4 
 
         | 
 
 2 7 3   |   f n   b y t e s _ p e r _ c e l l _ f o r _ c h a n n e l s ( c h a n n e l s :   & C h a n n e l S e t )   - >   u 6 4   { 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` p a c k _ c o o r d `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 7 7 : 8 
 
         | 
 
 2 7 7   |   p u b   f n   p a c k _ c o o r d ( p l a n :   & A t l a s B a t c h P l a n ,   l o c a t i o n _ i d :   L o c a t i o n I d ,   x :   u 3 2 ,   y :   u 3 2 )   - >   O p t i o n < ( u 3 2 ,   u 3 2 ) >   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` u n p a c k _ c o o r d `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 2 8 8 : 8 
 
         | 
 
 2 8 8   |   p u b   f n   u n p a c k _ c o o r d ( 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` t i l e _ s o u r c e _ a t _ a t l a s `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 3 0 8 : 4 
 
         | 
 
 3 0 8   |   f n   t i l e _ s o u r c e _ a t _ a t l a s ( p l a n :   & A t l a s B a t c h P l a n ,   c l a s s _ i d :   & s t r ,   a x :   u 3 2 ,   a y :   u 3 2 )   - >   O p t i o n < L o c a t i o n I d >   { 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` g _ z e r o _ s a m p l e `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 3 2 4 : 8 
 
         | 
 
 3 2 4   |   p u b   f n   g _ z e r o _ s a m p l e ( 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` a t l a s _ l i n e a r _ i n d e x `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 3 5 1 : 4 
 
         | 
 
 3 5 1   |   f n   a t l a s _ l i n e a r _ i n d e x ( a t l a s _ w i d t h :   u 3 2 ,   a x :   u 3 2 ,   a y :   u 3 2 )   - >   O p t i o n < u s i z e >   { 
 
         |         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c h a n n e l _ s e t _ m a t c h e s `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 3 5 5 : 8 
 
         | 
 
 3 5 5   |   p u b   f n   c h a n n e l _ s e t _ m a t c h e s ( l h s :   & C h a n n e l S e t ,   r h s :   & C h a n n e l S e t )   - >   b o o l   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c h a n n e l _ s e t _ h a s _ o w n e r _ i n d e x e d `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ p a c k . r s : 3 5 9 : 8 
 
         | 
 
 3 5 9   |   p u b   f n   c h a n n e l _ s e t _ h a s _ o w n e r _ i n d e x e d ( s e t :   & C h a n n e l S e t )   - >   b o o l   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ L O C _ I D `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 1 : 1 1 
 
     | 
 
 1   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ L O C _ I D :   & s t r   =   " A T L A S - B A T C H - 0 - L O C " ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ L O C _ S T A T U S _ P A S S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 2 : 1 1 
 
     | 
 
 2   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ L O C _ S T A T U S _ P A S S :   & s t r   = 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` E X P E C T E D _ T O T A L _ C E L L _ S L O T S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 7 : 1 1 
 
     | 
 
 7   |   p u b   c o n s t   E X P E C T E D _ T O T A L _ C E L L _ S L O T S :   u 3 2   =   3 0 0 0 ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   m e t h o d s   ` o c c u p a n t s _ a t `   a n d   ` l o c a t i o n s _ b y _ r o l e `   a r e   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 2 4 2 : 1 2 
 
         | 
 
   9 3   |   i m p l   L o c a t i o n M a t e r i a l i z a t i o n   { 
 
         |   - - - - - - - - - - - - - - - - - - - - - - - - - - - -   m e t h o d s   i n   t h i s   i m p l e m e n t a t i o n 
 
 . . . 
 
 2 4 2   |           p u b   f n   o c c u p a n t s _ a t ( & s e l f ,   l o c a t i o n _ i d :   L o c a t i o n I d ,   c e l l :   G r i d C e l l )   - >   V e c < & O c c u p a n t P l a c e m e n t >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 2 4 9   |           p u b   f n   l o c a t i o n s _ b y _ r o l e ( & s e l f ,   r o l e :   L o c a t i o n R o l e )   - >   V e c < & L o c a t i o n G r i d D e s c r i p t o r >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c e l l _ i n d e x `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 2 5 7 : 8 
 
         | 
 
 2 5 7   |   p u b   f n   c e l l _ i n d e x ( m a p _ b a s e :   u 3 2 ,   w i d t h :   u 3 2 ,   x :   u 3 2 ,   y :   u 3 2 )   - >   u 3 2   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   f u n c t i o n   ` c h a n n e l _ s e t _ h a s _ k i n d `   i s   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ l o c . r s : 3 1 7 : 8 
 
         | 
 
 3 1 7   |   p u b   f n   c h a n n e l _ s e t _ h a s _ k i n d ( s e t :   & C h a n n e l S e t ,   e x p e c t e d :   C h a n n e l K i n d )   - >   b o o l   { 
 
         |                 ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ G E N _ I D `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 1 : 1 1 
 
     | 
 
 1   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ G E N _ I D :   & s t r   =   " A T L A S - B A T C H - 0 - G E N " ; 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ G E N _ S T A T U S _ P A S S `   i s   n e v e r   u s e d 
 
   - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 2 : 1 1 
 
     | 
 
 2   |   p u b   c o n s t   D R E S S _ R E H E A R S A L _ A T L A S _ B A T C H _ 0 _ G E N _ S T A T U S _ P A S S :   & s t r   = 
 
     |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   c o n s t a n t   ` P I R A T E _ S T A R P O R T _ C O U N T `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 1 3 : 1 1 
 
       | 
 
 1 3   |   p u b   c o n s t   P I R A T E _ S T A R P O R T _ C O U N T :   u s i z e   =   1 ; 
 
       |                       ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   m e t h o d s   ` i n _ b o u n d s ` ,   ` c h e b y s h e v _ d i s t a n c e ` ,   a n d   ` e m p t y _ c e l l s _ b e t w e e n `   a r e   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 5 4 : 1 2 
 
       | 
 
 4 9   |   i m p l   G r i d C e l l   { 
 
       |   - - - - - - - - - - - - -   m e t h o d s   i n   t h i s   i m p l e m e n t a t i o n 
 
 . . . 
 
 5 4   |           p u b   f n   i n _ b o u n d s ( s e l f ,   s i d e :   u 3 2 )   - >   b o o l   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 5 8   |           p u b   f n   c h e b y s h e v _ d i s t a n c e ( s e l f ,   o t h e r :   S e l f )   - >   u 3 2   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 6 2   |           p u b   f n   e m p t y _ c e l l s _ b e t w e e n ( s e l f ,   o t h e r :   S e l f )   - >   u 3 2   { 
 
       |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   m e t h o d   ` c e l l _ c o u n t `   i s   n e v e r   u s e d 
 
     - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 8 1 : 1 8 
 
       | 
 
 7 3   |   i m p l   G r i d D i m s   { 
 
       |   - - - - - - - - - - - - -   m e t h o d   i n   t h i s   i m p l e m e n t a t i o n 
 
 . . . 
 
 8 1   |           p u b   c o n s t   f n   c e l l _ c o u n t ( & s e l f )   - >   u 3 2   { 
 
       |                                     ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   m e t h o d s   ` t e r r a n _ s y s t e m s ` ,   ` p i r a t e _ s y s t e m s ` ,   ` s t a r p o r t s ` ,   ` f l e e t s _ b y _ o w n e r ` ,   ` m i n i m u m _ t e r r a n _ e m p t y _ s p a c i n g ` ,   
 
 a n d   ` p i r a t e _ w i t h i n _ o n e _ e m p t y _ c e l l _ o f _ t e r r a n `   a r e   n e v e r   u s e d 
 
       - - >   c r a t e s \ s i m t h i n g - d r i v e r \ t e s t s \ . . \ s r c \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ g e n . r s : 1 8 2 : 1 2 
 
         | 
 
 1 3 7   |   i m p l   D r e s s R e h e a r s a l M a p   { 
 
         |   - - - - - - - - - - - - - - - - - - - - - -   m e t h o d s   i n   t h i s   i m p l e m e n t a t i o n 
 
 . . . 
 
 1 8 2   |           p u b   f n   t e r r a n _ s y s t e m s ( & s e l f )   - >   i m p l   I t e r a t o r < I t e m   =   & S y s t e m D e s c r i p t o r >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 1 8 8   |           p u b   f n   p i r a t e _ s y s t e m s ( & s e l f )   - >   i m p l   I t e r a t o r < I t e m   =   & S y s t e m D e s c r i p t o r >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 1 9 4   |           p u b   f n   s t a r p o r t s ( & s e l f )   - >   i m p l   I t e r a t o r < I t e m   =   & B u i l d i n g P l a c e m e n t >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 2 0 0   |           p u b   f n   f l e e t s _ b y _ o w n e r ( & s e l f ,   o w n e r :   O w n e r )   - >   i m p l   I t e r a t o r < I t e m   =   & F l e e t P l a c e m e n t >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 2 0 4   |           p u b   f n   m i n i m u m _ t e r r a n _ e m p t y _ s p a c i n g ( & s e l f )   - >   O p t i o n < u 3 2 >   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 . . . 
 
 2 1 8   |           p u b   f n   p i r a t e _ w i t h i n _ o n e _ e m p t y _ c e l l _ o f _ t e r r a n ( & s e l f ,   p i r a t e :   & S y s t e m D e s c r i p t o r )   - >   b o o l   { 
 
         |                         ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ 
 
 
 
 w a r n i n g :   ` s i m t h i n g - d r i v e r `   ( t e s t   " d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u " )   g e n e r a t e d   4 9   w a r n i n g s   ( r u n   ` c a r g o   f i x   
 
 - - t e s t   " d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u "   - p   s i m t h i n g - d r i v e r `   t o   a p p l y   2   s u g g e s t i o n s ) 
 
         F i n i s h e d   ` t e s t `   p r o f i l e   [ o p t i m i z e d   +   d e b u g i n f o ]   t a r g e t ( s )   i n   0 . 2 9 s 
 
           R u n n i n g   t e s t s \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u . r s   
 
 ( t a r g e t \ d e b u g \ d e p s \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u - 8 3 1 a 1 9 9 d 6 2 3 9 6 6 4 e . e x e ) 
 
 
 
 r u n n i n g   1   t e s t 
 
 a d a p t e r _ i n v e n t o r y :   [ I n t e l ( R )   R a p t o r L a k e - S   M o b i l e   G r a p h i c s   C o n t r o l l e r ,   N V I D I A   G e F o r c e   R T X   4 0 8 0   L a p t o p   G P U ,   I n t e l ( R )   R a p t o r L a k e - S   M o b i l e   G r a p h i c s   C o n t r o l l e r ,   N V I D I A   G e F o r c e   R T X   4 0 8 0   L a p t o p   G P U ,   I n t e l ( R )   U H D   G r a p h i c s ,   M i c r o s o f t   B a s i c   R e n d e r   D r i v e r ,   I n t e l ( R )   U H D   G r a p h i c s ] 
 
 r e q u e s t e d _ a d a p t e r _ s u b s t r i n g :   R T X 
 
 r e q u i r e _ a d a p t e r _ m a t c h :   t r u e 
 
 s e l e c t e d _ a d a p t e r _ n a m e :   N V I D I A   G e F o r c e   R T X   4 0 8 0   L a p t o p   G P U 
 
 a d a p t e r _ t a r g e t _ m a t c h e d :   t r u e 
 
 s e l e c t e d _ a d a p t e r _ i s _ d i s c r e t e _ r t x :   t r u e 
 
 g p u _ t i e r _ r a n :   t r u e 
 
 t e s t   g p u _ a d a p t e r _ i s _ d i s c r e t e _ r t x _ t a r g e t   . . .   o k 
 
 
 
 t e s t   r e s u l t :   o k .   1   p a s s e d ;   0   f a i l e d ;   0   i g n o r e d ;   0   m e a s u r e d ;   9   f i l t e r e d   o u t ;   f i n i s h e d   i n   0 . 9 0 s 
 
 
 
 