//! simthing-sim — day boundary orchestration and structural mutation.
//!
//! Implements design_v4.md §10 "The Day Boundary" — the 10-step boundary
//! protocol that executes between ticks when the `DispatchCoordinator`
//! signals `boundary_reached = true`.
//!
//! ## Module map
//!
//! - `threshold_registry` — CPU-side event_kind registry. Maps every `u32`
//!   event_kind emitted by GPU Pass 7 to a `ThresholdSemantic` (fission,
//!   fusion, property expiry, velocity alert). Also contains the
//!   `ThresholdBuilder` that derives both GPU `ThresholdRegistration` structs
//!   and the parallel CPU semantics vec from the live SimThing tree.
//!
//! - `overlay_lifecycle` — steps 4 + 7. Checks dissolution conditions
//!   (PropertyReaches, PropertyBelow, AfterTicks, OverrideReceived) against
//!   current GPU values + day counter; culls dissolved overlays and decrements
//!   AfterTicks counters. Applies `on_expire` `ExpireEffect`s to the CPU
//!   shadow. Routes new instruction overlays from `BoundaryRequest::AttachOverlay`.
//!
//! - `property_expiry` — step 5. Consumes `ThresholdEvent`s whose `event_kind`
//!   maps to `ThresholdSemantic::PropertyExpiry`. Removes the property from the
//!   SimThing's `properties` HashMap and tombstones the registry column if this
//!   was its last live instance.
//!
//! - `fission` — step 6. Executes fission events from Pass 7 output and
//!   contains the current placeholder fusion event handler.
//!   Fission: spawns a new child `SimThing`, allocs a slot, seeds its GPU row
//!   from the parent's current values. Fusion lineage threshold registration
//!   is not wired yet.
//!
//! - `tree_mutation` — steps 7 + 8. Executes every `BoundaryRequest` variant:
//!   `AddChild` (alloc slot, attach), `Remove` (tombstone subtree, detach),
//!   `Reparent` (move subtree, slots preserved — the whole point of slot
//!   stability), `AttachOverlay` (origin-to-target route into the overlay vec),
//!   `AddDimension` (boundary-time registry activation + GPU layout rebuild).
//!
//! - `gpu_sync` — step 9. After all structural mutations are done, rebuilds the
//!   GPU buffer state: derived overlay span projection → upload, threshold registration
//!   rebuild → upload, dirty-row flush via the `DispatchCoordinator` shadow.
//!
//! - `boundary` — top-level `BoundaryProtocol` struct that owns the SimThing
//!   tree root and orchestrates the full §10 sequence in one call.
//!
//! Semantic kind names must not be imported from this crate
//! (`sim_public_surface_rejects_kind_import_compile_fail`):
//!
//! ```compile_fail,E0432
//! use simthing_sim::SimThingKind;
//! ```

#![forbid(unsafe_code)]

pub mod accumulator_plan_tick;
pub mod anchor_remap_encode;
pub mod boundary;
pub mod delta_log;
pub mod fission;
pub mod fission_clone_source_view;
pub mod gpu_sync;
pub mod growth_entitlement;
pub mod legacy_oracle;
pub mod mapping_atlas_scheduler;
pub mod mapping_plan_tick;
pub mod observability;
pub mod overlay_lifecycle;
pub mod property_expiry;
pub mod reduced_field;
pub mod replay;
pub mod resolution_site;
pub mod sim_runtime_tree;
pub mod threshold_registry;
pub(crate) mod tree_index;
pub mod tree_mutation;

pub use accumulator_plan_tick::{
    execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu,
    execute_accumulator_plan_tick_with_backend, gpu_context_blocking,
    readback_resident_accumulator_values_for_proof, AccumulatorTickBackend,
    SimGpuAccumulatorTickState, SimGpuReadbackPolicy, SimTickError,
};
pub use anchor_remap_encode::{
    build_exact_anchor_remap_section, gate_structural_gpu_encode, gate_structural_gpu_encode_exact,
    required_anchored_loci_for_boundary, snapshot_anchored_loci,
};
pub use boundary::{
    BoundaryHookContext, BoundaryOutcome, BoundaryProtocol, BoundaryTiming, PipelineFlags,
};
pub use delta_log::BoundaryDeltaEntry;
pub use fission::{ClonedCapabilityRoot, FissionLineageRecord, FissionOutcome};
pub use fission_clone_source_view::FissionCloneSourceView;
pub use gpu_sync::{GpuSyncError, GpuSyncOutcome};
pub use growth_entitlement::{
    GrowthEntitlementBatchError, GrowthEntitlementDecision, OrdinaryGrowthCandidate,
    OrdinaryGrowthOrigin, OrdinaryGrowthRefusal, OrdinaryGrowthRefusalReason,
    RecordedGrowthResidencyFact,
};
pub use legacy_oracle::{
    apply_oracle_flags, assert_events_oracle, assert_values_oracle, run_family_oracle,
    LegacyOracleRun, OracleCapture, OracleExactness, OracleFamily, OracleScenario,
};
pub use mapping_atlas_scheduler::{
    CompiledMappingAtlas, MappingAtlasTickInputs, MappingAtlasTickOutput, MappingTheaterSlot,
    SimGpuMappingAtlasScheduler,
};
pub use mapping_plan_tick::{
    cpu_min_plus_d_from_composed_interleaved, cpu_structured_field_horizon, CompiledMappingPlan,
    CompiledMappingStep, MappingTickInputs, SimGpuMappingReadbackPolicy, SimGpuMappingTickOutput,
    SimGpuMappingTickState,
};
pub use observability::{
    ObservabilityReport, ObserveFidelity, OverlayContribution, PropertyObservation,
    SubFieldObservation,
};
pub use reduced_field::ReducedField;
pub use replay::{
    ReplayDriver, ReplayError, ReplayFrame, ReplayGrowthError, ReplayReader, ReplayRecord,
    ReplaySnapshot, ReplayWriter,
};
pub use resolution_site::{
    collect_aggregate_alerts_vendorized, collect_velocity_alerts_vendorized,
    mint_attach_overlay_at_barrier, reattach_aggregate_alerts_at_barrier,
    reattach_velocity_alerts_at_barrier, ResolutionSite, SlotIdentityReattachError,
    SlotSpaceOverlayDraft,
};
pub use sim_runtime_tree::SimRuntimeTree;
pub use threshold_registry::{
    assert_no_hard_trigger_on_soft_aggregate, AggregateAlertEvent, AggregateAlertRegistration,
    CostBandSemantic, SoftAggregateViolation, ThresholdBuilder, ThresholdRegistry,
    ThresholdSemantic, VelocityAlertEvent, VelocityAlertRegistration,
};
pub use tree_mutation::{
    apply_structural_mutations, StructuralCommitmentApplicationDoor,
    StructuralCommitmentApplicationError,
};

#[cfg(test)]
mod dependency_budget;
#[cfg(test)]
mod threshold_event_test_fixtures;
