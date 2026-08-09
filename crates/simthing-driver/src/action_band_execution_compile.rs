//! Driver lowering from the frozen 7.1 admission product to the sanctioned
//! sparse ActionBand GPU operator.  This is a numeric projection only: labels
//! never enter a key, and crossing evidence remains kernel-sealed.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    eml_nodes::execution_class_to_u32, EmlExpressionRegistry, EmlTreeId, SlotIndex,
};
use simthing_feeder::{BoundaryRequest, FeederSender};
use simthing_gpu::{
    action_band_target_kind, ActionBandActiveInstanceGpu, ActionBandBandGpu,
    ActionBandEmissionBindingGpu, ActionBandEmissionDestination, ActionBandExecutionBucket,
    ActionBandExecutionError, ActionBandExecutionPlan, ActionBandTemplateGpu, EmlTreeRangeGpu,
    StructuralCommitment, ACTIONBAND_NO_PROGRAM,
};
use simthing_sim::{StructuralCommitmentApplicationDoor, StructuralCommitmentApplicationError};
use simthing_spec::{
    ActionBandTemplateIndex, AdmittedActionBandTarget, FrozenActionBandTemplates,
    ScalarBoundDirection,
};
use thiserror::Error;

/// Sparse session-build activation. There is deliberately no label or domain
/// field; execution identity is admitted numeric template identity + slot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionBandActiveInstance {
    template: ActionBandTemplateIndex,
    slot: SlotIndex,
    params: [f32; 4],
}

impl ActionBandActiveInstance {
    pub fn new(template: ActionBandTemplateIndex, slot: SlotIndex, params: [f32; 4]) -> Self {
        Self {
            template,
            slot,
            params,
        }
    }

    pub fn template(self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn slot(self) -> SlotIndex {
        self.slot
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
    #[error(transparent)]
    Kernel(#[from] ActionBandExecutionError),
}

#[derive(Clone, Copy, Debug)]
struct FrozenStructuralSource {
    event_kind: u32,
    destination_index: u32,
}

/// One source-bound compilation of the frozen 7.1 admission product. The GPU
/// plan and structural application provenance are derived together from the
/// same immutable band/binding rows, so a caller cannot supply a detached
/// `(event_kind, binding)` assertion beside the plan.
#[derive(Clone, Debug)]
pub struct CompiledActionBandGpuExecution {
    plan: ActionBandExecutionPlan,
    structural_sources: Vec<FrozenStructuralSource>,
}

impl CompiledActionBandGpuExecution {
    pub fn execution_plan(&self) -> &ActionBandExecutionPlan {
        &self.plan
    }

    pub fn into_execution_plan(self) -> ActionBandExecutionPlan {
        self.plan
    }
}

/// Session-fixed structural consequences addressed by the numeric destination
/// index in an admitted emission binding. The boundary applies the selected row
/// verbatim; it does not inspect payload, distance, satisfaction, or EML.
#[derive(Clone, Debug)]
pub struct FrozenActionBandStructuralRequests {
    door: StructuralCommitmentApplicationDoor,
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
        })
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
    let required_bindings = frozen.budget().emission_binding_count;
    if pre_admitted_emission_bindings.len() != required_bindings as usize {
        return Err(ActionBandExecutionCompileError::EmissionTableWidth {
            required: required_bindings,
            actual: pre_admitted_emission_bindings.len(),
        });
    }

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

    let buckets = deterministic_buckets(
        &bands,
        &band_binding_indices,
        pre_admitted_emission_bindings,
    )?;
    let depth1_crossing_fast_path = depth1_crossing_fast_path(frozen, active_instances)?;
    let active_instances = active_instances
        .iter()
        .map(|instance| ActionBandActiveInstanceGpu {
            slot: instance.slot.raw(),
            template_index: instance.template.raw(),
            projection_start: 0,
            generation: 0,
            params: instance.params,
        })
        .collect();
    let plan = ActionBandExecutionPlan::from_admitted_numeric_tables(
        templates,
        target_channels,
        target_data,
        bands,
        band_binding_indices,
        pre_admitted_emission_bindings.to_vec(),
        eml_nodes,
        eml_ranges,
        active_instances,
        buckets,
        frozen.budget().storage_rows,
        depth1_crossing_fast_path,
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
            let binding = pre_admitted_emission_bindings
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
    Ok(CompiledActionBandGpuExecution {
        plan,
        structural_sources,
    })
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
