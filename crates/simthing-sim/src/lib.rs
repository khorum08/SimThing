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
//!   shadow. Attaches new instruction overlays from `BoundaryRequest::AttachOverlay`.
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
//!   stability), `AttachOverlay` (append to target's overlay vec),
//!   `AddDimension` (boundary-time registry activation + GPU layout rebuild).
//!
//! - `gpu_sync` — step 9. After all structural mutations are done, rebuilds the
//!   GPU buffer state: `build_overlay_deltas` → upload, threshold registration
//!   rebuild → upload, dirty-row flush via the `DispatchCoordinator` shadow.
//!
//! - `boundary` — top-level `BoundaryProtocol` struct that owns the SimThing
//!   tree root and orchestrates the full §10 sequence in one call.

pub mod accumulator_plan_tick;
pub mod boundary;
pub mod delta_log;
pub mod fission;
pub mod gpu_sync;
pub mod legacy_oracle;
pub mod observability;
pub mod overlay_lifecycle;
pub mod property_expiry;
pub mod reduced_field;
pub mod replay;
pub mod threshold_registry;
pub mod tree_index;
pub mod tree_mutation;

pub use accumulator_plan_tick::{
    execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu,
    execute_accumulator_plan_tick_with_backend, gpu_context_blocking, AccumulatorTickBackend,
    SimTickError,
};
pub use boundary::{
    BoundaryHookContext, BoundaryOutcome, BoundaryProtocol, BoundaryTiming, PipelineFlags,
};
pub use delta_log::{entries_from_outcome, BoundaryDeltaEntry};
pub use fission::{ClonedCapabilityRoot, FissionLineageRecord, FissionOutcome};
pub use legacy_oracle::{
    apply_oracle_flags, assert_events_oracle, assert_values_oracle, run_family_oracle,
    LegacyOracleRun, OracleCapture, OracleExactness, OracleFamily, OracleScenario,
};
pub use observability::{
    ObservabilityReport, ObserveFidelity, OverlayContribution, PropertyObservation,
    SubFieldObservation,
};
pub use reduced_field::ReducedField;
pub use replay::{
    ReplayDriver, ReplayError, ReplayFrame, ReplayReader, ReplayRecord, ReplaySnapshot,
    ReplayWriter,
};
pub use threshold_registry::{
    assert_no_hard_trigger_on_soft_aggregate, AggregateAlertEvent, AggregateAlertRegistration,
    SoftAggregateViolation, ThresholdBuilder, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertEvent, VelocityAlertRegistration,
};
pub use tree_mutation::apply_structural_mutations;
