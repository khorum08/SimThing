//! FrontierV2 multi-tick closed-loop consumer fixture support (test-only).

#[path = "frontier_v1.rs"]
mod frontier_v1;

pub use frontier_v1::*;

pub const FRONTIER_V2_FIXTURE_ID: &str = "frontier_v2_0_closed_loop_consumer_v1";
pub const FRONTIER_V2_1_FIXTURE_ID: &str = "frontier_v2_1_candidate_evolution_v1";
pub const FRONTIER_V2_2_FIXTURE_ID: &str = "frontier_v2_2_movement_feedback_application_v1";
pub const FRONTIER_V2_3_FIXTURE_ID: &str = "frontier_v2_3_structural_feedback_application_v1";
pub const FRONTIER_V2_CLOSED_LOOP_TICKS: u32 = 2;
pub const FRONTIER_V2_2_MOVEMENT_FEEDBACK_TICKS: u32 = 3;

/// Per-field status for FrontierV2-0 closed-loop reporting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2FieldStatus {
    GpuVerified,
    ReplayAccepted,
    FixtureCandidate,
    FixtureOnly,
    NotImplemented,
    Pending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2PhaseClosureStatus {
    NotDeclared,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2ClauseThingStatus {
    NotImplemented,
}

/// Classification for fixture-only movement/structural writes (not production SimThing state).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2WriteClassification {
    OwnColumnShadowWrite,
    BoundaryRequestShadowWrite,
    RejectedCrossEntity,
}

/// Fixture-only own-column shadow position (not production simthing-sim state).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2OwnColumnShadow {
    pub unit_id: u32,
    pub row: u32,
    pub col: u32,
    pub tick_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2MovementWriteError {
    CrossEntityTarget { source_unit_id: u32, shadow_unit_id: u32 },
}

/// Fixture-only BoundaryRequest shadow record (not production commitment emission).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2BoundaryRequestShadow {
    pub source_unit_id: u32,
    pub proposal_code: u32,
    pub boundary_request_code: u32,
    pub route_code: u32,
    pub dispatch_count: u32,
    pub tick_index: u32,
    pub applied: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2StructuralWriteError {
    InvalidRoute { route_code: u32 },
}

/// Fixture-only movement candidate tied to closed-loop feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2MovementCandidate {
    pub source_unit_id: u32,
    pub delta_row: i32,
    pub delta_col: i32,
    pub route_code: u32,
    pub dispatch_count: u32,
}

/// Fixture-only structural candidate tied to closed-loop feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2StructuralCandidate {
    pub proposal_code: u32,
    pub boundary_request_code: u32,
    pub route_code: u32,
    pub dispatch_count: u32,
}

/// One tick of the FrontierV2 closed-loop fixture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrontierV2TickRun {
    pub tick_index: u32,
    pub mapping_hash: u64,
    pub self_ai_hash: u64,
    pub proposal_dispatch_hash: u64,
    pub feedback: FrontierV1LiveSelfAiFeedbackCandidate,
    pub movement: FrontierV2MovementCandidate,
    pub structural: FrontierV2StructuralCandidate,
    pub threat: f32,
    pub urgency: f32,
    pub proposal_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2ClosedLoopSummary {
    pub tick0_mapping_hash: u64,
    pub tick1_mapping_hash: u64,
    pub tick0_self_ai_hash: u64,
    pub tick1_self_ai_hash: u64,
    pub feedback_candidate_hash: u64,
    pub closed_loop_delta_hash: u64,
    pub overflow_flags: u32,
    pub tick0_resource_route_status: FrontierV2FieldStatus,
    pub tick1_resource_route_status: FrontierV2FieldStatus,
    pub closed_loop_feedback_status: FrontierV2FieldStatus,
    pub resource_flow_status: FrontierV2FieldStatus,
    pub movement_candidate_status: FrontierV2FieldStatus,
    pub structural_candidate_status: FrontierV2FieldStatus,
    pub clause_thing_status: FrontierV2ClauseThingStatus,
    pub phase_closure_status: FrontierV2PhaseClosureStatus,
}

impl FrontierV2ClosedLoopSummary {
    pub fn combined_hex(&self) -> String {
        let combined = fnv_mix(self.tick0_mapping_hash)
            ^ fnv_mix(self.tick1_mapping_hash)
            ^ fnv_mix(self.tick0_self_ai_hash)
            ^ fnv_mix(self.tick1_self_ai_hash)
            ^ fnv_mix(self.feedback_candidate_hash)
            ^ fnv_mix(self.closed_loop_delta_hash)
            ^ fnv_mix(u64::from(self.overflow_flags));
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

pub fn frontier_v2_smoke_skeleton() -> FrontierV1ScenarioSkeleton {
    let mut skeleton = frontier_v1_1_smoke_skeleton();
    skeleton.profile_name = FRONTIER_V2_PROFILE_NAME;
    skeleton
}

pub fn validate_frontier_v2_admission(
    skeleton: &FrontierV1ScenarioSkeleton,
) -> FrontierV1AdmissionReport {
    if skeleton.profile_name != FRONTIER_V2_PROFILE_NAME {
        return FrontierV1AdmissionReport {
            accepted: false,
            mapping_ok: false,
            flat_star_ok: false,
            sead_v1_ok: false,
            coupling_ok: false,
            default_off_ok: false,
            rejected_reasons: vec!["profile_name must be FrontierV2"],
        };
    }
    validate_frontier_v1_admission(skeleton)
}

/// Apply tick-0 feedback as fixture-only next-tick seed deltas (no simthing-sim state).
pub fn apply_feedback_to_config(
    base: &FrontierV1FixtureConfig,
    feedback: &FrontierV1LiveSelfAiFeedbackCandidate,
) -> FrontierV1FixtureConfig {
    let seed_delta_a = feedback.faction_a_allocation / 8;
    let seed_delta_b = feedback.faction_b_allocation / 8;
    FrontierV1FixtureConfig {
        district_output_a: base
            .district_output_a
            .saturating_add(seed_delta_a)
            .min(base.source_cap),
        district_output_b: base
            .district_output_b
            .saturating_add(seed_delta_b)
            .min(base.source_cap),
        ..*base
    }
}

pub fn build_movement_candidate(
    feedback: &FrontierV1LiveSelfAiFeedbackCandidate,
) -> FrontierV2MovementCandidate {
    let imbalance =
        feedback.faction_a_allocation as i32 - feedback.faction_b_allocation as i32;
    let delta_row = imbalance.signum();
    let delta_col = if feedback.dispatch_count > 0 { 1 } else { 0 };
    FrontierV2MovementCandidate {
        source_unit_id: feedback.source_unit_id,
        delta_row,
        delta_col,
        route_code: FRONTIER_V1_MOVEMENT_ROUTE_CODE,
        dispatch_count: feedback.dispatch_count,
    }
}

pub fn build_structural_candidate(
    feedback: &FrontierV1LiveSelfAiFeedbackCandidate,
) -> FrontierV2StructuralCandidate {
    FrontierV2StructuralCandidate {
        proposal_code: feedback.proposal_code,
        boundary_request_code: feedback.field_feedback_code,
        route_code: FRONTIER_V1_STRUCTURAL_ROUTE_CODE,
        dispatch_count: feedback.dispatch_count,
    }
}

/// Tick-aware movement candidate evolution tied to closed-loop field/urgency feedback.
pub fn build_evolved_movement_candidate(
    feedback: &FrontierV1LiveSelfAiFeedbackCandidate,
    mapping_hash: u64,
    urgency: f32,
    tick_index: u32,
) -> FrontierV2MovementCandidate {
    let imbalance =
        feedback.faction_a_allocation as i32 - feedback.faction_b_allocation as i32;
    let row_delta = imbalance.signum() + tick_index as i32;
    let urgency_bucket = (urgency * 10.0).round() as u32;
    let col_delta = feedback
        .dispatch_count
        .saturating_add((mapping_hash & 0x7) as u32)
        .saturating_add(urgency_bucket % 4)
        .saturating_add(tick_index);
    FrontierV2MovementCandidate {
        source_unit_id: feedback.source_unit_id,
        delta_row: row_delta,
        delta_col: col_delta as i32,
        route_code: FRONTIER_V1_MOVEMENT_ROUTE_CODE,
        dispatch_count: feedback.dispatch_count,
    }
}

/// Tick-aware structural candidate evolution tied to closed-loop allocator/field feedback.
pub fn build_evolved_structural_candidate(
    feedback: &FrontierV1LiveSelfAiFeedbackCandidate,
    mapping_hash: u64,
    tick_index: u32,
) -> FrontierV2StructuralCandidate {
    let reinforce_bucket = feedback
        .allocator_total
        .saturating_div(10)
        .saturating_add(((mapping_hash >> 8) & 0xF) as u32)
        .saturating_add(tick_index.saturating_mul(100));
    FrontierV2StructuralCandidate {
        proposal_code: feedback.proposal_code.wrapping_add(tick_index),
        boundary_request_code: feedback
            .field_feedback_code
            .wrapping_add(reinforce_bucket),
        route_code: FRONTIER_V1_STRUCTURAL_ROUTE_CODE,
        dispatch_count: feedback.dispatch_count,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2CandidateEvolutionSummary {
    pub tick0_movement_hash: u64,
    pub tick1_movement_hash: u64,
    pub tick0_structural_hash: u64,
    pub tick1_structural_hash: u64,
    pub candidate_delta_hash: u64,
    pub closed_loop_delta_hash: u64,
    pub overflow_flags: u32,
    pub tick0_resource_route_status: FrontierV2FieldStatus,
    pub tick1_resource_route_status: FrontierV2FieldStatus,
    pub movement_evolution_status: FrontierV2FieldStatus,
    pub structural_evolution_status: FrontierV2FieldStatus,
    pub closed_loop_feedback_status: FrontierV2FieldStatus,
    pub clause_thing_status: FrontierV2ClauseThingStatus,
    pub phase_closure_status: FrontierV2PhaseClosureStatus,
}

impl FrontierV2CandidateEvolutionSummary {
    pub fn combined_hex(&self) -> String {
        let combined = fnv_mix(self.tick0_movement_hash)
            ^ fnv_mix(self.tick1_movement_hash)
            ^ fnv_mix(self.tick0_structural_hash)
            ^ fnv_mix(self.tick1_structural_hash)
            ^ fnv_mix(self.candidate_delta_hash)
            ^ fnv_mix(self.closed_loop_delta_hash)
            ^ fnv_mix(u64::from(self.overflow_flags));
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

pub fn hash_candidate_pair_delta(
    tick0_movement: FrontierV2MovementCandidate,
    tick1_movement: FrontierV2MovementCandidate,
    tick0_structural: FrontierV2StructuralCandidate,
    tick1_structural: FrontierV2StructuralCandidate,
) -> u64 {
    let mut h = fnv64(b"frontier_v2_1_candidate_delta");
    h = fnv_append_u32(h, tick0_movement.delta_row as u32);
    h = fnv_append_u32(h, tick1_movement.delta_row as u32);
    h = fnv_append_u32(h, tick0_movement.delta_col as u32);
    h = fnv_append_u32(h, tick1_movement.delta_col as u32);
    h = fnv_append_u32(h, tick0_structural.boundary_request_code);
    h = fnv_append_u32(h, tick1_structural.boundary_request_code);
    h = fnv_append_u32(h, tick0_structural.proposal_code);
    h = fnv_append_u32(h, tick1_structural.proposal_code);
    h
}

pub fn hash_tick_proposal_dispatch(
    proposal_count: u32,
    admission_code: u32,
    dispatch_count: u32,
) -> u64 {
    let mut h = fnv64(b"frontier_v2_tick_proposal_dispatch");
    h = fnv_append_u32(h, proposal_count);
    h = fnv_append_u32(h, admission_code);
    h = fnv_append_u32(h, dispatch_count);
    h
}

pub fn hash_movement_candidate(c: FrontierV2MovementCandidate) -> u64 {
    let mut h = fnv64(b"frontier_v2_movement");
    h = fnv_append_u32(h, c.source_unit_id);
    h = fnv_append_u32(h, c.delta_row as u32);
    h = fnv_append_u32(h, c.delta_col as u32);
    h = fnv_append_u32(h, c.route_code);
    h = fnv_append_u32(h, c.dispatch_count);
    h
}

pub fn hash_structural_candidate(c: FrontierV2StructuralCandidate) -> u64 {
    let mut h = fnv64(b"frontier_v2_structural");
    h = fnv_append_u32(h, c.proposal_code);
    h = fnv_append_u32(h, c.boundary_request_code);
    h = fnv_append_u32(h, c.route_code);
    h = fnv_append_u32(h, c.dispatch_count);
    h
}

pub fn hash_closed_loop_delta(tick0: &FrontierV2TickRun, tick1: &FrontierV2TickRun) -> u64 {
    let mut h = fnv64(b"frontier_v2_closed_loop_delta");
    h = fnv_append_u32(h, (tick0.mapping_hash ^ tick1.mapping_hash) as u32);
    h = fnv_append_u32(h, ((tick0.mapping_hash ^ tick1.mapping_hash) >> 32) as u32);
    h = fnv_append_u32(
        h,
        (tick0.proposal_dispatch_hash ^ tick1.proposal_dispatch_hash) as u32,
    );
    h = fnv_append_u32(
        h,
        ((tick0.proposal_dispatch_hash ^ tick1.proposal_dispatch_hash) >> 32) as u32,
    );
    h = fnv_append_u32(h, tick0.self_ai_hash as u32);
    h = fnv_append_u32(h, (tick0.self_ai_hash >> 32) as u32);
    h = fnv_append_u32(h, tick1.self_ai_hash as u32);
    h = fnv_append_u32(h, (tick1.self_ai_hash >> 32) as u32);
    h
}

fn fnv64(seed: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in seed {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn fnv_append_u32(mut hash: u64, v: u32) -> u64 {
    for b in v.to_le_bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn fnv_mix(v: u64) -> u64 {
    v.wrapping_mul(0x9E3779B97F4A7C15)
}

pub fn initial_own_column_shadow(unit_id: u32) -> FrontierV2OwnColumnShadow {
    FrontierV2OwnColumnShadow {
        unit_id,
        row: 0,
        col: 0,
        tick_index: 0,
    }
}

pub fn validate_movement_write_target(
    movement: &FrontierV2MovementCandidate,
    shadow: &FrontierV2OwnColumnShadow,
) -> Result<(), FrontierV2MovementWriteError> {
    if movement.source_unit_id != shadow.unit_id {
        return Err(FrontierV2MovementWriteError::CrossEntityTarget {
            source_unit_id: movement.source_unit_id,
            shadow_unit_id: shadow.unit_id,
        });
    }
    Ok(())
}

pub fn clamp_grid_coord(value: i32, grid_size: u32) -> u32 {
    let max = grid_size.saturating_sub(1) as i32;
    value.clamp(0, max) as u32
}

/// Apply movement candidate to own-column shadow state (fixture-only, not production state).
pub fn apply_movement_to_own_column_shadow(
    shadow: &FrontierV2OwnColumnShadow,
    movement: &FrontierV2MovementCandidate,
    grid_size: u32,
    tick_index: u32,
) -> Result<FrontierV2OwnColumnShadow, FrontierV2MovementWriteError> {
    validate_movement_write_target(movement, shadow)?;
    Ok(FrontierV2OwnColumnShadow {
        unit_id: shadow.unit_id,
        row: clamp_grid_coord(shadow.row as i32 + movement.delta_row, grid_size),
        col: clamp_grid_coord(shadow.col as i32 + movement.delta_col, grid_size),
        tick_index,
    })
}

pub fn source_seed_placement(
    config: &FrontierV1FixtureConfig,
    shadow: Option<&FrontierV2OwnColumnShadow>,
) -> ((u32, u32), (u32, u32)) {
    let row_b = config.grid_size.saturating_sub(1);
    let col_b = config.grid_size.saturating_sub(1);
    match shadow {
        Some(s) => ((s.row, s.col), (row_b, col_b)),
        None => ((0, 0), (row_b, col_b)),
    }
}

pub fn hash_own_column_shadow(shadow: FrontierV2OwnColumnShadow) -> u64 {
    let mut h = fnv64(b"frontier_v2_2_own_column_shadow");
    h = fnv_append_u32(h, shadow.unit_id);
    h = fnv_append_u32(h, shadow.row);
    h = fnv_append_u32(h, shadow.col);
    h = fnv_append_u32(h, shadow.tick_index);
    h
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2MovementFeedbackSummary {
    pub tick0_movement_hash: u64,
    pub tick1_movement_hash: u64,
    pub shadow_before_hash: u64,
    pub shadow_after_hash: u64,
    pub tick1_mapping_hash: u64,
    pub tick2_mapping_hash: u64,
    pub tick1_proposal_dispatch_hash: u64,
    pub tick2_proposal_dispatch_hash: u64,
    pub movement_feedback_delta_hash: u64,
    pub overflow_flags: u32,
    pub tick0_resource_route_status: FrontierV2FieldStatus,
    pub tick1_resource_route_status: FrontierV2FieldStatus,
    pub tick2_resource_route_status: FrontierV2FieldStatus,
    pub movement_evolution_status: FrontierV2FieldStatus,
    pub movement_application_status: FrontierV2WriteClassification,
    pub closed_loop_feedback_status: FrontierV2FieldStatus,
    pub structural_candidate_status: FrontierV2FieldStatus,
    pub clause_thing_status: FrontierV2ClauseThingStatus,
    pub phase_closure_status: FrontierV2PhaseClosureStatus,
}

impl FrontierV2MovementFeedbackSummary {
    pub fn combined_hex(&self) -> String {
        let combined = fnv_mix(self.tick0_movement_hash)
            ^ fnv_mix(self.tick1_movement_hash)
            ^ fnv_mix(self.shadow_before_hash)
            ^ fnv_mix(self.shadow_after_hash)
            ^ fnv_mix(self.tick1_mapping_hash)
            ^ fnv_mix(self.tick2_mapping_hash)
            ^ fnv_mix(self.movement_feedback_delta_hash)
            ^ fnv_mix(u64::from(self.overflow_flags));
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

pub fn hash_movement_feedback_delta(
    shadow_before: FrontierV2OwnColumnShadow,
    shadow_after: FrontierV2OwnColumnShadow,
    tick1_mapping_hash: u64,
    tick2_mapping_hash: u64,
) -> u64 {
    let mut h = fnv64(b"frontier_v2_2_movement_feedback_delta");
    h = fnv_append_u32(h, shadow_before.row);
    h = fnv_append_u32(h, shadow_before.col);
    h = fnv_append_u32(h, shadow_after.row);
    h = fnv_append_u32(h, shadow_after.col);
    h = fnv_append_u32(h, tick1_mapping_hash as u32);
    h = fnv_append_u32(h, (tick1_mapping_hash >> 32) as u32);
    h = fnv_append_u32(h, tick2_mapping_hash as u32);
    h = fnv_append_u32(h, (tick2_mapping_hash >> 32) as u32);
    h
}

pub fn apply_structural_to_boundary_request_shadow(
    structural: &FrontierV2StructuralCandidate,
    source_unit_id: u32,
    tick_index: u32,
) -> Result<FrontierV2BoundaryRequestShadow, FrontierV2StructuralWriteError> {
    if structural.route_code != FRONTIER_V1_STRUCTURAL_ROUTE_CODE {
        return Err(FrontierV2StructuralWriteError::InvalidRoute {
            route_code: structural.route_code,
        });
    }
    Ok(FrontierV2BoundaryRequestShadow {
        source_unit_id,
        proposal_code: structural.proposal_code,
        boundary_request_code: structural.boundary_request_code,
        route_code: structural.route_code,
        dispatch_count: structural.dispatch_count,
        tick_index,
        applied: true,
    })
}

pub fn derive_next_tick_structural_feedback_code(
    shadow: &FrontierV2BoundaryRequestShadow,
) -> u32 {
    shadow
        .boundary_request_code
        .wrapping_add(shadow.proposal_code)
        .wrapping_add(shadow.dispatch_count)
}

/// Apply fixture-only structural feedback as economy seed delta modifier (not production state).
pub fn apply_structural_feedback_to_config(
    base: &FrontierV1FixtureConfig,
    structural_feedback_code: u32,
) -> FrontierV1FixtureConfig {
    let seed_delta_a = (structural_feedback_code % 19).saturating_add(6);
    let seed_delta_b = (structural_feedback_code % 11).saturating_add(4);
    FrontierV1FixtureConfig {
        district_output_a: base
            .district_output_a
            .saturating_add(seed_delta_a)
            .min(base.source_cap),
        district_output_b: base
            .district_output_b
            .saturating_add(seed_delta_b)
            .min(base.source_cap),
        ..*base
    }
}

pub fn hash_boundary_request_shadow(shadow: FrontierV2BoundaryRequestShadow) -> u64 {
    let mut h = fnv64(b"frontier_v2_3_boundary_request_shadow");
    h = fnv_append_u32(h, shadow.source_unit_id);
    h = fnv_append_u32(h, shadow.proposal_code);
    h = fnv_append_u32(h, shadow.boundary_request_code);
    h = fnv_append_u32(h, shadow.route_code);
    h = fnv_append_u32(h, shadow.dispatch_count);
    h = fnv_append_u32(h, shadow.tick_index);
    h = fnv_append_u32(h, u32::from(shadow.applied));
    h
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV2StructuralFeedbackSummary {
    pub tick0_structural_hash: u64,
    pub tick1_structural_hash: u64,
    pub shadow_before_hash: u64,
    pub shadow_after_hash: u64,
    pub tick2_structural_feedback_code: u32,
    pub tick1_mapping_hash: u64,
    pub tick2_mapping_hash: u64,
    pub tick1_proposal_dispatch_hash: u64,
    pub tick2_proposal_dispatch_hash: u64,
    pub structural_feedback_delta_hash: u64,
    pub overflow_flags: u32,
    pub tick0_resource_route_status: FrontierV2FieldStatus,
    pub tick1_resource_route_status: FrontierV2FieldStatus,
    pub tick2_resource_route_status: FrontierV2FieldStatus,
    pub structural_evolution_status: FrontierV2FieldStatus,
    pub structural_application_status: FrontierV2WriteClassification,
    pub closed_loop_feedback_status: FrontierV2FieldStatus,
    pub movement_candidate_status: FrontierV2FieldStatus,
    pub clause_thing_status: FrontierV2ClauseThingStatus,
    pub phase_closure_status: FrontierV2PhaseClosureStatus,
}

impl FrontierV2StructuralFeedbackSummary {
    pub fn combined_hex(&self) -> String {
        let combined = fnv_mix(self.tick0_structural_hash)
            ^ fnv_mix(self.tick1_structural_hash)
            ^ fnv_mix(self.shadow_before_hash)
            ^ fnv_mix(self.shadow_after_hash)
            ^ fnv_mix(self.tick1_mapping_hash)
            ^ fnv_mix(self.tick2_mapping_hash)
            ^ fnv_mix(self.structural_feedback_delta_hash)
            ^ fnv_mix(u64::from(self.overflow_flags))
            ^ fnv_mix(u64::from(self.tick2_structural_feedback_code));
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

pub fn hash_structural_feedback_delta(
    shadow_before: FrontierV2BoundaryRequestShadow,
    shadow_after: FrontierV2BoundaryRequestShadow,
    structural_feedback_code: u32,
    tick1_mapping_hash: u64,
    tick2_mapping_hash: u64,
) -> u64 {
    let mut h = fnv64(b"frontier_v2_3_structural_feedback_delta");
    h = fnv_append_u32(h, shadow_before.boundary_request_code);
    h = fnv_append_u32(h, shadow_after.boundary_request_code);
    h = fnv_append_u32(h, structural_feedback_code);
    h = fnv_append_u32(h, tick1_mapping_hash as u32);
    h = fnv_append_u32(h, (tick1_mapping_hash >> 32) as u32);
    h = fnv_append_u32(h, tick2_mapping_hash as u32);
    h = fnv_append_u32(h, (tick2_mapping_hash >> 32) as u32);
    h
}

pub fn empty_boundary_request_shadow(source_unit_id: u32) -> FrontierV2BoundaryRequestShadow {
    FrontierV2BoundaryRequestShadow {
        source_unit_id,
        proposal_code: 0,
        boundary_request_code: 0,
        route_code: 0,
        dispatch_count: 0,
        tick_index: 0,
        applied: false,
    }
}
