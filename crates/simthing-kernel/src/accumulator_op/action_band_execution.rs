//! Sparse, domain-free GPU execution for admitted ActionBand templates.
//!
//! This module owns numerical ActionBand state.  The CPU supplies immutable
//! descriptor tables at session build and the already-sealed Phase-5
//! [`BandCrossingDelta`] rows at a tick boundary; it never compares thresholds,
//! re-evaluates an ActionBand result, or chooses an emission destination.

use std::collections::HashSet;
use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::{ColumnIndex, EmlNodeGpu, SimPropertyId, SimThingId, SlotIndex, SubFieldRole};
use thiserror::Error;
use wgpu::util::DeviceExt;

use super::{
    FacilityPlaneError, FacilityPlaneGenerationBoundary, FacilityPlaneOwner, FacilityResidentPlane,
};
use crate::sealed::{ThresholdEmission, ThresholdEmissionGpu};
use crate::{
    debug_readback_allowed, BandCrossingDelta, BandCrossingDirection, BoundaryEmissionToken,
    DecisionIngressError, EmissionToken, EmlTreeRangeGpu, GpuContext, StructuralCommitment,
    ThresholdCrossingToken,
};

pub const ACTIONBAND_NO_PROGRAM: u32 = u32::MAX;
pub const ACTIONBAND_INSTANCE_INITIALLY_ACTIVE: u32 = 1;
pub const ACTIONBAND_INSTANCE_SUBORDINATE: u32 = 1 << 1;
pub const ACTIONBAND_STATE_ACTIVE: u32 = 1;

pub mod target_kind {
    pub const POINT: u32 = 0;
    pub const SCALAR_AT_LEAST: u32 = 1;
    pub const SCALAR_AT_MOST: u32 = 2;
    pub const INTERVAL: u32 = 3;
    pub const AXIS_ALIGNED_BOX: u32 = 4;
    pub const LOCUS_RADIUS: u32 = 5;
    pub const PALMA_REACHABLE_SET: u32 = 6;
    pub const EML_PROJECTED_SET: u32 = 7;
}

/// Closed set of generic surfaces which an admitted binding may target.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionBandEmissionDestination {
    PropertyNext = 0,
    RfClaim = 1,
    CostBand = 2,
    OverlayEvent = 3,
    StructuralRequest = 4,
    Telemetry = 5,
}

impl ActionBandEmissionDestination {
    fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::PropertyNext,
            1 => Self::RfClaim,
            2 => Self::CostBand,
            3 => Self::OverlayEvent,
            4 => Self::StructuralRequest,
            5 => Self::Telemetry,
            _ => return None,
        })
    }
}

/// Existing world-value write semantics used by a fixed PropertyNext binding.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionBandPropertyWrite {
    Set = 0,
    Add = 1,
}

/// One row from the existing pre-admitted generic emission-binding table.
/// Destination kind is closed; ActionBand EML cannot manufacture or alter it.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ActionBandEmissionBindingGpu {
    destination_kind: u32,
    destination_index: u32,
    auxiliary0: u32,
    auxiliary1: u32,
}

impl ActionBandEmissionBindingGpu {
    pub const CONSERVED_BOUND_NONE: u32 = 0;
    pub const CONSERVED_BOUND_RF_GRANT: u32 = 1;
    pub const CONSERVED_BOUND_GU_YANG_AVAILABLE: u32 = 2;
    pub const CONSERVED_BOUND_GU_YANG_REALIZED: u32 = 3;

    fn fixed(
        destination: ActionBandEmissionDestination,
        destination_index: u32,
        auxiliary0: u32,
    ) -> Self {
        Self {
            destination_kind: destination as u32,
            destination_index,
            auxiliary0,
            auxiliary1: 0,
        }
    }

    /// Bind a payload to the existing authoritative world-value lane.
    pub fn property_next(column: u32, write: ActionBandPropertyWrite) -> Self {
        Self::fixed(
            ActionBandEmissionDestination::PropertyNext,
            column,
            write as u32,
        )
    }

    /// Bind a payload as an ordinary additive RF claim column write.
    pub fn rf_claim(column: u32) -> Self {
        Self::fixed(ActionBandEmissionDestination::RfClaim, column, 1)
    }

    /// Bind a payload to an ordinary scalar CostBand input/progress column.
    pub fn cost_band(column: u32) -> Self {
        Self::fixed(ActionBandEmissionDestination::CostBand, column, 1)
    }

    /// Bind to the existing sealed threshold/event packet surface.
    pub fn overlay_event(event_index: u32) -> Self {
        Self::fixed(ActionBandEmissionDestination::OverlayEvent, event_index, 0)
    }

    /// Bind to a pre-admitted structural request at the existing boundary.
    pub fn structural_request(request_index: u32) -> Self {
        Self::fixed(
            ActionBandEmissionDestination::StructuralRequest,
            request_index,
            0,
        )
    }

    /// Bind to the existing sealed event/telemetry packet surface.
    pub fn telemetry(event_index: u32) -> Self {
        Self::fixed(ActionBandEmissionDestination::Telemetry, event_index, 0)
    }

    pub fn destination(self) -> ActionBandEmissionDestination {
        ActionBandEmissionDestination::from_raw(self.destination_kind)
            .expect("constructed from closed destination enum")
    }

    pub fn destination_index(self) -> u32 {
        self.destination_index
    }

    /// Lower one already-admitted conserved-progress source into the existing
    /// reserved binding word. Table validation keeps this wire code closed.
    pub fn with_conserved_progress_bound_source(mut self, source: u32) -> Self {
        self.auxiliary1 = source;
        self
    }

    pub fn conserved_progress_bound_source(self) -> u32 {
        self.auxiliary1
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandTemplateGpu {
    pub target_kind: u32,
    pub channel_start: u32,
    pub channel_count: u32,
    pub target_data_start: u32,
    pub projection_width: u32,
    pub band_start: u32,
    pub band_count: u32,
    pub membership_range: u32,
    pub projection_range: u32,
    pub velocity_current_channel: u32,
    pub velocity_previous_channel: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandBandGpu {
    pub threshold_registration: u32,
    pub program_range: u32,
    pub binding_start: u32,
    pub binding_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandActiveInstanceGpu {
    pub slot: u32,
    pub template_index: u32,
    pub projection_start: u32,
    pub generation: u32,
    pub params: [f32; 4],
    pub dependency_start: u32,
    pub dependency_count: u32,
    pub flags: u32,
    pub reserved: u32,
}

/// One session-built edge from an instance to a pre-admitted child row. The
/// row is numeric and stable for the session; it is never appended at runtime.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ActionBandDependencyGpu {
    pub child_instance_row: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ActionBandStateGpu {
    pub satisfied: u32,
    pub generation: u32,
    pub projection_start: u32,
    pub projection_len: u32,
    pub distance: f32,
    pub velocity: f32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
struct ActionBandCrossingInputGpu {
    instance_row: u32,
    band_index: u32,
    output_start: u32,
    output_count: u32,
    post_value: f32,
    threshold: f32,
    crossing_col: u32,
    reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ActionBandDispatchParams {
    n_dims: u32,
    instance_count: u32,
    crossing_start: u32,
    crossing_count: u32,
}

/// Label-free deterministic bucket formed from program and binding shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBandExecutionBucket {
    pub program_range: u32,
    pub destination_shape: Vec<ActionBandEmissionDestination>,
    pub band_indices: Vec<u32>,
}

/// Immutable, domain-free GPU tables compiled from the frozen admission product.
#[derive(Clone, Debug)]
pub struct ActionBandExecutionPlan {
    templates: Vec<ActionBandTemplateGpu>,
    target_channels: Vec<u32>,
    target_data: Vec<f32>,
    bands: Vec<ActionBandBandGpu>,
    band_binding_indices: Vec<u32>,
    emission_bindings: Vec<ActionBandEmissionBindingGpu>,
    eml_nodes: Vec<EmlNodeGpu>,
    eml_ranges: Vec<EmlTreeRangeGpu>,
    active_instances: Vec<ActionBandActiveInstanceGpu>,
    dependencies: Vec<ActionBandDependencyGpu>,
    buckets: Vec<ActionBandExecutionBucket>,
    band_to_bucket: Vec<u32>,
    projection_floats: u32,
    depth1_crossing_fast_path: bool,
    depth2_common_fast_shape: bool,
    native_destinations: bool,
    fingerprint: u64,
}

impl ActionBandExecutionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted_numeric_tables(
        templates: Vec<ActionBandTemplateGpu>,
        target_channels: Vec<u32>,
        target_data: Vec<f32>,
        bands: Vec<ActionBandBandGpu>,
        band_binding_indices: Vec<u32>,
        emission_bindings: Vec<ActionBandEmissionBindingGpu>,
        eml_nodes: Vec<EmlNodeGpu>,
        eml_ranges: Vec<EmlTreeRangeGpu>,
        mut active_instances: Vec<ActionBandActiveInstanceGpu>,
        dependencies: Vec<ActionBandDependencyGpu>,
        buckets: Vec<ActionBandExecutionBucket>,
        reserved_instance_rows: u32,
        depth1_crossing_fast_path: bool,
        native_destinations: bool,
    ) -> Result<Self, ActionBandExecutionError> {
        if active_instances.len() > reserved_instance_rows as usize {
            return Err(ActionBandExecutionError::SparseRowBudgetExceeded {
                active: active_instances.len(),
                reserved: reserved_instance_rows,
            });
        }
        active_instances.sort_by_key(|row| (row.template_index, row.slot));
        if active_instances.windows(2).any(|rows| {
            (rows[0].template_index, rows[0].slot) == (rows[1].template_index, rows[1].slot)
        }) {
            return Err(ActionBandExecutionError::DuplicateActiveInstance);
        }

        let mut projection_floats = 0u32;
        for instance in &mut active_instances {
            let template = templates.get(instance.template_index as usize).ok_or(
                ActionBandExecutionError::UnknownTemplate(instance.template_index),
            )?;
            instance.projection_start = projection_floats;
            projection_floats = projection_floats
                .checked_add(template.projection_width)
                .ok_or(ActionBandExecutionError::TableOverflow)?;
        }

        validate_tables(
            &templates,
            &target_channels,
            &target_data,
            &bands,
            &band_binding_indices,
            &emission_bindings,
            &eml_ranges,
            &active_instances,
            &dependencies,
            native_destinations,
        )?;
        if !dependencies.is_empty() && !depth1_crossing_fast_path {
            return Err(ActionBandExecutionError::RecursiveShapeDeferred);
        }
        let mut band_to_bucket = vec![u32::MAX; bands.len()];
        for (bucket_index, bucket) in buckets.iter().enumerate() {
            for &band_index in &bucket.band_indices {
                let Some(slot) = band_to_bucket.get_mut(band_index as usize) else {
                    return Err(ActionBandExecutionError::InvalidBucketPartition);
                };
                if *slot != u32::MAX {
                    return Err(ActionBandExecutionError::InvalidBucketPartition);
                }
                *slot = bucket_index as u32;
            }
        }
        if band_to_bucket.iter().any(|&index| index == u32::MAX) {
            return Err(ActionBandExecutionError::InvalidBucketPartition);
        }
        let mut fingerprint = plan_fingerprint(
            &templates,
            &target_channels,
            &target_data,
            &bands,
            &band_binding_indices,
            &emission_bindings,
            &eml_nodes,
            &eml_ranges,
            &active_instances,
            &dependencies,
        );
        if depth1_crossing_fast_path {
            fingerprint ^= 0xD1F4_57A7_5EA1_ED01;
        }
        let depth2_common_fast_shape = !dependencies.is_empty();
        if depth2_common_fast_shape {
            fingerprint ^= 0xD2C0_6D00_0000_0001;
        }
        if native_destinations {
            fingerprint ^= 0xA73A_71E0_0000_0001;
        }
        Ok(Self {
            templates,
            target_channels,
            target_data,
            bands,
            band_binding_indices,
            emission_bindings,
            eml_nodes,
            eml_ranges,
            active_instances,
            dependencies,
            buckets,
            band_to_bucket,
            projection_floats,
            depth1_crossing_fast_path,
            depth2_common_fast_shape,
            native_destinations,
            fingerprint,
        })
    }

    pub fn active_instance_rows(&self) -> usize {
        self.active_instances.len()
    }

    pub fn hot_state_bytes(&self) -> usize {
        self.active_instances.len() * std::mem::size_of::<ActionBandStateGpu>()
            + self.projection_floats as usize * std::mem::size_of::<f32>()
    }

    pub fn buckets(&self) -> &[ActionBandExecutionBucket] {
        &self.buckets
    }

    pub fn numeric_fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// True only when every active row is a one-band, direct single-channel
    /// target with no velocity gather. Such rows consume the sealed crossing's
    /// already-hot value directly in the emission dispatch.
    pub fn uses_depth1_crossing_fast_path(&self) -> bool {
        self.depth1_crossing_fast_path
    }

    /// The recursive common shape reuses the depth-1 crossing entry and the
    /// same descriptor/EML tables; only the flat dependency rows are added.
    pub fn uses_depth2_common_fast_shape(&self) -> bool {
        self.depth2_common_fast_shape
    }

    pub fn dependency_row_count(&self) -> usize {
        self.dependencies.len()
    }

    /// The only ActionBand crossing bridge. Input evidence is the existing
    /// sealed Phase-5 product; this method performs joins only, never compares.
    pub fn crossings_from_sealed(
        &self,
        deltas: &[BandCrossingDelta],
    ) -> Result<ActionBandCrossingBatch, ActionBandExecutionError> {
        let mut joined = Vec::new();
        for delta in deltas {
            for (band_index, band) in self.bands.iter().enumerate() {
                if band.threshold_registration != delta.reg_idx() {
                    continue;
                }
                for (instance_row, instance) in self.active_instances.iter().enumerate() {
                    let template = &self.templates[instance.template_index as usize];
                    let in_template = band_index as u32 >= template.band_start
                        && (band_index as u32) < template.band_start + template.band_count;
                    if in_template && instance.slot == delta.slot().raw() {
                        joined.push((
                            self.band_to_bucket[band_index],
                            ActionBandCrossingInputGpu {
                                instance_row: instance_row as u32,
                                band_index: band_index as u32,
                                output_start: 0,
                                output_count: band.binding_count,
                                post_value: delta.post_value(),
                                threshold: delta.threshold(),
                                crossing_col: delta.col().raw() as u32,
                                reserved: 0,
                            },
                            delta.clone(),
                        ));
                    }
                }
            }
        }
        joined.sort_by_key(|(bucket, row, _)| (*bucket, row.band_index, row.instance_row));
        if self.depth1_crossing_fast_path {
            let mut instance_rows = joined
                .iter()
                .map(|(_, row, _)| row.instance_row)
                .collect::<Vec<_>>();
            instance_rows.sort_unstable();
            if instance_rows.windows(2).any(|rows| rows[0] == rows[1]) {
                return Err(ActionBandExecutionError::DuplicateDepth1Crossing);
            }
        }
        let mut rows = Vec::with_capacity(joined.len());
        let mut output_count = 0u32;
        let mut commitment_inputs = Vec::new();
        let mut consumption_keys = Vec::new();
        let mut seen_consumption_keys = HashSet::new();
        let mut bucket_ranges = Vec::new();
        for (bucket_index, mut row, delta) in joined {
            let consumption_key = ActionBandCrossingConsumptionKey {
                plan_fingerprint: self.fingerprint,
                generation: delta.generation(),
                reg_idx: delta.reg_idx(),
                sim_thing_id: delta.sim_thing_id(),
                property_id: delta.property_id(),
                role: delta.role().clone(),
                slot: delta.slot(),
                col: delta.col(),
                threshold_bits: delta.threshold().to_bits(),
                direction: delta.direction(),
                post_value_bits: delta.post_value().to_bits(),
                event_kind: delta.event_kind(),
            };
            if seen_consumption_keys.insert(consumption_key.clone()) {
                consumption_keys.push(consumption_key);
            }
            if bucket_ranges
                .last()
                .is_none_or(|range: &ActionBandBucketDispatch| range.bucket_index != bucket_index)
            {
                bucket_ranges.push(ActionBandBucketDispatch {
                    bucket_index,
                    crossing_start: rows.len() as u32,
                    crossing_count: 0,
                });
            }
            row.output_start = output_count;
            let band = &self.bands[row.band_index as usize];
            for local_index in 0..band.binding_count {
                let binding_index =
                    self.band_binding_indices[(band.binding_start + local_index) as usize] as usize;
                if self.emission_bindings[binding_index].destination()
                    == ActionBandEmissionDestination::StructuralRequest
                {
                    commitment_inputs.push((output_count + local_index, delta.clone()));
                }
            }
            output_count = output_count
                .checked_add(row.output_count)
                .ok_or(ActionBandExecutionError::TableOverflow)?;
            rows.push(row);
            bucket_ranges
                .last_mut()
                .expect("range just inserted")
                .crossing_count += 1;
        }
        Ok(ActionBandCrossingBatch {
            rows,
            output_count,
            commitment_inputs,
            consumption_keys,
            bucket_ranges,
            plan_fingerprint: self.fingerprint,
        })
    }
}

/// Opaque batch that cannot be forged from a second comparator or raw integers.
#[derive(Debug)]
pub struct ActionBandCrossingBatch {
    rows: Vec<ActionBandCrossingInputGpu>,
    output_count: u32,
    commitment_inputs: Vec<(u32, BandCrossingDelta)>,
    consumption_keys: Vec<ActionBandCrossingConsumptionKey>,
    bucket_ranges: Vec<ActionBandBucketDispatch>,
    plan_fingerprint: u64,
}

/// Opaque semantic identity for one sealed Phase-5 crossing consumed by one
/// frozen ActionBand plan. A later real crossing has a distinct generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ActionBandCrossingConsumptionKey {
    plan_fingerprint: u64,
    generation: u32,
    reg_idx: u32,
    sim_thing_id: SimThingId,
    property_id: SimPropertyId,
    role: SubFieldRole,
    slot: SlotIndex,
    col: ColumnIndex,
    threshold_bits: u32,
    direction: BandCrossingDirection,
    post_value_bits: u32,
    event_kind: u32,
}

impl ActionBandCrossingConsumptionKey {
    /// Generation of the sealed Phase-5 crossing represented by this key.
    /// Consumers compare this stamp with their executable generation; it is
    /// not a request to retain crossing history.
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug)]
struct ActionBandBucketDispatch {
    bucket_index: u32,
    crossing_start: u32,
    crossing_count: u32,
}

impl ActionBandCrossingBatch {
    pub fn crossing_count(&self) -> usize {
        self.rows.len()
    }

    pub fn emission_count(&self) -> usize {
        self.output_count as usize
    }

    pub fn bucket_dispatch_count(&self) -> usize {
        self.bucket_ranges.len()
    }

    pub fn consumption_keys(&self) -> &[ActionBandCrossingConsumptionKey] {
        &self.consumption_keys
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBandExecutionReadback {
    pub states: Vec<ActionBandStateGpu>,
    pub projection: Vec<f32>,
    pub commitments: Vec<StructuralCommitment>,
    /// Proof-only observation of GPU EML results. Production structural
    /// selection never reads these values.
    pub emission_payloads: Vec<f32>,
    /// Sum of ActionBand compute-pass GPU timestamps. `None` when the adapter
    /// does not expose timestamp queries; excludes the separately measured
    /// state carry and CPU readback.
    pub gpu_time_ns: Option<f64>,
    pub carry_gpu_time_ns: Option<f64>,
    pub evaluation_gpu_time_ns: Option<f64>,
    pub emission_gpu_time_ns: Option<f64>,
}

/// Production dispatch result. Numerical state and projections stay resident;
/// only sealed structural commitments cross the CPU boundary in rung 7.2.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBandProductionDispatch {
    pub commitments: Vec<StructuralCommitment>,
    pub bucket_dispatches: u32,
    pub gpu_time_ns: Option<f64>,
    pub carry_gpu_time_ns: Option<f64>,
    pub evaluation_gpu_time_ns: Option<f64>,
    pub emission_gpu_time_ns: Option<f64>,
}

#[derive(Debug, Error)]
pub enum ActionBandExecutionError {
    #[error("active ActionBand rows {active} exceed frozen reservation {reserved}")]
    SparseRowBudgetExceeded { active: usize, reserved: u32 },
    #[error("duplicate active ActionBand template/slot row")]
    DuplicateActiveInstance,
    #[error("ActionBand instance references unknown template {0}")]
    UnknownTemplate(u32),
    #[error("ActionBand descriptor table span is out of bounds")]
    InvalidTableSpan,
    #[error("ActionBand bucket rows do not form a total disjoint band partition")]
    InvalidBucketPartition,
    #[error("ActionBand table size overflow")]
    TableOverflow,
    #[error("crossing batch belongs to a different immutable ActionBand plan")]
    ForeignCrossingBatch,
    #[error("ActionBand destination column {column} is outside world width {n_dims}")]
    DestinationColumnOutOfBounds { column: u32, n_dims: u32 },
    #[error("ActionBand 7.2 defers destination {destination:?}")]
    DestinationDeferred {
        destination: ActionBandEmissionDestination,
    },
    #[error("ActionBand 7.2 admits exactly one structural binding per band, found {count}")]
    StructuralBindingCount { count: u32 },
    #[error("ActionBand native destinations require an ordinary external next-state buffer")]
    NativeNextRequired,
    #[error("ActionBand facility-local resident Next plane was not admitted at session build")]
    ResidentNextNotAdmitted,
    #[error("ActionBand current/next buffers differ in size")]
    NativeNextSizeMismatch,
    #[error("ActionBand native bindings would create more than one writer for slot {slot}, column {column}")]
    NativeDestinationCollision { slot: u32, column: u32 },
    #[error("depth-1 fast path received more than one sealed crossing for one active instance")]
    DuplicateDepth1Crossing,
    #[error("ActionBand recursive dependencies require the shared depth-1/2 fast shape")]
    RecursiveShapeDeferred,
    #[error("GPU structural packet does not preserve sealed crossing identity")]
    StructuralPacketIdentityMismatch,
    #[error(transparent)]
    DecisionIngress(#[from] DecisionIngressError),
    #[error("ActionBand numerical readback is disabled outside an explicit proof scope")]
    ProofReadbackDisabled,
    #[error("GPU readback map failed")]
    MapFailed,
    #[error("ActionBand shader source markers are missing")]
    ShaderSourceMarkersMissing,
    #[error(transparent)]
    FacilityPlane(#[from] FacilityPlaneError),
}

/// Zero-row execution has no GPU buffers and therefore zero hot bytes.
pub enum ActionBandGpuExecution {
    Inactive,
    Active(ActionBandGpuSession),
}

impl ActionBandGpuExecution {
    pub fn new(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
    ) -> Result<Self, ActionBandExecutionError> {
        if plan.active_instances.is_empty() {
            return Ok(Self::Inactive);
        }
        Ok(Self::Active(ActionBandGpuSession::new(ctx, plan)?))
    }

    /// Open the 7.8 consequence posture with one facility-local resident
    /// Current/Next plane. The plane is admitted under the ActionBand session's
    /// existing generation boundary; callers never receive either buffer or an
    /// owner capability.
    pub fn new_with_resident_next(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
        resident_values: &[f32],
    ) -> Result<Self, ActionBandExecutionError> {
        if plan.active_instances.is_empty() {
            return Ok(Self::Inactive);
        }
        Ok(Self::Active(ActionBandGpuSession::new_inner(
            ctx,
            plan,
            Some(resident_values),
        )?))
    }

    pub fn active_instance_rows(&self) -> usize {
        match self {
            Self::Inactive => 0,
            Self::Active(session) => session.plan.active_instance_rows(),
        }
    }

    pub fn hot_state_bytes(&self) -> usize {
        match self {
            Self::Inactive => 0,
            Self::Active(session) => session.plan.hot_state_bytes(),
        }
    }
}

pub struct ActionBandGpuSession {
    plan: ActionBandExecutionPlan,
    layout: wgpu::BindGroupLayout,
    evaluate_pipeline: wgpu::ComputePipeline,
    emit_pipeline: wgpu::ComputePipeline,
    emit_depth1_pipeline: wgpu::ComputePipeline,
    templates: wgpu::Buffer,
    target_channels: wgpu::Buffer,
    target_data: wgpu::Buffer,
    instances: wgpu::Buffer,
    state_boundary: FacilityPlaneGenerationBoundary,
    state_owner: FacilityPlaneOwner,
    state_plane: FacilityResidentPlane,
    resident_owner: Option<FacilityPlaneOwner>,
    resident_plane: Option<FacilityResidentPlane>,
    projection_next: wgpu::Buffer,
    bands: wgpu::Buffer,
    band_binding_indices: wgpu::Buffer,
    emission_bindings: wgpu::Buffer,
    eml_nodes: wgpu::Buffer,
    eml_ranges: wgpu::Buffer,
    dependencies: wgpu::Buffer,
    timestamp_query_set: Option<wgpu::QuerySet>,
    timestamp_resolve: Option<wgpu::Buffer>,
    timestamp_readback: Option<wgpu::Buffer>,
    generation: u32,
}

impl ActionBandGpuSession {
    /// Join the existing sealed Phase-5 crossing product against this
    /// session's immutable admitted plan. This is the same join used by the
    /// proof harness; exposing it on the owning session lets an ordinary
    /// production lifecycle retain one dispatcher without cloning its plan.
    pub fn crossings_from_sealed(
        &self,
        deltas: &[BandCrossingDelta],
    ) -> Result<ActionBandCrossingBatch, ActionBandExecutionError> {
        self.plan.crossings_from_sealed(deltas)
    }

    fn new(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
    ) -> Result<Self, ActionBandExecutionError> {
        Self::new_inner(ctx, plan, None)
    }

    fn new_inner(
        ctx: &GpuContext,
        plan: ActionBandExecutionPlan,
        resident_values: Option<&[f32]>,
    ) -> Result<Self, ActionBandExecutionError> {
        let shader_source = action_band_shader_source()?;
        let device = &ctx.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actionband_gpu_execution_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });
        let entries: Vec<_> = (0..18)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: if binding == 8 {
                        wgpu::BufferBindingType::Uniform
                    } else if binding == 17 {
                        wgpu::BufferBindingType::Storage { read_only: false }
                    } else {
                        wgpu::BufferBindingType::Storage {
                            read_only: !matches!(binding, 5 | 6 | 13),
                        }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("actionband_gpu_execution_bgl"),
            entries: &entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("actionband_gpu_execution_pl"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = |entry_point| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            })
        };
        let evaluate_pipeline = pipeline("actionband_evaluate");
        let emit_pipeline = pipeline("actionband_emit");
        let emit_depth1_pipeline = pipeline("actionband_emit_depth1");

        let state = plan
            .active_instances
            .iter()
            .map(|instance| ActionBandStateGpu {
                reserved: [
                    if instance.flags & ACTIONBAND_INSTANCE_INITIALLY_ACTIVE != 0 {
                        ACTIONBAND_STATE_ACTIVE
                    } else {
                        0
                    },
                    0,
                ],
                ..ActionBandStateGpu::zeroed()
            })
            .collect::<Vec<_>>();
        let projection = vec![0.0f32; plan.projection_floats.max(1) as usize];
        let state_boundary = FacilityPlaneGenerationBoundary::new();
        let state_owner = state_boundary.admit_facility();
        let state_plane = FacilityResidentPlane::from_rows(
            ctx,
            "actionband_state",
            &state_boundary,
            &state_owner,
            &state,
        )?;
        let (resident_owner, resident_plane) = if let Some(values) = resident_values {
            let owner = state_boundary.admit_facility();
            let plane = FacilityResidentPlane::from_rows(
                ctx,
                "actionband_resident_consequence",
                &state_boundary,
                &owner,
                values,
            )?;
            (Some(owner), Some(plane))
        } else {
            (None, None)
        };
        let (timestamp_query_set, timestamp_resolve, timestamp_readback) =
            if ctx.timestamp_supported() {
                (
                    Some(device.create_query_set(&wgpu::QuerySetDescriptor {
                        label: Some("actionband_timestamp_query_set"),
                        ty: wgpu::QueryType::Timestamp,
                        count: 4,
                    })),
                    Some(device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("actionband_timestamp_resolve"),
                        size: 32,
                        usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                        mapped_at_creation: false,
                    })),
                    Some(staging(device, "actionband_timestamp_readback", 32)),
                )
            } else {
                (None, None, None)
            };
        Ok(Self {
            templates: storage(device, "actionband_templates", &plan.templates),
            target_channels: storage(device, "actionband_target_channels", &plan.target_channels),
            target_data: storage(device, "actionband_target_data", &plan.target_data),
            instances: storage(
                device,
                "actionband_active_instances",
                &plan.active_instances,
            ),
            state_boundary,
            state_owner,
            state_plane,
            resident_owner,
            resident_plane,
            projection_next: storage_rw(device, "actionband_projection_next", &projection),
            bands: storage(device, "actionband_bands", &plan.bands),
            band_binding_indices: storage(
                device,
                "actionband_band_binding_indices",
                &plan.band_binding_indices,
            ),
            emission_bindings: storage(
                device,
                "actionband_emission_bindings",
                &plan.emission_bindings,
            ),
            eml_nodes: storage(device, "actionband_eml_nodes", &plan.eml_nodes),
            eml_ranges: storage(device, "actionband_eml_ranges", &plan.eml_ranges),
            dependencies: storage(device, "actionband_dependencies", &plan.dependencies),
            timestamp_query_set,
            timestamp_resolve,
            timestamp_readback,
            generation: 0,
            plan,
            layout,
            evaluate_pipeline,
            emit_pipeline,
            emit_depth1_pipeline,
        })
    }

    /// Production dispatch. Numerical state/projection never leave the GPU;
    /// only sealed structural commitments are returned.
    pub fn dispatch(
        &mut self,
        ctx: &GpuContext,
        world_values: &wgpu::Buffer,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandProductionDispatch, ActionBandExecutionError> {
        let (production, _, _, _) = self.dispatch_internal(
            ctx,
            Some(world_values),
            None,
            n_dims,
            crossings,
            false,
            false,
        )?;
        Ok(production)
    }

    /// Production dispatch for an admitted native destination. The caller-owned
    /// buffer is the ordinary authoritative next-state surface; this operator
    /// only copies current to next and applies its fixed admitted writes.
    pub fn dispatch_with_native_next(
        &mut self,
        ctx: &GpuContext,
        world_values: &wgpu::Buffer,
        world_values_next: &wgpu::Buffer,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandProductionDispatch, ActionBandExecutionError> {
        let (production, _, _, _) = self.dispatch_internal(
            ctx,
            Some(world_values),
            Some(world_values_next),
            n_dims,
            crossings,
            false,
            false,
        )?;
        Ok(production)
    }

    /// Canonical 7.8 dispatch. Resident Current and Next are facility-local and
    /// advance under the same boundary as ActionBand state; no caller-supplied
    /// buffer can be substituted as a foreign resident plane.
    pub fn dispatch_resident_next(
        &mut self,
        ctx: &GpuContext,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandProductionDispatch, ActionBandExecutionError> {
        let (production, _, _, _) =
            self.dispatch_internal(ctx, None, None, n_dims, crossings, false, true)?;
        Ok(production)
    }

    /// Proof-only snapshot of the facility-local resident Current plane.
    pub fn readback_resident_current_for_proof(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<f32>, ActionBandExecutionError> {
        if !debug_readback_allowed() {
            return Err(ActionBandExecutionError::ProofReadbackDisabled);
        }
        let owner = self
            .resident_owner
            .as_ref()
            .ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
        let plane = self
            .resident_plane
            .as_ref()
            .ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
        let stage = staging(
            &ctx.device,
            "actionband_resident_current_readback",
            plane.bytes_per_plane() as u64,
        );
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("actionband_resident_current_readback_encoder"),
            });
        encoder.copy_buffer_to_buffer(
            plane.current_for(owner)?,
            0,
            &stage,
            0,
            plane.bytes_per_plane() as u64,
        );
        ctx.queue.submit(Some(encoder.finish()));
        readback::<f32>(&ctx.device, &stage, plane.rows())
    }

    /// Proof-only view of the exact production dispatch. This uses the same
    /// pipelines and bind groups, adding copies only after the production work.
    pub fn dispatch_and_readback(
        &mut self,
        ctx: &GpuContext,
        world_values: &wgpu::Buffer,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandExecutionReadback, ActionBandExecutionError> {
        if !debug_readback_allowed() {
            return Err(ActionBandExecutionError::ProofReadbackDisabled);
        }
        let (production, states, projection, emission_payloads) = self.dispatch_internal(
            ctx,
            Some(world_values),
            None,
            n_dims,
            crossings,
            true,
            false,
        )?;
        Ok(ActionBandExecutionReadback {
            states: states.expect("proof dispatch requests state readback"),
            projection: projection.expect("proof dispatch requests projection readback"),
            commitments: production.commitments,
            emission_payloads,
            gpu_time_ns: production.gpu_time_ns,
            carry_gpu_time_ns: production.carry_gpu_time_ns,
            evaluation_gpu_time_ns: production.evaluation_gpu_time_ns,
            emission_gpu_time_ns: production.emission_gpu_time_ns,
        })
    }

    /// Proof view of [`Self::dispatch_with_native_next`].
    pub fn dispatch_with_native_next_and_readback(
        &mut self,
        ctx: &GpuContext,
        world_values: &wgpu::Buffer,
        world_values_next: &wgpu::Buffer,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
    ) -> Result<ActionBandExecutionReadback, ActionBandExecutionError> {
        if !debug_readback_allowed() {
            return Err(ActionBandExecutionError::ProofReadbackDisabled);
        }
        let (production, states, projection, emission_payloads) = self.dispatch_internal(
            ctx,
            Some(world_values),
            Some(world_values_next),
            n_dims,
            crossings,
            true,
            false,
        )?;
        Ok(ActionBandExecutionReadback {
            states: states.expect("proof dispatch requests state readback"),
            projection: projection.expect("proof dispatch requests projection readback"),
            commitments: production.commitments,
            emission_payloads,
            gpu_time_ns: production.gpu_time_ns,
            carry_gpu_time_ns: production.carry_gpu_time_ns,
            evaluation_gpu_time_ns: production.evaluation_gpu_time_ns,
            emission_gpu_time_ns: production.emission_gpu_time_ns,
        })
    }

    fn dispatch_internal(
        &mut self,
        ctx: &GpuContext,
        external_world_values: Option<&wgpu::Buffer>,
        world_values_next: Option<&wgpu::Buffer>,
        n_dims: u32,
        crossings: &ActionBandCrossingBatch,
        proof_readback: bool,
        resident_mode: bool,
    ) -> Result<
        (
            ActionBandProductionDispatch,
            Option<Vec<ActionBandStateGpu>>,
            Option<Vec<f32>>,
            Vec<f32>,
        ),
        ActionBandExecutionError,
    > {
        if crossings.plan_fingerprint != self.plan.fingerprint {
            return Err(ActionBandExecutionError::ForeignCrossingBatch);
        }
        let resident_owner = self.resident_owner.as_ref();
        let resident_plane = self.resident_plane.as_ref();
        let (world_values, world_values_next) = if resident_mode {
            let owner = resident_owner.ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
            let plane = resident_plane.ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
            (plane.current_for(owner)?, Some(plane.next_for(owner)?))
        } else {
            (
                external_world_values.ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?,
                world_values_next,
            )
        };
        if self.plan.native_destinations && world_values_next.is_none() {
            return Err(ActionBandExecutionError::NativeNextRequired);
        }
        if let Some(next) = world_values_next {
            if next.size() != world_values.size() {
                return Err(ActionBandExecutionError::NativeNextSizeMismatch);
            }
        }
        for template in &self.plan.templates {
            for column in [
                template.velocity_current_channel,
                template.velocity_previous_channel,
            ] {
                if column != ACTIONBAND_NO_PROGRAM && column >= n_dims {
                    return Err(ActionBandExecutionError::DestinationColumnOutOfBounds {
                        column,
                        n_dims,
                    });
                }
            }
        }
        for binding in &self.plan.emission_bindings {
            if self.plan.native_destinations
                && matches!(
                    binding.destination(),
                    ActionBandEmissionDestination::PropertyNext
                        | ActionBandEmissionDestination::RfClaim
                        | ActionBandEmissionDestination::CostBand
                )
                && binding.destination_index >= n_dims
            {
                return Err(ActionBandExecutionError::DestinationColumnOutOfBounds {
                    column: binding.destination_index,
                    n_dims,
                });
            }
        }
        let device = &ctx.device;
        let native_next_dummy = storage_rw(device, "actionband_native_next_dummy", &[0u32]);
        let native_next = world_values_next.unwrap_or(&native_next_dummy);
        let crossing_buffer = storage(device, "actionband_sealed_crossings", &crossings.rows);
        // An unresolved parent or inactive child performs no shader write; the
        // impossible column sentinel therefore remains and cannot mint a CPU
        // consequence. Authorized rows keep the graduated 16-byte packet.
        let consequence_zeros = vec![
            ThresholdEmissionGpu {
                reg_idx: 0,
                slot: 0,
                col: u32::MAX,
                value: 0.0,
            };
            crossings.output_count.max(1) as usize
        ];
        let consequence_buffer = storage_rw(
            device,
            "actionband_existing_surface_packets",
            &consequence_zeros,
        );
        let evaluation_params = ActionBandDispatchParams {
            n_dims,
            instance_count: self.plan.active_instances.len() as u32,
            crossing_start: 0,
            crossing_count: 0,
        };
        let evaluation_params_buffer =
            uniform(device, "actionband_evaluation_params", &evaluation_params);
        let evaluation_bind_group = self.bind_group(
            device,
            world_values,
            &evaluation_params_buffer,
            &crossing_buffer,
            &consequence_buffer,
            native_next,
        );
        let mut bucket_params_buffers = Vec::with_capacity(crossings.bucket_ranges.len());
        let mut bucket_bind_groups = Vec::with_capacity(crossings.bucket_ranges.len());
        for range in &crossings.bucket_ranges {
            let params = ActionBandDispatchParams {
                n_dims,
                instance_count: self.plan.active_instances.len() as u32,
                crossing_start: range.crossing_start,
                crossing_count: range.crossing_count,
            };
            bucket_params_buffers.push(uniform(device, "actionband_bucket_params", &params));
            bucket_bind_groups.push(
                self.bind_group(
                    device,
                    world_values,
                    bucket_params_buffers
                        .last()
                        .expect("bucket params inserted"),
                    &crossing_buffer,
                    &consequence_buffer,
                    native_next,
                ),
            );
        }

        let state_bytes =
            (self.plan.active_instances.len() * std::mem::size_of::<ActionBandStateGpu>()) as u64;
        let projection_bytes =
            (self.plan.projection_floats.max(1) as usize * std::mem::size_of::<f32>()) as u64;
        let consequence_bytes = (crossings.output_count.max(1) as usize
            * std::mem::size_of::<ThresholdEmissionGpu>()) as u64;
        let state_stage =
            proof_readback.then(|| staging(device, "actionband_state_readback", state_bytes));
        let projection_stage = proof_readback
            .then(|| staging(device, "actionband_projection_readback", projection_bytes));
        let consequence_stage = (!crossings.commitment_inputs.is_empty()).then(|| {
            staging(
                device,
                "actionband_existing_surface_packet_readback",
                consequence_bytes,
            )
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("actionband_gpu_execution_encoder"),
        });
        if self.plan.native_destinations {
            encoder.copy_buffer_to_buffer(world_values, 0, native_next, 0, world_values.size());
        }
        let depth1_advances =
            self.plan.depth1_crossing_fast_path && !crossings.bucket_ranges.is_empty();
        let carry_timestamped = depth1_advances
            && ctx.encoder_timestamp_supported()
            && self.timestamp_query_set.is_some();
        if depth1_advances {
            // Preserve rows that did not cross without evaluating or gathering
            // them. The fast shader overwrites only crossing rows in StateNext;
            // the ordinary whole-buffer swap remains the generation boundary.
            if carry_timestamped {
                encoder.write_timestamp(
                    self.timestamp_query_set
                        .as_ref()
                        .expect("carry timestamp support creates a query set"),
                    0,
                );
            }
            self.state_plane
                .encode_carry(&self.state_owner, &mut encoder)?;
            if carry_timestamped {
                encoder.write_timestamp(
                    self.timestamp_query_set
                        .as_ref()
                        .expect("carry timestamp support creates a query set"),
                    1,
                );
            }
        }
        if !self.plan.depth1_crossing_fast_path {
            let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
                wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
                }
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("actionband_target_evaluation"),
                timestamp_writes,
            });
            pass.set_pipeline(&self.evaluate_pipeline);
            pass.set_bind_group(0, &evaluation_bind_group, &[]);
            pass.dispatch_workgroups((evaluation_params.instance_count + 63) / 64, 1, 1);
        }
        if !crossings.bucket_ranges.is_empty() {
            let timestamp_start = if self.plan.depth1_crossing_fast_path {
                if carry_timestamped {
                    2
                } else {
                    0
                }
            } else {
                2
            };
            let timestamp_writes = self.timestamp_query_set.as_ref().map(|query_set| {
                wgpu::ComputePassTimestampWrites {
                    query_set,
                    beginning_of_pass_write_index: Some(timestamp_start),
                    end_of_pass_write_index: Some(timestamp_start + 1),
                }
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some(if self.plan.depth1_crossing_fast_path {
                    "actionband_depth1_sealed_crossing_emission"
                } else {
                    "actionband_sealed_crossing_emission"
                }),
                timestamp_writes,
            });
            pass.set_pipeline(if self.plan.depth1_crossing_fast_path {
                &self.emit_depth1_pipeline
            } else {
                &self.emit_pipeline
            });
            for (range, bind_group) in crossings.bucket_ranges.iter().zip(&bucket_bind_groups) {
                pass.set_bind_group(0, bind_group, &[]);
                pass.dispatch_workgroups((range.crossing_count + 63) / 64, 1, 1);
            }
        }
        if let Some(stage) = state_stage.as_ref() {
            let result_state = if self.plan.depth1_crossing_fast_path && !depth1_advances {
                self.state_plane.current_for(&self.state_owner)?
            } else {
                self.state_plane.next_for(&self.state_owner)?
            };
            encoder.copy_buffer_to_buffer(result_state, 0, stage, 0, state_bytes);
        }
        if let Some(stage) = projection_stage.as_ref() {
            encoder.copy_buffer_to_buffer(&self.projection_next, 0, stage, 0, projection_bytes);
        }
        if let Some(stage) = consequence_stage.as_ref() {
            encoder.copy_buffer_to_buffer(&consequence_buffer, 0, stage, 0, consequence_bytes);
        }
        let timestamp_count = if self.plan.depth1_crossing_fast_path {
            if crossings.bucket_ranges.is_empty() {
                0
            } else if carry_timestamped {
                4
            } else {
                2
            }
        } else if crossings.bucket_ranges.is_empty() {
            2
        } else {
            4
        };
        if let (Some(query_set), Some(resolve), Some(readback)) = (
            self.timestamp_query_set.as_ref(),
            self.timestamp_resolve.as_ref(),
            self.timestamp_readback.as_ref(),
        ) {
            if timestamp_count > 0 {
                encoder.resolve_query_set(query_set, 0..timestamp_count, resolve, 0);
                encoder.copy_buffer_to_buffer(
                    resolve,
                    0,
                    readback,
                    0,
                    u64::from(timestamp_count) * 8,
                );
            }
        }
        ctx.queue.submit(Some(encoder.finish()));
        self.generation = self.generation.saturating_add(1);
        if !self.plan.depth1_crossing_fast_path || depth1_advances {
            if resident_mode {
                let resident_owner = self
                    .resident_owner
                    .as_ref()
                    .ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
                let resident_plane = self
                    .resident_plane
                    .as_mut()
                    .ok_or(ActionBandExecutionError::ResidentNextNotAdmitted)?;
                self.state_boundary.advance(&mut [
                    (&self.state_owner, &mut self.state_plane),
                    (resident_owner, resident_plane),
                ])?;
            } else {
                self.state_boundary
                    .advance(&mut [(&self.state_owner, &mut self.state_plane)])?;
            }
        }

        let states = state_stage
            .as_ref()
            .map(|stage| {
                readback::<ActionBandStateGpu>(device, stage, self.plan.active_instances.len())
            })
            .transpose()?;
        let projection = projection_stage
            .as_ref()
            .map(|stage| readback::<f32>(device, stage, self.plan.projection_floats as usize))
            .transpose()?;
        let (commitments, emission_payloads) = if let Some(stage) = consequence_stage.as_ref() {
            let packets =
                readback::<ThresholdEmissionGpu>(device, stage, crossings.output_count as usize)?;
            let mut commitments = Vec::with_capacity(crossings.commitment_inputs.len());
            let mut payloads = Vec::with_capacity(crossings.commitment_inputs.len());
            for (index, delta) in &crossings.commitment_inputs {
                let packet = &packets[*index as usize];
                if packet.col == u32::MAX {
                    continue;
                }
                let emission = ThresholdEmission::from_gpu_readback(packet, self.generation);
                if emission.reg_idx() != delta.reg_idx()
                    || emission.slot() != delta.slot().raw()
                    || emission.col() as usize != delta.col().raw()
                {
                    return Err(ActionBandExecutionError::StructuralPacketIdentityMismatch);
                }
                let threshold = ThresholdCrossingToken::from_sealed_band_crossing(delta);
                let emission_token = EmissionToken::from_sealed_threshold_emission(&emission);
                let boundary = BoundaryEmissionToken::bind(threshold, emission_token)?;
                commitments.push(StructuralCommitment::mint_from_sealed_path(
                    threshold,
                    emission_token,
                    boundary,
                )?);
                payloads.push(emission.value());
            }
            (commitments, payloads)
        } else {
            (Vec::new(), Vec::new())
        };
        let (carry_gpu_time_ns, evaluation_gpu_time_ns, emission_gpu_time_ns) =
            if timestamp_count == 0 {
                (None, None, None)
            } else if let Some(timestamp_buffer) = self.timestamp_readback.as_ref() {
                let stamps = readback::<u64>(device, timestamp_buffer, timestamp_count as usize)?;
                let period = ctx.timestamp_period_ns() as f64;
                if self.plan.depth1_crossing_fast_path {
                    if carry_timestamped {
                        (
                            Some((stamps[1] - stamps[0]) as f64 * period),
                            None,
                            Some((stamps[3] - stamps[2]) as f64 * period),
                        )
                    } else {
                        (None, None, Some((stamps[1] - stamps[0]) as f64 * period))
                    }
                } else {
                    (
                        None,
                        Some((stamps[1] - stamps[0]) as f64 * period),
                        (timestamp_count == 4).then(|| (stamps[3] - stamps[2]) as f64 * period),
                    )
                }
            } else {
                (None, None, None)
            };
        let gpu_time_ns = if evaluation_gpu_time_ns.is_some() || emission_gpu_time_ns.is_some() {
            Some(evaluation_gpu_time_ns.unwrap_or(0.0) + emission_gpu_time_ns.unwrap_or(0.0))
        } else {
            None
        };
        Ok((
            ActionBandProductionDispatch {
                commitments,
                bucket_dispatches: crossings.bucket_ranges.len() as u32,
                gpu_time_ns,
                carry_gpu_time_ns,
                evaluation_gpu_time_ns,
                emission_gpu_time_ns,
            },
            states,
            projection,
            emission_payloads,
        ))
    }

    fn bind_group(
        &self,
        device: &wgpu::Device,
        world_values: &wgpu::Buffer,
        params: &wgpu::Buffer,
        crossings: &wgpu::Buffer,
        consequences: &wgpu::Buffer,
        world_values_next: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let resources = [
            &self.templates,
            &self.target_channels,
            &self.target_data,
            &self.instances,
            self.state_plane
                .current_for(&self.state_owner)
                .expect("ActionBand owns its facility Current plane"),
            self.state_plane
                .next_for(&self.state_owner)
                .expect("ActionBand owns its facility Next plane"),
            &self.projection_next,
            world_values,
            params,
            &self.bands,
            &self.band_binding_indices,
            &self.emission_bindings,
            crossings,
            consequences,
            &self.eml_nodes,
            &self.eml_ranges,
            &self.dependencies,
            world_values_next,
        ];
        let entries: Vec<_> = resources
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("actionband_gpu_execution_bg"),
            layout: &self.layout,
            entries: &entries,
        })
    }

    /// Returns the pre-existing dispatch/emission sequence, which advances on every dispatch (including no-swap fast paths); resident-plane swap generation is owned by `FacilityPlaneGenerationBoundary`.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Returns the actual facility-plane generation advanced by the sole
    /// Current/Next swap authority. Unlike [`Self::generation`], this does not
    /// change for a depth-1 no-crossing dispatch that performs no swap.
    pub fn facility_generation(&self) -> u32 {
        self.state_boundary.generation()
    }

    pub fn last_bucket_partition(&self) -> &[ActionBandExecutionBucket] {
        &self.plan.buckets
    }
}

fn validate_tables(
    templates: &[ActionBandTemplateGpu],
    target_channels: &[u32],
    target_data: &[f32],
    bands: &[ActionBandBandGpu],
    band_binding_indices: &[u32],
    emission_bindings: &[ActionBandEmissionBindingGpu],
    eml_ranges: &[EmlTreeRangeGpu],
    instances: &[ActionBandActiveInstanceGpu],
    dependencies: &[ActionBandDependencyGpu],
    native_destinations: bool,
) -> Result<(), ActionBandExecutionError> {
    for template in templates {
        let channels_end = template.channel_start as usize + template.channel_count as usize;
        if channels_end > target_channels.len() {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        let data_len = match template.target_kind {
            target_kind::POINT => template.projection_width,
            target_kind::SCALAR_AT_LEAST
            | target_kind::SCALAR_AT_MOST
            | target_kind::LOCUS_RADIUS
            | target_kind::PALMA_REACHABLE_SET => 1,
            target_kind::INTERVAL => 2,
            target_kind::AXIS_ALIGNED_BOX => template.projection_width * 2,
            target_kind::EML_PROJECTED_SET => 0,
            _ => return Err(ActionBandExecutionError::InvalidTableSpan),
        };
        if template.target_data_start as usize + data_len as usize > target_data.len()
            || template.band_start as usize + template.band_count as usize > bands.len()
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        for range in [template.membership_range, template.projection_range] {
            if range != ACTIONBAND_NO_PROGRAM && range as usize >= eml_ranges.len() {
                return Err(ActionBandExecutionError::InvalidTableSpan);
            }
        }
        if (template.velocity_current_channel == ACTIONBAND_NO_PROGRAM)
            != (template.velocity_previous_channel == ACTIONBAND_NO_PROGRAM)
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
    }
    for band in bands {
        if band.binding_start as usize + band.binding_count as usize > band_binding_indices.len()
            || (band.program_range != ACTIONBAND_NO_PROGRAM
                && band.program_range as usize >= eml_ranges.len())
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        if !native_destinations && band.binding_count != 1 {
            return Err(ActionBandExecutionError::StructuralBindingCount {
                count: band.binding_count,
            });
        } else if native_destinations && band.binding_count == 0 {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
    }
    if band_binding_indices
        .iter()
        .any(|&i| i as usize >= emission_bindings.len())
    {
        return Err(ActionBandExecutionError::InvalidTableSpan);
    }
    for binding in emission_bindings {
        let Some(destination) = ActionBandEmissionDestination::from_raw(binding.destination_kind)
        else {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        };
        let valid_shape = matches!(
            binding.auxiliary1,
            ActionBandEmissionBindingGpu::CONSERVED_BOUND_NONE
                ..=ActionBandEmissionBindingGpu::CONSERVED_BOUND_GU_YANG_REALIZED
        ) && match destination {
            ActionBandEmissionDestination::PropertyNext => binding.auxiliary0 <= 1,
            ActionBandEmissionDestination::RfClaim | ActionBandEmissionDestination::CostBand => {
                binding.auxiliary0 == 1
            }
            ActionBandEmissionDestination::OverlayEvent
            | ActionBandEmissionDestination::StructuralRequest
            | ActionBandEmissionDestination::Telemetry => binding.auxiliary0 == 0,
        };
        if !valid_shape {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
        if destination != ActionBandEmissionDestination::StructuralRequest
            && (!native_destinations
                || !matches!(
                    destination,
                    ActionBandEmissionDestination::PropertyNext
                        | ActionBandEmissionDestination::RfClaim
                        | ActionBandEmissionDestination::CostBand
                ))
        {
            return Err(ActionBandExecutionError::DestinationDeferred { destination });
        }
    }
    if native_destinations {
        validate_native_write_collisions(
            templates,
            bands,
            band_binding_indices,
            emission_bindings,
            instances,
        )?;
    }
    for instance in instances {
        let end = instance.dependency_start as usize + instance.dependency_count as usize;
        if end > dependencies.len()
            || instance.flags
                & !(ACTIONBAND_INSTANCE_INITIALLY_ACTIVE | ACTIONBAND_INSTANCE_SUBORDINATE)
                != 0
            || instance.reserved != 0
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
    }
    if dependencies
        .iter()
        .any(|dependency| dependency.child_instance_row as usize >= instances.len())
    {
        return Err(ActionBandExecutionError::InvalidTableSpan);
    }
    validate_dependency_rows(instances, dependencies)?;
    Ok(())
}

fn validate_native_write_collisions(
    templates: &[ActionBandTemplateGpu],
    bands: &[ActionBandBandGpu],
    band_binding_indices: &[u32],
    emission_bindings: &[ActionBandEmissionBindingGpu],
    instances: &[ActionBandActiveInstanceGpu],
) -> Result<(), ActionBandExecutionError> {
    let mut writers = std::collections::BTreeMap::new();
    for (instance_row, instance) in instances.iter().enumerate() {
        let template = &templates[instance.template_index as usize];
        for band_index in template.band_start..template.band_start + template.band_count {
            let band = &bands[band_index as usize];
            let mut band_writes = std::collections::BTreeSet::new();
            for local in 0..band.binding_count {
                let index = band_binding_indices[(band.binding_start + local) as usize] as usize;
                let binding = emission_bindings[index];
                if !matches!(
                    binding.destination(),
                    ActionBandEmissionDestination::PropertyNext
                        | ActionBandEmissionDestination::RfClaim
                        | ActionBandEmissionDestination::CostBand
                ) {
                    continue;
                }
                let key = (instance.slot, binding.destination_index);
                let duplicate_in_band = !band_writes.insert(key);
                let rival_instance = writers
                    .insert(key, instance_row)
                    .is_some_and(|prior| prior != instance_row);
                if duplicate_in_band || rival_instance {
                    return Err(ActionBandExecutionError::NativeDestinationCollision {
                        slot: instance.slot,
                        column: binding.destination_index,
                    });
                }
            }
        }
    }
    Ok(())
}

fn validate_dependency_rows(
    instances: &[ActionBandActiveInstanceGpu],
    dependencies: &[ActionBandDependencyGpu],
) -> Result<(), ActionBandExecutionError> {
    let mut covered_dependencies = vec![0u8; dependencies.len()];
    let mut claimed_children = vec![false; instances.len()];
    for (parent_row, instance) in instances.iter().enumerate() {
        let start = instance.dependency_start as usize;
        let end = start + instance.dependency_count as usize;
        for dependency_index in start..end {
            covered_dependencies[dependency_index] = covered_dependencies[dependency_index]
                .checked_add(1)
                .ok_or(ActionBandExecutionError::InvalidTableSpan)?;
            let child_row = dependencies[dependency_index].child_instance_row as usize;
            if child_row == parent_row || claimed_children[child_row] {
                return Err(ActionBandExecutionError::InvalidTableSpan);
            }
            claimed_children[child_row] = true;
        }
    }
    if covered_dependencies.iter().any(|&count| count != 1) {
        return Err(ActionBandExecutionError::InvalidTableSpan);
    }
    for (row, instance) in instances.iter().enumerate() {
        let subordinate = instance.flags & ACTIONBAND_INSTANCE_SUBORDINATE != 0;
        let initially_active = instance.flags & ACTIONBAND_INSTANCE_INITIALLY_ACTIVE != 0;
        if subordinate != claimed_children[row]
            || (subordinate && initially_active)
            || (!subordinate && !initially_active)
        {
            return Err(ActionBandExecutionError::InvalidTableSpan);
        }
    }

    let mut marks = vec![0u8; instances.len()];
    for row in 0..instances.len() {
        validate_dependency_acyclic(row, instances, dependencies, &mut marks)?;
    }
    Ok(())
}

fn validate_dependency_acyclic(
    row: usize,
    instances: &[ActionBandActiveInstanceGpu],
    dependencies: &[ActionBandDependencyGpu],
    marks: &mut [u8],
) -> Result<(), ActionBandExecutionError> {
    match marks[row] {
        1 => return Err(ActionBandExecutionError::InvalidTableSpan),
        2 => return Ok(()),
        _ => {}
    }
    marks[row] = 1;
    let instance = &instances[row];
    let start = instance.dependency_start as usize;
    let end = start + instance.dependency_count as usize;
    for dependency in &dependencies[start..end] {
        validate_dependency_acyclic(
            dependency.child_instance_row as usize,
            instances,
            dependencies,
            marks,
        )?;
    }
    marks[row] = 2;
    Ok(())
}

fn plan_fingerprint(
    templates: &[ActionBandTemplateGpu],
    target_channels: &[u32],
    target_data: &[f32],
    bands: &[ActionBandBandGpu],
    binding_indices: &[u32],
    emission_bindings: &[ActionBandEmissionBindingGpu],
    eml_nodes: &[EmlNodeGpu],
    eml_ranges: &[EmlTreeRangeGpu],
    instances: &[ActionBandActiveInstanceGpu],
    dependencies: &[ActionBandDependencyGpu],
) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for bytes in [
        bytemuck::cast_slice::<_, u8>(templates),
        bytemuck::cast_slice::<_, u8>(target_channels),
        bytemuck::cast_slice::<_, u8>(target_data),
        bytemuck::cast_slice::<_, u8>(bands),
        bytemuck::cast_slice::<_, u8>(binding_indices),
        bytemuck::cast_slice::<_, u8>(emission_bindings),
        bytemuck::cast_slice::<_, u8>(eml_nodes),
        bytemuck::cast_slice::<_, u8>(eml_ranges),
        bytemuck::cast_slice::<_, u8>(instances),
        bytemuck::cast_slice::<_, u8>(dependencies),
    ] {
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

fn action_band_shader_source() -> Result<String, ActionBandExecutionError> {
    let canonical = include_str!("../shaders/accumulator_op.wgsl");
    let start = canonical
        .find("struct EmlNodeGpu {")
        .ok_or(ActionBandExecutionError::ShaderSourceMarkersMissing)?;
    let end = canonical
        .find("fn atomic_add_f32_at")
        .ok_or(ActionBandExecutionError::ShaderSourceMarkersMissing)?;
    let shared_eml = &canonical[start..end];
    Ok(format!(
        "{shared_eml}\n{}",
        include_str!("../shaders/action_band_execution.wgsl")
    ))
}

fn uniform<T: Pod>(device: &wgpu::Device, label: &str, value: &T) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(value),
        usage: wgpu::BufferUsages::UNIFORM,
    })
}

fn storage<T: Pod>(device: &wgpu::Device, label: &str, rows: &[T]) -> wgpu::Buffer {
    let zero = [0u32; 8];
    let contents = if rows.is_empty() {
        bytemuck::cast_slice(&zero)
    } else {
        bytemuck::cast_slice(rows)
    };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE,
    })
}

fn storage_rw<T: Pod>(device: &wgpu::Device, label: &str, rows: &[T]) -> wgpu::Buffer {
    let zero = [0u32; 8];
    let contents = if rows.is_empty() {
        bytemuck::cast_slice(&zero)
    } else {
        bytemuck::cast_slice(rows)
    };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    })
}

fn staging(device: &wgpu::Device, label: &str, size: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn readback<T: Pod>(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    len: usize,
) -> Result<Vec<T>, ActionBandExecutionError> {
    if len == 0 {
        return Ok(Vec::new());
    }
    let byte_len = (len * std::mem::size_of::<T>()) as u64;
    let slice = buffer.slice(..byte_len);
    let (tx, rx) = mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .ok()
        .and_then(Result::ok)
        .ok_or(ActionBandExecutionError::MapFailed)?;
    let mapped = slice.get_mapped_range();
    let result = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    buffer.unmap();
    Ok(result)
}
