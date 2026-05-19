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
//! - `fission` — step 6. Executes fission and fusion events from Pass 7 output.
//!   Fission: spawns a new child `SimThing`, allocs a slot, seeds its GPU row
//!   from the parent's current values, registers its `FusionThreshold`.
//!   Fusion: applies the scar coefficient to the parent, removes the child,
//!   tombstones its slot.
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

pub mod boundary;
pub mod fission;
pub mod gpu_sync;
pub mod observability;
pub mod overlay_lifecycle;
pub mod property_expiry;
pub mod threshold_registry;
pub mod tree_mutation;

pub use boundary::{BoundaryOutcome, BoundaryProtocol};
pub use observability::{
    ObservabilityReport, OverlayContribution, PropertyObservation, SubFieldObservation,
};
pub use threshold_registry::{
    ThresholdBuilder, ThresholdRegistry, ThresholdSemantic, VelocityAlertEvent,
    VelocityAlertRegistration,
};
pub use tree_mutation::apply_structural_mutations;
