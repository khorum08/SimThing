# Frozen Phase-14.1 comparator context

Status: PROBATION / bounded STOP return / clearance blocked / OPEN / UNMERGED.

HD-RECEIPT `6b2e43dd4120`; coding ORIENT-RECEIPT `28f56884d309`; rule stamp `73e54d6b56b7266b`.
The independent gate blockers are the HD-required anchor reach log rejected by the rehearsal class and the absent 0.0.8.8 lifecycle-track registration. Neither is repaired in this coding scope. No optimization or graduation claim.

The frozen comparator ran at ingress code head `0b7453fa47849848200a3262f7a3581466434165`: **4 passed, 0 failed, 0 ignored**, 68.28 seconds of test execution. Instrument source, report formatter, test body and recorded contract are byte-unchanged. The test's generated legacy report was copied into this packet and its tracked original restored. The dated numbers below are current context for the frozen CPU-host door; they do not replace the historical record and are not the Phase-15 resident `T_R`.

Exact command (unchanged):

```powershell
$env:CARGO_BUILD_JOBS='2'
cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture
```

Build contract: default cargo-test, opt-level=1 inherited from profile.dev, debug assertions/debuginfo enabled; no custom RUSTFLAGS. Release is deliberately not substituted for this frozen recorded profile. First command compiled existing dependencies in 3m50s; build time is outside the recorded workload samples. Persistent OS/driver caches were not cleared. Each workload's exact warmup and repetition counts, including the three-repetition million-row leg, remain as recorded below. Seed 81921.

Same reference machine: Windows 11 Home 10.0.26200 build 26200, x86_64; Intel64 Family 6 Model 183 Stepping 1; NVIDIA GeForce RTX 4080 Laptop GPU, Vulkan, vendor 0x10de/device 0x27a0, NVIDIA 595.79 (Windows driver 32.0.15.9579), 12282 MiB dedicated memory; Rust 1.95.0 / LLVM22.1.2 / wgpu22.1.0 / naga22.1.0. This CPU-host comparator records adapter context but does not claim resident GPU qualification or GPU execution. Its absent GPU legs remain visibly absent.

| Instrument validity | Current-path characterization | Historical comparison | Candidate release | Candidate execution and result | Inherited budget/debt disposition |
|---|---|---|---|---|---|
| Frozen contract executed, 4/4 PASS | Current context for the frozen CPU-host instrument only | Original reference unchanged; no speedup or pass-ratio claim | Unopened | Unrun | M2 captured; no substitute for resident M3 or application 3.1 measurements |

Binding to code and profile: the following frozen file hashes identify the instrument and its immutable workload construction. The report's exact workloads/cardinalities/raw samples are part of the profile; no scenario reshaping occurs. There is no UI/Studio workload: selection/subscription, recipes, pending structural products and presentation GPU memory are not measured by this instrument. Its own memory/byte/dispatch door absences and workload measures are preserved verbatim below rather than filled with inferred resident metrics.

- `Cargo.toml`: SHA-256 `ffb1816086cd50ff987664cde6e2e75d624de4c1ad93626f089fce00442bd7b6`
- `Cargo.lock`: SHA-256 `e62921d73d527a38516d14eefa73e1f2ed9a6b7c0619ee9f7df779415503495a`
- `crates/simthing-workshop/src/generation_critical_path_baseline.rs`: SHA-256 `8a7c50a707ef678836b3227adf811a4f767978fcf6b360ac9dd03ceab5213d6b`
- `crates/simthing-workshop/src/generation_critical_path_baseline_report.rs`: SHA-256 `3ddf38c0999686100ebb183cdaf10ee7a496261d56b8d40a1378f37cae746ab5`
- `crates/simthing-workshop/tests/generation_critical_path_baseline_0.rs`: SHA-256 `17487e8ecd261798474e20566a14b41083294e52a431b7c4ecd52002420ab02e`

Full raw comparator packet (including signed samplewise residuals and all workload legs):

```text
GENERATION-CRITICAL-PATH-BASELINE-0
comparator only; not a go/no-go gate; no portable timing law

## Envelope
tested_commit: 0b7453fa47849848200a3262f7a3581466434165
utc_date: 2026-09-09T11:16:42Z
cpu: Intel64 Family 6 Model 183 Stepping 1, GenuineIntel
gpu: NVIDIA GeForce RTX 4080 Laptop GPU vendor=0x10de device=0x27a0 type=DiscreteGpu
adapter_backend: Vulkan
driver: NVIDIA 595.79
os: windows-x86_64
compiler_toolchain: rustc 1.95.0 (59807616e 2026-04-14)
profile: cargo-test (optimized+debuginfo)
deterministic_seed: 81921
exact_command: cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture

## Host clearing-door census
- clear_constrained_claims_at_generation @ crates/simthing-spec/src/spec/constrained_clearing.rs:274 authority=caller-supplied ClearingRemainderAuthority { granter, generation } posture=ordinary production CPU-host clearing door 14.6=narrow behind CpuVendorizedOracle; production migrates to the 14.2+ resident germ reexports=simthing-spec lib.rs + spec/mod.rs; simthing-embedder run::cpu_filter_oracle callers=production: simthing-driver/src/growth_entitlement.rs; wrapper: clear_reduced_owner_channels_at_generation; tests: contention_arena_executed_0, clearing_weight_span_unification_0, clearing_weight_deformation_lifecycle_0, stemthing_b_flow_market_germ_0, stemthing_b_vram_residency_0, grant_disbursement_lane_0, unified_facility_convergence_witness_0, protected_representative_restore, vendor_door_triad_surface_0
- clear_reduced_owner_channels @ crates/simthing-spec/src/spec/constrained_clearing.rs:441 authority=generationless compatibility: granter=SimThingId::from_session_raw(0), generation=GenerationStamp::new(0) posture=generationless compatibility shim / test oracle 14.6=DELETE reexports=simthing-spec lib.rs + spec/mod.rs callers=test: simthing-driver/tests/contention_arena_executed_0.rs (priority + price cases); no production caller
- clear_reduced_owner_channels_at_generation @ crates/simthing-spec/src/spec/constrained_clearing.rs:458 authority=caller-supplied ClearingRemainderAuthority; converts reduce-up buckets through ConstrainedClaim::from_runtime_demand then the ordinary door posture=conversion wrapper over the ordinary at-generation door 14.6=narrow behind CpuVendorizedOracle (or delete once callers are gone) reexports=simthing-spec lib.rs + spec/mod.rs callers=wrappers: clear_reduced_owner_channels (generationless), clear_stamped_owner_channels; no direct production caller
- clear_stamped_owner_channels @ crates/simthing-spec/src/spec/constrained_clearing.rs:509 authority=generation taken from StampedReduceUpProduct; granter supplied by caller posture=canonical stamped-RF market binding over the ordinary door 14.6=narrow behind CpuVendorizedOracle reexports=simthing-spec lib.rs + spec/mod.rs callers=test/germ: stemthing_b_flow_market_germ_0.rs; no other production caller on this base
- produce_runtime_rf_next_generation_demands @ crates/simthing-spec/src/spec/runtime_rf_tick.rs:149 authority=RuntimeRfDemandGenerationAuthority once-mint over caller ClearingRemainderAuthority; performs the generation-N clear inside the door posture=ordinary production Current->Next demand door (14.3 row-11 substrate recurrence, DA ruling 5488315659) 14.6=narrow behind CpuVendorizedOracle; recurrence migrates with the resident germ at cutover reexports=simthing-spec lib.rs + spec/mod.rs; driver wrapper produce_runtime_rf_next_generation_demands_for_tick callers=production: simthing-driver runtime_rf_tick_compile; witness: resident_clearing_score_and_bands_0

## Path diagram
ordinary generation critical path (CPU-host clearing door; GPU legs door-absent):
  RuntimeOwnerSiloDemandBucket (per-tree admission)
    -> ConstrainedClaim::from_runtime_demand  [claim_production_completion]
    -> (no GPU map/readback in this door)     [gpu_to_host_synchronization_readback = 0 bytes]
    -> group by OwnerChannelScopeKey          [host_conversion_grouping]
    -> TransformOp::apply_with_params         [eml_scoring]
    -> sort score-bits then id; equal-bit bands [score_sorting_banding]
    -> largest remainder + generation-rotated ties [integer_apportionment]
    -> ConstrainedGrant::from_clearance + ConstrainedClearingResult [grant_result_construction]
  enclosing production authority: clear_constrained_claims_at_generation
    -> (no GPU write_buffer/upload in this door) [host_to_gpu_upload = 0 bytes]
    -> grants available
    -> construct generation+1 authority        [n_plus_one_launch_delay; GPU launch = 0]
    -> (optional comparator) full gen+1 clear  [next_generation_host_reclear]
    -> record_cleared_grant -> IntegrationSchedule [cpu_schedule_replay_recording; not in N+1 delay]
    -> fund_unresolved_persistence            [lawful_structural_consequence]
  instrument (D2 shape 2; inside e2e, named, not residual):
    nested restatement pass                   [instrument_restatement = grouping+scoring+sorting+apportionment]
    second uninstrumented production clear    [neutrality_reclear]
  D6 residual[i] = end_to_end[i] - Σ accounted_leg[i]  [observation_overhead_residual]
  difference_of_medians = median(e2e) - Σ median(accounted)  [derived figure, not residual]

## Leg definitions
- claim_production_completion: ConstrainedClaim::from_runtime_demand over already-built RuntimeOwnerSiloDemandBucket rows (ordinary per-tree admission; uses SimThingId::from_session_raw)
- gpu_to_host_synchronization_readback: absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns. GPU adapter is queried only for the envelope.
- host_conversion_grouping: BTreeMap grouping of scored claims by OwnerChannelScopeKey, matching the live door's claims_by_scope insert
- eml_scoring: AuthoredClearingProgram::score_program().apply_with_params(order_weight, priority) plus the live finite/non-negative/signed-zero canonicalize
- score_sorting_banding: sort by score.total_cmp descending then source_simthing_id; equal score.to_bits() bands
- integer_apportionment: workshop-local restatement of the live largest-remainder + generation-rotated exact-tie loop on the same scored/sorted/banded input; production door remains authority; isolation is proven by matching grant.granted; this is a component of instrument_restatement, not extra end-to-end work beyond that pass
- grant_result_construction: signed remainder enclosing_clear − (grouping+scoring+sorting+apportionment); i64 samples; negative values are retained (D1: no .max(0) clamp); this is a partition of enclosing_clear, not additional end-to-end work
- host_to_gpu_upload: absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns
- n_plus_one_launch_delay: grants-available (production door has returned ConstrainedGrant values) → N+1-ready/launch: construct generation+1 ClearingRemainderAuthority. GPU launch is door-absent 0 ns / 0 dispatches. Schedule-append is NOT in this boundary; the next host clear is a pure function of supplies/claims/program/authority and does not require IntegrationSchedule to have been appended. No overlap with cpu_schedule_replay_recording.
- cpu_schedule_replay_recording: AdmittedSpecializationFlowMarket::record_cleared_grant into a fresh per-tree IntegrationSchedule for every produced ConstrainedGrant with granted>0. Not a prerequisite for n_plus_one_launch_delay.
- lawful_structural_consequence: UnresolvedDemandObservation::from_grant + fund_unresolved_persistence at observed_generation+1; overlays are dropped after mint so the allocator is observed without retaining the population
- instrument_restatement: D2 shape-2 instrument-only: wall time of the workshop-local nested grouping+scoring+sorting+apportionment pass that also publishes those four component legs. Inside the end-to-end envelope.
- neutrality_reclear: D2 shape-2 instrument-only: second uninstrumented clear_constrained_claims_at_generation used only to prove observation neutrality. Inside the end-to-end envelope.
- next_generation_host_reclear: D3 comparator: full clear_constrained_claims_at_generation of the same supplies/claims at generation+1. Not the N+1 launch delay.
- observation_overhead_residual: D6 samplewise residual: residual[i] = end_to_end[i] − Σ(E2E_ACCOUNTED_LEGS[i]). Signed i64; not clamped. Not the difference of medians.

## Disclaimer
Comparator only. Wall-clock values are dated facts about one reproducibility envelope. They are not a go/no-go gate, portable CI threshold, or authority to cancel or narrow Owner-ruled Phase-14 placement.

## D1 signed remainder
D1: grant_result_construction is i64; `.max(0)` removed. Before/after: scale_1000 grant_result_construction prior packet min_ns=0 with an explicit 0 sample after `.max(0)` (samples included 166600, 790200, 41100, 153900, 446300, 157200, 252100, 0, ...); the 0 was a clamped negative remainder of enclosing minus nested grouping+scoring+sorting+apportionment. Corrected samples keep the signed remainder. Serialization boundary: LegSamples.sample_ns/median_ns/p95_ns/min_ns/max_ns are i64.

## D2 envelope shape
shape-2-instrument-legs-inside-e2e

## D3 N+1 boundary
n_plus_one_launch_delay = grants-available → generation+1 ClearingRemainderAuthority construction (GPU launch 0). next_generation_host_reclear = full ordinary-door re-clear at generation+1. cpu_schedule_replay_recording is not inside the N+1 delay (no overlap; next host clear does not require a schedule append).

## D6 samplewise residual
observation_overhead_residual[i] = end_to_end[i] − Σ E2E_ACCOUNTED_LEGS[i] (claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear). Signed i64; not clamped. difference_of_medians_ns is median(e2e) − Σ median(accounted) and is not the residual.

## Workload scale_1000
trees=1 claims_per_tree=1000 claims_total=1000 scopes=1 supplies=1 claimants=1000 granters=1 generations=[10] overlapping=false door=
warm_ups=3 samples=11 setup_allocation_ns=1173800 grants_total=1000 unresolved_total=1997
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=860100 p95_ns=1320000 variance_ns2=46367826000.000 min_ns=750100 max_ns=1320000 mean_ns=949100.000 samples=[906500, 1192800, 1302000, 1320000, 858600, 777400, 767700, 860100, 750100, 894800, 810100]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=4649100 p95_ns=7938300 variance_ns2=1565477328181.818 min_ns=4214300 max_ns=7938300 mean_ns=5433927.273 samples=[6875800, 6110200, 7938300, 6671000, 5096200, 4504800, 4539700, 4566100, 4214300, 4649100, 4607700]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=70300 p95_ns=170400 variance_ns2=1240060545.455 min_ns=65000 max_ns=170400 mean_ns=95663.636 samples=[70300, 131500, 120100, 170400, 113300, 65000, 69400, 70000, 67200, 106900, 68200]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=133500 p95_ns=256800 variance_ns2=2396640909.091 min_ns=115700 max_ns=256800 mean_ns=158990.909 samples=[141600, 173100, 169000, 256800, 245400, 132700, 124800, 123400, 115700, 132900, 133500]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=86300 p95_ns=163800 variance_ns2=798485636.364 min_ns=75600 max_ns=163800 mean_ns=101281.818 samples=[83800, 126100, 125200, 163800, 120400, 78900, 83700, 86300, 75600, 86300, 84000]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=1500 p95_ns=3500 variance_ns2=378545.455 min_ns=1400 max_ns=3500 mean_ns=1836.364 samples=[1900, 2100, 1500, 2200, 3500, 1500, 1500, 1600, 1400, 1500, 1500]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=426100 p95_ns=1050900 variance_ns2=47173824181.818 min_ns=379400 max_ns=1050900 mean_ns=539372.727 samples=[1050900, 607800, 428200, 646500, 794700, 393800, 406400, 426100, 379400, 403400, 395900]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=195200 p95_ns=578100 variance_ns2=71388619636.364 min_ns=-371700 max_ns=578100 mean_ns=147618.182 samples=[-371700, 283700, 578100, 250700, -305400, 170500, 151300, 222700, 178000, 270700, 195200]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=100 variance_ns2=2181.818 min_ns=0 max_ns=100 mean_ns=27.273 samples=[100, 0, 0, 0, 0, 100, 0, 0, 0, 100, 0]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=319800 p95_ns=751200 variance_ns2=21775743636.364 min_ns=294100 max_ns=751200 mean_ns=385981.818 samples=[592200, 305000, 751200, 332800, 294100, 319800, 342200, 309700, 313400, 304700, 380700]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=160100 p95_ns=383600 variance_ns2=7313470909.091 min_ns=146200 max_ns=383600 mean_ns=203309.091 samples=[256600, 383600, 344600, 161000, 152700, 158300, 159600, 146200, 150000, 160100, 163700]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=983300 p95_ns=2115600 variance_ns2=154323828181.818 min_ns=834700 max_ns=2115600 mean_ns=1177127.273 samples=[2115600, 1312600, 1132200, 1498700, 1462100, 884600, 905900, 983300, 834700, 917900, 900800]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=762600 p95_ns=1295700 variance_ns2=40557910181.818 min_ns=708700 max_ns=1295700 mean_ns=842772.727 samples=[767000, 830300, 1295700, 1188400, 727200, 755000, 734200, 729800, 708700, 771600, 762600]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=746500 p95_ns=1883200 variance_ns2=141460321636.364 min_ns=694300 max_ns=1883200 mean_ns=930118.182 samples=[1354200, 738900, 1883200, 1090600, 728800, 759900, 736400, 715400, 694300, 746500, 783100]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=784700 p95_ns=1215500 variance_ns2=27537292181.818 min_ns=695900 max_ns=1215500 mean_ns=849827.273 samples=[813300, 1215500, 1109300, 909100, 759400, 784700, 824300, 751600, 695900, 746500, 738500]
difference_of_medians_ns=746400 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload scale_10000
trees=1 claims_per_tree=10000 claims_total=10000 scopes=1 supplies=1 claimants=10000 granters=1 generations=[10] overlapping=false door=
warm_ups=2 samples=7 setup_allocation_ns=841000 grants_total=10000 unresolved_total=19999
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=22621500 p95_ns=27514600 variance_ns2=72669627749047.609 min_ns=9281100 max_ns=27514600 mean_ns=18773528.571 samples=[24858500, 9955500, 26953800, 10229700, 27514600, 22621500, 9281100]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=75076800 p95_ns=103290300 variance_ns2=298211552595714.312 min_ns=59834600 max_ns=103290300 mean_ns=79466828.571 samples=[95338400, 64644900, 103290300, 59834600, 92176400, 75076800, 65906400]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=857100 p95_ns=3839500 variance_ns2=1247971188095.238 min_ns=824700 max_ns=3839500 mean_ns=1311014.286 samples=[857100, 978200, 995300, 835600, 3839500, 846700, 824700]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=1333400 p95_ns=1662800 variance_ns2=20935045714.286 min_ns=1227000 max_ns=1662800 mean_ns=1395871.429 samples=[1325300, 1409700, 1333400, 1497400, 1315500, 1662800, 1227000]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=860000 p95_ns=925000 variance_ns2=1132089523.810 min_ns=836500 max_ns=925000 mean_ns=873042.857 samples=[852500, 895000, 925000, 842200, 860000, 900100, 836500]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=45600 p95_ns=51500 variance_ns2=11751428.571 min_ns=42400 max_ns=51500 mean_ns=46214.286 samples=[49500, 45600, 42600, 47300, 51500, 42400, 44600]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=6747000 p95_ns=13352600 variance_ns2=13154653062380.953 min_ns=5120500 max_ns=13352600 mean_ns=8150428.571 samples=[5739800, 13352600, 7574400, 5120500, 5252300, 6747000, 13266400]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=13269200 p95_ns=20035300 variance_ns2=124815396342380.953 min_ns=-6093400 max_ns=20035300 mean_ns=8307971.429 samples=[16891400, -5747400, 17078400, 2722300, 20035300, 13269200, -6093400]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=200 variance_ns2=3333.333 min_ns=0 max_ns=200 mean_ns=100.000 samples=[0, 200, 100, 100, 100, 100, 100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=3423200 p95_ns=5633500 variance_ns2=785055160000.000 min_ns=3120100 max_ns=5633500 mean_ns=3816900.000 samples=[3903800, 3423200, 4146600, 3263300, 5633500, 3227800, 3120100]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=1927200 p95_ns=9698300 variance_ns2=8437236183333.333 min_ns=1803100 max_ns=9698300 mean_ns=3205300.000 samples=[1842300, 1818900, 1803100, 2164000, 9698300, 1927200, 3183300]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=15197500 p95_ns=23275800 variance_ns2=20091063849523.812 min_ns=11482000 max_ns=23275800 mean_ns=16517742.857 samples=[15197500, 19841300, 19493100, 11520600, 11482000, 14813900, 23275800]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=9449400 p95_ns=18373800 variance_ns2=11877125558095.236 min_ns=8681900 max_ns=18373800 mean_ns=10618214.286 samples=[9243500, 9449400, 18373800, 9917000, 9719900, 8942000, 8681900]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=9946600 p95_ns=21521000 variance_ns2=30732027804761.906 min_ns=8598700 max_ns=21521000 mean_ns=12849985.714 samples=[21521000, 9859200, 20213200, 8856300, 10954900, 9946600, 8598700]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=12751000 p95_ns=17914700 variance_ns2=9077749802857.143 min_ns=8940700 max_ns=17914700 mean_ns=12374042.857 samples=[17914700, 9319000, 11311300, 13048000, 13333600, 12751000, 8940700]
difference_of_medians_ns=11654200 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload scale_100000
trees=1 claims_per_tree=100000 claims_total=100000 scopes=1 supplies=1 claimants=100000 granters=1 generations=[10] overlapping=false door=
warm_ups=1 samples=5 setup_allocation_ns=8242300 grants_total=100000 unresolved_total=200001
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=152519200 p95_ns=190849700 variance_ns2=486720986677000.000 min_ns=139832600 max_ns=190849700 mean_ns=160860720.000 samples=[139832600, 152519200, 190849700, 176915500, 144186600]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=958258900 p95_ns=1043785300 variance_ns2=3006903313383000.000 min_ns=922304600 max_ns=1043785300 mean_ns=976147360.000 samples=[922304600, 932230400, 1024157600, 1043785300, 958258900]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=9124000 p95_ns=14128700 variance_ns2=5282217847000.000 min_ns=8687200 max_ns=14128700 mean_ns=10054380.000 samples=[14128700, 9494300, 8687200, 8837700, 9124000]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=15099000 p95_ns=22402100 variance_ns2=16297733217000.000 min_ns=12156800 max_ns=22402100 mean_ns=16565180.000 samples=[18767700, 15099000, 22402100, 12156800, 14400300]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=9272400 p95_ns=23352100 variance_ns2=39944015875000.000 min_ns=8688300 max_ns=23352100 mean_ns=12078700.000 samples=[23352100, 9272400, 10004000, 8688300, 9076700]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=463900 p95_ns=490300 variance_ns2=682942000.000 min_ns=427100 max_ns=490300 mean_ns=455880.000 samples=[465800, 427100, 463900, 490300, 432300]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=105327300 p95_ns=125242300 variance_ns2=308387049728000.000 min_ns=80988300 max_ns=125242300 mean_ns=106982760.000 samples=[105327300, 102254700, 125242300, 121101200, 80988300]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=32737400 p95_ns=39289000 variance_ns2=362042463215000.000 min_ns=-8080300 max_ns=39289000 mean_ns=24778200.000 samples=[-8080300, 25466000, 32737400, 34478900, 39289000]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=200 variance_ns2=3000.000 min_ns=100 max_ns=200 mean_ns=140.000 samples=[200, 100, 100, 200, 100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=39355800 p95_ns=48530000 variance_ns2=28564662493000.000 min_ns=37656300 max_ns=48530000 mean_ns=42391040.000 samples=[39355800, 38536900, 47876200, 48530000, 37656300]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=20568400 p95_ns=42878600 variance_ns2=135392131582000.000 min_ns=17326900 max_ns=42878600 mean_ns=27008580.000 samples=[35922800, 17326900, 42878600, 20568400, 18346200]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=217370100 p95_ns=246803700 variance_ns2=391032132737000.000 min_ns=198669800 max_ns=246803700 mean_ns=224013580.000 samples=[246803700, 217370100, 241182000, 216042300, 198669800]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=183998700 p95_ns=204557500 variance_ns2=369241410088000.000 min_ns=151986600 max_ns=204557500 mean_ns=179669840.000 samples=[151986600, 204557500, 183998700, 184873700, 172932700]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=176006300 p95_ns=253871100 variance_ns2=1841062479583000.000 min_ns=136880200 max_ns=253871100 mean_ns=184547240.000 samples=[176006300, 170009800, 136880200, 185968800, 253871100]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=123472100 p95_ns=202048700 variance_ns2=1406829937738000.000 min_ns=118267900 max_ns=202048700 mean_ns=147601840.000 samples=[118267900, 122415600, 171804900, 202048700, 123472100]
difference_of_medians_ns=159316300 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload scale_1000000
trees=1 claims_per_tree=1000000 claims_total=1000000 scopes=1 supplies=1 claimants=1000000 granters=1 generations=[10] overlapping=false door=
warm_ups=1 samples=3 setup_allocation_ns=90088700 grants_total=1000000 unresolved_total=2000000
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=1947023600 p95_ns=2000115000 variance_ns2=21315852932573332.000 min_ns=1724905800 max_ns=2000115000 mean_ns=1890681466.667 samples=[2000115000, 1724905800, 1947023600]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=10988599800 p95_ns=11936817100 variance_ns2=338614775067843328.000 min_ns=10878322200 max_ns=11936817100 mean_ns=11267913033.333 samples=[11936817100, 10988599800, 10878322200]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=123306400 p95_ns=183170700 variance_ns2=1724475510823333.500 min_ns=103382500 max_ns=183170700 mean_ns=136619866.667 samples=[183170700, 123306400, 103382500]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=186473800 p95_ns=238333700 variance_ns2=2172622179543333.500 min_ns=145315900 max_ns=238333700 mean_ns=190041133.333 samples=[238333700, 186473800, 145315900]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=130102900 p95_ns=152301400 variance_ns2=395133104730000.000 min_ns=112639600 max_ns=152301400 mean_ns=131681300.000 samples=[152301400, 112639600, 130102900]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=5209100 p95_ns=8473400 variance_ns2=4104452730000.000 min_ns=4762400 max_ns=8473400 mean_ns=6148300.000 samples=[8473400, 5209100, 4762400]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=1282906600 p95_ns=1635142600 variance_ns2=43555331327520000.000 min_ns=1265083000 max_ns=1635142600 mean_ns=1394377400.000 samples=[1635142600, 1282906600, 1265083000]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=137676700 p95_ns=401759400 variance_ns2=48210699600563336.000 min_ns=-34136100 max_ns=401759400 mean_ns=168433333.333 samples=[-34136100, 137676700, 401759400]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=300 variance_ns2=13333.333 min_ns=100 max_ns=300 mean_ns=166.667 samples=[100, 300, 100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=432774300 p95_ns=448863800 variance_ns2=1400366992630000.000 min_ns=377518600 max_ns=448863800 mean_ns=419718900.000 samples=[377518600, 432774300, 448863800]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=198161100 p95_ns=276813400 variance_ns2=2578217406863333.500 min_ns=181854400 max_ns=276813400 mean_ns=218942966.667 samples=[276813400, 181854400, 198161100]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=2584659600 p95_ns=3393887100 variance_ns2=232396147500363328.000 min_ns=2535344300 max_ns=3393887100 mean_ns=2837963666.667 samples=[3393887100, 2584659600, 2535344300]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=1794552800 p95_ns=1901140000 variance_ns2=4445642627680000.000 min_ns=1778446800 max_ns=1901140000 mean_ns=1824713200.000 samples=[1901140000, 1794552800, 1778446800]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=1935245400 p95_ns=2221023000 variance_ns2=28581073400243332.000 min_ns=1921636300 max_ns=2221023000 mean_ns=2025968233.333 samples=[1935245400, 2221023000, 1921636300]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=1925523200 p95_ns=1945463700 variance_ns2=1576445515803333.500 min_ns=1868926800 max_ns=1945463700 mean_ns=1913304566.667 samples=[1868926800, 1925523200, 1945463700]
difference_of_medians_ns=1972876500 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload one_large_tree
trees=1 claims_per_tree=100000 claims_total=100000 scopes=1 supplies=1 claimants=100000 granters=1 generations=[10] overlapping=false door=
warm_ups=1 samples=5 setup_allocation_ns=8259500 grants_total=100000 unresolved_total=200001
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=146729600 p95_ns=189221100 variance_ns2=1073551654315000.000 min_ns=115277800 max_ns=189221100 mean_ns=154168800.000 samples=[133030000, 189221100, 115277800, 146729600, 186585500]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=821994100 p95_ns=967635800 variance_ns2=10383069336283000.000 min_ns=720945100 max_ns=967635800 mean_ns=835493060.000 samples=[720945100, 967635800, 760101000, 821994100, 906789300]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=8241500 p95_ns=10840800 variance_ns2=1481496735000.000 min_ns=7821800 max_ns=10840800 mean_ns=8718600.000 samples=[10840800, 8579400, 7821800, 8241500, 8109500]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=21934300 p95_ns=49683300 variance_ns2=227343090317000.000 min_ns=12713600 max_ns=49683300 mean_ns=25602380.000 samples=[21934300, 49683300, 14042900, 12713600, 29637800]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=9219300 p95_ns=33922800 variance_ns2=124327063635000.000 min_ns=7720000 max_ns=33922800 mean_ns=15216700.000 samples=[17173500, 33922800, 8047900, 9219300, 7720000]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=558200 p95_ns=1244200 variance_ns2=140861087000.000 min_ns=409100 max_ns=1244200 mean_ns=766280.000 samples=[526800, 1244200, 558200, 409100, 1093100]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=75633800 p95_ns=114943100 variance_ns2=441448741167000.000 min_ns=64378700 max_ns=114943100 mean_ns=83873220.000 samples=[75633800, 114943100, 64378700, 69033900, 95376600]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=28250100 p95_ns=55353700 variance_ns2=737842779327000.000 min_ns=-10572300 max_ns=55353700 mean_ns=28710220.000 samples=[17761600, -10572300, 28250100, 55353700, 52758000]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=300 variance_ns2=8000.000 min_ns=100 max_ns=300 mean_ns=160.000 samples=[300, 200, 100, 100, 100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=39714900 p95_ns=57090300 variance_ns2=89383230948000.000 min_ns=33535600 max_ns=57090300 mean_ns=41618440.000 samples=[34743300, 43008100, 33535600, 39714900, 57090300]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=17377100 p95_ns=26523500 variance_ns2=17564739212000.000 min_ns=16319300 max_ns=26523500 mean_ns=19624220.000 samples=[17250700, 16319300, 17377100, 20650500, 26523500]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=193897900 p95_ns=255500400 variance_ns2=980817353093000.000 min_ns=172667400 max_ns=255500400 mean_ns=201928260.000 samples=[172667400, 255500400, 193592300, 193983300, 193897900]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=154576400 p95_ns=191846300 variance_ns2=1021730059767000.000 min_ns=113272800 max_ns=191846300 mean_ns=154579380.000 samples=[113272800, 191846300, 134423100, 154576400, 178778300]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=120756900 p95_ns=145775600 variance_ns2=270773089268000.000 min_ns=106238100 max_ns=145775600 mean_ns=123386440.000 samples=[110303100, 133858500, 120756900, 145775600, 106238100]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=129302500 p95_ns=149566100 variance_ns2=184979655298000.000 min_ns=112322200 max_ns=149566100 mean_ns=131468760.000 samples=[128836700, 129302500, 137316300, 112322200, 149566100]
difference_of_medians_ns=140699700 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload many_independent_small_trees
trees=100 claims_per_tree=100 claims_total=10000 scopes=1 supplies=1 claimants=100 granters=1 generations=[10] overlapping=false door=
warm_ups=1 samples=5 setup_allocation_ns=4367400 grants_total=10000 unresolved_total=20200
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=8479500 p95_ns=10995400 variance_ns2=3963176167000.000 min_ns=5414100 max_ns=10995400 mean_ns=8379020.000 samples=[8206700, 8799400, 5414100, 8479500, 10995400]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=45622500 p95_ns=64783200 variance_ns2=144926607958000.000 min_ns=31202500 max_ns=64783200 mean_ns=46104560.000 samples=[43215500, 45699100, 31202500, 45622500, 64783200]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=661200 p95_ns=701500 variance_ns2=1190917000.000 min_ns=608000 max_ns=701500 mean_ns=654920.000 samples=[701500, 664900, 608000, 639000, 661200]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=1337500 p95_ns=2300100 variance_ns2=255013942000.000 min_ns=944000 max_ns=2300100 mean_ns=1483280.000 samples=[1290700, 1544100, 944000, 1337500, 2300100]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=875800 p95_ns=1389500 variance_ns2=80050663000.000 min_ns=617500 max_ns=1389500 mean_ns=948060.000 samples=[875800, 999800, 617500, 857700, 1389500]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=44400 p95_ns=65100 variance_ns2=219600000.000 min_ns=25200 max_ns=65100 mean_ns=42900.000 samples=[34800, 45000, 25200, 44400, 65100]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=3160800 p95_ns=5132400 variance_ns2=1139022528000.000 min_ns=2223200 max_ns=5132400 mean_ns=3460440.000 samples=[3137000, 3648800, 2223200, 3160800, 5132400]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=2561700 p95_ns=3079100 variance_ns2=353819653000.000 min_ns=1604200 max_ns=3079100 mean_ns=2444340.000 samples=[2868400, 2561700, 1604200, 3079100, 2108300]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=3700 p95_ns=4300 variance_ns2=338000.000 min_ns=3000 max_ns=4300 mean_ns=3640.000 samples=[3700, 3100, 3000, 4300, 4100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=2791800 p95_ns=4621400 variance_ns2=905675432000.000 min_ns=2047700 max_ns=4621400 mean_ns=3037180.000 samples=[2942000, 2791800, 2047700, 2783000, 4621400]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=2313500 p95_ns=3897900 variance_ns2=651837958000.000 min_ns=1736500 max_ns=3897900 mean_ns=2539960.000 samples=[2296200, 2455700, 1736500, 2313500, 3897900]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=8211100 p95_ns=12222200 variance_ns2=5710925553000.000 min_ns=5641700 max_ns=12222200 mean_ns=8516560.000 samples=[7699000, 8808800, 5641700, 8211100, 12222200]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=7642900 p95_ns=11437900 variance_ns2=4615611302000.000 min_ns=5562000 max_ns=11437900 mean_ns=8105920.000 samples=[7385900, 7642900, 5562000, 8500900, 11437900]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=7712200 p95_ns=11207700 variance_ns2=4104522763000.000 min_ns=5591900 max_ns=11207700 mean_ns=7979060.000 samples=[7519800, 7712200, 5591900, 7863700, 11207700]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=6820300 p95_ns=9735400 variance_ns2=3386111825000.000 min_ns=4597600 max_ns=9735400 mean_ns=6888300.000 samples=[6460700, 6820300, 4597600, 6827500, 9735400]
difference_of_medians_ns=7806600 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload divergent_generation_trees
trees=4 claims_per_tree=1000 claims_total=4000 scopes=1 supplies=1 claimants=1000 granters=1 generations=[1, 10, 100, 1000] overlapping=false door=
warm_ups=1 samples=5 setup_allocation_ns=418700 grants_total=4000 unresolved_total=7988
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=3136500 p95_ns=5021700 variance_ns2=1367991758000.000 min_ns=2679000 max_ns=5021700 mean_ns=3695560.000 samples=[2739500, 4901100, 2679000, 5021700, 3136500]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=24583300 p95_ns=28610600 variance_ns2=39306285547000.000 min_ns=15371700 max_ns=28610600 mean_ns=22060120.000 samples=[15371700, 28610600, 15373400, 24583300, 26361600]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=304800 p95_ns=325900 variance_ns2=1289847000.000 min_ns=247600 max_ns=325900 mean_ns=290820.000 samples=[325900, 247600, 318300, 304800, 257500]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=395600 p95_ns=987000 variance_ns2=71291108000.000 min_ns=364800 max_ns=987000 mean_ns=511440.000 samples=[395600, 987000, 378600, 364800, 431200]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=261700 p95_ns=439100 variance_ns2=6602197000.000 min_ns=238500 max_ns=439100 mean_ns=297020.000 samples=[259000, 439100, 261700, 238500, 286800]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=7400 p95_ns=24200 variance_ns2=55023000.000 min_ns=7300 max_ns=24200 mean_ns=10960.000 samples=[7400, 24200, 7300, 7400, 8500]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=1752700 p95_ns=2499300 variance_ns2=177191523000.000 min_ns=1459600 max_ns=2499300 mean_ns=1829860.000 samples=[1503300, 2499300, 1459600, 1934400, 1752700]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=657300 p95_ns=2476600 variance_ns2=663523697000.000 min_ns=571800 max_ns=2476600 mean_ns=1046280.000 samples=[574200, 951500, 571800, 2476600, 657300]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=400 variance_ns2=20000.000 min_ns=100 max_ns=400 mean_ns=200.000 samples=[100, 100, 100, 400, 300]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=926600 p95_ns=1737500 variance_ns2=155090908000.000 min_ns=859800 max_ns=1737500 mean_ns=1161540.000 samples=[859800, 1737500, 926600, 875800, 1408000]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=619900 p95_ns=1115400 variance_ns2=50331847000.000 min_ns=596700 max_ns=1115400 mean_ns=754120.000 samples=[606200, 1115400, 619900, 596700, 832400]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=4090100 p95_ns=5755800 variance_ns2=1045377595000.000 min_ns=3232600 max_ns=5755800 mean_ns=4149600.000 samples=[3306100, 5755800, 3232600, 4363400, 4090100]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=4383600 p95_ns=6539300 variance_ns2=2668162423000.000 min_ns=2585800 max_ns=6539300 mean_ns=4149640.000 samples=[2645000, 4594500, 2585800, 4383600, 6539300]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=2960400 p95_ns=7288500 variance_ns2=4133285397000.000 min_ns=2545000 max_ns=7288500 mean_ns=4057720.000 samples=[2545000, 4845100, 2649600, 2960400, 7288500]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=2809000 p95_ns=6076500 variance_ns2=3239209582000.000 min_ns=2344100 max_ns=6076500 mean_ns=3800920.000 samples=[2344100, 5413500, 2361500, 6076500, 2809000]
difference_of_medians_ns=8161300 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Workload overlapping_local_ids
trees=2 claims_per_tree=1000 claims_total=2000 scopes=1 supplies=1 claimants=1000 granters=1 generations=[10, 10] overlapping=true door=ConstrainedClaim::from_runtime_demand -> SimThingId::from_session_raw (ordinary production admission; same raw {1..=N} including 7 under distinct owner_ref/scope_id/granter)
warm_ups=1 samples=7 setup_allocation_ns=137400 grants_total=2000 unresolved_total=4000
isolation_matches_production=true neutrality_clears_identical=true
leg enclosing_clear isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=2098700 p95_ns=6654300 variance_ns2=3800325480000.000 min_ns=1323600 max_ns=6654300 mean_ns=2939600.000 samples=[1589400, 3908900, 2098700, 3627500, 1323600, 1374800, 6654300]
leg end_to_end isolation=production-door observation bytes_read_back=0 bytes_uploaded=0 median_ns=10791400 p95_ns=17172700 variance_ns2=11916053713333.334 min_ns=7621900 max_ns=17172700 mean_ns=11653900.000 samples=[8478300, 17172700, 10791400, 12430000, 7621900, 10115000, 14968000]
leg claim_production_completion isolation=ConstrainedClaim::from_runtime_demand (ordinary per-tree admission) bytes_read_back=0 bytes_uploaded=0 median_ns=115900 p95_ns=476700 variance_ns2=18163380000.000 min_ns=112700 max_ns=476700 mean_ns=174600.000 samples=[113600, 112700, 120200, 476700, 115900, 113500, 169600]
leg gpu_to_host_synchronization_readback isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0]
leg host_conversion_grouping isolation=BTreeMap group by OwnerChannelScopeKey bytes_read_back=0 bytes_uploaded=0 median_ns=212300 p95_ns=468500 variance_ns2=15771039047.619 min_ns=179500 max_ns=468500 mean_ns=282628.571 samples=[179500, 453000, 180900, 468500, 211800, 272400, 212300]
leg eml_scoring isolation=TransformOp::apply_with_params via AuthoredClearingProgram::score_program bytes_read_back=0 bytes_uploaded=0 median_ns=130700 p95_ns=308200 variance_ns2=7385702857.143 min_ns=121100 max_ns=308200 mean_ns=180942.857 samples=[125900, 308200, 126000, 303700, 130700, 121100, 151000]
leg score_sorting_banding isolation=score.total_cmp desc then source id; equal to_bits bands bytes_read_back=0 bytes_uploaded=0 median_ns=4300 p95_ns=7900 variance_ns2=3549047.619 min_ns=3500 max_ns=7900 mean_ns=5128.571 samples=[3500, 7800, 3800, 7900, 4300, 4300, 4300]
leg integer_apportionment isolation=workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement bytes_read_back=0 bytes_uploaded=0 median_ns=748200 p95_ns=3284000 variance_ns2=909041549047.619 min_ns=681400 max_ns=3284000 mean_ns=1276071.429 samples=[681400, 3284000, 712800, 1637100, 700800, 748200, 1168200]
leg grant_result_construction isolation=signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp bytes_read_back=0 bytes_uploaded=0 median_ns=599100 p95_ns=5118500 variance_ns2=3222471205714.286 min_ns=-144100 max_ns=5118500 mean_ns=1194828.571 samples=[599100, -144100, 1075200, 1210300, 276000, 228800, 5118500]
leg host_to_gpu_upload isolation=door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit) bytes_read_back=0 bytes_uploaded=0 median_ns=0 p95_ns=0 variance_ns2=0.000 min_ns=0 max_ns=0 mean_ns=0.000 samples=[0, 0, 0, 0, 0, 0, 0]
leg n_plus_one_launch_delay isolation=grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary bytes_read_back=0 bytes_uploaded=0 median_ns=100 p95_ns=200 variance_ns2=5714.286 min_ns=0 max_ns=200 mean_ns=71.429 samples=[200, 100, 0, 0, 0, 100, 100]
leg cpu_schedule_replay_recording isolation=AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay bytes_read_back=0 bytes_uploaded=0 median_ns=524600 p95_ns=1018200 variance_ns2=38982062857.143 min_ns=458900 max_ns=1018200 mean_ns=581742.857 samples=[473300, 546700, 1018200, 458900, 472000, 578500, 524600]
leg lawful_structural_consequence isolation=UnresolvedDemandObservation::from_grant + fund_unresolved_persistence bytes_read_back=0 bytes_uploaded=0 median_ns=344200 p95_ns=740000 variance_ns2=40513824761.905 min_ns=295200 max_ns=740000 mean_ns=447214.286 samples=[295200, 737500, 740000, 316300, 311200, 386100, 344200]
leg instrument_restatement isolation=D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=1715100 p95_ns=5345500 variance_ns2=2069416752857.143 min_ns=1584300 max_ns=5345500 mean_ns=2596442.857 samples=[1715100, 5345500, 1584300, 3730400, 1610100, 1710100, 2479600]
leg neutrality_reclear isolation=D2 shape-2: second uninstrumented production-door clear inside e2e bytes_read_back=0 bytes_uploaded=0 median_ns=1579800 p95_ns=2467200 variance_ns2=202538874761.905 min_ns=1331200 max_ns=2467200 mean_ns=1705285.714 samples=[1340700, 2467200, 1579800, 1331200, 1339500, 2166000, 1712600]
leg next_generation_host_reclear isolation=full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay bytes_read_back=0 bytes_uploaded=0 median_ns=1508300 p95_ns=1832200 variance_ns2=51237802857.143 min_ns=1258700 max_ns=1832200 mean_ns=1511942.857 samples=[1751800, 1258700, 1508300, 1315300, 1319300, 1832200, 1598000]
leg observation_overhead_residual isolation=D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians bytes_read_back=0 bytes_uploaded=0 median_ns=1485000 p95_ns=2795400 variance_ns2=393042206666.667 min_ns=1130300 max_ns=2795400 mean_ns=1697000.000 samples=[1199000, 2795400, 2141900, 1173700, 1130300, 1953700, 1485000]
difference_of_medians_ns=2904700 (derived figure; not the residual)
reconciliation: D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.
overlapping_raw=7 contexts=["overlap-alpha owner_ref=overlap-alpha granter_raw=1001 scope_boundary=1011", "overlap-beta owner_ref=overlap-beta granter_raw=2001 scope_boundary=2011"]
coupling: simthing_allocator=SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand. overlay_allocator=OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0. host_lock=none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority. shared_generation=ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them. shared_schedule=IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule. all_tree_sync=none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.

## Lossless JSON follows
{
  "envelope": {
    "tested_commit": "0b7453fa47849848200a3262f7a3581466434165",
    "utc_date": "2026-09-09T11:16:42Z",
    "cpu": "Intel64 Family 6 Model 183 Stepping 1, GenuineIntel",
    "gpu": "NVIDIA GeForce RTX 4080 Laptop GPU vendor=0x10de device=0x27a0 type=DiscreteGpu",
    "adapter_backend": "Vulkan",
    "driver": "NVIDIA 595.79",
    "os": "windows-x86_64",
    "compiler_toolchain": "rustc 1.95.0 (59807616e 2026-04-14)",
    "profile": "cargo-test (optimized+debuginfo)",
    "deterministic_seed": 81921,
    "exact_command": "cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture"
  },
  "door_census": [
    {
      "symbol": "clear_constrained_claims_at_generation",
      "path": "crates/simthing-spec/src/spec/constrained_clearing.rs",
      "line": 274,
      "generation_authority_form": "caller-supplied ClearingRemainderAuthority { granter, generation }",
      "reexports": "simthing-spec lib.rs + spec/mod.rs; simthing-embedder run::cpu_filter_oracle",
      "callers": "production: simthing-driver/src/growth_entitlement.rs; wrapper: clear_reduced_owner_channels_at_generation; tests: contention_arena_executed_0, clearing_weight_span_unification_0, clearing_weight_deformation_lifecycle_0, stemthing_b_flow_market_germ_0, stemthing_b_vram_residency_0, grant_disbursement_lane_0, unified_facility_convergence_witness_0, protected_representative_restore, vendor_door_triad_surface_0",
      "ordinary_or_oracle_posture": "ordinary production CPU-host clearing door",
      "disposition_14_6": "narrow behind CpuVendorizedOracle; production migrates to the 14.2+ resident germ"
    },
    {
      "symbol": "clear_reduced_owner_channels",
      "path": "crates/simthing-spec/src/spec/constrained_clearing.rs",
      "line": 441,
      "generation_authority_form": "generationless compatibility: granter=SimThingId::from_session_raw(0), generation=GenerationStamp::new(0)",
      "reexports": "simthing-spec lib.rs + spec/mod.rs",
      "callers": "test: simthing-driver/tests/contention_arena_executed_0.rs (priority + price cases); no production caller",
      "ordinary_or_oracle_posture": "generationless compatibility shim / test oracle",
      "disposition_14_6": "DELETE"
    },
    {
      "symbol": "clear_reduced_owner_channels_at_generation",
      "path": "crates/simthing-spec/src/spec/constrained_clearing.rs",
      "line": 458,
      "generation_authority_form": "caller-supplied ClearingRemainderAuthority; converts reduce-up buckets through ConstrainedClaim::from_runtime_demand then the ordinary door",
      "reexports": "simthing-spec lib.rs + spec/mod.rs",
      "callers": "wrappers: clear_reduced_owner_channels (generationless), clear_stamped_owner_channels; no direct production caller",
      "ordinary_or_oracle_posture": "conversion wrapper over the ordinary at-generation door",
      "disposition_14_6": "narrow behind CpuVendorizedOracle (or delete once callers are gone)"
    },
    {
      "symbol": "clear_stamped_owner_channels",
      "path": "crates/simthing-spec/src/spec/constrained_clearing.rs",
      "line": 509,
      "generation_authority_form": "generation taken from StampedReduceUpProduct; granter supplied by caller",
      "reexports": "simthing-spec lib.rs + spec/mod.rs",
      "callers": "test/germ: stemthing_b_flow_market_germ_0.rs; no other production caller on this base",
      "ordinary_or_oracle_posture": "canonical stamped-RF market binding over the ordinary door",
      "disposition_14_6": "narrow behind CpuVendorizedOracle"
    },
    {
      "symbol": "produce_runtime_rf_next_generation_demands",
      "path": "crates/simthing-spec/src/spec/runtime_rf_tick.rs",
      "line": 149,
      "generation_authority_form": "RuntimeRfDemandGenerationAuthority once-mint over caller ClearingRemainderAuthority; performs the generation-N clear inside the door",
      "reexports": "simthing-spec lib.rs + spec/mod.rs; driver wrapper produce_runtime_rf_next_generation_demands_for_tick",
      "callers": "production: simthing-driver runtime_rf_tick_compile; witness: resident_clearing_score_and_bands_0",
      "ordinary_or_oracle_posture": "ordinary production Current->Next demand door (14.3 row-11 substrate recurrence, DA ruling 5488315659)",
      "disposition_14_6": "narrow behind CpuVendorizedOracle; recurrence migrates with the resident germ at cutover"
    }
  ],
  "path_diagram": "ordinary generation critical path (CPU-host clearing door; GPU legs door-absent):\n  RuntimeOwnerSiloDemandBucket (per-tree admission)\n    -> ConstrainedClaim::from_runtime_demand  [claim_production_completion]\n    -> (no GPU map/readback in this door)     [gpu_to_host_synchronization_readback = 0 bytes]\n    -> group by OwnerChannelScopeKey          [host_conversion_grouping]\n    -> TransformOp::apply_with_params         [eml_scoring]\n    -> sort score-bits then id; equal-bit bands [score_sorting_banding]\n    -> largest remainder + generation-rotated ties [integer_apportionment]\n    -> ConstrainedGrant::from_clearance + ConstrainedClearingResult [grant_result_construction]\n  enclosing production authority: clear_constrained_claims_at_generation\n    -> (no GPU write_buffer/upload in this door) [host_to_gpu_upload = 0 bytes]\n    -> grants available\n    -> construct generation+1 authority        [n_plus_one_launch_delay; GPU launch = 0]\n    -> (optional comparator) full gen+1 clear  [next_generation_host_reclear]\n    -> record_cleared_grant -> IntegrationSchedule [cpu_schedule_replay_recording; not in N+1 delay]\n    -> fund_unresolved_persistence            [lawful_structural_consequence]\n  instrument (D2 shape 2; inside e2e, named, not residual):\n    nested restatement pass                   [instrument_restatement = grouping+scoring+sorting+apportionment]\n    second uninstrumented production clear    [neutrality_reclear]\n  D6 residual[i] = end_to_end[i] - Σ accounted_leg[i]  [observation_overhead_residual]\n  difference_of_medians = median(e2e) - Σ median(accounted)  [derived figure, not residual]",
  "leg_definitions": [
    {
      "name": "claim_production_completion",
      "boundary": "ConstrainedClaim::from_runtime_demand over already-built RuntimeOwnerSiloDemandBucket rows (ordinary per-tree admission; uses SimThingId::from_session_raw)"
    },
    {
      "name": "gpu_to_host_synchronization_readback",
      "boundary": "absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns. GPU adapter is queried only for the envelope."
    },
    {
      "name": "host_conversion_grouping",
      "boundary": "BTreeMap grouping of scored claims by OwnerChannelScopeKey, matching the live door's claims_by_scope insert"
    },
    {
      "name": "eml_scoring",
      "boundary": "AuthoredClearingProgram::score_program().apply_with_params(order_weight, priority) plus the live finite/non-negative/signed-zero canonicalize"
    },
    {
      "name": "score_sorting_banding",
      "boundary": "sort by score.total_cmp descending then source_simthing_id; equal score.to_bits() bands"
    },
    {
      "name": "integer_apportionment",
      "boundary": "workshop-local restatement of the live largest-remainder + generation-rotated exact-tie loop on the same scored/sorted/banded input; production door remains authority; isolation is proven by matching grant.granted; this is a component of instrument_restatement, not extra end-to-end work beyond that pass"
    },
    {
      "name": "grant_result_construction",
      "boundary": "signed remainder enclosing_clear − (grouping+scoring+sorting+apportionment); i64 samples; negative values are retained (D1: no .max(0) clamp); this is a partition of enclosing_clear, not additional end-to-end work"
    },
    {
      "name": "host_to_gpu_upload",
      "boundary": "absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns"
    },
    {
      "name": "n_plus_one_launch_delay",
      "boundary": "grants-available (production door has returned ConstrainedGrant values) → N+1-ready/launch: construct generation+1 ClearingRemainderAuthority. GPU launch is door-absent 0 ns / 0 dispatches. Schedule-append is NOT in this boundary; the next host clear is a pure function of supplies/claims/program/authority and does not require IntegrationSchedule to have been appended. No overlap with cpu_schedule_replay_recording."
    },
    {
      "name": "cpu_schedule_replay_recording",
      "boundary": "AdmittedSpecializationFlowMarket::record_cleared_grant into a fresh per-tree IntegrationSchedule for every produced ConstrainedGrant with granted>0. Not a prerequisite for n_plus_one_launch_delay."
    },
    {
      "name": "lawful_structural_consequence",
      "boundary": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence at observed_generation+1; overlays are dropped after mint so the allocator is observed without retaining the population"
    },
    {
      "name": "instrument_restatement",
      "boundary": "D2 shape-2 instrument-only: wall time of the workshop-local nested grouping+scoring+sorting+apportionment pass that also publishes those four component legs. Inside the end-to-end envelope."
    },
    {
      "name": "neutrality_reclear",
      "boundary": "D2 shape-2 instrument-only: second uninstrumented clear_constrained_claims_at_generation used only to prove observation neutrality. Inside the end-to-end envelope."
    },
    {
      "name": "next_generation_host_reclear",
      "boundary": "D3 comparator: full clear_constrained_claims_at_generation of the same supplies/claims at generation+1. Not the N+1 launch delay."
    },
    {
      "name": "observation_overhead_residual",
      "boundary": "D6 samplewise residual: residual[i] = end_to_end[i] − Σ(E2E_ACCOUNTED_LEGS[i]). Signed i64; not clamped. Not the difference of medians."
    }
  ],
  "workloads": [
    {
      "cardinalities": {
        "name": "scale_1000",
        "tree_count": 1,
        "claims_per_tree": 1000,
        "claims_total": 1000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 1000,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 3,
      "sample_count": 11,
      "setup_allocation_ns": 1173800,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            70300,
            131500,
            120100,
            170400,
            113300,
            65000,
            69400,
            70000,
            67200,
            106900,
            68200
          ],
          "median_ns": 70300,
          "p95_ns": 170400,
          "variance_ns2": 1240060545.4545455,
          "min_ns": 65000,
          "max_ns": 170400,
          "mean_ns": 95663.63636363637
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            141600,
            173100,
            169000,
            256800,
            245400,
            132700,
            124800,
            123400,
            115700,
            132900,
            133500
          ],
          "median_ns": 133500,
          "p95_ns": 256800,
          "variance_ns2": 2396640909.090909,
          "min_ns": 115700,
          "max_ns": 256800,
          "mean_ns": 158990.9090909091
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            83800,
            126100,
            125200,
            163800,
            120400,
            78900,
            83700,
            86300,
            75600,
            86300,
            84000
          ],
          "median_ns": 86300,
          "p95_ns": 163800,
          "variance_ns2": 798485636.3636364,
          "min_ns": 75600,
          "max_ns": 163800,
          "mean_ns": 101281.81818181818
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1900,
            2100,
            1500,
            2200,
            3500,
            1500,
            1500,
            1600,
            1400,
            1500,
            1500
          ],
          "median_ns": 1500,
          "p95_ns": 3500,
          "variance_ns2": 378545.4545454545,
          "min_ns": 1400,
          "max_ns": 3500,
          "mean_ns": 1836.3636363636363
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1050900,
            607800,
            428200,
            646500,
            794700,
            393800,
            406400,
            426100,
            379400,
            403400,
            395900
          ],
          "median_ns": 426100,
          "p95_ns": 1050900,
          "variance_ns2": 47173824181.81819,
          "min_ns": 379400,
          "max_ns": 1050900,
          "mean_ns": 539372.7272727273
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            -371700,
            283700,
            578100,
            250700,
            -305400,
            170500,
            151300,
            222700,
            178000,
            270700,
            195200
          ],
          "median_ns": 195200,
          "p95_ns": 578100,
          "variance_ns2": 71388619636.36365,
          "min_ns": -371700,
          "max_ns": 578100,
          "mean_ns": 147618.18181818182
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            100,
            0,
            0,
            0,
            0,
            100,
            0,
            0,
            0,
            100,
            0
          ],
          "median_ns": 0,
          "p95_ns": 100,
          "variance_ns2": 2181.818181818181,
          "min_ns": 0,
          "max_ns": 100,
          "mean_ns": 27.272727272727273
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            592200,
            305000,
            751200,
            332800,
            294100,
            319800,
            342200,
            309700,
            313400,
            304700,
            380700
          ],
          "median_ns": 319800,
          "p95_ns": 751200,
          "variance_ns2": 21775743636.363636,
          "min_ns": 294100,
          "max_ns": 751200,
          "mean_ns": 385981.8181818182
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            256600,
            383600,
            344600,
            161000,
            152700,
            158300,
            159600,
            146200,
            150000,
            160100,
            163700
          ],
          "median_ns": 160100,
          "p95_ns": 383600,
          "variance_ns2": 7313470909.090909,
          "min_ns": 146200,
          "max_ns": 383600,
          "mean_ns": 203309.0909090909
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2115600,
            1312600,
            1132200,
            1498700,
            1462100,
            884600,
            905900,
            983300,
            834700,
            917900,
            900800
          ],
          "median_ns": 983300,
          "p95_ns": 2115600,
          "variance_ns2": 154323828181.81818,
          "min_ns": 834700,
          "max_ns": 2115600,
          "mean_ns": 1177127.2727272727
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            767000,
            830300,
            1295700,
            1188400,
            727200,
            755000,
            734200,
            729800,
            708700,
            771600,
            762600
          ],
          "median_ns": 762600,
          "p95_ns": 1295700,
          "variance_ns2": 40557910181.818184,
          "min_ns": 708700,
          "max_ns": 1295700,
          "mean_ns": 842772.7272727273
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1354200,
            738900,
            1883200,
            1090600,
            728800,
            759900,
            736400,
            715400,
            694300,
            746500,
            783100
          ],
          "median_ns": 746500,
          "p95_ns": 1883200,
          "variance_ns2": 141460321636.36362,
          "min_ns": 694300,
          "max_ns": 1883200,
          "mean_ns": 930118.1818181818
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          906500,
          1192800,
          1302000,
          1320000,
          858600,
          777400,
          767700,
          860100,
          750100,
          894800,
          810100
        ],
        "median_ns": 860100,
        "p95_ns": 1320000,
        "variance_ns2": 46367826000.0,
        "min_ns": 750100,
        "max_ns": 1320000,
        "mean_ns": 949100.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          6875800,
          6110200,
          7938300,
          6671000,
          5096200,
          4504800,
          4539700,
          4566100,
          4214300,
          4649100,
          4607700
        ],
        "median_ns": 4649100,
        "p95_ns": 7938300,
        "variance_ns2": 1565477328181.818,
        "min_ns": 4214300,
        "max_ns": 7938300,
        "mean_ns": 5433927.2727272725
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          813300,
          1215500,
          1109300,
          909100,
          759400,
          784700,
          824300,
          751600,
          695900,
          746500,
          738500
        ],
        "median_ns": 784700,
        "p95_ns": 1215500,
        "variance_ns2": 27537292181.818184,
        "min_ns": 695900,
        "max_ns": 1215500,
        "mean_ns": 849827.2727272727
      },
      "difference_of_medians_ns": 746400,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 1000,
      "unresolved_total": 1997,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "scale_10000",
        "tree_count": 1,
        "claims_per_tree": 10000,
        "claims_total": 10000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 10000,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 2,
      "sample_count": 7,
      "setup_allocation_ns": 841000,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            857100,
            978200,
            995300,
            835600,
            3839500,
            846700,
            824700
          ],
          "median_ns": 857100,
          "p95_ns": 3839500,
          "variance_ns2": 1247971188095.2383,
          "min_ns": 824700,
          "max_ns": 3839500,
          "mean_ns": 1311014.2857142857
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1325300,
            1409700,
            1333400,
            1497400,
            1315500,
            1662800,
            1227000
          ],
          "median_ns": 1333400,
          "p95_ns": 1662800,
          "variance_ns2": 20935045714.28571,
          "min_ns": 1227000,
          "max_ns": 1662800,
          "mean_ns": 1395871.4285714286
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            852500,
            895000,
            925000,
            842200,
            860000,
            900100,
            836500
          ],
          "median_ns": 860000,
          "p95_ns": 925000,
          "variance_ns2": 1132089523.8095238,
          "min_ns": 836500,
          "max_ns": 925000,
          "mean_ns": 873042.8571428572
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            49500,
            45600,
            42600,
            47300,
            51500,
            42400,
            44600
          ],
          "median_ns": 45600,
          "p95_ns": 51500,
          "variance_ns2": 11751428.571428573,
          "min_ns": 42400,
          "max_ns": 51500,
          "mean_ns": 46214.28571428572
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            5739800,
            13352600,
            7574400,
            5120500,
            5252300,
            6747000,
            13266400
          ],
          "median_ns": 6747000,
          "p95_ns": 13352600,
          "variance_ns2": 13154653062380.953,
          "min_ns": 5120500,
          "max_ns": 13352600,
          "mean_ns": 8150428.571428572
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            16891400,
            -5747400,
            17078400,
            2722300,
            20035300,
            13269200,
            -6093400
          ],
          "median_ns": 13269200,
          "p95_ns": 20035300,
          "variance_ns2": 124815396342380.95,
          "min_ns": -6093400,
          "max_ns": 20035300,
          "mean_ns": 8307971.428571428
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            200,
            100,
            100,
            100,
            100,
            100
          ],
          "median_ns": 100,
          "p95_ns": 200,
          "variance_ns2": 3333.3333333333335,
          "min_ns": 0,
          "max_ns": 200,
          "mean_ns": 100.0
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3903800,
            3423200,
            4146600,
            3263300,
            5633500,
            3227800,
            3120100
          ],
          "median_ns": 3423200,
          "p95_ns": 5633500,
          "variance_ns2": 785055160000.0,
          "min_ns": 3120100,
          "max_ns": 5633500,
          "mean_ns": 3816900.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1842300,
            1818900,
            1803100,
            2164000,
            9698300,
            1927200,
            3183300
          ],
          "median_ns": 1927200,
          "p95_ns": 9698300,
          "variance_ns2": 8437236183333.333,
          "min_ns": 1803100,
          "max_ns": 9698300,
          "mean_ns": 3205300.0
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            15197500,
            19841300,
            19493100,
            11520600,
            11482000,
            14813900,
            23275800
          ],
          "median_ns": 15197500,
          "p95_ns": 23275800,
          "variance_ns2": 20091063849523.812,
          "min_ns": 11482000,
          "max_ns": 23275800,
          "mean_ns": 16517742.857142856
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            9243500,
            9449400,
            18373800,
            9917000,
            9719900,
            8942000,
            8681900
          ],
          "median_ns": 9449400,
          "p95_ns": 18373800,
          "variance_ns2": 11877125558095.236,
          "min_ns": 8681900,
          "max_ns": 18373800,
          "mean_ns": 10618214.285714285
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            21521000,
            9859200,
            20213200,
            8856300,
            10954900,
            9946600,
            8598700
          ],
          "median_ns": 9946600,
          "p95_ns": 21521000,
          "variance_ns2": 30732027804761.906,
          "min_ns": 8598700,
          "max_ns": 21521000,
          "mean_ns": 12849985.714285715
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          24858500,
          9955500,
          26953800,
          10229700,
          27514600,
          22621500,
          9281100
        ],
        "median_ns": 22621500,
        "p95_ns": 27514600,
        "variance_ns2": 72669627749047.61,
        "min_ns": 9281100,
        "max_ns": 27514600,
        "mean_ns": 18773528.57142857
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          95338400,
          64644900,
          103290300,
          59834600,
          92176400,
          75076800,
          65906400
        ],
        "median_ns": 75076800,
        "p95_ns": 103290300,
        "variance_ns2": 298211552595714.3,
        "min_ns": 59834600,
        "max_ns": 103290300,
        "mean_ns": 79466828.57142857
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          17914700,
          9319000,
          11311300,
          13048000,
          13333600,
          12751000,
          8940700
        ],
        "median_ns": 12751000,
        "p95_ns": 17914700,
        "variance_ns2": 9077749802857.143,
        "min_ns": 8940700,
        "max_ns": 17914700,
        "mean_ns": 12374042.857142856
      },
      "difference_of_medians_ns": 11654200,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 10000,
      "unresolved_total": 19999,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "scale_100000",
        "tree_count": 1,
        "claims_per_tree": 100000,
        "claims_total": 100000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 100000,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 1,
      "sample_count": 5,
      "setup_allocation_ns": 8242300,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            14128700,
            9494300,
            8687200,
            8837700,
            9124000
          ],
          "median_ns": 9124000,
          "p95_ns": 14128700,
          "variance_ns2": 5282217847000.0,
          "min_ns": 8687200,
          "max_ns": 14128700,
          "mean_ns": 10054380.0
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            18767700,
            15099000,
            22402100,
            12156800,
            14400300
          ],
          "median_ns": 15099000,
          "p95_ns": 22402100,
          "variance_ns2": 16297733217000.0,
          "min_ns": 12156800,
          "max_ns": 22402100,
          "mean_ns": 16565180.0
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            23352100,
            9272400,
            10004000,
            8688300,
            9076700
          ],
          "median_ns": 9272400,
          "p95_ns": 23352100,
          "variance_ns2": 39944015875000.0,
          "min_ns": 8688300,
          "max_ns": 23352100,
          "mean_ns": 12078700.0
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            465800,
            427100,
            463900,
            490300,
            432300
          ],
          "median_ns": 463900,
          "p95_ns": 490300,
          "variance_ns2": 682942000.0,
          "min_ns": 427100,
          "max_ns": 490300,
          "mean_ns": 455880.0
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            105327300,
            102254700,
            125242300,
            121101200,
            80988300
          ],
          "median_ns": 105327300,
          "p95_ns": 125242300,
          "variance_ns2": 308387049728000.0,
          "min_ns": 80988300,
          "max_ns": 125242300,
          "mean_ns": 106982760.0
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            -8080300,
            25466000,
            32737400,
            34478900,
            39289000
          ],
          "median_ns": 32737400,
          "p95_ns": 39289000,
          "variance_ns2": 362042463215000.0,
          "min_ns": -8080300,
          "max_ns": 39289000,
          "mean_ns": 24778200.0
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            200,
            100,
            100,
            200,
            100
          ],
          "median_ns": 100,
          "p95_ns": 200,
          "variance_ns2": 3000.0,
          "min_ns": 100,
          "max_ns": 200,
          "mean_ns": 140.0
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            39355800,
            38536900,
            47876200,
            48530000,
            37656300
          ],
          "median_ns": 39355800,
          "p95_ns": 48530000,
          "variance_ns2": 28564662493000.0,
          "min_ns": 37656300,
          "max_ns": 48530000,
          "mean_ns": 42391040.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            35922800,
            17326900,
            42878600,
            20568400,
            18346200
          ],
          "median_ns": 20568400,
          "p95_ns": 42878600,
          "variance_ns2": 135392131582000.0,
          "min_ns": 17326900,
          "max_ns": 42878600,
          "mean_ns": 27008580.0
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            246803700,
            217370100,
            241182000,
            216042300,
            198669800
          ],
          "median_ns": 217370100,
          "p95_ns": 246803700,
          "variance_ns2": 391032132737000.0,
          "min_ns": 198669800,
          "max_ns": 246803700,
          "mean_ns": 224013580.0
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            151986600,
            204557500,
            183998700,
            184873700,
            172932700
          ],
          "median_ns": 183998700,
          "p95_ns": 204557500,
          "variance_ns2": 369241410088000.0,
          "min_ns": 151986600,
          "max_ns": 204557500,
          "mean_ns": 179669840.0
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            176006300,
            170009800,
            136880200,
            185968800,
            253871100
          ],
          "median_ns": 176006300,
          "p95_ns": 253871100,
          "variance_ns2": 1841062479583000.0,
          "min_ns": 136880200,
          "max_ns": 253871100,
          "mean_ns": 184547240.0
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          139832600,
          152519200,
          190849700,
          176915500,
          144186600
        ],
        "median_ns": 152519200,
        "p95_ns": 190849700,
        "variance_ns2": 486720986677000.0,
        "min_ns": 139832600,
        "max_ns": 190849700,
        "mean_ns": 160860720.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          922304600,
          932230400,
          1024157600,
          1043785300,
          958258900
        ],
        "median_ns": 958258900,
        "p95_ns": 1043785300,
        "variance_ns2": 3006903313383000.0,
        "min_ns": 922304600,
        "max_ns": 1043785300,
        "mean_ns": 976147360.0
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          118267900,
          122415600,
          171804900,
          202048700,
          123472100
        ],
        "median_ns": 123472100,
        "p95_ns": 202048700,
        "variance_ns2": 1406829937738000.0,
        "min_ns": 118267900,
        "max_ns": 202048700,
        "mean_ns": 147601840.0
      },
      "difference_of_medians_ns": 159316300,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 100000,
      "unresolved_total": 200001,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "scale_1000000",
        "tree_count": 1,
        "claims_per_tree": 1000000,
        "claims_total": 1000000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 1000000,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 1,
      "sample_count": 3,
      "setup_allocation_ns": 90088700,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            183170700,
            123306400,
            103382500
          ],
          "median_ns": 123306400,
          "p95_ns": 183170700,
          "variance_ns2": 1724475510823333.5,
          "min_ns": 103382500,
          "max_ns": 183170700,
          "mean_ns": 136619866.66666666
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            238333700,
            186473800,
            145315900
          ],
          "median_ns": 186473800,
          "p95_ns": 238333700,
          "variance_ns2": 2172622179543333.5,
          "min_ns": 145315900,
          "max_ns": 238333700,
          "mean_ns": 190041133.33333334
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            152301400,
            112639600,
            130102900
          ],
          "median_ns": 130102900,
          "p95_ns": 152301400,
          "variance_ns2": 395133104730000.0,
          "min_ns": 112639600,
          "max_ns": 152301400,
          "mean_ns": 131681300.0
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            8473400,
            5209100,
            4762400
          ],
          "median_ns": 5209100,
          "p95_ns": 8473400,
          "variance_ns2": 4104452730000.0,
          "min_ns": 4762400,
          "max_ns": 8473400,
          "mean_ns": 6148300.0
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1635142600,
            1282906600,
            1265083000
          ],
          "median_ns": 1282906600,
          "p95_ns": 1635142600,
          "variance_ns2": 4.355533132752e+16,
          "min_ns": 1265083000,
          "max_ns": 1635142600,
          "mean_ns": 1394377400.0
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            -34136100,
            137676700,
            401759400
          ],
          "median_ns": 137676700,
          "p95_ns": 401759400,
          "variance_ns2": 4.8210699600563336e+16,
          "min_ns": -34136100,
          "max_ns": 401759400,
          "mean_ns": 168433333.33333334
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            100,
            300,
            100
          ],
          "median_ns": 100,
          "p95_ns": 300,
          "variance_ns2": 13333.333333333334,
          "min_ns": 100,
          "max_ns": 300,
          "mean_ns": 166.66666666666666
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            377518600,
            432774300,
            448863800
          ],
          "median_ns": 432774300,
          "p95_ns": 448863800,
          "variance_ns2": 1400366992630000.0,
          "min_ns": 377518600,
          "max_ns": 448863800,
          "mean_ns": 419718900.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            276813400,
            181854400,
            198161100
          ],
          "median_ns": 198161100,
          "p95_ns": 276813400,
          "variance_ns2": 2578217406863333.5,
          "min_ns": 181854400,
          "max_ns": 276813400,
          "mean_ns": 218942966.66666666
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3393887100,
            2584659600,
            2535344300
          ],
          "median_ns": 2584659600,
          "p95_ns": 3393887100,
          "variance_ns2": 2.3239614750036333e+17,
          "min_ns": 2535344300,
          "max_ns": 3393887100,
          "mean_ns": 2837963666.6666665
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1901140000,
            1794552800,
            1778446800
          ],
          "median_ns": 1794552800,
          "p95_ns": 1901140000,
          "variance_ns2": 4445642627680000.0,
          "min_ns": 1778446800,
          "max_ns": 1901140000,
          "mean_ns": 1824713200.0
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1935245400,
            2221023000,
            1921636300
          ],
          "median_ns": 1935245400,
          "p95_ns": 2221023000,
          "variance_ns2": 2.8581073400243332e+16,
          "min_ns": 1921636300,
          "max_ns": 2221023000,
          "mean_ns": 2025968233.3333333
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          2000115000,
          1724905800,
          1947023600
        ],
        "median_ns": 1947023600,
        "p95_ns": 2000115000,
        "variance_ns2": 2.1315852932573332e+16,
        "min_ns": 1724905800,
        "max_ns": 2000115000,
        "mean_ns": 1890681466.6666667
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          11936817100,
          10988599800,
          10878322200
        ],
        "median_ns": 10988599800,
        "p95_ns": 11936817100,
        "variance_ns2": 3.386147750678433e+17,
        "min_ns": 10878322200,
        "max_ns": 11936817100,
        "mean_ns": 11267913033.333334
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          1868926800,
          1925523200,
          1945463700
        ],
        "median_ns": 1925523200,
        "p95_ns": 1945463700,
        "variance_ns2": 1576445515803333.5,
        "min_ns": 1868926800,
        "max_ns": 1945463700,
        "mean_ns": 1913304566.6666667
      },
      "difference_of_medians_ns": 1972876500,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 1000000,
      "unresolved_total": 2000000,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "one_large_tree",
        "tree_count": 1,
        "claims_per_tree": 100000,
        "claims_total": 100000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 100000,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 1,
      "sample_count": 5,
      "setup_allocation_ns": 8259500,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            10840800,
            8579400,
            7821800,
            8241500,
            8109500
          ],
          "median_ns": 8241500,
          "p95_ns": 10840800,
          "variance_ns2": 1481496735000.0,
          "min_ns": 7821800,
          "max_ns": 10840800,
          "mean_ns": 8718600.0
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            21934300,
            49683300,
            14042900,
            12713600,
            29637800
          ],
          "median_ns": 21934300,
          "p95_ns": 49683300,
          "variance_ns2": 227343090317000.0,
          "min_ns": 12713600,
          "max_ns": 49683300,
          "mean_ns": 25602380.0
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            17173500,
            33922800,
            8047900,
            9219300,
            7720000
          ],
          "median_ns": 9219300,
          "p95_ns": 33922800,
          "variance_ns2": 124327063635000.0,
          "min_ns": 7720000,
          "max_ns": 33922800,
          "mean_ns": 15216700.0
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            526800,
            1244200,
            558200,
            409100,
            1093100
          ],
          "median_ns": 558200,
          "p95_ns": 1244200,
          "variance_ns2": 140861087000.0,
          "min_ns": 409100,
          "max_ns": 1244200,
          "mean_ns": 766280.0
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            75633800,
            114943100,
            64378700,
            69033900,
            95376600
          ],
          "median_ns": 75633800,
          "p95_ns": 114943100,
          "variance_ns2": 441448741167000.0,
          "min_ns": 64378700,
          "max_ns": 114943100,
          "mean_ns": 83873220.0
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            17761600,
            -10572300,
            28250100,
            55353700,
            52758000
          ],
          "median_ns": 28250100,
          "p95_ns": 55353700,
          "variance_ns2": 737842779327000.0,
          "min_ns": -10572300,
          "max_ns": 55353700,
          "mean_ns": 28710220.0
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            300,
            200,
            100,
            100,
            100
          ],
          "median_ns": 100,
          "p95_ns": 300,
          "variance_ns2": 8000.0,
          "min_ns": 100,
          "max_ns": 300,
          "mean_ns": 160.0
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            34743300,
            43008100,
            33535600,
            39714900,
            57090300
          ],
          "median_ns": 39714900,
          "p95_ns": 57090300,
          "variance_ns2": 89383230948000.0,
          "min_ns": 33535600,
          "max_ns": 57090300,
          "mean_ns": 41618440.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            17250700,
            16319300,
            17377100,
            20650500,
            26523500
          ],
          "median_ns": 17377100,
          "p95_ns": 26523500,
          "variance_ns2": 17564739212000.0,
          "min_ns": 16319300,
          "max_ns": 26523500,
          "mean_ns": 19624220.0
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            172667400,
            255500400,
            193592300,
            193983300,
            193897900
          ],
          "median_ns": 193897900,
          "p95_ns": 255500400,
          "variance_ns2": 980817353093000.0,
          "min_ns": 172667400,
          "max_ns": 255500400,
          "mean_ns": 201928260.0
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            113272800,
            191846300,
            134423100,
            154576400,
            178778300
          ],
          "median_ns": 154576400,
          "p95_ns": 191846300,
          "variance_ns2": 1021730059767000.0,
          "min_ns": 113272800,
          "max_ns": 191846300,
          "mean_ns": 154579380.0
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            110303100,
            133858500,
            120756900,
            145775600,
            106238100
          ],
          "median_ns": 120756900,
          "p95_ns": 145775600,
          "variance_ns2": 270773089268000.0,
          "min_ns": 106238100,
          "max_ns": 145775600,
          "mean_ns": 123386440.0
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          133030000,
          189221100,
          115277800,
          146729600,
          186585500
        ],
        "median_ns": 146729600,
        "p95_ns": 189221100,
        "variance_ns2": 1073551654315000.0,
        "min_ns": 115277800,
        "max_ns": 189221100,
        "mean_ns": 154168800.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          720945100,
          967635800,
          760101000,
          821994100,
          906789300
        ],
        "median_ns": 821994100,
        "p95_ns": 967635800,
        "variance_ns2": 1.0383069336283e+16,
        "min_ns": 720945100,
        "max_ns": 967635800,
        "mean_ns": 835493060.0
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          128836700,
          129302500,
          137316300,
          112322200,
          149566100
        ],
        "median_ns": 129302500,
        "p95_ns": 149566100,
        "variance_ns2": 184979655298000.0,
        "min_ns": 112322200,
        "max_ns": 149566100,
        "mean_ns": 131468760.0
      },
      "difference_of_medians_ns": 140699700,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 100000,
      "unresolved_total": 200001,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "many_independent_small_trees",
        "tree_count": 100,
        "claims_per_tree": 100,
        "claims_total": 10000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 100,
        "granters_per_tree": 1,
        "generations": [
          10
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 1,
      "sample_count": 5,
      "setup_allocation_ns": 4367400,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            701500,
            664900,
            608000,
            639000,
            661200
          ],
          "median_ns": 661200,
          "p95_ns": 701500,
          "variance_ns2": 1190917000.0,
          "min_ns": 608000,
          "max_ns": 701500,
          "mean_ns": 654920.0
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1290700,
            1544100,
            944000,
            1337500,
            2300100
          ],
          "median_ns": 1337500,
          "p95_ns": 2300100,
          "variance_ns2": 255013942000.0,
          "min_ns": 944000,
          "max_ns": 2300100,
          "mean_ns": 1483280.0
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            875800,
            999800,
            617500,
            857700,
            1389500
          ],
          "median_ns": 875800,
          "p95_ns": 1389500,
          "variance_ns2": 80050663000.0,
          "min_ns": 617500,
          "max_ns": 1389500,
          "mean_ns": 948060.0
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            34800,
            45000,
            25200,
            44400,
            65100
          ],
          "median_ns": 44400,
          "p95_ns": 65100,
          "variance_ns2": 219600000.0,
          "min_ns": 25200,
          "max_ns": 65100,
          "mean_ns": 42900.0
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3137000,
            3648800,
            2223200,
            3160800,
            5132400
          ],
          "median_ns": 3160800,
          "p95_ns": 5132400,
          "variance_ns2": 1139022528000.0,
          "min_ns": 2223200,
          "max_ns": 5132400,
          "mean_ns": 3460440.0
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2868400,
            2561700,
            1604200,
            3079100,
            2108300
          ],
          "median_ns": 2561700,
          "p95_ns": 3079100,
          "variance_ns2": 353819653000.0,
          "min_ns": 1604200,
          "max_ns": 3079100,
          "mean_ns": 2444340.0
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3700,
            3100,
            3000,
            4300,
            4100
          ],
          "median_ns": 3700,
          "p95_ns": 4300,
          "variance_ns2": 338000.0,
          "min_ns": 3000,
          "max_ns": 4300,
          "mean_ns": 3640.0
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2942000,
            2791800,
            2047700,
            2783000,
            4621400
          ],
          "median_ns": 2791800,
          "p95_ns": 4621400,
          "variance_ns2": 905675432000.0,
          "min_ns": 2047700,
          "max_ns": 4621400,
          "mean_ns": 3037180.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2296200,
            2455700,
            1736500,
            2313500,
            3897900
          ],
          "median_ns": 2313500,
          "p95_ns": 3897900,
          "variance_ns2": 651837958000.0,
          "min_ns": 1736500,
          "max_ns": 3897900,
          "mean_ns": 2539960.0
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            7699000,
            8808800,
            5641700,
            8211100,
            12222200
          ],
          "median_ns": 8211100,
          "p95_ns": 12222200,
          "variance_ns2": 5710925553000.0,
          "min_ns": 5641700,
          "max_ns": 12222200,
          "mean_ns": 8516560.0
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            7385900,
            7642900,
            5562000,
            8500900,
            11437900
          ],
          "median_ns": 7642900,
          "p95_ns": 11437900,
          "variance_ns2": 4615611302000.0,
          "min_ns": 5562000,
          "max_ns": 11437900,
          "mean_ns": 8105920.0
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            7519800,
            7712200,
            5591900,
            7863700,
            11207700
          ],
          "median_ns": 7712200,
          "p95_ns": 11207700,
          "variance_ns2": 4104522763000.0,
          "min_ns": 5591900,
          "max_ns": 11207700,
          "mean_ns": 7979060.0
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          8206700,
          8799400,
          5414100,
          8479500,
          10995400
        ],
        "median_ns": 8479500,
        "p95_ns": 10995400,
        "variance_ns2": 3963176167000.0,
        "min_ns": 5414100,
        "max_ns": 10995400,
        "mean_ns": 8379020.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          43215500,
          45699100,
          31202500,
          45622500,
          64783200
        ],
        "median_ns": 45622500,
        "p95_ns": 64783200,
        "variance_ns2": 144926607958000.0,
        "min_ns": 31202500,
        "max_ns": 64783200,
        "mean_ns": 46104560.0
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          6460700,
          6820300,
          4597600,
          6827500,
          9735400
        ],
        "median_ns": 6820300,
        "p95_ns": 9735400,
        "variance_ns2": 3386111825000.0,
        "min_ns": 4597600,
        "max_ns": 9735400,
        "mean_ns": 6888300.0
      },
      "difference_of_medians_ns": 7806600,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 10000,
      "unresolved_total": 20200,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "divergent_generation_trees",
        "tree_count": 4,
        "claims_per_tree": 1000,
        "claims_total": 4000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 1000,
        "granters_per_tree": 1,
        "generations": [
          1,
          10,
          100,
          1000
        ],
        "overlapping_raw_local_ids": false,
        "overlapping_construction_door": ""
      },
      "warm_up_count": 1,
      "sample_count": 5,
      "setup_allocation_ns": 418700,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            325900,
            247600,
            318300,
            304800,
            257500
          ],
          "median_ns": 304800,
          "p95_ns": 325900,
          "variance_ns2": 1289847000.0,
          "min_ns": 247600,
          "max_ns": 325900,
          "mean_ns": 290820.0
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            395600,
            987000,
            378600,
            364800,
            431200
          ],
          "median_ns": 395600,
          "p95_ns": 987000,
          "variance_ns2": 71291108000.0,
          "min_ns": 364800,
          "max_ns": 987000,
          "mean_ns": 511440.0
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            259000,
            439100,
            261700,
            238500,
            286800
          ],
          "median_ns": 261700,
          "p95_ns": 439100,
          "variance_ns2": 6602197000.0,
          "min_ns": 238500,
          "max_ns": 439100,
          "mean_ns": 297020.0
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            7400,
            24200,
            7300,
            7400,
            8500
          ],
          "median_ns": 7400,
          "p95_ns": 24200,
          "variance_ns2": 55023000.0,
          "min_ns": 7300,
          "max_ns": 24200,
          "mean_ns": 10960.0
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1503300,
            2499300,
            1459600,
            1934400,
            1752700
          ],
          "median_ns": 1752700,
          "p95_ns": 2499300,
          "variance_ns2": 177191523000.0,
          "min_ns": 1459600,
          "max_ns": 2499300,
          "mean_ns": 1829860.0
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            574200,
            951500,
            571800,
            2476600,
            657300
          ],
          "median_ns": 657300,
          "p95_ns": 2476600,
          "variance_ns2": 663523697000.0,
          "min_ns": 571800,
          "max_ns": 2476600,
          "mean_ns": 1046280.0
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            100,
            100,
            100,
            400,
            300
          ],
          "median_ns": 100,
          "p95_ns": 400,
          "variance_ns2": 20000.0,
          "min_ns": 100,
          "max_ns": 400,
          "mean_ns": 200.0
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            859800,
            1737500,
            926600,
            875800,
            1408000
          ],
          "median_ns": 926600,
          "p95_ns": 1737500,
          "variance_ns2": 155090908000.0,
          "min_ns": 859800,
          "max_ns": 1737500,
          "mean_ns": 1161540.0
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            606200,
            1115400,
            619900,
            596700,
            832400
          ],
          "median_ns": 619900,
          "p95_ns": 1115400,
          "variance_ns2": 50331847000.0,
          "min_ns": 596700,
          "max_ns": 1115400,
          "mean_ns": 754120.0
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3306100,
            5755800,
            3232600,
            4363400,
            4090100
          ],
          "median_ns": 4090100,
          "p95_ns": 5755800,
          "variance_ns2": 1045377595000.0,
          "min_ns": 3232600,
          "max_ns": 5755800,
          "mean_ns": 4149600.0
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2645000,
            4594500,
            2585800,
            4383600,
            6539300
          ],
          "median_ns": 4383600,
          "p95_ns": 6539300,
          "variance_ns2": 2668162423000.0,
          "min_ns": 2585800,
          "max_ns": 6539300,
          "mean_ns": 4149640.0
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            2545000,
            4845100,
            2649600,
            2960400,
            7288500
          ],
          "median_ns": 2960400,
          "p95_ns": 7288500,
          "variance_ns2": 4133285397000.0,
          "min_ns": 2545000,
          "max_ns": 7288500,
          "mean_ns": 4057720.0
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          2739500,
          4901100,
          2679000,
          5021700,
          3136500
        ],
        "median_ns": 3136500,
        "p95_ns": 5021700,
        "variance_ns2": 1367991758000.0,
        "min_ns": 2679000,
        "max_ns": 5021700,
        "mean_ns": 3695560.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          15371700,
          28610600,
          15373400,
          24583300,
          26361600
        ],
        "median_ns": 24583300,
        "p95_ns": 28610600,
        "variance_ns2": 39306285547000.0,
        "min_ns": 15371700,
        "max_ns": 28610600,
        "mean_ns": 22060120.0
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          2344100,
          5413500,
          2361500,
          6076500,
          2809000
        ],
        "median_ns": 2809000,
        "p95_ns": 6076500,
        "variance_ns2": 3239209582000.0,
        "min_ns": 2344100,
        "max_ns": 6076500,
        "mean_ns": 3800920.0
      },
      "difference_of_medians_ns": 8161300,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": null,
      "overlapping_tree_contexts": [],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 4000,
      "unresolved_total": 7988,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    },
    {
      "cardinalities": {
        "name": "overlapping_local_ids",
        "tree_count": 2,
        "claims_per_tree": 1000,
        "claims_total": 2000,
        "scopes_per_tree": 1,
        "supplies_per_tree": 1,
        "claimants_per_tree": 1000,
        "granters_per_tree": 1,
        "generations": [
          10,
          10
        ],
        "overlapping_raw_local_ids": true,
        "overlapping_construction_door": "ConstrainedClaim::from_runtime_demand -> SimThingId::from_session_raw (ordinary production admission; same raw {1..=N} including 7 under distinct owner_ref/scope_id/granter)"
      },
      "warm_up_count": 1,
      "sample_count": 7,
      "setup_allocation_ns": 137400,
      "legs": [
        {
          "name": "claim_production_completion",
          "isolation": "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            113600,
            112700,
            120200,
            476700,
            115900,
            113500,
            169600
          ],
          "median_ns": 115900,
          "p95_ns": 476700,
          "variance_ns2": 18163380000.0,
          "min_ns": 112700,
          "max_ns": 476700,
          "mean_ns": 174600.0
        },
        {
          "name": "gpu_to_host_synchronization_readback",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "host_conversion_grouping",
          "isolation": "BTreeMap group by OwnerChannelScopeKey",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            179500,
            453000,
            180900,
            468500,
            211800,
            272400,
            212300
          ],
          "median_ns": 212300,
          "p95_ns": 468500,
          "variance_ns2": 15771039047.61905,
          "min_ns": 179500,
          "max_ns": 468500,
          "mean_ns": 282628.5714285714
        },
        {
          "name": "eml_scoring",
          "isolation": "TransformOp::apply_with_params via AuthoredClearingProgram::score_program",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            125900,
            308200,
            126000,
            303700,
            130700,
            121100,
            151000
          ],
          "median_ns": 130700,
          "p95_ns": 308200,
          "variance_ns2": 7385702857.142858,
          "min_ns": 121100,
          "max_ns": 308200,
          "mean_ns": 180942.85714285713
        },
        {
          "name": "score_sorting_banding",
          "isolation": "score.total_cmp desc then source id; equal to_bits bands",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            3500,
            7800,
            3800,
            7900,
            4300,
            4300,
            4300
          ],
          "median_ns": 4300,
          "p95_ns": 7900,
          "variance_ns2": 3549047.619047619,
          "min_ns": 3500,
          "max_ns": 7900,
          "mean_ns": 5128.571428571428
        },
        {
          "name": "integer_apportionment",
          "isolation": "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority; component of instrument_restatement",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            681400,
            3284000,
            712800,
            1637100,
            700800,
            748200,
            1168200
          ],
          "median_ns": 748200,
          "p95_ns": 3284000,
          "variance_ns2": 909041549047.6191,
          "min_ns": 681400,
          "max_ns": 3284000,
          "mean_ns": 1276071.4285714286
        },
        {
          "name": "grant_result_construction",
          "isolation": "signed i64 remainder of enclosing_clear minus nested grouping+scoring+sorting+apportionment; negatives retained; D1 no .max(0) clamp",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            599100,
            -144100,
            1075200,
            1210300,
            276000,
            228800,
            5118500
          ],
          "median_ns": 599100,
          "p95_ns": 5118500,
          "variance_ns2": 3222471205714.2856,
          "min_ns": -144100,
          "max_ns": 5118500,
          "mean_ns": 1194828.5714285714
        },
        {
          "name": "host_to_gpu_upload",
          "isolation": "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ],
          "median_ns": 0,
          "p95_ns": 0,
          "variance_ns2": 0.0,
          "min_ns": 0,
          "max_ns": 0,
          "mean_ns": 0.0
        },
        {
          "name": "n_plus_one_launch_delay",
          "isolation": "grants-available → construct generation+1 ClearingRemainderAuthority; GPU launch 0; schedule-append not in this boundary",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            200,
            100,
            0,
            0,
            0,
            100,
            100
          ],
          "median_ns": 100,
          "p95_ns": 200,
          "variance_ns2": 5714.285714285714,
          "min_ns": 0,
          "max_ns": 200,
          "mean_ns": 71.42857142857143
        },
        {
          "name": "cpu_schedule_replay_recording",
          "isolation": "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule; not part of n_plus_one_launch_delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            473300,
            546700,
            1018200,
            458900,
            472000,
            578500,
            524600
          ],
          "median_ns": 524600,
          "p95_ns": 1018200,
          "variance_ns2": 38982062857.14285,
          "min_ns": 458900,
          "max_ns": 1018200,
          "mean_ns": 581742.8571428572
        },
        {
          "name": "lawful_structural_consequence",
          "isolation": "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            295200,
            737500,
            740000,
            316300,
            311200,
            386100,
            344200
          ],
          "median_ns": 344200,
          "p95_ns": 740000,
          "variance_ns2": 40513824761.90476,
          "min_ns": 295200,
          "max_ns": 740000,
          "mean_ns": 447214.28571428574
        },
        {
          "name": "instrument_restatement",
          "isolation": "D2 shape-2: wall time of nested grouping+scoring+sorting+apportionment pass inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1715100,
            5345500,
            1584300,
            3730400,
            1610100,
            1710100,
            2479600
          ],
          "median_ns": 1715100,
          "p95_ns": 5345500,
          "variance_ns2": 2069416752857.1428,
          "min_ns": 1584300,
          "max_ns": 5345500,
          "mean_ns": 2596442.8571428573
        },
        {
          "name": "neutrality_reclear",
          "isolation": "D2 shape-2: second uninstrumented production-door clear inside e2e",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1340700,
            2467200,
            1579800,
            1331200,
            1339500,
            2166000,
            1712600
          ],
          "median_ns": 1579800,
          "p95_ns": 2467200,
          "variance_ns2": 202538874761.90475,
          "min_ns": 1331200,
          "max_ns": 2467200,
          "mean_ns": 1705285.7142857143
        },
        {
          "name": "next_generation_host_reclear",
          "isolation": "full ordinary-door re-clear at generation+1; comparator only; not the N+1 launch delay",
          "bytes_read_back": 0,
          "bytes_uploaded": 0,
          "sample_ns": [
            1751800,
            1258700,
            1508300,
            1315300,
            1319300,
            1832200,
            1598000
          ],
          "median_ns": 1508300,
          "p95_ns": 1832200,
          "variance_ns2": 51237802857.14286,
          "min_ns": 1258700,
          "max_ns": 1832200,
          "mean_ns": 1511942.857142857
        }
      ],
      "enclosing_clear_ns": {
        "name": "enclosing_clear",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          1589400,
          3908900,
          2098700,
          3627500,
          1323600,
          1374800,
          6654300
        ],
        "median_ns": 2098700,
        "p95_ns": 6654300,
        "variance_ns2": 3800325480000.0,
        "min_ns": 1323600,
        "max_ns": 6654300,
        "mean_ns": 2939600.0
      },
      "end_to_end_ns": {
        "name": "end_to_end",
        "isolation": "production-door observation",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          8478300,
          17172700,
          10791400,
          12430000,
          7621900,
          10115000,
          14968000
        ],
        "median_ns": 10791400,
        "p95_ns": 17172700,
        "variance_ns2": 11916053713333.334,
        "min_ns": 7621900,
        "max_ns": 17172700,
        "mean_ns": 11653900.0
      },
      "observation_overhead_residual": {
        "name": "observation_overhead_residual",
        "isolation": "D6 samplewise residual: end_to_end[i] − Σ accounted_leg[i]; signed i64; not clamped; not the difference of medians",
        "bytes_read_back": 0,
        "bytes_uploaded": 0,
        "sample_ns": [
          1199000,
          2795400,
          2141900,
          1173700,
          1130300,
          1953700,
          1485000
        ],
        "median_ns": 1485000,
        "p95_ns": 2795400,
        "variance_ns2": 393042206666.6667,
        "min_ns": 1130300,
        "max_ns": 2795400,
        "mean_ns": 1697000.0
      },
      "difference_of_medians_ns": 2904700,
      "reconciliation_note": "D6: observation_overhead_residual[i] = end_to_end[i] − Σ accounted_leg[i] over E2E_ACCOUNTED_LEGS; signed i64; not clamped; sample count equals workload N. difference_of_medians_ns = median(end_to_end) − Σ median(accounted) is a derived figure and is not the residual. D2 shape-2 accounted set: claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear. grouping/scoring/sorting/apportionment are components of instrument_restatement. grant_result_construction is a partition of enclosing.",
      "isolation_matches_production": true,
      "neutrality_clears_identical": true,
      "overlapping_raw_value": 7,
      "overlapping_tree_contexts": [
        "overlap-alpha owner_ref=overlap-alpha granter_raw=1001 scope_boundary=1011",
        "overlap-beta owner_ref=overlap-beta granter_raw=2001 scope_boundary=2011"
      ],
      "coupling": {
        "process_global_simthing_allocator": "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.",
        "process_global_overlay_allocator": "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.",
        "host_wide_clearing_lock": "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.",
        "shared_generation_authority": "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.",
        "shared_integration_schedule": "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.",
        "all_tree_synchronization": "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair."
      },
      "grants_total": 2000,
      "unresolved_total": 4000,
      "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
      "residual_accounted_legs": [
        "claim_production_completion",
        "instrument_restatement",
        "enclosing_clear",
        "n_plus_one_launch_delay",
        "next_generation_host_reclear",
        "cpu_schedule_replay_recording",
        "lawful_structural_consequence",
        "neutrality_reclear"
      ]
    }
  ],
  "comparator_disclaimer": "Comparator only. Wall-clock values are dated facts about one reproducibility envelope. They are not a go/no-go gate, portable CI threshold, or authority to cancel or narrow Owner-ruled Phase-14 placement.",
  "d1_signed_remainder_note": "D1: grant_result_construction is i64; `.max(0)` removed. Before/after: scale_1000 grant_result_construction prior packet min_ns=0 with an explicit 0 sample after `.max(0)` (samples included 166600, 790200, 41100, 153900, 446300, 157200, 252100, 0, ...); the 0 was a clamped negative remainder of enclosing minus nested grouping+scoring+sorting+apportionment. Corrected samples keep the signed remainder. Serialization boundary: LegSamples.sample_ns/median_ns/p95_ns/min_ns/max_ns are i64.",
  "d2_envelope_shape": "shape-2-instrument-legs-inside-e2e",
  "d3_nplus_boundary": "n_plus_one_launch_delay = grants-available → generation+1 ClearingRemainderAuthority construction (GPU launch 0). next_generation_host_reclear = full ordinary-door re-clear at generation+1. cpu_schedule_replay_recording is not inside the N+1 delay (no overlap; next host clear does not require a schedule append).",
  "d6_residual_definition": "observation_overhead_residual[i] = end_to_end[i] − Σ E2E_ACCOUNTED_LEGS[i] (claim_production_completion, instrument_restatement, enclosing_clear, n_plus_one_launch_delay, next_generation_host_reclear, cpu_schedule_replay_recording, lawful_structural_consequence, neutrality_reclear). Signed i64; not clamped. difference_of_medians_ns is median(e2e) − Σ median(accounted) and is not the residual."
}
```
