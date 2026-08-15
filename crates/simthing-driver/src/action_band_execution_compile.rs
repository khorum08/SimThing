//! Driver lowering from the frozen 7.1 admission product to the sanctioned
//! sparse ActionBand GPU operator.  This is a numeric projection only: labels
//! never enter a key, and crossing evidence remains kernel-sealed.

use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

use simthing_core::{
    eml_nodes::execution_class_to_u32, ColumnIndex, CompiledAccumulatorOpPlan, DimensionRegistry,
    EmitOnThresholdRegistration, EmlExpressionRegistry, EmlTreeId, SimPropertyId, SlotIndex,
    SubFieldRole,
};
use simthing_feeder::{BoundaryRequest, FeederSender};
use simthing_gpu::{
    action_band_target_kind, ActionBandActiveInstanceGpu, ActionBandBandGpu,
    ActionBandDependencyGpu, ActionBandEmissionBindingGpu, ActionBandEmissionDestination,
    ActionBandExecutionBucket, ActionBandExecutionError, ActionBandExecutionPlan,
    ActionBandTemplateGpu, EmlTreeRangeGpu, StructuralCommitment,
    ACTIONBAND_INSTANCE_INITIALLY_ACTIVE, ACTIONBAND_INSTANCE_SUBORDINATE, ACTIONBAND_NO_PROGRAM,
};
use simthing_sim::{
    StructuralCommitmentApplicationDoor, StructuralCommitmentApplicationError, ThresholdRegistry,
};
use simthing_spec::{
    ActionBandTemplateIndex, AdmittedActionBandConservedProgressBoundSource,
    AdmittedActionBandTarget, FrozenActionBandTemplates, ScalarBoundDirection,
};
use thiserror::Error;

/// Sparse session-build activation. There is deliberately no label or domain
/// field; execution identity is admitted numeric template identity + slot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionBandActiveInstance {
    template: ActionBandTemplateIndex,
    slot: SlotIndex,
    params: [f32; 4],
    initially_active: bool,
}

impl ActionBandActiveInstance {
    pub fn new(template: ActionBandTemplateIndex, slot: SlotIndex, params: [f32; 4]) -> Self {
        Self {
            template,
            slot,
            params,
            initially_active: true,
        }
    }

    /// Materialize one already-reserved child row without activating it. Only
    /// a frozen parent dependency may activate this row on the GPU.
    pub fn pre_admitted_subordinate(
        template: ActionBandTemplateIndex,
        slot: SlotIndex,
        params: [f32; 4],
    ) -> Self {
        Self {
            template,
            slot,
            params,
            initially_active: false,
        }
    }

    pub fn template(self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn slot(self) -> SlotIndex {
        self.slot
    }

    pub fn is_initially_active(self) -> bool {
        self.initially_active
    }
}

#[derive(Debug, Error)]
pub enum ActionBandExecutionCompileError {
    #[error("pre-admitted emission table has {actual} rows; frozen ActionBand budget requires {required}")]
    EmissionTableWidth { required: u32, actual: usize },
    #[error("frozen ActionBand product references missing EML program {0}")]
    MissingEmlProgram(u32),
    #[error("frozen ActionBand table span is invalid")]
    InvalidFrozenSpan,
    #[error("frozen ActionBand crossing provenance is incomplete or inconsistent")]
    InvalidFrozenCrossingSource,
    #[error("ActionBand parent template {parent_template} dependency span {dependency_count} exceeds its frozen concurrent cap {max_active}")]
    DependencyCapacityDeferred {
        parent_template: u32,
        dependency_count: u32,
        max_active: u32,
    },
    #[error("ActionBand parent template {parent_template} slot {slot} has no pre-admitted child row for template {child_template}")]
    MissingPreAdmittedChild {
        parent_template: u32,
        child_template: u32,
        slot: u32,
    },
    #[error(
        "ActionBand child template {child_template} slot {slot} is claimed by more than one parent"
    )]
    SharedChildLifecycle { child_template: u32, slot: u32 },
    #[error("ActionBand child template {child_template} slot {slot} must begin inactive")]
    ChildMustBeginInactive { child_template: u32, slot: u32 },
    #[error(
        "inactive ActionBand template {template} slot {slot} is not claimed by a frozen dependency"
    )]
    UnclaimedInactiveRow { template: u32, slot: u32 },
    #[error("ActionBand template {template} materializes {actual} rows beyond its frozen reservation {reserved}")]
    TemplateRowBudgetExceeded {
        template: u32,
        actual: u32,
        reserved: u32,
    },
    #[error("ActionBand frozen dependency graph contains a runtime lifecycle cycle")]
    DependencyCycle,
    #[error("ActionBand native destination {destination:?} column {column} is not admitted by its existing authoritative lane")]
    NativeDestinationNotAdmitted {
        destination: ActionBandEmissionDestination,
        column: u32,
    },
    #[error("ActionBand conserved-progress binding for band {band_table_index} / emission {emission_binding_index} is inconsistent with the frozen admission product")]
    InvalidConservedProgressBinding {
        band_table_index: u32,
        emission_binding_index: u32,
    },
    #[error("ActionBand conserved-progress binding {emission_binding_index} requires an existing authoritative native next-state lane")]
    ConservedProgressRequiresNativeLane { emission_binding_index: u32 },
    #[error("ActionBand conserved progress cannot target {destination:?}")]
    ConservedProgressDestinationDeferred {
        destination: ActionBandEmissionDestination,
    },
    #[error(transparent)]
    Kernel(#[from] ActionBandExecutionError),
}

/// Source-bound permission to lower ActionBand payloads into existing ordinary
/// next-state lanes. It carries no values or lifecycle authority: RF columns
/// come from compiled accumulator plans and CostBand columns from admitted sink
/// registrations in the ordinary threshold registry.
#[derive(Clone, Debug, Default)]
pub struct ActionBandNativeLaneAdmission {
    property_next_columns: BTreeSet<u32>,
    rf_claim_columns: BTreeSet<u32>,
    cost_band_columns: BTreeSet<u32>,
    logical_destinations: BTreeMap<u32, (SimPropertyId, SubFieldRole)>,
}

impl ActionBandNativeLaneAdmission {
    pub fn from_existing_surfaces(
        dimensions: &DimensionRegistry,
        property_next_columns: &[ColumnIndex],
        rf_plans: &[CompiledAccumulatorOpPlan],
        threshold_registrations: &[EmitOnThresholdRegistration],
        threshold_registry: &ThresholdRegistry,
    ) -> Self {
        let in_bounds = |column: u32| column < dimensions.total_columns as u32;
        let property_next_columns: BTreeSet<u32> = property_next_columns
            .iter()
            .map(|column| column.raw_u32())
            .filter(|&column| in_bounds(column))
            .collect();
        let rf_claim_columns: BTreeSet<u32> = rf_plans
            .iter()
            .map(|plan| plan.input_channel.raw())
            .filter(|&column| in_bounds(column))
            .collect();
        let cost_band_columns: BTreeSet<u32> = threshold_registrations
            .iter()
            .filter(|registration| {
                threshold_registry
                    .cost_band(registration.event_kind)
                    .is_sink
            })
            .map(|registration| registration.col.raw_u32())
            .filter(|&column| in_bounds(column))
            .collect();
        let logical_destinations = property_next_columns
            .iter()
            .chain(rf_claim_columns.iter())
            .chain(cost_band_columns.iter())
            .filter_map(|&column| {
                logical_destination(dimensions, column).map(|identity| (column, identity))
            })
            .collect();
        Self {
            property_next_columns,
            rf_claim_columns,
            cost_band_columns,
            logical_destinations,
        }
    }

    pub(crate) fn admits(&self, binding: ActionBandEmissionBindingGpu) -> bool {
        let column = binding.destination_index();
        match binding.destination() {
            ActionBandEmissionDestination::PropertyNext => {
                self.property_next_columns.contains(&column)
            }
            ActionBandEmissionDestination::RfClaim => self.rf_claim_columns.contains(&column),
            ActionBandEmissionDestination::CostBand => self.cost_band_columns.contains(&column),
            ActionBandEmissionDestination::StructuralRequest => true,
            ActionBandEmissionDestination::OverlayEvent
            | ActionBandEmissionDestination::Telemetry => false,
        }
    }

    pub(crate) fn logical_destination(&self, column: u32) -> Option<(SimPropertyId, SubFieldRole)> {
        self.logical_destinations.get(&column).cloned()
    }
}

fn logical_destination(
    dimensions: &DimensionRegistry,
    column: u32,
) -> Option<(SimPropertyId, SubFieldRole)> {
    let (property_id, offset) = *dimensions.column_owners.get(column as usize)?;
    let property = dimensions.try_property(property_id)?;
    let mut start = 0usize;
    for sub_field in &property.layout.sub_fields {
        if offset < start + sub_field.width {
            return Some((property_id, sub_field.role.clone()));
        }
        start += sub_field.width;
    }
    None
}

#[derive(Clone, Copy, Debug)]
struct FrozenStructuralSource {
    event_kind: u32,
    destination_index: u32,
}

/// Opaque association identity for one compile of an ActionBand admission product.
///
/// Minted once per `compile_action_band_gpu_execution*` call. Distinct even when
/// two admissions share a bit-identical numeric plan fingerprint (identity-blind
/// labels). Association metadata only — not numerical or structural authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionBandSessionOrigin(u64);

impl ActionBandSessionOrigin {
    fn mint() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Compiled proof that one conserved-progress emission consumes the sealed
/// value from its band's existing threshold registration. This is association
/// metadata only: there is no FieldSweep handle, solver, or numerical mirror.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompiledActionBandConservedProgressBinding {
    template: ActionBandTemplateIndex,
    band_table_index: u32,
    emission_binding_index: u32,
    bound_source: AdmittedActionBandConservedProgressBoundSource,
    threshold_column: ColumnIndex,
    destination: ActionBandEmissionDestination,
}

impl CompiledActionBandConservedProgressBinding {
    pub fn template(self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn band_table_index(self) -> u32 {
        self.band_table_index
    }

    pub fn emission_binding_index(self) -> u32 {
        self.emission_binding_index
    }

    pub fn bound_source(self) -> AdmittedActionBandConservedProgressBoundSource {
        self.bound_source
    }

    pub fn threshold_column(self) -> ColumnIndex {
        self.threshold_column
    }

    pub fn destination(self) -> ActionBandEmissionDestination {
        self.destination
    }
}

/// Association-only binding of a compile product to the frozen 7.1 admission it
/// was lowered from.
///
/// Includes **logical** opaque identity (`template` index + `authored_id`) and
/// numeric plan shape so a foreign logical admission with an equal numeric plan
/// still diverges. **Does not** include human-readable `label`/designation —
/// designation is post-authority metadata and must not gate dispatch/seal.
pub fn frozen_admission_binding_id(frozen: &FrozenActionBandTemplates) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for row in frozen.semantic_shadow() {
        row.template().raw().hash(&mut hasher);
        // Logical opaque admission identity (not human-readable designation).
        row.authored_id().hash(&mut hasher);
        // Intentionally omit row.label() — identity-blindness fence.
    }
    // Numeric plan tables also contribute so a pure numeric mutation still diverges.
    frozen.budget().axis_channel_count.hash(&mut hasher);
    frozen.budget().storage_rows.hash(&mut hasher);
    frozen.budget().emission_binding_count.hash(&mut hasher);
    for template in frozen.templates() {
        template.index().raw().hash(&mut hasher);
        template.reserved_instance_rows().hash(&mut hasher);
    }
    for binding in frozen.conserved_progress_bindings() {
        binding.hash(&mut hasher);
    }
    hasher.finish()
}

/// One source-bound compilation of the frozen 7.1 admission product. The GPU
/// plan and structural application provenance are derived together from the
/// same immutable band/binding rows, so a caller cannot supply a detached
/// `(event_kind, binding)` assertion beside the plan.
///
/// Also carries:
/// - opaque plan fingerprint + event_kind→template map for 7.5 sealing
/// - unique [`ActionBandSessionOrigin`] for same-shape cross-session association
/// - frozen admission binding id so the compile cannot be paired with a foreign
///   frozen product that happens to share a numeric plan fingerprint
#[derive(Clone, Debug)]
pub struct CompiledActionBandGpuExecution {
    plan: ActionBandExecutionPlan,
    structural_sources: Vec<FrozenStructuralSource>,
    /// Numeric plan identity of this compile product.
    plan_fingerprint: u64,
    /// Admission crossing map: sealed commitment event_kind → template index.
    event_kind_to_template: BTreeMap<u32, ActionBandTemplateIndex>,
    /// Unique per-compile association origin (not the numeric fingerprint).
    session_origin: ActionBandSessionOrigin,
    /// Binding of this compile to the frozen admission it was lowered from.
    frozen_admission_binding: u64,
    conserved_progress_bindings: Vec<CompiledActionBandConservedProgressBinding>,
}

impl CompiledActionBandGpuExecution {
    pub fn execution_plan(&self) -> &ActionBandExecutionPlan {
        &self.plan
    }

    pub fn into_execution_plan(self) -> ActionBandExecutionPlan {
        self.plan
    }

    pub fn plan_fingerprint(&self) -> u64 {
        self.plan_fingerprint
    }

    pub fn session_origin(&self) -> ActionBandSessionOrigin {
        self.session_origin
    }

    pub fn frozen_admission_binding(&self) -> u64 {
        self.frozen_admission_binding
    }

    pub fn conserved_progress_bindings(&self) -> &[CompiledActionBandConservedProgressBinding] {
        &self.conserved_progress_bindings
    }

    pub(crate) fn template_for_event_kind(
        &self,
        event_kind: u32,
    ) -> Option<ActionBandTemplateIndex> {
        self.event_kind_to_template.get(&event_kind).copied()
    }
}

/// Session-fixed structural consequences addressed by the numeric destination
/// index in an admitted emission binding. The boundary applies the selected row
/// verbatim; it does not inspect payload, distance, satisfaction, or EML.
///
/// Carries the compile [`ActionBandSessionOrigin`] so a structural door cannot
/// be paired with a foreign same-shape compile/semantic session.
#[derive(Clone, Debug)]
pub struct FrozenActionBandStructuralRequests {
    door: StructuralCommitmentApplicationDoor,
    session_origin: ActionBandSessionOrigin,
}

impl FrozenActionBandStructuralRequests {
    pub fn from_compiled_admission(
        compiled: &CompiledActionBandGpuExecution,
        rows: Vec<Option<BoundaryRequest>>,
    ) -> Result<Self, ActionBandStructuralApplyError> {
        let mut admitted = Vec::with_capacity(compiled.structural_sources.len());
        for source in &compiled.structural_sources {
            let request = rows
                .get(source.destination_index as usize)
                .and_then(Option::as_ref)
                .ok_or(ActionBandStructuralApplyError::MissingPreAdmittedRequest(
                    source.destination_index,
                ))?;
            admitted.push((source.event_kind, request.clone()));
        }
        Ok(Self {
            door: StructuralCommitmentApplicationDoor::from_pre_admitted_requests(admitted)?,
            session_origin: compiled.session_origin(),
        })
    }

    pub fn session_origin(&self) -> ActionBandSessionOrigin {
        self.session_origin
    }

    pub fn submit_committed(
        &self,
        commitments: &[StructuralCommitment],
        boundary: &FeederSender,
    ) -> Result<usize, ActionBandStructuralApplyError> {
        self.door
            .submit_committed(commitments, boundary)
            .map_err(Into::into)
    }

    /// Read-only access to the admitted structural request for a sealed event_kind.
    pub fn request_for_event_kind(&self, event_kind: u32) -> Option<&BoundaryRequest> {
        self.door.request_for_event_kind(event_kind)
    }
}

#[derive(Debug, Error)]
pub enum ActionBandStructuralApplyError {
    #[error("GPU-authorized structural destination {0} has no pre-admitted boundary request")]
    MissingPreAdmittedRequest(u32),
    #[error("ActionBand 7.2 defers structural-door destination {0:?}")]
    DeferredDestination(ActionBandEmissionDestination),
    #[error(transparent)]
    Door(#[from] StructuralCommitmentApplicationError),
}

/// Lower one already-frozen admission product. Human-readable semantic shadow
/// data is intentionally not read by this function.
pub fn compile_action_band_gpu_execution(
    frozen: &FrozenActionBandTemplates,
    eml_registry: &EmlExpressionRegistry,
    pre_admitted_emission_bindings: &[ActionBandEmissionBindingGpu],
    active_instances: &[ActionBandActiveInstance],
) -> Result<CompiledActionBandGpuExecution, ActionBandExecutionCompileError> {
    compile_action_band_gpu_execution_inner(
        frozen,
        eml_registry,
        pre_admitted_emission_bindings,
        active_instances,
        None,
    )
}

/// 7.3 lowering door for bindings proven to be existing authoritative native
/// lanes. The default 7.2 compiler remains structural-only.
pub fn compile_action_band_gpu_execution_with_native_lanes(
    frozen: &FrozenActionBandTemplates,
    eml_registry: &EmlExpressionRegistry,
    pre_admitted_emission_bindings: &[ActionBandEmissionBindingGpu],
    active_instances: &[ActionBandActiveInstance],
    native_lanes: &ActionBandNativeLaneAdmission,
) -> Result<CompiledActionBandGpuExecution, ActionBandExecutionCompileError> {
    for &binding in pre_admitted_emission_bindings {
        if !native_lanes.admits(binding) {
            return Err(
                ActionBandExecutionCompileError::NativeDestinationNotAdmitted {
                    destination: binding.destination(),
                    column: binding.destination_index(),
                },
            );
        }
    }
    compile_action_band_gpu_execution_inner(
        frozen,
        eml_registry,
        pre_admitted_emission_bindings,
        active_instances,
        Some(native_lanes),
    )
}

fn compile_action_band_gpu_execution_inner(
    frozen: &FrozenActionBandTemplates,
    eml_registry: &EmlExpressionRegistry,
    pre_admitted_emission_bindings: &[ActionBandEmissionBindingGpu],
    active_instances: &[ActionBandActiveInstance],
    native_lanes: Option<&ActionBandNativeLaneAdmission>,
) -> Result<CompiledActionBandGpuExecution, ActionBandExecutionCompileError> {
    let required_bindings = frozen.budget().emission_binding_count;
    if pre_admitted_emission_bindings.len() != required_bindings as usize {
        return Err(ActionBandExecutionCompileError::EmissionTableWidth {
            required: required_bindings,
            actual: pre_admitted_emission_bindings.len(),
        });
    }

    let mut emission_bindings = pre_admitted_emission_bindings.to_vec();
    let conserved_progress_bindings =
        lower_conserved_progress_bindings(frozen, &mut emission_bindings, native_lanes.is_some())?;

    let mut program_ids = BTreeSet::new();
    for template in frozen.templates() {
        if let AdmittedActionBandTarget::EmlProjectedSet {
            membership_program,
            projection_program,
            ..
        } = template.target()
        {
            program_ids.insert(*membership_program);
            program_ids.insert(*projection_program);
        }
    }
    for band in frozen.bands() {
        if let Some(program) = band.eml_program() {
            program_ids.insert(program);
        }
    }

    let mut program_ranges = BTreeMap::new();
    let mut eml_nodes = Vec::new();
    let mut eml_ranges = Vec::new();
    for tree_id in program_ids {
        let meta =
            eml_registry
                .get(tree_id)
                .ok_or(ActionBandExecutionCompileError::MissingEmlProgram(
                    tree_id.raw(),
                ))?;
        let nodes = eml_registry.get_nodes(tree_id).ok_or(
            ActionBandExecutionCompileError::MissingEmlProgram(tree_id.raw()),
        )?;
        let range_index = eml_ranges.len() as u32;
        eml_ranges.push(EmlTreeRangeGpu {
            node_offset: eml_nodes.len() as u32,
            node_count: nodes.len() as u32,
            execution_class: execution_class_to_u32(meta.execution_class),
            flags: 0,
        });
        eml_nodes.extend_from_slice(nodes);
        program_ranges.insert(tree_id, range_index);
    }

    let mut target_channels = Vec::new();
    let mut target_data = Vec::new();
    let mut templates = Vec::with_capacity(frozen.templates().len());
    for template in frozen.templates() {
        let channel_start = target_channels.len() as u32;
        let target_data_start = target_data.len() as u32;
        let (target_kind, channel_count, projection_width, membership_range, projection_range) =
            match template.target() {
                AdmittedActionBandTarget::Point {
                    current_channels,
                    target,
                } => {
                    target_channels
                        .extend(current_channels.iter().map(|column| column.raw() as u32));
                    target_data.extend_from_slice(target);
                    (
                        action_band_target_kind::POINT,
                        current_channels.len() as u32,
                        target.len() as u32,
                        ACTIONBAND_NO_PROGRAM,
                        ACTIONBAND_NO_PROGRAM,
                    )
                }
                AdmittedActionBandTarget::ScalarBound {
                    channel,
                    bound,
                    direction,
                } => {
                    target_channels.push(channel.raw() as u32);
                    target_data.push(*bound);
                    let kind = match direction {
                        ScalarBoundDirection::AtLeast => action_band_target_kind::SCALAR_AT_LEAST,
                        ScalarBoundDirection::AtMost => action_band_target_kind::SCALAR_AT_MOST,
                    };
                    (kind, 1, 1, ACTIONBAND_NO_PROGRAM, ACTIONBAND_NO_PROGRAM)
                }
                AdmittedActionBandTarget::Interval { channel, lo, hi } => {
                    target_channels.push(channel.raw() as u32);
                    target_data.extend([*lo, *hi]);
                    (
                        action_band_target_kind::INTERVAL,
                        1,
                        1,
                        ACTIONBAND_NO_PROGRAM,
                        ACTIONBAND_NO_PROGRAM,
                    )
                }
                AdmittedActionBandTarget::AxisAlignedBox { channels, lo, hi } => {
                    target_channels.extend(channels.iter().map(|column| column.raw() as u32));
                    target_data.extend_from_slice(lo);
                    target_data.extend_from_slice(hi);
                    (
                        action_band_target_kind::AXIS_ALIGNED_BOX,
                        channels.len() as u32,
                        channels.len() as u32,
                        ACTIONBAND_NO_PROGRAM,
                        ACTIONBAND_NO_PROGRAM,
                    )
                }
                AdmittedActionBandTarget::LocusRadius {
                    distance_channel,
                    radius,
                } => {
                    target_channels.push(distance_channel.raw() as u32);
                    target_data.push(*radius);
                    (
                        action_band_target_kind::LOCUS_RADIUS,
                        1,
                        1,
                        ACTIONBAND_NO_PROGRAM,
                        ACTIONBAND_NO_PROGRAM,
                    )
                }
                AdmittedActionBandTarget::PalmaReachableSet {
                    distance_channel,
                    maximum_distance,
                } => {
                    target_channels.push(distance_channel.raw() as u32);
                    target_data.push(*maximum_distance);
                    (
                        action_band_target_kind::PALMA_REACHABLE_SET,
                        1,
                        1,
                        ACTIONBAND_NO_PROGRAM,
                        ACTIONBAND_NO_PROGRAM,
                    )
                }
                AdmittedActionBandTarget::EmlProjectedSet {
                    input_channels,
                    membership_program,
                    projection_program,
                    projection_width,
                } => {
                    target_channels.extend(input_channels.iter().map(|column| column.raw() as u32));
                    (
                        action_band_target_kind::EML_PROJECTED_SET,
                        input_channels.len() as u32,
                        *projection_width,
                        range_for(*membership_program, &program_ranges)?,
                        range_for(*projection_program, &program_ranges)?,
                    )
                }
            };
        let (velocity_current_channel, velocity_previous_channel) = template
            .velocity()
            .map(|velocity| {
                (
                    velocity.current_channel().raw() as u32,
                    velocity.previous_generation_channel().raw() as u32,
                )
            })
            .unwrap_or((ACTIONBAND_NO_PROGRAM, ACTIONBAND_NO_PROGRAM));
        templates.push(ActionBandTemplateGpu {
            target_kind,
            channel_start,
            channel_count,
            target_data_start,
            projection_width,
            band_start: template.band_span().start(),
            band_count: template.band_span().len(),
            membership_range,
            projection_range,
            velocity_current_channel,
            velocity_previous_channel,
        });
    }

    let band_binding_indices: Vec<u32> = frozen
        .emission_bindings()
        .iter()
        .map(|index| index.raw())
        .collect();
    let mut bands = Vec::with_capacity(frozen.bands().len());
    for band in frozen.bands() {
        let span = band.emission_binding_span();
        if span.start() as usize + span.len() as usize > band_binding_indices.len() {
            return Err(ActionBandExecutionCompileError::InvalidFrozenSpan);
        }
        bands.push(ActionBandBandGpu {
            threshold_registration: band.threshold_registration().raw(),
            program_range: band
                .eml_program()
                .map(|id| range_for(id, &program_ranges))
                .transpose()?
                .unwrap_or(ACTIONBAND_NO_PROGRAM),
            binding_start: span.start(),
            binding_count: span.len(),
        });
    }

    let buckets = deterministic_buckets(&bands, &band_binding_indices, &emission_bindings)?;
    let mut materialized_instances = active_instances.to_vec();
    materialized_instances.sort_by_key(|instance| (instance.template.raw(), instance.slot.raw()));
    let (dependencies, subordinate_rows) = lower_dependency_rows(frozen, &materialized_instances)?;
    let depth1_crossing_fast_path = depth1_crossing_fast_path(frozen, &materialized_instances)?;
    if !dependencies.is_empty() && !depth1_crossing_fast_path {
        return Err(ActionBandExecutionError::RecursiveShapeDeferred.into());
    }
    let active_instances = materialized_instances
        .iter()
        .enumerate()
        .map(|(row, instance)| ActionBandActiveInstanceGpu {
            slot: instance.slot.raw(),
            template_index: instance.template.raw(),
            projection_start: 0,
            generation: 0,
            params: instance.params,
            dependency_start: dependencies
                .iter()
                .take_while(|dependency| dependency.parent_row < row as u32)
                .count() as u32,
            dependency_count: dependencies
                .iter()
                .filter(|dependency| dependency.parent_row == row as u32)
                .count() as u32,
            flags: if instance.initially_active {
                ACTIONBAND_INSTANCE_INITIALLY_ACTIVE
            } else {
                0
            } | if subordinate_rows.contains(&row) {
                ACTIONBAND_INSTANCE_SUBORDINATE
            } else {
                0
            },
            reserved: 0,
        })
        .collect::<Vec<_>>();
    let dependencies = dependencies
        .into_iter()
        .map(|dependency| ActionBandDependencyGpu {
            child_instance_row: dependency.child_row,
        })
        .collect();
    let plan = ActionBandExecutionPlan::from_admitted_numeric_tables(
        templates,
        target_channels,
        target_data,
        bands,
        band_binding_indices,
        emission_bindings.clone(),
        eml_nodes,
        eml_ranges,
        active_instances,
        dependencies,
        buckets,
        frozen.budget().storage_rows,
        depth1_crossing_fast_path,
        native_lanes.is_some(),
    )
    .map_err(ActionBandExecutionCompileError::from)?;

    let mut structural_sources = Vec::with_capacity(frozen.bands().len());
    for (band_index, band) in frozen.bands().iter().enumerate() {
        let source = frozen
            .crossing_binding_for_band(band_index as u32)
            .filter(|source| source.threshold_registration() == band.threshold_registration())
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenCrossingSource)?;
        let span = band.emission_binding_span();
        let binding_indices = frozen
            .emission_bindings()
            .get(span.start() as usize..(span.start() + span.len()) as usize)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
        for binding_index in binding_indices {
            let binding = emission_bindings
                .get(binding_index.raw() as usize)
                .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
            if binding.destination() == ActionBandEmissionDestination::StructuralRequest {
                structural_sources.push(FrozenStructuralSource {
                    event_kind: source.event_kind(),
                    destination_index: binding.destination_index(),
                });
            }
        }
    }
    let mut event_kind_to_template = BTreeMap::new();
    for band_index in 0..frozen.bands().len() {
        if let Some(source) = frozen.crossing_binding_for_band(band_index as u32) {
            event_kind_to_template
                .entry(source.event_kind())
                .or_insert_with(|| source.template());
        }
    }
    let plan_fingerprint = plan.numeric_fingerprint();
    Ok(CompiledActionBandGpuExecution {
        plan,
        structural_sources,
        plan_fingerprint,
        event_kind_to_template,
        session_origin: ActionBandSessionOrigin::mint(),
        frozen_admission_binding: frozen_admission_binding_id(frozen),
        conserved_progress_bindings,
    })
}

fn lower_conserved_progress_bindings(
    frozen: &FrozenActionBandTemplates,
    emission_bindings: &mut [ActionBandEmissionBindingGpu],
    has_native_lanes: bool,
) -> Result<Vec<CompiledActionBandConservedProgressBinding>, ActionBandExecutionCompileError> {
    let mut compiled = Vec::with_capacity(frozen.conserved_progress_bindings().len());
    for admitted in frozen.conserved_progress_bindings() {
        let band = frozen
            .bands()
            .get(admitted.band_table_index() as usize)
            .ok_or(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            )?;
        if band.threshold_registration() != admitted.bound_source().threshold_registration() {
            return Err(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            );
        }
        let span = band.emission_binding_span();
        let in_band = frozen
            .emission_bindings()
            .get(span.start() as usize..(span.start() + span.len()) as usize)
            .is_some_and(|rows| rows.iter().any(|row| row == &admitted.emission_binding()));
        if !in_band {
            return Err(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            );
        }
        if !has_native_lanes {
            return Err(
                ActionBandExecutionCompileError::ConservedProgressRequiresNativeLane {
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            );
        }
        let emission = emission_bindings
            .get_mut(admitted.emission_binding().raw() as usize)
            .ok_or(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            )?;
        if emission.conserved_progress_bound_source()
            != ActionBandEmissionBindingGpu::CONSERVED_BOUND_NONE
        {
            return Err(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            );
        }
        match emission.destination() {
            ActionBandEmissionDestination::PropertyNext
            | ActionBandEmissionDestination::RfClaim
            | ActionBandEmissionDestination::CostBand => {}
            destination => {
                return Err(
                    ActionBandExecutionCompileError::ConservedProgressDestinationDeferred {
                        destination,
                    },
                )
            }
        }
        let source = frozen
            .crossing_binding_for_band(admitted.band_table_index())
            .filter(|source| source.threshold_registration() == band.threshold_registration())
            .ok_or(
                ActionBandExecutionCompileError::InvalidConservedProgressBinding {
                    band_table_index: admitted.band_table_index(),
                    emission_binding_index: admitted.emission_binding().raw(),
                },
            )?;
        let bound_source_code = match admitted.bound_source() {
            AdmittedActionBandConservedProgressBoundSource::RfGrant(_) => {
                ActionBandEmissionBindingGpu::CONSERVED_BOUND_RF_GRANT
            }
            AdmittedActionBandConservedProgressBoundSource::GuYangAvailable(_) => {
                ActionBandEmissionBindingGpu::CONSERVED_BOUND_GU_YANG_AVAILABLE
            }
            AdmittedActionBandConservedProgressBoundSource::GuYangRealized(_) => {
                ActionBandEmissionBindingGpu::CONSERVED_BOUND_GU_YANG_REALIZED
            }
        };
        *emission = emission.with_conserved_progress_bound_source(bound_source_code);
        compiled.push(CompiledActionBandConservedProgressBinding {
            template: admitted.template(),
            band_table_index: admitted.band_table_index(),
            emission_binding_index: admitted.emission_binding().raw(),
            bound_source: admitted.bound_source(),
            threshold_column: source.threshold_column(),
            destination: emission.destination(),
        });
    }
    Ok(compiled)
}

#[derive(Clone, Copy, Debug)]
struct LoweredDependency {
    parent_row: u32,
    child_row: u32,
}

fn lower_dependency_rows(
    frozen: &FrozenActionBandTemplates,
    instances: &[ActionBandActiveInstance],
) -> Result<(Vec<LoweredDependency>, BTreeSet<usize>), ActionBandExecutionCompileError> {
    let mut rows = BTreeMap::new();
    let mut rows_per_template = BTreeMap::<u32, u32>::new();
    for (row, instance) in instances.iter().enumerate() {
        if rows
            .insert(
                (instance.template().raw(), instance.slot().raw()),
                row as u32,
            )
            .is_some()
        {
            return Err(ActionBandExecutionError::DuplicateActiveInstance.into());
        }
        *rows_per_template
            .entry(instance.template().raw())
            .or_default() += 1;
    }
    if instances.len() <= frozen.budget().storage_rows as usize {
        for (&template_index, &actual) in &rows_per_template {
            let template = frozen
                .templates()
                .get(template_index as usize)
                .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
            if actual > template.reserved_instance_rows() {
                return Err(ActionBandExecutionCompileError::TemplateRowBudgetExceeded {
                    template: template_index,
                    actual,
                    reserved: template.reserved_instance_rows(),
                });
            }
        }
    }

    let mut lowered = Vec::new();
    let mut subordinate_rows = BTreeSet::new();
    for (parent_row, instance) in instances.iter().enumerate() {
        let template = frozen
            .templates()
            .get(instance.template().raw() as usize)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
        let span = template.dependency_span();
        if span.len() > template.max_active_subordinates() {
            return Err(
                ActionBandExecutionCompileError::DependencyCapacityDeferred {
                    parent_template: instance.template().raw(),
                    dependency_count: span.len(),
                    max_active: template.max_active_subordinates(),
                },
            );
        }
        let dependencies = frozen
            .dependencies()
            .get(span.start() as usize..(span.start() + span.len()) as usize)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
        for child_template in dependencies {
            let key = (child_template.raw(), instance.slot().raw());
            let Some(&child_row) = rows.get(&key) else {
                return Err(ActionBandExecutionCompileError::MissingPreAdmittedChild {
                    parent_template: instance.template().raw(),
                    child_template: child_template.raw(),
                    slot: instance.slot().raw(),
                });
            };
            if child_row == parent_row as u32 {
                return Err(ActionBandExecutionCompileError::DependencyCycle);
            }
            if !subordinate_rows.insert(child_row as usize) {
                return Err(ActionBandExecutionCompileError::SharedChildLifecycle {
                    child_template: child_template.raw(),
                    slot: instance.slot().raw(),
                });
            }
            lowered.push(LoweredDependency {
                parent_row: parent_row as u32,
                child_row,
            });
        }
    }

    for (row, instance) in instances.iter().enumerate() {
        if subordinate_rows.contains(&row) && instance.is_initially_active() {
            return Err(ActionBandExecutionCompileError::ChildMustBeginInactive {
                child_template: instance.template().raw(),
                slot: instance.slot().raw(),
            });
        }
        if !subordinate_rows.contains(&row) && !instance.is_initially_active() {
            return Err(ActionBandExecutionCompileError::UnclaimedInactiveRow {
                template: instance.template().raw(),
                slot: instance.slot().raw(),
            });
        }
    }

    // Dependency order is not semantic. Canonical physical row order makes an
    // authored append/reversal perturbation compile to the identical table.
    lowered.sort_by_key(|dependency| (dependency.parent_row, dependency.child_row));

    let mut marks = vec![0u8; instances.len()];
    for row in 0..instances.len() {
        reject_dependency_cycle(row, &lowered, &mut marks)?;
    }
    Ok((lowered, subordinate_rows))
}

fn reject_dependency_cycle(
    row: usize,
    dependencies: &[LoweredDependency],
    marks: &mut [u8],
) -> Result<(), ActionBandExecutionCompileError> {
    match marks[row] {
        1 => return Err(ActionBandExecutionCompileError::DependencyCycle),
        2 => return Ok(()),
        _ => {}
    }
    marks[row] = 1;
    for dependency in dependencies
        .iter()
        .filter(|dependency| dependency.parent_row as usize == row)
    {
        reject_dependency_cycle(dependency.child_row as usize, dependencies, marks)?;
    }
    marks[row] = 2;
    Ok(())
}

fn depth1_crossing_fast_path(
    frozen: &FrozenActionBandTemplates,
    active_instances: &[ActionBandActiveInstance],
) -> Result<bool, ActionBandExecutionCompileError> {
    if active_instances.is_empty() {
        return Ok(false);
    }
    for instance in active_instances {
        let template = frozen
            .templates()
            .get(instance.template().raw() as usize)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
        if template.band_span().len() != 1 || template.velocity().is_some() {
            return Ok(false);
        }
        let target_channel = match template.target() {
            AdmittedActionBandTarget::Point {
                current_channels, ..
            } if current_channels.len() == 1 => current_channels[0],
            AdmittedActionBandTarget::ScalarBound { channel, .. }
            | AdmittedActionBandTarget::Interval { channel, .. } => *channel,
            AdmittedActionBandTarget::AxisAlignedBox { channels, .. } if channels.len() == 1 => {
                channels[0]
            }
            AdmittedActionBandTarget::LocusRadius {
                distance_channel, ..
            }
            | AdmittedActionBandTarget::PalmaReachableSet {
                distance_channel, ..
            } => *distance_channel,
            _ => return Ok(false),
        };
        let band_index = template.band_span().start();
        let source = frozen
            .crossing_binding_for_band(band_index)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenCrossingSource)?;
        if source.threshold_column() != target_channel {
            return Ok(false);
        }
    }
    Ok(true)
}

fn range_for(
    tree_id: EmlTreeId,
    ranges: &BTreeMap<EmlTreeId, u32>,
) -> Result<u32, ActionBandExecutionCompileError> {
    ranges
        .get(&tree_id)
        .copied()
        .ok_or(ActionBandExecutionCompileError::MissingEmlProgram(
            tree_id.raw(),
        ))
}

fn deterministic_buckets(
    bands: &[ActionBandBandGpu],
    binding_indices: &[u32],
    bindings: &[ActionBandEmissionBindingGpu],
) -> Result<Vec<ActionBandExecutionBucket>, ActionBandExecutionCompileError> {
    let mut grouped: BTreeMap<(u32, Vec<ActionBandEmissionDestination>), Vec<u32>> =
        BTreeMap::new();
    for (band_index, band) in bands.iter().enumerate() {
        let start = band.binding_start as usize;
        let end = start + band.binding_count as usize;
        let indices = binding_indices
            .get(start..end)
            .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)?;
        let shape = indices
            .iter()
            .map(|&index| {
                bindings
                    .get(index as usize)
                    .copied()
                    .map(ActionBandEmissionBindingGpu::destination)
                    .ok_or(ActionBandExecutionCompileError::InvalidFrozenSpan)
            })
            .collect::<Result<Vec<_>, _>>()?;
        grouped
            .entry((band.program_range, shape))
            .or_default()
            .push(band_index as u32);
    }
    Ok(grouped
        .into_iter()
        .map(
            |((program_range, destination_shape), band_indices)| ActionBandExecutionBucket {
                program_range,
                destination_shape,
                band_indices,
            },
        )
        .collect())
}
