//! One session-frozen post-crossing consequence door for ActionBand (7.8).
//!
//! The door consumes only the existing sealed Phase-5 crossing surface. Its
//! three arms compile to the existing ActionBand emission table and the
//! existing boundary channel; there is no comparator, listener, or second
//! dispatcher here.

use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    admit_overlay_lifecycle, GenerationStamp, GenerationStamped, Overlay,
    OverlayLifecycleAdmitError, SimPropertyId, SimThingId, SubFieldRole,
};
use simthing_feeder::{BoundaryRequest, FeederError, FeederSender};
use simthing_gpu::{
    ActionBandCrossingBatch, ActionBandCrossingConsumptionKey, ActionBandEmissionBindingGpu,
    ActionBandEmissionDestination, ActionBandGpuExecution, ActionBandGpuSession,
    ActionBandProductionDispatch, BandCrossingDelta, GpuContext,
};
use simthing_spec::FrozenActionBandTemplates;
use thiserror::Error;

use crate::action_band_execution_compile::{
    compile_action_band_gpu_execution_with_native_lanes, ActionBandActiveInstance,
    ActionBandExecutionCompileError, ActionBandNativeLaneAdmission, ActionBandNativeLaneOrigin,
    ActionBandSessionOrigin, CompiledActionBandGpuExecution,
};

/// The complete admitted post-crossing vocabulary. Adding a fourth arm is an
/// API change, not runtime registration data.
#[derive(Clone, Debug)]
pub enum CrossingConsequenceBinding {
    ResidentNextWrite(ResidentNextWrite),
    RoutedOverlayDelivery(RoutedOverlayDelivery),
    StructuralAuthorization(StructuralAuthorization),
}

impl CrossingConsequenceBinding {
    /// Shared boundary submission for ActionBand and legacy commitment crossings.
    pub fn submit_boundary(
        &self,
        source_generation: GenerationStamp,
        boundary: &FeederSender,
    ) -> Result<(), CrossingConsequenceDispatchError> {
        match self {
            Self::ResidentNextWrite(_) => {
                Err(CrossingConsequenceDispatchError::ResidentWriteReadback)
            }
            Self::RoutedOverlayDelivery(route) => {
                route.submit(source_generation, boundary)?;
                Ok(())
            }
            Self::StructuralAuthorization(authorization) => {
                boundary.submit_boundary(authorization.request.clone())?;
                Ok(())
            }
        }
    }
}

/// A write to an already-admitted native Next lane.
///
/// Logical property/role identity is retained with the physical column chosen
/// for this compile. No physical row, slot, or buffer handle can be stored.
#[derive(Clone, Debug)]
pub struct ResidentNextWrite {
    gpu_binding: ActionBandEmissionBindingGpu,
    native_lane_origin: ActionBandNativeLaneOrigin,
    property_id: SimPropertyId,
    role: SubFieldRole,
}

impl ResidentNextWrite {
    pub fn property_id(&self) -> SimPropertyId {
        self.property_id
    }

    pub fn role(&self) -> &SubFieldRole {
        &self.role
    }

    pub fn destination(&self) -> ActionBandEmissionDestination {
        self.gpu_binding.destination()
    }

    pub fn column(&self) -> u32 {
        self.gpu_binding.destination_index()
    }
}

impl ActionBandNativeLaneAdmission {
    /// Bind one resident write only after the ordinary native-lane admission
    /// has proved both its destination class and logical PropertyId/role.
    pub fn bind_resident_next(
        &self,
        binding: ActionBandEmissionBindingGpu,
    ) -> Result<CrossingConsequenceBinding, CrossingConsequenceAdmissionError> {
        if !self.admits(binding) {
            return Err(CrossingConsequenceAdmissionError::UnadmittedResidentLane {
                destination: binding.destination(),
                column: binding.destination_index(),
            });
        }
        let (property_id, role) = self
            .logical_destination(binding.destination_index())
            .ok_or(
                CrossingConsequenceAdmissionError::MissingLogicalDestination(
                    binding.destination_index(),
                ),
            )?;
        Ok(CrossingConsequenceBinding::ResidentNextWrite(
            ResidentNextWrite {
                gpu_binding: binding,
                native_lane_origin: self.origin(),
                property_id,
                role,
            },
        ))
    }
}

/// Frozen routed delivery. Duration remains authored inside `overlay.lifecycle`;
/// source generation is supplied only by the sealed dispatch at execution.
/// There is deliberately no absolute-deadline field.
#[derive(Clone, Debug)]
pub struct RoutedOverlayDelivery {
    target: SimThingId,
    overlay: Overlay,
}

/// Production carrier for a routed consequence crossing a tree seam.
/// Absolute deadlines are not fields and unknown serialized fields are rejected.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutedOverlayProduct {
    target: SimThingId,
    overlay: Overlay,
}

impl RoutedOverlayDelivery {
    pub fn admit(
        target: SimThingId,
        overlay: Overlay,
    ) -> Result<CrossingConsequenceBinding, CrossingConsequenceAdmissionError> {
        admit_overlay_lifecycle(&overlay.lifecycle)?;
        Ok(CrossingConsequenceBinding::RoutedOverlayDelivery(Self {
            target,
            overlay,
        }))
    }

    pub fn target(&self) -> SimThingId {
        self.target
    }

    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn stamped_product(
        &self,
        source_generation: GenerationStamp,
    ) -> GenerationStamped<RoutedOverlayProduct> {
        GenerationStamped::stamp(
            source_generation,
            RoutedOverlayProduct {
                target: self.target,
                overlay: self.overlay.clone(),
            },
        )
    }

    pub fn submit(
        &self,
        source_generation: GenerationStamp,
        boundary: &FeederSender,
    ) -> Result<(), FeederError> {
        submit_routed_overlay_product(self.stamped_product(source_generation), boundary)
    }
}

pub fn submit_routed_overlay_product(
    stamped: GenerationStamped<RoutedOverlayProduct>,
    boundary: &FeederSender,
) -> Result<(), FeederError> {
    boundary.submit_boundary(BoundaryRequest::AttachOverlay {
        target: stamped.product.target,
        overlay: stamped.product.overlay,
        source_generation: stamped.generation,
    })
}

/// Authorization for an existing structural boundary verb. This type has no
/// GPU destination, plane owner, column, value, or write method; using this arm
/// for a same-facility state-plane write is unrepresentable.
#[derive(Clone, Debug)]
pub struct StructuralAuthorization {
    request: BoundaryRequest,
}

impl StructuralAuthorization {
    pub fn admit(
        request: BoundaryRequest,
    ) -> Result<CrossingConsequenceBinding, CrossingConsequenceAdmissionError> {
        match request {
            request @ (BoundaryRequest::AddChild { .. }
            | BoundaryRequest::Remove { .. }
            | BoundaryRequest::Reparent { .. }) => {
                Ok(CrossingConsequenceBinding::StructuralAuthorization(Self {
                    request,
                }))
            }
            BoundaryRequest::AttachOverlay { .. }
            | BoundaryRequest::ActivateOverlay { .. }
            | BoundaryRequest::SuspendOverlay { .. }
            | BoundaryRequest::AddDimension { .. } => {
                Err(CrossingConsequenceAdmissionError::NonStructuralBoundaryVerb)
            }
        }
    }

    pub fn request(&self) -> &BoundaryRequest {
        &self.request
    }
}

#[derive(Clone, Debug)]
struct FrozenConsequences {
    session_origin: ActionBandSessionOrigin,
    by_event_kind: BTreeMap<u32, CrossingConsequenceBinding>,
}

#[derive(Debug, Default)]
struct GenerationBoundCrossingDedupe {
    generation_offset: Option<u32>,
    generation: Option<u32>,
    keys: HashSet<ActionBandCrossingConsumptionKey>,
}

impl GenerationBoundCrossingDedupe {
    fn admit(
        &mut self,
        facility_generation: u32,
        keys: &[ActionBandCrossingConsumptionKey],
    ) -> Result<(), CrossingConsequenceDispatchError> {
        let Some(generation_offset) = self.generation_offset.or_else(|| {
            keys.first()
                .and_then(|key| key.generation().checked_sub(facility_generation))
        }) else {
            if let Some(key) = keys.first() {
                return Err(
                    CrossingConsequenceDispatchError::CrossingGenerationMismatch {
                        expected: facility_generation,
                        actual: key.generation(),
                    },
                );
            }
            return Ok(());
        };
        let executable_generation = generation_offset
            .checked_add(facility_generation)
            .ok_or(CrossingConsequenceDispatchError::GenerationWatermarkOverflow)?;
        self.synchronize(executable_generation);
        if let Some(key) = keys
            .iter()
            .find(|key| key.generation() != executable_generation)
        {
            return Err(
                CrossingConsequenceDispatchError::CrossingGenerationMismatch {
                    expected: executable_generation,
                    actual: key.generation(),
                },
            );
        }
        if keys.iter().any(|key| self.keys.contains(key)) {
            return Err(CrossingConsequenceDispatchError::DuplicateCrossingConsumption);
        }
        self.generation_offset = Some(generation_offset);
        self.keys.extend(keys.iter().cloned());
        Ok(())
    }

    fn observe_boundary(
        &mut self,
        facility_generation: u32,
    ) -> Result<(), CrossingConsequenceDispatchError> {
        let Some(generation_offset) = self.generation_offset else {
            return Ok(());
        };
        let executable_generation = generation_offset
            .checked_add(facility_generation)
            .ok_or(CrossingConsequenceDispatchError::GenerationWatermarkOverflow)?;
        self.synchronize(executable_generation);
        Ok(())
    }

    fn synchronize(&mut self, executable_generation: u32) {
        if self.generation != Some(executable_generation) {
            self.generation = Some(executable_generation);
            self.keys.clear();
        }
    }

    fn proof_snapshot(&self) -> (Option<u32>, usize) {
        (self.generation, self.keys.len())
    }
}

/// One source-bound compile product: GPU plan and all three consequence arms
/// are frozen together and cannot be paired across sessions.
///
/// This type is deliberately neither `Clone` nor multiply bindable:
/// [`Self::bind_dispatch`] consumes it, so one admitted consequence session
/// owns exactly one facility generation boundary.
#[derive(Debug)]
pub struct CrossingConsequenceSession {
    compiled: CompiledActionBandGpuExecution,
    frozen: FrozenConsequences,
}

impl CrossingConsequenceSession {
    pub fn compiled(&self) -> &CompiledActionBandGpuExecution {
        &self.compiled
    }

    pub fn binding_for_event_kind(&self, event_kind: u32) -> Option<&CrossingConsequenceBinding> {
        self.frozen.by_event_kind.get(&event_kind)
    }

    pub fn bind_dispatch(
        self,
        ctx: &GpuContext,
        resident_values: &[f32],
    ) -> Result<CrossingConsequenceDispatch, CrossingConsequenceDispatchError> {
        if self.frozen.session_origin != self.compiled.session_origin() {
            return Err(CrossingConsequenceDispatchError::ForeignCompile);
        }
        let execution = match ActionBandGpuExecution::new_with_resident_next(
            ctx,
            self.compiled.execution_plan().clone(),
            resident_values,
        )
        .map_err(|error| CrossingConsequenceDispatchError::Gpu(error.to_string()))?
        {
            ActionBandGpuExecution::Active(session) => session,
            ActionBandGpuExecution::Inactive => {
                return Err(CrossingConsequenceDispatchError::Gpu(
                    "ActionBand consequence session is inactive".into(),
                ))
            }
        };
        Ok(CrossingConsequenceDispatch {
            frozen: self.frozen,
            execution,
            generation_dedupe: GenerationBoundCrossingDedupe::default(),
        })
    }
}

/// Compile one complete consequence table. Rows are indexed by the pre-admitted
/// emission indices in `FrozenActionBandTemplates`; runtime table growth is
/// impossible because the returned session owns the frozen rows.
pub fn compile_crossing_consequence_session(
    frozen: &FrozenActionBandTemplates,
    eml_registry: &simthing_core::EmlExpressionRegistry,
    consequence_rows: &[CrossingConsequenceBinding],
    active_instances: &[ActionBandActiveInstance],
    native_lanes: &ActionBandNativeLaneAdmission,
) -> Result<CrossingConsequenceSession, CrossingConsequenceAdmissionError> {
    let required = frozen
        .emission_bindings()
        .iter()
        .map(|index| index.raw() as usize + 1)
        .max()
        .unwrap_or(0);
    if consequence_rows.len() != required {
        return Err(CrossingConsequenceAdmissionError::ConsequenceTableWidth {
            required,
            actual: consequence_rows.len(),
        });
    }
    if consequence_rows.iter().any(|row| {
        matches!(
            row,
            CrossingConsequenceBinding::ResidentNextWrite(write)
                if write.native_lane_origin != native_lanes.origin()
        )
    }) {
        return Err(CrossingConsequenceAdmissionError::ForeignResidentLaneAdmission);
    }

    // Structural destination indices address the compact pre-admitted
    // boundary table, not the wider emission table. Preserve that graduated
    // ABI so adding native rows before a boundary row cannot renumber it.
    let mut boundary_row = 0u32;
    let gpu_bindings = consequence_rows
        .iter()
        .map(|consequence| match consequence {
            CrossingConsequenceBinding::ResidentNextWrite(write) => write.gpu_binding,
            CrossingConsequenceBinding::RoutedOverlayDelivery(_)
            | CrossingConsequenceBinding::StructuralAuthorization(_) => {
                let binding = ActionBandEmissionBindingGpu::structural_request(boundary_row);
                boundary_row += 1;
                binding
            }
        })
        .collect::<Vec<_>>();
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        frozen,
        eml_registry,
        &gpu_bindings,
        active_instances,
        native_lanes,
    )?;

    let mut by_event_kind = BTreeMap::new();
    for (band_index, band) in frozen.bands().iter().enumerate() {
        let crossing = frozen
            .crossing_binding_for_band(band_index as u32)
            .ok_or(CrossingConsequenceAdmissionError::InvalidFrozenCrossing)?;
        let span = band.emission_binding_span();
        for index in frozen
            .emission_bindings()
            .get(span.start() as usize..(span.start() + span.len()) as usize)
            .ok_or(CrossingConsequenceAdmissionError::InvalidFrozenCrossing)?
        {
            let consequence = consequence_rows
                .get(index.raw() as usize)
                .ok_or(CrossingConsequenceAdmissionError::InvalidFrozenCrossing)?;
            if matches!(
                consequence,
                CrossingConsequenceBinding::ResidentNextWrite(_)
            ) {
                continue;
            }
            if by_event_kind
                .insert(crossing.event_kind(), consequence.clone())
                .is_some()
            {
                return Err(
                    CrossingConsequenceAdmissionError::AmbiguousBoundaryConsequence(
                        crossing.event_kind(),
                    ),
                );
            }
        }
    }
    Ok(CrossingConsequenceSession {
        frozen: FrozenConsequences {
            session_origin: compiled.session_origin(),
            by_event_kind,
        },
        compiled,
    })
}

/// The sole 7.8 post-crossing dispatch. It performs native GPU Next writes and
/// submits routed/structural requests in the same call. Sealed commitments are
/// consumed internally and are not returned for a rival dispatcher to replay.
/// `dispatch_and_apply` requires exclusive access, and every successful
/// non-empty depth-1 dispatch advances the facility boundary before returning;
/// therefore two successful batches cannot execute in one actual generation.
pub struct CrossingConsequenceDispatch {
    frozen: FrozenConsequences,
    execution: ActionBandGpuSession,
    generation_dedupe: GenerationBoundCrossingDedupe,
}

impl CrossingConsequenceDispatch {
    pub fn generation(&self) -> u32 {
        self.execution.facility_generation()
    }

    pub fn dispatch_and_apply(
        &mut self,
        ctx: &GpuContext,
        n_dims: u32,
        crossings: ActionBandCrossingBatch,
        boundary: &FeederSender,
    ) -> Result<CrossingConsequenceDispatchOutcome, CrossingConsequenceDispatchError> {
        let crossing_count = crossings.crossing_count() as u32;
        self.generation_dedupe.admit(
            self.execution.facility_generation(),
            crossings.consumption_keys(),
        )?;
        let production = self
            .execution
            .dispatch_resident_next(ctx, n_dims, &crossings)
            .map_err(|error| CrossingConsequenceDispatchError::Gpu(error.to_string()))?;
        self.generation_dedupe
            .observe_boundary(self.execution.facility_generation())?;
        let mut outcome = self.apply_boundary_consequences(production, boundary)?;
        outcome.crossing_count = crossing_count;
        Ok(outcome)
    }

    /// Ordinary-session ingress from the canonical Phase-5 boundary product.
    /// Empty boundaries do not manufacture facility generations; non-empty
    /// batches reuse the sole consuming dispatcher above.
    pub fn dispatch_sealed_and_apply(
        &mut self,
        ctx: &GpuContext,
        n_dims: u32,
        deltas: &[BandCrossingDelta],
        boundary: &FeederSender,
    ) -> Result<Option<CrossingConsequenceDispatchOutcome>, CrossingConsequenceDispatchError> {
        let crossings = self
            .execution
            .crossings_from_sealed(deltas)
            .map_err(|error| CrossingConsequenceDispatchError::Gpu(error.to_string()))?;
        if crossings.crossing_count() == 0 {
            return Ok(None);
        }
        self.dispatch_and_apply(ctx, n_dims, crossings, boundary)
            .map(Some)
    }

    /// Proof-only view of the bounded current-generation dedupe window.
    /// The actual facility boundary changes replace the set before dispatch
    /// returns; prior-generation identities are neither retained nor exposed.
    pub fn generation_dedupe_for_proof(
        &self,
    ) -> Result<(Option<u32>, usize), CrossingConsequenceDispatchError> {
        Ok(self.generation_dedupe.proof_snapshot())
    }

    pub fn resident_current_for_proof(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<f32>, CrossingConsequenceDispatchError> {
        self.execution
            .readback_resident_current_for_proof(ctx)
            .map_err(|error| CrossingConsequenceDispatchError::Gpu(error.to_string()))
    }

    fn apply_boundary_consequences(
        &self,
        production: ActionBandProductionDispatch,
        boundary: &FeederSender,
    ) -> Result<CrossingConsequenceDispatchOutcome, CrossingConsequenceDispatchError> {
        let source_generation = GenerationStamp::new(self.execution.facility_generation());
        let mut routed = 0u32;
        let mut structural = 0u32;
        for commitment in production.commitments {
            let binding = self
                .frozen
                .by_event_kind
                .get(&commitment.event_kind())
                .ok_or(CrossingConsequenceDispatchError::UnboundCommitment(
                    commitment.event_kind(),
                ))?;
            match binding {
                CrossingConsequenceBinding::ResidentNextWrite(_) => {
                    return Err(CrossingConsequenceDispatchError::ResidentWriteReadback)
                }
                CrossingConsequenceBinding::RoutedOverlayDelivery(_) => {
                    routed += 1;
                }
                CrossingConsequenceBinding::StructuralAuthorization(_) => {
                    structural += 1;
                }
            }
            binding.submit_boundary(source_generation, boundary)?;
        }
        Ok(CrossingConsequenceDispatchOutcome {
            generation: source_generation,
            crossing_count: 0,
            routed_deliveries: routed,
            structural_authorizations: structural,
            bucket_dispatches: production.bucket_dispatches,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrossingConsequenceDispatchOutcome {
    pub generation: GenerationStamp,
    pub crossing_count: u32,
    pub routed_deliveries: u32,
    pub structural_authorizations: u32,
    pub bucket_dispatches: u32,
}

#[derive(Debug, Error)]
pub enum CrossingConsequenceAdmissionError {
    #[error("consequence table has {actual} rows; frozen ActionBand table requires {required}")]
    ConsequenceTableWidth { required: usize, actual: usize },
    #[error("resident destination {destination:?} column {column} is not admitted locally")]
    UnadmittedResidentLane {
        destination: ActionBandEmissionDestination,
        column: u32,
    },
    #[error("resident destination column {0} has no logical PropertyId/role identity")]
    MissingLogicalDestination(u32),
    #[error("resident binding was admitted by a foreign native-lane facility")]
    ForeignResidentLaneAdmission,
    #[error("boundary verb is outside the closed structural-authorization vocabulary")]
    NonStructuralBoundaryVerb,
    #[error("frozen ActionBand crossing provenance is invalid")]
    InvalidFrozenCrossing,
    #[error("event_kind {0} has more than one boundary consequence")]
    AmbiguousBoundaryConsequence(u32),
    #[error(transparent)]
    Lifecycle(#[from] OverlayLifecycleAdmitError),
    #[error(transparent)]
    Compile(#[from] ActionBandExecutionCompileError),
}

#[derive(Debug, Error)]
pub enum CrossingConsequenceDispatchError {
    #[error("consequence door was paired with a foreign ActionBand compile")]
    ForeignCompile,
    #[error("sealed commitment event_kind {0} has no frozen consequence")]
    UnboundCommitment(u32),
    #[error("a resident Next write appeared on the boundary packet surface")]
    ResidentWriteReadback,
    #[error("sealed ActionBand crossing was already consumed by this consequence session")]
    DuplicateCrossingConsumption,
    #[error(
        "sealed ActionBand crossing generation {actual} is not executable generation {expected}"
    )]
    CrossingGenerationMismatch { expected: u32, actual: u32 },
    #[error("ActionBand crossing generation watermark overflowed")]
    GenerationWatermarkOverflow,
    #[error("ActionBand GPU consequence dispatch failed: {0}")]
    Gpu(String),
    #[error(transparent)]
    Feeder(#[from] FeederError),
}
