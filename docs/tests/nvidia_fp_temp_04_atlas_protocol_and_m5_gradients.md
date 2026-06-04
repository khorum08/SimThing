# NVIDIA FP temporary battery 04 — atlas protocol oracle + M5 gradients

**Temporary file:** `docs/tests/nvidia_fp_temp_04_atlas_protocol_and_m5_gradients.md`
**Track:** `docs/nvidia_fp_determinism_test.md`
**Date:** 2026-06-03
**Battery:** `04 - atlas protocol + M5 gradients`
**Status:** PASS

## Commands

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture  # remedial 02A same-file adapter gate
cargo test -p simthing-driver --test phase_m_c0_m4_atlas_protocol_oracle -- --nocapture
cargo test -p simthing-driver --test phase_m_m5b_gradient_l3_composition_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_m5c_gradient_need_signal_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_m5e_gradient_scarcity_opportunity_fixture -- --nocapture
```

(Handoff listed `phase_m_m5b` / `phase_m_m5c` / `phase_m_m5e` name filters; **substitution** to integration binaries above — exact filter names match `--test` targets.)

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

Direct proof: same-file `gpu_adapter_is_discrete_rtx_target` run (see raw log append at end of file). Atlas/M5 battery commands do not print adapter lines.

## Results summary

| Suite | Tests | Failed | Ignored |
|---|---:|---:|---:|
| phase_m_c0_m4_atlas_protocol_oracle | 13 | 0 | 0 |
| phase_m_m5b_gradient_l3_composition_fixture | 9 | 0 | 0 |
| phase_m_m5c_gradient_need_signal_fixture | 6 | 0 | 0 |
| phase_m_m5e_gradient_scarcity_opportunity_fixture | 7 | 0 | 0 |
| **Total** | **35** | **0** | **0** |

## Performance/timing capture

| Command block | Cargo build | Test runtime |
|---|---|---|
| adapter gate (remedial 02A) | 0.21s | 0.88s |
| atlas protocol oracle | 1.55s | 1.24s |
| M5B | 0.14s | 1.22s |
| M5C | 0.12s | 0.60s |
| M5E | 1.60s | 0.58s |

Diagnostic only unless timestamp-query backed.

## Tolerance/parity standard

- C-0 atlas protocol: full_tile f32 Linf <= 1e-4 (observed ~3.05e-5)
- M5 GPU parity: existing fixture thresholds (GpuVerified f32)

## Intel baseline comparison

| Target | Prior Intel result | NVIDIA RTX result | Notes |
|---|---|---|---|
| phase_m_c0_m4_atlas_protocol_oracle | not found in committed logs for this target | 13/0/0; Linf=0.000030517578 | Committed oracle doc cites same Linf on unspecified adapter |
| phase_m_m5b | not found in committed logs for this target | 9/0/0 | phase_m_m5b doc: 7 passed for fixture binary only |
| phase_m_m5c | not found in committed logs for this target | 6/0/0 | phase_m_m5c doc: 5 passed for fixture binary only |
| phase_m_m5e | not found in committed logs for this target | 7/0/0 | phase_m_m5e doc: 7 passed for fixture binary only |
| Cargo timings | not found in committed logs for this target | see table above | |

## Raw decisive excerpts

```text
Adapter gate (remedial 02A): selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU; adapter_target_matched: true; test gpu_adapter_is_discrete_rtx_target ... ok
C-0 parity: full_tile_max_abs_error=0.000030517578 l_inf=0.000030517578 cells=256
test m5b_gradient_fields_gpu_parity_single_target ... ok
test m5c_gradient_fields_gpu_parity_single_target ... ok
test m5e_gradient_fields_gpu_parity_single_target ... ok
test result: ok. 13 passed; 0 failed; 0 ignored (atlas)
```

## Failures / blocked reason

none

## Interpretation

Direct same-file adapter gate confirms the battery was run under the RTX-targeted environment; atlas/M5 commands themselves do not print adapter lines. Atlas protocol oracle and M5B/M5C/M5E gradient GPU parity families pass on RTX 4080 with existing tolerances.

## §0.5 check

Evidence-only NVIDIA validation; no shader/math/tolerance changes, no gameplay resource-flow behavior, no simthing-sim semantic expansion, no default session wiring.

---

## Raw cargo log (below)

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-486c9ede-6a1b-4bfe-a0be-44e7c7245123.ps1:82 char:297
+ ... d_field.md; cargo test -p simthing-driver --test phase_m_c0_m4_atlas_ ...
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
warning: constant `C0_REPLAY_FINGERPRINT` is never used
  --> crates\simthing-driver\tests\support\c0_atlas_protocol_oracle.rs:12:11
   |
12 | pub const C0_REPLAY_FINGERPRINT: &str = "a974fe44e20620f3";
   |           ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `fnv64` is never used
   --> crates\simthing-driver\tests\support\c0_atlas_protocol_oracle.rs:176:8
    |
176 | pub fn fnv64(seed: &[u8]) -> u64 {
    |        ^^^^^

warning: function `fnv_append_u32` is never used
   --> crates\simthing-driver\tests\support\c0_atlas_protocol_oracle.rs:185:8
    |
185 | pub fn fnv_append_u32(hash: u64, v: u32) -> u64 {
    |        ^^^^^^^^^^^^^^

warning: function `hash_vram_report` is never used
   --> crates\simthing-driver\tests\support\c0_atlas_protocol_oracle.rs:191:8
    |
191 | pub fn hash_vram_report(report: &C0VramBudgetReport) -> u64 {
    |        ^^^^^^^^^^^^^^^^

warning: `simthing-driver` (test "phase_m_c0_m4_atlas_protocol_oracle") generated 4 warnings
    Finished `test` profile [optimized + debuginfo] target(s) in 1.55s
     Running tests\phase_m_c0_m4_atlas_protocol_oracle.rs 
(target\debug\deps\phase_m_c0_m4_atlas_protocol_oracle-2df38e24defdc9bf.exe)

running 13 tests
test c0_default_budget_is_1p5_gib_configurable_no_hard_cap ... ok
test c0_mapping_profile_default_remains_disabled ... ok
gutter multiplier 8├ù8=9 10├ù10 ref=6.76 bytes=73728

thread 'c0_rejects_non_homogeneous_square_batch_for_g0_mask' (29716) panicked at 
crates\simthing-gpu\src\atlas_mask.rs:94:9:
assertion `left == right` failed: C-0 v1 requires homogeneous square tiles
  left: 8
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
VRAM report: C0VramBudgetReport {
    active_budget_bytes: 1610612736,
    active_budget_gib: 1.5,
    budget_configurable: true,
    architectural_hard_cap: false,
    multiplier_reporting_required: true,
    tile_count: 4,
    tile_width: 8,
    tile_height: 8,
    horizon: 8,
    atlas_width: 16,
    atlas_height: 16,
    payload_cell_count: 256,
    isolation_cell_count: 256,
    total_atlas_cell_count: 256,
    n_dims: 4,
    bytes_per_cell: 16,
    buffer_multiplier: 2.0,
    algebraic_mask_multiplier: 1.0,
    algebraic_mask_bytes: 8192,
    algebraic_mask_fits_active_budget: true,
    physical_gutter_multiplier: 9.0,
    physical_gutter_bytes: 73728,
    physical_gutter_fits_active_budget: true,
    headroom_bytes: 1610604544,
    headroom_percent: 99.99949137369791,
    isolation_policy: AlgebraicTileLocalMaskG0,
    fallback_isolation: PhysicalGutterGteH,
}
test c0_cpu_protocol_oracle_matches_itself ... ok
test c0_no_semantic_wgsl_or_simthing_sim_awareness ... ok
test c0_physical_gutter_fallback_reports_6p76x_or_formula ... ok
test c0_request_atlas_batching_still_rejected_until_gate_acceptance ... ok
test c0_no_implementation_of_a0_b0_l3_frontierv2_5 ... ok
test c0_vram_multiplier_report_uses_active_budget ... ok
test c0_no_active_mask_or_source_identity ... ok
test c0_rejects_non_homogeneous_square_batch_for_g0_mask ... ok
C-0 parity: full_tile_max_abs_error=0.000030517578 l_inf=0.000030517578 cells=256 fingerprint=9d6d628a29d83f51
test c0_happy_path_algebraic_mask_atlas_protocol_oracle_parity ... ok
full_tile=0.000030517578 corridor_t44=0.00000047683716 (corridor non-authoritative)
test c0_full_tile_parity_not_corridor_only ... ok
C-0 replay fingerprint: 9d6d628a29d83f51
test c0_replay_reproducibility ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.24s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-486c9ede-6a1b-4bfe-a0be-44e7c7245123.ps1:82 char:468
+ ... adients.md; cargo test -p simthing-driver --test phase_m_m5b_gradient ...
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests\phase_m_m5b_gradient_l3_composition_fixture.rs 
(target\debug\deps\phase_m_m5b_gradient_l3_composition_fixture-85f5bbc882190687.exe)

running 9 tests
test m5b_field_rons_admit_with_single_target_gradients ... ok
test m5b_frame_gradient_sink_validation_admits ... ok
test m5b_l3_composition_oracle_is_deterministic_and_finite ... ok
test m5b_integrated_parent_columns_feed_l3_composite ... ok
test m5b_reference_scenario_admits_and_default_profile_disabled ... ok
test m5b_l3_gadget_stack_admits_with_ema_and_weighted_accumulator ... ok
test m5b_posture_no_new_substrate ... ok
test m5b_gradient_fields_gpu_parity_single_target ... ok
test m5b_reference_scenario_gpu_commitment_path_no_cpu_emission ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-486c9ede-6a1b-4bfe-a0be-44e7c7245123.ps1:82 char:655
+ ... adients.md; cargo test -p simthing-driver --test phase_m_m5c_gradient ...
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
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running tests\phase_m_m5c_gradient_need_signal_fixture.rs 
(target\debug\deps\phase_m_m5c_gradient_need_signal_fixture-a28355327c5a69d7.exe)

running 6 tests
test m5c_need_signal_fields_admit_with_single_target_gradients ... ok
test m5c_frame_gradient_sink_validation_admits ... ok
test m5c_posture_no_cpu_commitment_or_new_substrate ... ok
test m5c_routing_signal_l3_stack_admits_with_ema_and_weighted_accumulator ... ok
test m5c_integrated_need_routing_signal_is_finite_and_deterministic ... ok
test m5c_gradient_fields_gpu_parity_single_target ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.60s

cargo : warning: unused import: `EmlConsumerKind`
At C:\Users\mvorm\AppData\Local\Temp\ps-script-486c9ede-6a1b-4bfe-a0be-44e7c7245123.ps1:82 char:839
+ ... adients.md; cargo test -p simthing-driver --test phase_m_m5e_gradient ...
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
    Finished `test` profile [optimized + debuginfo] target(s) in 1.60s
     Running tests\phase_m_m5e_gradient_scarcity_opportunity_fixture.rs 
(target\debug\deps\phase_m_m5e_gradient_scarcity_opportunity_fixture-03a0dfa80996bec0.exe)

running 7 tests
test m5e_ron_fixtures_load_and_frame_admits ... ok
test m5e_posture_no_cpu_commitment_or_new_substrate ... ok
test m5e_fields_use_single_target_gradients_and_slotrange_sum ... ok
test m5e_integrated_pressure_signal_is_finite_and_deterministic ... ok
test m5e_l3_stack_admits_with_ema_and_weighted_accumulator ... ok
test m5e_pressure_rises_with_higher_scarcity_seed ... ok
test m5e_gradient_fields_gpu_parity_single_target ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s

c a r g o   :           B l o c k i n g   w a i t i n g   f o r   f i l e   l o c k   o n   a r t i f a c t   d i r e c t o r y 
 
 A t   C : \ U s e r s \ m v o r m \ A p p D a t a \ L o c a l \ T e m p \ p s - s c r i p t - 3 a 2 0 6 6 7 f - a 5 5 7 - 4 0 7 8 - 9 8 b b - c 7 3 7 4 e f 8 b d 4 f . p s 1 : 8 2   c h a r : 1 4 7 
 
 +   . . .   E R _ M A T C H = 1 ;   c a r g o   t e s t   - p   s i m t h i n g - d r i v e r   - - t e s t   d r e s s _ r e h e a r s a l _ a t l a   . . . 
 
 +                                   ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ 
 
         +   C a t e g o r y I n f o                     :   N o t S p e c i f i e d :   (         B l o c k i n g   w a . . . i f a c t   d i r e c t o r y : S t r i n g )   [ ] ,   R e m o t e E x c e p t i o n 
 
         +   F u l l y Q u a l i f i e d E r r o r I d   :   N a t i v e C o m m a n d E r r o r 
 
   
 
 w a r n i n g :   u n u s e d   i m p o r t :   ` E m l C o n s u m e r K i n d ` 
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
 r u n n i n g   1   t e s t 
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
         F i n i s h e d   ` t e s t `   p r o f i l e   [ o p t i m i z e d   +   d e b u g i n f o ]   t a r g e t ( s )   i n   0 . 2 1 s 
 
           R u n n i n g   t e s t s \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u . r s   
 
 ( t a r g e t \ d e b u g \ d e p s \ d r e s s _ r e h e a r s a l _ a t l a s _ b a t c h _ 0 _ s t o r e _ g p u - 8 3 1 a 1 9 9 d 6 2 3 9 6 6 4 e . e x e ) 
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
 t e s t   r e s u l t :   o k .   1   p a s s e d ;   0   f a i l e d ;   0   i g n o r e d ;   0   m e a s u r e d ;   9   f i l t e r e d   o u t ;   f i n i s h e d   i n   0 . 8 8 s 
 
 
 
 