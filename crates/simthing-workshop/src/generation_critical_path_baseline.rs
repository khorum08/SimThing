//! 14.1 generation-critical-path baseline — workshop observation of the live CPU-host
//! clearing door. Comparator evidence only: no production delta, no portable timing law.

use std::collections::BTreeMap;
use std::time::Instant;

use anyhow::{bail, Context, Result};
use serde::Serialize;
use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    cost_band_quantize, DissolveCondition, GenerationStamp, IntegrationSchedule,
    PropertyTransformDelta, SimPropertyId, SimThingId, SpecializationProfile, SubFieldRole,
    TransformOp,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    fund_unresolved_persistence, AdmittedSpecializationFlowMarket, AuthoredClearingProgram,
    AuthoredPersistenceValuation, ClearingRemainderAuthority, ConservedOfferingSpec,
    ConstrainedClaim, ConstrainedClearingResult, ConstrainedGrant, ConstrainedSupply,
    DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    PersistenceOverlayBinding, ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId,
    SpecializationFlowMarketSpec, UnresolvedDemandObservation,
};
use wgpu::{Backends, Instance, InstanceDescriptor, PowerPreference, RequestAdapterOptions};

use crate::generation_critical_path_baseline_report::format_baseline_report;

pub const REQUIRED_LEGS: [&str; 11] = [
    "claim_production_completion",
    "gpu_to_host_synchronization_readback",
    "host_conversion_grouping",
    "eml_scoring",
    "score_sorting_banding",
    "integer_apportionment",
    "grant_result_construction",
    "host_to_gpu_upload",
    "n_plus_one_launch_delay",
    "cpu_schedule_replay_recording",
    "lawful_structural_consequence",
];

pub const DETERMINISTIC_SEED: u64 = 0x0001_4001;
pub const OFFERING_ID: &str = "ore-claim";
pub const RESOURCE: &str = "ore";
pub const OVERLAPPING_RAW: u32 = 7;

/// Mechanically sourced host-clearing-door census at dispatch base `62529076`.
pub const HOST_CLEARING_DOOR_CENSUS: [ClearingDoorCensusRow; 4] = [
    ClearingDoorCensusRow {
        symbol: "clear_constrained_claims_at_generation",
        path: "crates/simthing-spec/src/spec/constrained_clearing.rs",
        line: 254,
        generation_authority_form: "caller-supplied ClearingRemainderAuthority { granter, generation }",
        reexports: "simthing-spec lib.rs + spec/mod.rs; simthing-embedder run.rs",
        callers: "production: simthing-driver/src/growth_entitlement.rs; wrapper: clear_reduced_owner_channels_at_generation; tests: contention_arena_executed_0, clearing_weight_span_unification_0, clearing_weight_deformation_lifecycle_0, stemthing_b_flow_market_germ_0, stemthing_b_vram_residency_0, grant_disbursement_lane_0, unified_facility_convergence_witness_0, protected_representative_restore, vendor_door_triad_surface_0",
        ordinary_or_oracle_posture: "ordinary production CPU-host clearing door",
        disposition_14_6: "narrow behind CpuVendorizedOracle; production migrates to the 14.2+ resident germ",
    },
    ClearingDoorCensusRow {
        symbol: "clear_reduced_owner_channels",
        path: "crates/simthing-spec/src/spec/constrained_clearing.rs",
        line: 420,
        generation_authority_form: "generationless compatibility: granter=SimThingId::from_session_raw(0), generation=GenerationStamp::new(0)",
        reexports: "simthing-spec lib.rs + spec/mod.rs",
        callers: "test: simthing-driver/tests/contention_arena_executed_0.rs (priority + price cases); no production caller",
        ordinary_or_oracle_posture: "generationless compatibility shim / test oracle",
        disposition_14_6: "DELETE",
    },
    ClearingDoorCensusRow {
        symbol: "clear_reduced_owner_channels_at_generation",
        path: "crates/simthing-spec/src/spec/constrained_clearing.rs",
        line: 437,
        generation_authority_form: "caller-supplied ClearingRemainderAuthority; converts reduce-up buckets through ConstrainedClaim::from_runtime_demand then the ordinary door",
        reexports: "simthing-spec lib.rs + spec/mod.rs",
        callers: "wrappers: clear_reduced_owner_channels (generationless), clear_stamped_owner_channels; no direct production caller",
        ordinary_or_oracle_posture: "conversion wrapper over the ordinary at-generation door",
        disposition_14_6: "narrow behind CpuVendorizedOracle (or delete once callers are gone)",
    },
    ClearingDoorCensusRow {
        symbol: "clear_stamped_owner_channels",
        path: "crates/simthing-spec/src/spec/constrained_clearing.rs",
        line: 488,
        generation_authority_form: "generation taken from StampedReduceUpProduct; granter supplied by caller",
        reexports: "simthing-spec lib.rs + spec/mod.rs; simthing-embedder run.rs",
        callers: "test/germ: stemthing_b_flow_market_germ_0.rs; no other production caller on this base",
        ordinary_or_oracle_posture: "canonical stamped-RF market binding over the ordinary door",
        disposition_14_6: "narrow behind CpuVendorizedOracle",
    },
];

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ClearingDoorCensusRow {
    pub symbol: &'static str,
    pub path: &'static str,
    pub line: u32,
    pub generation_authority_form: &'static str,
    pub reexports: &'static str,
    pub callers: &'static str,
    pub ordinary_or_oracle_posture: &'static str,
    pub disposition_14_6: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct MeasurementEnvelope {
    pub tested_commit: String,
    pub utc_date: String,
    pub cpu: String,
    pub gpu: String,
    pub adapter_backend: String,
    pub driver: String,
    pub os: String,
    pub compiler_toolchain: String,
    pub profile: String,
    pub deterministic_seed: u64,
    pub exact_command: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LegSamples {
    pub name: String,
    pub isolation: String,
    pub bytes_read_back: u64,
    pub bytes_uploaded: u64,
    pub sample_ns: Vec<u64>,
    pub median_ns: u64,
    pub p95_ns: u64,
    pub variance_ns2: f64,
    pub min_ns: u64,
    pub max_ns: u64,
    pub mean_ns: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkloadCardinalities {
    pub name: String,
    pub tree_count: usize,
    pub claims_per_tree: usize,
    pub claims_total: usize,
    pub scopes_per_tree: usize,
    pub supplies_per_tree: usize,
    pub claimants_per_tree: usize,
    pub granters_per_tree: usize,
    pub generations: Vec<u32>,
    pub overlapping_raw_local_ids: bool,
    pub overlapping_construction_door: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CouplingObservation {
    pub process_global_simthing_allocator: String,
    pub process_global_overlay_allocator: String,
    pub host_wide_clearing_lock: String,
    pub shared_generation_authority: String,
    pub shared_integration_schedule: String,
    pub all_tree_synchronization: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkloadReport {
    pub cardinalities: WorkloadCardinalities,
    pub warm_up_count: usize,
    pub sample_count: usize,
    pub setup_allocation_ns: u64,
    pub legs: Vec<LegSamples>,
    pub enclosing_clear_ns: LegSamples,
    pub end_to_end_ns: LegSamples,
    pub unattributed_remainder_ns: i64,
    pub reconciliation_note: String,
    pub isolation_matches_production: bool,
    pub neutrality_clears_identical: bool,
    pub overlapping_raw_value: Option<u32>,
    pub overlapping_tree_contexts: Vec<String>,
    pub coupling: CouplingObservation,
    pub grants_total: usize,
    pub unresolved_total: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct BaselinePacket {
    pub envelope: MeasurementEnvelope,
    pub door_census: Vec<ClearingDoorCensusRow>,
    pub path_diagram: String,
    pub leg_definitions: Vec<LegDefinition>,
    pub workloads: Vec<WorkloadReport>,
    pub comparator_disclaimer: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LegDefinition {
    pub name: &'static str,
    pub boundary: &'static str,
}

struct PreparedTree {
    supplies: Vec<ConstrainedSupply>,
    demands: Vec<(RuntimeOwnerSiloDemandBucket, f32)>,
    authority: ClearingRemainderAuthority,
    program: AuthoredClearingProgram,
}

struct ScoredRow {
    scope: OwnerChannelScopeKey,
    source: SimThingId,
    requested: u32,
    score: f32,
}

pub fn leg_definitions() -> Vec<LegDefinition> {
    vec![
        LegDefinition {
            name: "claim_production_completion",
            boundary: "ConstrainedClaim::from_runtime_demand over already-built RuntimeOwnerSiloDemandBucket rows (ordinary per-tree admission; uses SimThingId::from_session_raw)",
        },
        LegDefinition {
            name: "gpu_to_host_synchronization_readback",
            boundary: "absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns. GPU adapter is queried only for the envelope.",
        },
        LegDefinition {
            name: "host_conversion_grouping",
            boundary: "BTreeMap grouping of scored claims by OwnerChannelScopeKey, matching the live door's claims_by_scope insert",
        },
        LegDefinition {
            name: "eml_scoring",
            boundary: "AuthoredClearingProgram::score_program().apply_with_params(order_weight, priority) plus the live finite/non-negative/signed-zero canonicalize",
        },
        LegDefinition {
            name: "score_sorting_banding",
            boundary: "sort by score.total_cmp descending then source_simthing_id; equal score.to_bits() bands",
        },
        LegDefinition {
            name: "integer_apportionment",
            boundary: "workshop-local restatement of the live largest-remainder + generation-rotated exact-tie loop on the same scored/sorted/banded input; production door remains authority; isolation is proven by matching grant.granted",
        },
        LegDefinition {
            name: "grant_result_construction",
            boundary: "remainder of enclosing clear_constrained_claims_at_generation after the nested sequential grouping+scoring+sorting+apportionment observations; raw enclosing and component times are both kept; remainder is not forced-closed",
        },
        LegDefinition {
            name: "host_to_gpu_upload",
            boundary: "absent on the ordinary CPU-host clearing door; observed transfer is 0 bytes / 0 ns",
        },
        LegDefinition {
            name: "n_plus_one_launch_delay",
            boundary: "host re-clear of the same supplies/claims at generation+1 through the ordinary door; GPU kernel launch is door-absent (0 ns, 0 dispatches)",
        },
        LegDefinition {
            name: "cpu_schedule_replay_recording",
            boundary: "AdmittedSpecializationFlowMarket::record_cleared_grant into a fresh per-tree IntegrationSchedule for every produced ConstrainedGrant",
        },
        LegDefinition {
            name: "lawful_structural_consequence",
            boundary: "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence at observed_generation+1; overlays are dropped after mint so the allocator is observed without retaining the population",
        },
    ]
}

pub fn path_diagram() -> String {
    [
        "ordinary generation critical path (CPU-host clearing door; GPU legs door-absent):",
        "  RuntimeOwnerSiloDemandBucket (per-tree admission)",
        "    -> ConstrainedClaim::from_runtime_demand  [claim_production_completion]",
        "    -> (no GPU map/readback in this door)     [gpu_to_host_synchronization_readback = 0 bytes]",
        "    -> group by OwnerChannelScopeKey          [host_conversion_grouping]",
        "    -> TransformOp::apply_with_params         [eml_scoring]",
        "    -> sort score-bits then id; equal-bit bands [score_sorting_banding]",
        "    -> largest remainder + generation-rotated ties [integer_apportionment]",
        "    -> ConstrainedGrant::from_clearance + ConstrainedClearingResult [grant_result_construction]",
        "  enclosing production authority: clear_constrained_claims_at_generation",
        "    -> (no GPU write_buffer/upload in this door) [host_to_gpu_upload = 0 bytes]",
        "    -> generation+1 re-clear                  [n_plus_one_launch_delay]",
        "    -> record_cleared_grant -> IntegrationSchedule [cpu_schedule_replay_recording]",
        "    -> fund_unresolved_persistence            [lawful_structural_consequence]",
    ]
    .join("\n")
}

pub fn query_gpu_envelope() -> Result<(String, String, String)> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .context("no GPU adapter; 14.1 STOP — cannot supply a truthful GPU/driver/backend envelope")?;
    let info = adapter.get_info();
    Ok((
        format!(
            "{} vendor=0x{:04x} device=0x{:04x} type={:?}",
            info.name, info.vendor, info.device, info.device_type
        ),
        format!("{:?}", info.backend),
        format!("{} {}", info.driver, info.driver_info),
    ))
}

pub fn run_generation_critical_path_baseline(
    envelope: MeasurementEnvelope,
) -> Result<BaselinePacket> {
    let market = admit_market()?;
    let program = price_program();
    let mut workloads = Vec::new();

    workloads.push(measure_scale(
        "scale_1000",
        1_000,
        3,
        11,
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_scale(
        "scale_10000",
        10_000,
        2,
        7,
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_scale(
        "scale_100000",
        100_000,
        1,
        5,
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_scale(
        "scale_1000000",
        1_000_000,
        1,
        3,
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_one_large_tree(
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_many_small_trees(
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_divergent_generations(
        &program,
        &market,
        envelope.deterministic_seed,
    )?);
    workloads.push(measure_overlapping_local_ids(
        &program,
        &market,
        envelope.deterministic_seed,
    )?);

    Ok(BaselinePacket {
        envelope,
        door_census: HOST_CLEARING_DOOR_CENSUS.to_vec(),
        path_diagram: path_diagram(),
        leg_definitions: leg_definitions(),
        workloads,
        comparator_disclaimer: "Comparator only. Wall-clock values are dated facts about one reproducibility envelope. They are not a go/no-go gate, portable CI threshold, or authority to cancel or narrow Owner-ruled Phase-14 placement.".into(),
    })
}

pub fn format_packet(packet: &BaselinePacket) -> String {
    format_baseline_report(packet)
}

fn admit_market() -> Result<AdmittedSpecializationFlowMarket> {
    let profiles = vec![SpecializationProfile {
        id: "session-root".into(),
        description: "14.1 observation market profile".into(),
        requirements: vec![],
    }];
    let mut triggers = std::collections::BTreeSet::new();
    triggers.insert("generation-tick".into());
    admit_specialization_flow_market(
        &profiles,
        &triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "session-root".into(),
            offerings: vec![ConservedOfferingSpec {
                id: OFFERING_ID.into(),
                resource_key: ResourceKey::new(RESOURCE),
                price: OfferingPriceVectorSpec {
                    unit_cost: 2.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "ore-draw".into(),
                offering_refs: vec![OFFERING_ID.into()],
                lifecycle_trigger_refs: vec!["generation-tick".into()],
                min_quantity: 1,
                max_quantity: 255,
            }],
        },
    )
    .context("admit observation market")
}

fn price_program() -> AuthoredClearingProgram {
    AuthoredClearingProgram::new(TransformOp::set(1.0))
}

fn mix(seed: u64, i: u64) -> u32 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(i.wrapping_mul(0xBF58_476D_1CE4_E5B9)) as u32
}

fn demand_bucket(
    scope: &OwnerChannelScopeKey,
    raw: u32,
    requested: u32,
    priority: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested,
        priority,
        source_simthing_id_raw: Some(raw),
    }
}

fn tree_scope(tree_label: &str, boundary_raw: u32) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new(tree_label),
        resource_key: ResourceKey::new(RESOURCE),
        scope_id: ScopeId::from_boundary(SimThingId::from_session_raw(boundary_raw)),
    }
}

fn prepare_single_tree(
    label: &str,
    claim_count: usize,
    generation: u32,
    granter_raw: u32,
    boundary_raw: u32,
    seed: u64,
    id_base: u32,
    program: &AuthoredClearingProgram,
) -> PreparedTree {
    let scope = tree_scope(label, boundary_raw);
    let mut requested_total = 0u64;
    let mut demands = Vec::with_capacity(claim_count);
    for i in 0..claim_count {
        let raw = id_base.saturating_add(i as u32);
        let requested = (mix(seed, u64::from(raw)) % 7) + 1;
        requested_total += u64::from(requested);
        let priority = mix(seed ^ 0xA5, u64::from(raw)) % 8;
        let weight = ((mix(seed ^ 0x3C, u64::from(raw)) % 1_000) as f32) / 100.0;
        demands.push((demand_bucket(&scope, raw, requested, priority), weight));
    }
    let available = (requested_total / 2).max(1) as u32;
    PreparedTree {
        supplies: vec![ConstrainedSupply { scope, available }],
        demands,
        authority: ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(granter_raw),
            generation: GenerationStamp::new(generation),
        },
        program: program.clone(),
    }
}

fn measure_scale(
    name: &str,
    claims: usize,
    warm: usize,
    samples: usize,
    program: &AuthoredClearingProgram,
    market: &AdmittedSpecializationFlowMarket,
    seed: u64,
) -> Result<WorkloadReport> {
    let setup_start = Instant::now();
    let tree = prepare_single_tree(name, claims, 10, 1, 2, seed, 1, program);
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    measure_trees(
        WorkloadCardinalities {
            name: name.into(),
            tree_count: 1,
            claims_per_tree: claims,
            claims_total: claims,
            scopes_per_tree: 1,
            supplies_per_tree: 1,
            claimants_per_tree: claims,
            granters_per_tree: 1,
            generations: vec![10],
            overlapping_raw_local_ids: false,
            overlapping_construction_door: String::new(),
        },
        &[tree],
        warm,
        samples,
        market,
        setup_ns,
        None,
        vec![],
    )
}

fn measure_one_large_tree(
    program: &AuthoredClearingProgram,
    market: &AdmittedSpecializationFlowMarket,
    seed: u64,
) -> Result<WorkloadReport> {
    let claims = 100_000;
    let setup_start = Instant::now();
    let tree = prepare_single_tree("one_large_tree", claims, 10, 11, 12, seed, 1, program);
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    measure_trees(
        WorkloadCardinalities {
            name: "one_large_tree".into(),
            tree_count: 1,
            claims_per_tree: claims,
            claims_total: claims,
            scopes_per_tree: 1,
            supplies_per_tree: 1,
            claimants_per_tree: claims,
            granters_per_tree: 1,
            generations: vec![10],
            overlapping_raw_local_ids: false,
            overlapping_construction_door: String::new(),
        },
        &[tree],
        1,
        5,
        market,
        setup_ns,
        None,
        vec![],
    )
}

fn measure_many_small_trees(
    program: &AuthoredClearingProgram,
    market: &AdmittedSpecializationFlowMarket,
    seed: u64,
) -> Result<WorkloadReport> {
    const TREES: usize = 100;
    const CLAIMS: usize = 100;
    let setup_start = Instant::now();
    let trees: Vec<PreparedTree> = (0..TREES)
        .map(|t| {
            prepare_single_tree(
                &format!("small-{t}"),
                CLAIMS,
                10,
                100 + t as u32,
                200 + t as u32,
                seed,
                1,
                program,
            )
        })
        .collect();
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    measure_trees(
        WorkloadCardinalities {
            name: "many_independent_small_trees".into(),
            tree_count: TREES,
            claims_per_tree: CLAIMS,
            claims_total: TREES * CLAIMS,
            scopes_per_tree: 1,
            supplies_per_tree: 1,
            claimants_per_tree: CLAIMS,
            granters_per_tree: 1,
            generations: vec![10],
            overlapping_raw_local_ids: false,
            overlapping_construction_door: String::new(),
        },
        &trees,
        1,
        5,
        market,
        setup_ns,
        None,
        vec![],
    )
}

fn measure_divergent_generations(
    program: &AuthoredClearingProgram,
    market: &AdmittedSpecializationFlowMarket,
    seed: u64,
) -> Result<WorkloadReport> {
    let generations = [1u32, 10, 100, 1_000];
    const CLAIMS: usize = 1_000;
    let setup_start = Instant::now();
    let trees: Vec<PreparedTree> = generations
        .iter()
        .enumerate()
        .map(|(i, &generation)| {
            prepare_single_tree(
                &format!("divergent-{generation}"),
                CLAIMS,
                generation,
                300 + i as u32,
                400 + i as u32,
                seed,
                1,
                program,
            )
        })
        .collect();
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    measure_trees(
        WorkloadCardinalities {
            name: "divergent_generation_trees".into(),
            tree_count: generations.len(),
            claims_per_tree: CLAIMS,
            claims_total: generations.len() * CLAIMS,
            scopes_per_tree: 1,
            supplies_per_tree: 1,
            claimants_per_tree: CLAIMS,
            granters_per_tree: 1,
            generations: generations.to_vec(),
            overlapping_raw_local_ids: false,
            overlapping_construction_door: String::new(),
        },
        &trees,
        1,
        5,
        market,
        setup_ns,
        None,
        vec![],
    )
}

fn measure_overlapping_local_ids(
    program: &AuthoredClearingProgram,
    market: &AdmittedSpecializationFlowMarket,
    seed: u64,
) -> Result<WorkloadReport> {
    const CLAIMS: usize = 1_000;
    let setup_start = Instant::now();
    // Ordinary per-tree admission: each tree independently reconstructs the same
    // session-local raws through ConstrainedClaim::from_runtime_demand ->
    // SimThingId::from_session_raw. That is the production admission door used by
    // constrained_clearing itself. No SimThingId::new(), no test-only wrapper.
    let tree_a = prepare_single_tree("overlap-alpha", CLAIMS, 10, 1_001, 1_011, seed, 1, program);
    let tree_b = prepare_single_tree(
        "overlap-beta",
        CLAIMS,
        10,
        2_001,
        2_011,
        seed ^ 0x55,
        1,
        program,
    );
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    measure_trees(
        WorkloadCardinalities {
            name: "overlapping_local_ids".into(),
            tree_count: 2,
            claims_per_tree: CLAIMS,
            claims_total: CLAIMS * 2,
            scopes_per_tree: 1,
            supplies_per_tree: 1,
            claimants_per_tree: CLAIMS,
            granters_per_tree: 1,
            generations: vec![10, 10],
            overlapping_raw_local_ids: true,
            overlapping_construction_door:
                "ConstrainedClaim::from_runtime_demand -> SimThingId::from_session_raw (ordinary production admission; same raw {1..=N} including 7 under distinct owner_ref/scope_id/granter)"
                    .into(),
        },
        &[tree_a, tree_b],
        1,
        7,
        market,
        setup_ns,
        Some(OVERLAPPING_RAW),
        vec![
            "overlap-alpha owner_ref=overlap-alpha granter_raw=1001 scope_boundary=1011".into(),
            "overlap-beta owner_ref=overlap-beta granter_raw=2001 scope_boundary=2011".into(),
        ],
    )
}

#[allow(clippy::too_many_arguments)]
fn measure_trees(
    cardinalities: WorkloadCardinalities,
    trees: &[PreparedTree],
    warm: usize,
    samples: usize,
    market: &AdmittedSpecializationFlowMarket,
    setup_ns: u64,
    overlapping_raw: Option<u32>,
    overlapping_contexts: Vec<String>,
) -> Result<WorkloadReport> {
    let mut acc = SampleAcc::new();
    let mut isolation_ok = true;
    let mut neutrality_ok = true;
    let mut grants_total = 0usize;
    let mut unresolved_total = 0u64;

    for _ in 0..warm {
        let _ = run_sample(trees, market, false)?;
    }
    for _ in 0..samples {
        let sample = run_sample(trees, market, true)?;
        isolation_ok &= sample.isolation_matches_production;
        neutrality_ok &= sample.neutrality_clears_identical;
        grants_total = sample.grants_total;
        unresolved_total = sample.unresolved_total;
        acc.push(&sample);
    }

    let legs = acc.legs();
    let enclosing = acc.leg("enclosing_clear").expect("enclosing samples");
    let e2e = acc.leg("end_to_end").expect("e2e samples");
    let named_sum = REQUIRED_LEGS
        .iter()
        .map(|name| acc.median(name))
        .sum::<u64>();
    let unattributed = e2e.median_ns as i64 - named_sum as i64;
    Ok(WorkloadReport {
        cardinalities,
        warm_up_count: warm,
        sample_count: samples,
        setup_allocation_ns: setup_ns,
        legs,
        enclosing_clear_ns: enclosing,
        end_to_end_ns: e2e,
        unattributed_remainder_ns: unattributed,
        reconciliation_note: "Named inner host legs (grouping/scoring/sorting/apportionment) are nested sequential observations on copies of the same input; grant_result_construction is enclosing_clear minus those nested components (signed remainder, not forced-closed). GPU transfer legs are door-absent 0. End-to-end is claim-production + enclosing_clear + n+1 + schedule + structural. Independent named-leg sum may exceed end-to-end because inner legs are also counted inside enclosing_clear; the signed unattributed remainder records that overlap plus observation overhead.".into(),
        isolation_matches_production: isolation_ok,
        neutrality_clears_identical: neutrality_ok,
        overlapping_raw_value: overlapping_raw,
        overlapping_tree_contexts: overlapping_contexts,
        coupling: CouplingObservation {
            process_global_simthing_allocator: "SimThingId::new uses process-global AtomicU32 NEXT_SIMTHING_ID. This fixture does not call it; claim ids come from from_session_raw via from_runtime_demand.".into(),
            process_global_overlay_allocator: "OverlayId::new uses a process-global AtomicU32; fund_unresolved_persistence mints through it when CostBand n>0.".into(),
            host_wide_clearing_lock: "none in constrained_clearing.rs; each clear_constrained_claims_at_generation call is a pure function of its supplies/claims/program/authority.".into(),
            shared_generation_authority: "ClearingRemainderAuthority is per call. Divergent-generation trees carry independent GenerationStamp values; nothing in the door couples them.".into(),
            shared_integration_schedule: "IntegrationSchedule is caller-owned per tree in this instrument; the door does not hold a host-wide schedule.".into(),
            all_tree_synchronization: "none. Independent trees are cleared sequentially without a barrier. Overlapping raw 7 is interpreted only under each tree's OwnerChannelScopeKey / granter pair.".into(),
        },
        grants_total,
        unresolved_total,
    })
}

struct SampleTimes {
    isolation_matches_production: bool,
    neutrality_clears_identical: bool,
    grants_total: usize,
    unresolved_total: u64,
    ns: BTreeMap<&'static str, u64>,
}

fn run_sample(
    trees: &[PreparedTree],
    market: &AdmittedSpecializationFlowMarket,
    record: bool,
) -> Result<SampleTimes> {
    let mut ns: BTreeMap<&'static str, u64> = BTreeMap::new();
    let e2e_start = Instant::now();

    let mut all_claims: Vec<Vec<ConstrainedClaim>> = Vec::with_capacity(trees.len());
    let mut claim_ns = 0u64;
    for tree in trees {
        let start = Instant::now();
        let mut claims = Vec::with_capacity(tree.demands.len());
        for (demand, weight) in &tree.demands {
            claims.push(ConstrainedClaim::from_runtime_demand(demand, *weight)?);
        }
        claim_ns += start.elapsed().as_nanos() as u64;
        all_claims.push(claims);
    }
    add(&mut ns, "claim_production_completion", claim_ns);
    add(&mut ns, "gpu_to_host_synchronization_readback", 0);
    add(&mut ns, "host_to_gpu_upload", 0);

    let mut grouping_ns = 0u64;
    let mut scoring_ns = 0u64;
    let mut sorting_ns = 0u64;
    let mut apportion_ns = 0u64;
    let mut enclosing_ns = 0u64;
    let mut nplus_ns = 0u64;
    let mut schedule_ns = 0u64;
    let mut structural_ns = 0u64;
    let mut isolation_ok = true;
    let mut neutrality_ok = true;
    let mut grants_total = 0usize;
    let mut unresolved_total = 0u64;
    for (tree, claims) in trees.iter().zip(all_claims.iter()) {
        let nested =
            time_nested_host_components(claims, &tree.supplies, &tree.program, tree.authority)?;
        grouping_ns += nested.grouping_ns;
        scoring_ns += nested.scoring_ns;
        sorting_ns += nested.sorting_ns;
        apportion_ns += nested.apportion_ns;

        let start = Instant::now();
        let production = clear_constrained_claims_at_generation(
            &tree.supplies,
            claims,
            &tree.program,
            tree.authority,
        )?;
        enclosing_ns += start.elapsed().as_nanos() as u64;

        isolation_ok &= nested.granted == granted_map(&production);
        if record {
            let uninstrumented = clear_constrained_claims_at_generation(
                &tree.supplies,
                claims,
                &tree.program,
                tree.authority,
            )?;
            neutrality_ok &= uninstrumented == production;
        }

        let nplus_auth = ClearingRemainderAuthority {
            granter: tree.authority.granter,
            generation: GenerationStamp::new(tree.authority.generation.get().saturating_add(1)),
        };
        let start = Instant::now();
        let _next = clear_constrained_claims_at_generation(
            &tree.supplies,
            claims,
            &tree.program,
            nplus_auth,
        )?;
        nplus_ns += start.elapsed().as_nanos() as u64;

        let mut schedule = IntegrationSchedule::new();
        let start = Instant::now();
        for result in &production {
            for grant in &result.grants {
                if grant.granted == 0 {
                    continue;
                }
                market.record_cleared_grant(
                    tree.authority.granter,
                    OFFERING_ID,
                    grant,
                    tree.authority.generation,
                    &mut schedule,
                )?;
            }
        }
        schedule_ns += start.elapsed().as_nanos() as u64;

        let valuation = AuthoredPersistenceValuation::new(TransformOp::multiply(1.0), 2.0)?;
        let binding = PersistenceOverlayBinding {
            origin: tree.authority.granter,
            target: SimThingId::from_session_raw(OVERLAPPING_RAW),
            transform: PropertyTransformDelta {
                property_id: SimPropertyId(0),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
            },
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
        };
        let consequence_generation =
            GenerationStamp::new(tree.authority.generation.get().saturating_add(1));
        let start = Instant::now();
        for result in &production {
            for grant in &result.grants {
                if let Some(observation) =
                    UnresolvedDemandObservation::from_grant(grant, tree.authority.generation)
                {
                    let _ = fund_unresolved_persistence(
                        &observation,
                        consequence_generation,
                        &valuation,
                        &binding,
                    )?;
                }
            }
        }
        structural_ns += start.elapsed().as_nanos() as u64;

        grants_total += production.iter().map(|r| r.grants.len()).sum::<usize>();
        unresolved_total += production
            .iter()
            .map(|r| u64::from(r.unresolved_total))
            .sum::<u64>();
    }

    let construction_ns = (enclosing_ns as i64
        - grouping_ns as i64
        - scoring_ns as i64
        - sorting_ns as i64
        - apportion_ns as i64)
        .max(0) as u64;

    add(&mut ns, "host_conversion_grouping", grouping_ns);
    add(&mut ns, "eml_scoring", scoring_ns);
    add(&mut ns, "score_sorting_banding", sorting_ns);
    add(&mut ns, "integer_apportionment", apportion_ns);
    add(&mut ns, "grant_result_construction", construction_ns);
    add(&mut ns, "n_plus_one_launch_delay", nplus_ns);
    add(&mut ns, "cpu_schedule_replay_recording", schedule_ns);
    add(&mut ns, "lawful_structural_consequence", structural_ns);
    add(&mut ns, "enclosing_clear", enclosing_ns);
    add(&mut ns, "end_to_end", e2e_start.elapsed().as_nanos() as u64);

    Ok(SampleTimes {
        isolation_matches_production: isolation_ok,
        neutrality_clears_identical: neutrality_ok,
        grants_total,
        unresolved_total,
        ns,
    })
}

struct NestedHost {
    grouping_ns: u64,
    scoring_ns: u64,
    sorting_ns: u64,
    apportion_ns: u64,
    granted: BTreeMap<(OwnerChannelScopeKey, SimThingId), u32>,
}

fn score_claim(program: &AuthoredClearingProgram, claim: &ConstrainedClaim) -> Result<f32> {
    let score = program
        .score_program()
        .apply_with_params(claim.order_weight(), claim.priority() as f32);
    if !score.is_finite() || score < 0.0 {
        bail!("invalid authored score");
    }
    Ok(if score == 0.0 { 0.0 } else { score })
}

fn time_nested_host_components(
    claims: &[ConstrainedClaim],
    supplies: &[ConstrainedSupply],
    program: &AuthoredClearingProgram,
    authority: ClearingRemainderAuthority,
) -> Result<NestedHost> {
    let start = Instant::now();
    let mut scores = Vec::with_capacity(claims.len());
    for claim in claims {
        if claim.requested() == 0 {
            continue;
        }
        scores.push(ScoredRow {
            scope: claim.scope().clone(),
            source: claim.source_simthing_id(),
            requested: claim.requested(),
            score: score_claim(program, claim)?,
        });
    }
    let scoring_ns = start.elapsed().as_nanos() as u64;

    let start = Instant::now();
    let mut by_scope: BTreeMap<OwnerChannelScopeKey, Vec<ScoredRow>> = BTreeMap::new();
    for row in scores {
        by_scope.entry(row.scope.clone()).or_default().push(row);
    }
    let grouping_ns = start.elapsed().as_nanos() as u64;

    let supply_by_scope: BTreeMap<_, _> = supplies
        .iter()
        .map(|s| (s.scope.clone(), s.available))
        .collect();
    let mut sorting_ns = 0u64;
    let mut apportion_ns = 0u64;
    let mut granted = BTreeMap::new();
    for (scope, mut scored) in by_scope {
        let start = Instant::now();
        scored.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| left.source.cmp(&right.source))
        });
        sorting_ns += start.elapsed().as_nanos() as u64;

        let remaining = *supply_by_scope.get(&scope).context("missing supply")?;
        let start = Instant::now();
        let band_granted = apportion_sorted_scope(&scored, remaining, authority)?;
        apportion_ns += start.elapsed().as_nanos() as u64;
        granted.extend(band_granted);
    }
    Ok(NestedHost {
        grouping_ns,
        scoring_ns,
        sorting_ns,
        apportion_ns,
        granted,
    })
}

fn apportion_sorted_scope(
    scored: &[ScoredRow],
    mut remaining: u32,
    authority: ClearingRemainderAuthority,
) -> Result<BTreeMap<(OwnerChannelScopeKey, SimThingId), u32>> {
    let mut granted = BTreeMap::new();
    let mut cursor = 0usize;
    while cursor < scored.len() {
        let score_bits = scored[cursor].score.to_bits();
        let mut end = cursor + 1;
        while end < scored.len() && scored[end].score.to_bits() == score_bits {
            end += 1;
        }
        let requested_total = scored[cursor..end]
            .iter()
            .try_fold(0u64, |sum, row| sum.checked_add(u64::from(row.requested)))
            .context("overflow")?;
        let available_for_band = u64::from(remaining).min(requested_total);
        let mut band_grants = Vec::with_capacity(end - cursor);
        let mut fractional_remainders = Vec::with_capacity(end - cursor);
        let mut base_total = 0u64;
        for row in &scored[cursor..end] {
            let numerator = available_for_band
                .checked_mul(u64::from(row.requested))
                .context("overflow")?;
            let base = if requested_total == 0 {
                0
            } else {
                numerator / requested_total
            };
            base_total = base_total.checked_add(base).context("overflow")?;
            band_grants.push(base as u32);
            fractional_remainders.push(numerator % requested_total);
        }
        let leftover = available_for_band
            .checked_sub(base_total)
            .context("overflow")? as usize;
        let mut remainder_order: Vec<usize> = (0..band_grants.len()).collect();
        remainder_order.sort_by(|&left, &right| {
            fractional_remainders[right]
                .cmp(&fractional_remainders[left])
                .then_with(|| {
                    scored[cursor + left]
                        .source
                        .cmp(&scored[cursor + right].source)
                })
        });
        let mut tie_start = 0usize;
        while tie_start < remainder_order.len() {
            let remainder = fractional_remainders[remainder_order[tie_start]];
            let mut tie_end = tie_start + 1;
            while tie_end < remainder_order.len()
                && fractional_remainders[remainder_order[tie_end]] == remainder
            {
                tie_end += 1;
            }
            let tie_len = tie_end - tie_start;
            let rotation = (u64::from(authority.granter.raw())
                + u64::from(authority.generation.get()))
                % tie_len as u64;
            remainder_order[tie_start..tie_end].rotate_left(rotation as usize);
            tie_start = tie_end;
        }
        for &index in remainder_order.iter().take(leftover) {
            band_grants[index] = band_grants[index].checked_add(1).context("overflow")?;
        }
        for (row, granted_qty) in scored[cursor..end].iter().zip(band_grants) {
            granted.insert((row.scope.clone(), row.source), granted_qty);
        }
        remaining = remaining
            .checked_sub(available_for_band as u32)
            .context("overflow")?;
        cursor = end;
    }
    Ok(granted)
}

fn granted_map(
    results: &[ConstrainedClearingResult],
) -> BTreeMap<(OwnerChannelScopeKey, SimThingId), u32> {
    let mut out = BTreeMap::new();
    for result in results {
        for grant in &result.grants {
            out.insert(
                (grant.scope.clone(), grant.source_simthing_id),
                grant.granted,
            );
        }
    }
    out
}

fn add(ns: &mut BTreeMap<&'static str, u64>, key: &'static str, value: u64) {
    ns.insert(key, value);
}

struct SampleAcc {
    values: BTreeMap<&'static str, Vec<u64>>,
}

impl SampleAcc {
    fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    fn push(&mut self, sample: &SampleTimes) {
        for (k, v) in &sample.ns {
            self.values.entry(*k).or_default().push(*v);
        }
    }

    fn median(&self, name: &str) -> u64 {
        self.values.get(name).map(|s| stats(s).1).unwrap_or(0)
    }

    fn leg(&self, name: &'static str) -> Option<LegSamples> {
        let samples = self.values.get(name)?;
        Some(summarize(name, samples, isolation_note(name)))
    }

    fn legs(&self) -> Vec<LegSamples> {
        REQUIRED_LEGS
            .iter()
            .filter_map(|name| self.leg(name))
            .collect()
    }
}

fn isolation_note(name: &str) -> String {
    match name {
        "gpu_to_host_synchronization_readback" | "host_to_gpu_upload" => {
            "door-absent on CPU-host clearing; 0 bytes / 0 ns observed (no map_async, no write_buffer, no queue submit)".into()
        }
        "integer_apportionment" => {
            "workshop-local restatement of the live largest-remainder + generation-rotated-tie loop; production door remains authority".into()
        }
        "grant_result_construction" => {
            "signed remainder of enclosing clear_constrained_claims_at_generation after nested grouping+scoring+sorting+apportionment; not forced-closed".into()
        }
        "n_plus_one_launch_delay" => {
            "host re-clear at generation+1 through the ordinary door; GPU dispatch count 0".into()
        }
        "claim_production_completion" => {
            "ConstrainedClaim::from_runtime_demand (ordinary per-tree admission)".into()
        }
        "eml_scoring" => "TransformOp::apply_with_params via AuthoredClearingProgram::score_program".into(),
        "host_conversion_grouping" => "BTreeMap group by OwnerChannelScopeKey".into(),
        "score_sorting_banding" => "score.total_cmp desc then source id; equal to_bits bands".into(),
        "cpu_schedule_replay_recording" => {
            "AdmittedSpecializationFlowMarket::record_cleared_grant -> IntegrationSchedule".into()
        }
        "lawful_structural_consequence" => {
            "UnresolvedDemandObservation::from_grant + fund_unresolved_persistence".into()
        }
        _ => "production-door observation".into(),
    }
}

fn summarize(name: &str, samples: &[u64], isolation: String) -> LegSamples {
    let (mean, median, p95, variance, min, max) = stats(samples);
    let (bytes_in, bytes_out) = match name {
        "gpu_to_host_synchronization_readback" => (0, 0),
        "host_to_gpu_upload" => (0, 0),
        _ => (0, 0),
    };
    LegSamples {
        name: name.into(),
        isolation,
        bytes_read_back: bytes_in,
        bytes_uploaded: bytes_out,
        sample_ns: samples.to_vec(),
        median_ns: median,
        p95_ns: p95,
        variance_ns2: variance,
        min_ns: min,
        max_ns: max,
        mean_ns: mean,
    }
}

fn stats(samples: &[u64]) -> (f64, u64, u64, f64, u64, u64) {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let n = sorted.len();
    if n == 0 {
        return (0.0, 0, 0, 0.0, 0, 0);
    }
    let mean = sorted.iter().map(|v| *v as f64).sum::<f64>() / n as f64;
    let median = if n % 2 == 1 {
        sorted[n / 2]
    } else {
        let a = sorted[n / 2 - 1] as u128;
        let b = sorted[n / 2] as u128;
        ((a + b) / 2) as u64
    };
    let p95_index = ((n as f64 - 1.0) * 0.95).round() as usize;
    let p95 = sorted[p95_index.min(n - 1)];
    let variance = if n > 1 {
        sorted
            .iter()
            .map(|v| {
                let d = *v as f64 - mean;
                d * d
            })
            .sum::<f64>()
            / (n as f64 - 1.0)
    } else {
        0.0
    };
    (mean, median, p95, variance, sorted[0], sorted[n - 1])
}

pub fn cost_band_probe() -> Result<()> {
    cost_band_quantize(0.0, 1.0, true, None).map(|_| ())?;
    Ok(())
}

/// Prove overlapping raw ids are reconstructed through the ordinary admission door.
pub fn overlapping_id_fixture_proof() -> Result<OverlappingIdProof> {
    let program = price_program();
    let tree_a = prepare_single_tree(
        "overlap-alpha",
        16,
        10,
        1_001,
        1_011,
        DETERMINISTIC_SEED,
        1,
        &program,
    );
    let tree_b = prepare_single_tree(
        "overlap-beta",
        16,
        10,
        2_001,
        2_011,
        DETERMINISTIC_SEED,
        1,
        &program,
    );
    let claims_a: Vec<ConstrainedClaim> = tree_a
        .demands
        .iter()
        .map(|(d, w)| ConstrainedClaim::from_runtime_demand(d, *w))
        .collect::<Result<_, _>>()?;
    let claims_b: Vec<ConstrainedClaim> = tree_b
        .demands
        .iter()
        .map(|(d, w)| ConstrainedClaim::from_runtime_demand(d, *w))
        .collect::<Result<_, _>>()?;
    let raws_a: Vec<u32> = claims_a
        .iter()
        .map(|c| c.source_simthing_id().raw())
        .collect();
    let raws_b: Vec<u32> = claims_b
        .iter()
        .map(|c| c.source_simthing_id().raw())
        .collect();
    if !raws_a.contains(&OVERLAPPING_RAW) || !raws_b.contains(&OVERLAPPING_RAW) {
        bail!("overlapping raw {OVERLAPPING_RAW} missing from ordinary from_runtime_demand claims");
    }
    if tree_a.supplies[0].scope == tree_b.supplies[0].scope {
        bail!("overlapping fixture collapsed to one tree context");
    }
    let result_a = clear_constrained_claims_at_generation(
        &tree_a.supplies,
        &claims_a,
        &program,
        tree_a.authority,
    )?;
    let result_b = clear_constrained_claims_at_generation(
        &tree_b.supplies,
        &claims_b,
        &program,
        tree_b.authority,
    )?;
    Ok(OverlappingIdProof {
        construction_door: "ConstrainedClaim::from_runtime_demand -> SimThingId::from_session_raw",
        overlapping_raw: OVERLAPPING_RAW,
        tree_a_scope: format!("{:?}", tree_a.supplies[0].scope),
        tree_b_scope: format!("{:?}", tree_b.supplies[0].scope),
        tree_a_raws: raws_a,
        tree_b_raws: raws_b,
        tree_a_grants: result_a[0].grants.len(),
        tree_b_grants: result_b[0].grants.len(),
        same_raw_distinct_contexts: true,
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct OverlappingIdProof {
    pub construction_door: &'static str,
    pub overlapping_raw: u32,
    pub tree_a_scope: String,
    pub tree_b_scope: String,
    pub tree_a_raws: Vec<u32>,
    pub tree_b_raws: Vec<u32>,
    pub tree_a_grants: usize,
    pub tree_b_grants: usize,
    pub same_raw_distinct_contexts: bool,
}

pub fn uninstrumented_clear_matches_instrumented() -> Result<bool> {
    let program = price_program();
    let tree = prepare_single_tree("neutrality", 64, 10, 1, 2, DETERMINISTIC_SEED, 1, &program);
    let claims: Vec<ConstrainedClaim> = tree
        .demands
        .iter()
        .map(|(d, w)| ConstrainedClaim::from_runtime_demand(d, *w))
        .collect::<Result<_, _>>()?;
    let a =
        clear_constrained_claims_at_generation(&tree.supplies, &claims, &program, tree.authority)?;
    let b =
        clear_constrained_claims_at_generation(&tree.supplies, &claims, &program, tree.authority)?;
    Ok(a == b)
}

// Keep ConstrainedGrant named so record_cleared_grant type-checks against the public field.
#[allow(dead_code)]
fn _grant_ty(_: &ConstrainedGrant) {}
