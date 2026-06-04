# Codex/Cursor Handoff 8 — GPU Adapter Targeting Remedial

**Recipient model: Cursor**  
**Role: production implementation agent**

**From:** ChatGPT / 0.0.8.0 production harness · **Date:** 2026-06-03 · **Gate:** remedial, required before any new GPU PASS claim.

## 0. Why this remedial exists

The committed ATLAS-BATCH GPU evidence selected the integrated Intel adapter rather than the user's discrete NVIDIA RTX 4080.

Evidence already in repo:

- `docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt` reports `adapter_name: Intel(R) RaptorLake-S Mobile Graphics Controller`.
- `docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt` reports the same Intel adapter and `gpu_tier_ran: true`.
- `crates/simthing-gpu/src/context.rs` currently calls `request_adapter` with `PowerPreference::default()`, no explicit adapter selection, no adapter inventory, and no fail-if-target-missing behavior.

This is not a correctness failure for the Intel adapter, but it is an **evidence-targeting gap**: the project cannot claim coverage of the user's discrete RTX 4080 path until raw logs prove the GPU tests ran on that adapter.

## 1. Fixed base harness

Cite these on handoff-back:

1. `docs/design_0_0_8_0.md` §0 — transient constitution; GPU-resident resource-flow posture; §0.5 discipline.
2. `docs/invariants.md` — Scenario Proof, `GpuVerified` vs `ExactDeterministic`, CPU-oracle parity discipline.
3. `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 — ATLAS-BATCH-0 ladder and GPU proof gates.
4. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — scenario target.
5. `crates/simthing-core/src/accumulator_op.rs` — reference only; do not edit for this remedial.
6. `docs/workshop/sead_self_ai_track.md` — SEAD charter; no CPU planner.

Rung-local citations:

- `crates/simthing-gpu/src/context.rs`
- `docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt`
- `docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt`
- `docs/handoffs/dress_rehearsal_codex_handoff_7_atlas_batch_0_store_gpu.md`

## 2. Required remedial outcome

Implement explicit GPU adapter targeting for test evidence.

The goal is not performance optimization and not gameplay logic. The goal is proof hygiene:

```text
GPU PASS evidence must say which adapter ran, and RTX-targeted gates must fail if the RTX adapter is not selected.
```

Future GPU raw logs must distinguish:

- adapter inventory observed;
- selected adapter;
- whether selection was explicit or default;
- whether the selected adapter matched the requested target;
- whether the GPU tier actually ran.

## 3. Required implementation shape

Modify GPU context initialization narrowly.

Allowed file:

```text
crates/simthing-gpu/src/context.rs
```

Add an options-based constructor while preserving existing call sites:

```rust
pub struct GpuContextOptions {
    pub adapter_name_contains: Option<String>,
    pub require_adapter_match: bool,
    pub power_preference: wgpu::PowerPreference,
}

impl Default for GpuContextOptions {
    fn default() -> Self {
        Self {
            adapter_name_contains: std::env::var("SIMTHING_GPU_ADAPTER_CONTAINS").ok(),
            require_adapter_match: std::env::var("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            power_preference: wgpu::PowerPreference::HighPerformance,
        }
    }
}
```

Provide:

```rust
GpuContext::new_with_options(options: GpuContextOptions)
GpuContext::new_blocking_with_options(options: GpuContextOptions)
```

Keep:

```rust
GpuContext::new()
GpuContext::new_blocking()
```

but make them delegate to default options so existing tests still compile.

Adapter selection requirements:

1. Enumerate adapters with `instance.enumerate_adapters(Backends::PRIMARY)` where available.
2. Record adapter info for every candidate: name, backend, device type, vendor/device IDs if available.
3. If `adapter_name_contains` is set, select the first adapter whose name contains that substring case-insensitively.
4. If no match and `require_adapter_match == true`, return a clear error such as `GpuInitError::AdapterTargetNotFound { requested, available }`.
5. If no match and `require_adapter_match == false`, fall back to `request_adapter` using `PowerPreference::HighPerformance`, but record that fallback happened.
6. Store selected adapter info in `GpuContext` and expose methods:

```rust
pub fn adapter_name(&self) -> &str
pub fn adapter_inventory_report(&self) -> String
pub fn adapter_target_matched(&self) -> bool
```

Do not add new WGSL, gameplay code, or production session wiring.

## 4. Required GPU test remediation

Update GPU tests to support RTX-targeted evidence.

Allowed files:

```text
crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs
crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store_gpu.rs   # if present after STORE-GPU implementation
```

If STORE-GPU is not implemented yet, do not create it only for this remedial. The immediate required re-run is PACK-GPU.

PACK-GPU must be re-run with explicit RTX targeting:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="NVIDIA"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_cargo_test_2026_06_03.txt
```

If the adapter name contains `RTX 4080` rather than `NVIDIA`, the target may be:

```powershell
$env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"
```

Use the most precise substring that matches the actual adapter inventory. The raw log must show the inventory and selected adapter.

## 5. Required RTX evidence artifacts

Create:

```text
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_cargo_test_2026_06_03.txt
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_parity_2026_06_03.txt
```

The RTX parity report must include:

```text
requested_adapter_substring: NVIDIA or RTX / exact substring used
require_adapter_match: true
adapter_inventory: <all adapters observed>
selected_adapter_name: <must be NVIDIA / RTX 4080 path>
selected_adapter_backend:
selected_adapter_device_type:
adapter_target_matched: true
gpu_tier_ran: true
tests_skipped: none
per-class parity results
EC-A2b_GpuVerified_closed_on_target_adapter: true/false
```

If no RTX adapter is found:

- keep the failure log;
- do not delete it;
- do not mark RTX coverage PASS;
- do not claim discrete-GPU coverage;
- return blocked status with the adapter inventory.

## 6. Do not delete prior Intel evidence

The Intel evidence remains valid evidence for that adapter. Do **not** delete:

```text
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt
```

Instead, update docs to distinguish:

```text
PACK-GPU Intel iGPU evidence: PASS
PACK-GPU RTX 4080 evidence: PASS / BLOCKED / not yet run
```

Delete only duplicate failed remedial logs if a final successful RTX log exists and the failed log is not referenced.

## 7. Production doc update requirements

After the RTX-targeted run, update:

```text
docs/design_0_0_8_0_consumer_pulled_production_track.md
docs/worklog.md
```

The production doc must state:

- Prior PACK-GPU evidence ran on Intel RaptorLake-S integrated GPU.
- This remedial added explicit adapter targeting.
- Whether RTX 4080-targeted PACK-GPU evidence is now PASS or BLOCKED.
- Future GPU rungs must record selected adapter and, when requested, must fail if the requested adapter is not selected.
- STORE-GPU may not claim RTX coverage unless its raw log shows selected adapter matches the requested RTX/NVIDIA target.

## 8. Tests required

Add/update tests so they prove adapter selection behavior without requiring the RTX path unless env vars are set:

1. `gpu_context_reports_selected_adapter` — adapter name/inventory is non-empty for any GPU run.
2. `gpu_context_honors_requested_adapter_substring_when_present` — when `SIMTHING_GPU_ADAPTER_CONTAINS` is set and a matching adapter exists, selected adapter contains it.
3. `gpu_context_fails_when_required_adapter_missing` — with an impossible substring and `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`, initialization fails cleanly and reports available adapters. This can be CPU-safe / no GPU dispatch beyond adapter enumeration.
4. Existing PACK-GPU tests must include selected-adapter diagnostics in raw output.

Run focused tests and save raw output under `docs/tests/`.

Required commands:

```powershell
cargo test -p simthing-gpu adapter -- --nocapture *>&1 | Tee-Object docs/tests/gpu_adapter_targeting_unit_2026_06_03.txt
```

and the RTX-targeted PACK-GPU command from §4.

If the exact test filter differs, record the exact command used.

## 9. Files allowed

May edit/create:

```text
crates/simthing-gpu/src/context.rs
crates/simthing-gpu/tests/*adapter*      # if a GPU crate test file is needed
crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs
docs/tests/gpu_adapter_targeting_unit_2026_06_03.txt
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_cargo_test_2026_06_03.txt
docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_parity_2026_06_03.txt
docs/design_0_0_8_0_consumer_pulled_production_track.md
docs/worklog.md
```

Only edit STORE-GPU test/report files if STORE-GPU implementation has already landed by the time this remedial is executed.

## 10. Files forbidden unless Opus explicitly authorizes

Do not edit:

```text
crates/simthing-core/**
crates/simthing-sim/**
crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_{gen,loc,pack,pack_gpu,store}.rs
crates/simthing-driver/src/lib.rs
WGSL shader files
```

Exception: if existing driver tests need diagnostic logging only, that is allowed in the test file named in §9.

## 11. Stop conditions

Stop and escalate if:

- selecting the RTX requires new WGSL;
- selecting the RTX requires core/sim architecture changes;
- Windows/wgpu cannot enumerate the RTX adapter;
- `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1` cannot be implemented cleanly;
- the RTX adapter is visible but tests fail only on RTX;
- the implementation would hide fallback-to-Intel behind a PASS claim.

## 12. Handoff-back format

Return:

```text
Recipient model: Opus
Role: design authority

GPU adapter targeting remedial implemented.

Prior evidence:
- PACK-GPU Intel iGPU evidence remains preserved.

Adapter targeting:
- requested_adapter_substring: <...>
- require_adapter_match: true/false
- adapter inventory: <...>
- selected adapter: <...>
- target matched: true/false

Tests:
- cargo test -p simthing-gpu adapter -- --nocapture
- Result: N passed; 0 failed
- Raw log: docs/tests/gpu_adapter_targeting_unit_2026_06_03.txt

RTX PACK-GPU rerun:
- command: <exact command>
- Result: N passed; 0 failed / BLOCKED
- Raw log: docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_cargo_test_2026_06_03.txt
- Parity: docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_rtx4080_parity_2026_06_03.txt

Deleted obsolete artifacts:
- none found / list

Production doc:
- Updated adapter evidence note.
- Future GPU rungs must record selected adapter and fail on target mismatch when requested.

§0.5 self-check:
Holds principles 1–6: adapter selection/evidence hygiene only; no gameplay resource-flow behavior, no new WGSL, no simthing-sim semantics, no default session wiring.
```

## 13. §0.5 self-check for this remedial contract

Holds principles 1–6: this remedial only makes GPU evidence targetable and auditable. It does not add gameplay resource-flow behavior, does not change the recursive allocation model, does not add CPU planner logic, does not add `simthing-sim` semantics, and does not default-wire any production session. It prevents future GPU proof claims from silently running on the wrong adapter.
