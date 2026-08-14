# FACILITY-RESIDENT-PLANE-SUBSTRATE-0 Results

- Track: 0.0.8.7 RF arena modernization (rung 7.6a)
- Status: **PROBATION — proof present; DA review pending**
- Handoff: Board comment `5294906778`
- DA authority: Board comment `5294698585` (A1/A2)
- HD-RECEIPT: `ccec55ce2422`
- ORIENT-RECEIPT: `8de008acfbdd`
- orientation_rule_stamp: `874354e66bc3ac81`
- Base: `6185447a69dcc53ab7e7fe6320175c36f82df6be`
- Branch: `codex/facility-resident-plane-substrate-0`

## Substrate delivered

- `FacilityPlaneGenerationBoundary` is the sole capability that advances a set of facility-local planes. Each `FacilityPlaneOwner` is opaque and non-cloneable; foreign-owner access, foreign-boundary advance, duplicate presentation, empty planes, and generation overflow fail closed before any swap.
- `FacilityResidentPlane` owns physically distinct GPU Current and Next buffers and exposes cardinality, bytes-per-plane, carry bytes, and boundary generation. Its reusable operation is a whole-plane Current-to-Next copy followed by one boundary-controlled swap; it performs no gather, comparison, crossing, or facility semantics.
- ActionBand's graduated state plane now uses that primitive with unchanged bind-group shape and unchanged numerical behavior.
- Owning generation is a third admitted Phase-5 threshold observation source carried in the existing final `AccumulatorTickParams` word. It reaches the existing `threshold_crossed` comparator and sealed threshold-emission buffers. There is still exactly one `threshold_crossed` function, no `generation_crossed` rival, and no added GPU buffer or binding.
- `RoutedGenerationDuration` carries only authored duration and source-generation provenance. Private fields plus `serde(deny_unknown_fields)` make a foreign absolute deadline absent from the type and rejected at admission. No deadline calculation or 7.7 lifecycle behavior is present.

## A2 pre-extraction numerical record

Captured on the real graduated ActionBand GPU path before the runtime extraction, then compared byte-for-byte after extraction inside `sparse_gpu_state_ping_pongs_and_matches_exact_eml_oracle`:

```text
ACTIONBAND-NUMERICAL-RECORD-V1 first_state=ActionBandStateGpu { satisfied: 0, generation: 1, projection_start: 0, projection_len: 1, distance: 0.5, velocity: 0.5, reserved: [1, 0] } second_state=ActionBandStateGpu { satisfied: 0, generation: 2, projection_start: 0, projection_len: 1, distance: 0.5, velocity: 0.5, reserved: [1, 0] } first_projection_bits=[1056964608] first_emission_bits=[1077936128] first_commitment=(0,0,3fc00000,701) all_satisfied=[0, 0, 1, 1, 1, 0, 1] all_projection_bits=[1056964608, 1056964608, 0, 0, 0, 1056964608, 0] fast_state=ActionBandStateGpu { satisfied: 0, generation: 1, projection_start: 0, projection_len: 1, distance: 0.5, velocity: 0.0, reserved: [1, 0] } fast_projection_bits=[1056964608] empty_state=ActionBandStateGpu { satisfied: 0, generation: 1, projection_start: 0, projection_len: 1, distance: 0.5, velocity: 0.0, reserved: [1, 0] } second_fast_state=ActionBandStateGpu { satisfied: 0, generation: 2, projection_start: 0, projection_len: 1, distance: 0.5, velocity: 0.0, reserved: [1, 0] }
```

The real-adapter whole-plane carry median was **4,096 ns before extraction and 4,096 ns after extraction** (32 bytes, 31 samples, NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan). This is the pre-compaction carry measurement; 7.6a performs no compaction.

## Load-bearing proofs

- A Phase-5 owning-generation threshold at `3.5` with owning generation `4` emits exactly one sealed event (`event_kind=7601`, value bits equal `4.0f32`). Unknown observation source `99` is rejected during packing.
- Two planes with different row widths advance together under one boundary. A sibling owner cannot validate against the other plane, and a second boundary cannot advance it; the rejected attempt leaves the plane at generation zero.
- ActionBand's inherited authority-fence test now requires calls into the reusable carry/advance primitive and requires the copy and swap to live in that primitive, while rejecting reintroduced private ActionBand Current/Next buffers.
- The routed carrier round-trips only `{authored_duration, provenance}` and rejects a planted `foreign_absolute_deadline` field.
- No new `#[test]` function was admitted: the proofs extend the already-ledgered ActionBand authority and routed-delivery tests. No test-inventory row or CI surface was edited.

## Verification

```text
cargo test -p simthing-driver --test actionband_gpu_execution_0
  5 passed; 0 failed
cargo test -p simthing-driver --test actionband_recursive_composition_0
  5 passed; 0 failed
cargo test -p simthing-core --test event_generation_stamp_0
  4 passed; 0 failed
cargo test -p simthing-kernel --lib
  33 passed; 0 failed
cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-driver
  PASS
bash scripts/ci/overlay_germ_archaeology_census_check.sh --check
  RECONCILIATION: routes=74 discovery=71 residue=49 unclassified=0 open=0
  CENSUS-CHECK-VERDICT: PASS
bash scripts/ci/agent_scan.sh
  DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0
  AGENT-SCAN-VERDICT: PASS delta_inspect=0
```

`TEST-INVENTORY-DRIFT-CHECK` still reports the pre-existing line-keyed doctest rename pair in `sealed/emission.rs` (`compile_fail_line_131/142` to `125/136`). The implementation does not touch that file or remediate the checker; this is reported for DA handling.

## Fences held

- No lifecycle semantics, absolute-deadline calculation, 7.7 behavior, new comparator, new crossing record, new GPU buffer, or new binding.
- No edit under `.github/workflows/**`; no CI checker code, allowlist, ladder, orientation source, binding-condition, or doctrine-anchor edit.
- The graduated ActionBand numerical behavior is byte-identical across extraction.
