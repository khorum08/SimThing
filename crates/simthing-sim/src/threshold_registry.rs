//! CPU-side threshold semantic registry.
//!
//! GPU Pass 7 emits `ThresholdEvent { slot, col, value, event_kind }`.
//! `event_kind` is an opaque `u32` assigned at threshold-registration time.
//! This module owns the mapping from that u32 back to a meaningful CPU action.
//!
//! ## Design
//!
//! `ThresholdRegistry` is a `Vec<ThresholdSemantic>` indexed directly by
//! `event_kind`. Registration is append-only within a day; the index grows
//! monotonically. At each day boundary the registry is rebuilt from scratch
//! so that tombstoned slots and newly-spawned SimThings are reflected.
//!
//! `ThresholdBuilder` walks the SimThing tree and derives both:
//! - `Vec<ThresholdRegistration>` (GPU upload via `state.upload_thresholds`)
//! - `Vec<ThresholdSemantic>` (CPU lookup via `ThresholdRegistry`)
//!
//! In parallel, keyed by the same sequential `event_kind` index.
//!
//! ## Sources of thresholds
//!
//! Per design_v4.md §6 and §7:
//!
//! 1. **`FissionThreshold`** on a `SimProperty` — when Amount/Intensity
//!    crosses the threshold on any live SimThing that has that property.
//!    One GPU `ThresholdRegistration` per (live sim_thing, fission_template).
//!
//! 2. **Fusion thresholds** are registered from `FissionLineageRecord`s held
//!    by the boundary protocol. Each lineage record produces one
//!    `FusionTrigger` watching the child's activating-property Intensity
//!    column, threshold = template.fusion_intensity_threshold, direction =
//!    Upward (intensity climbs back up as the schism dissolves). When the
//!    threshold fires, fusion executes: the parent absorbs a scar
//!    multiplier on its activating-property Amount and the child is
//!    tombstoned.
//!
//! 3. **`DecayBehavior::OnThreshold`** — property self-removes when its own
//!    Amount crosses a threshold. Emits `PropertyExpiry`.
//!
//! 4. **`DecayBehavior::IntensityGated`** — property self-removes when
//!    intensity drops below the floor. Emits `PropertyExpiry`.
//!
//! 5. **`DecayBehavior::WhenProperty`** — property self-removes when another
//!    property on the same SimThing crosses a threshold. Emits `PropertyExpiry`.
//!
//! 6. **Velocity alerts** — registered by the AI layer against a specific
//!    SimThing/property/sub-field trajectory.

use serde::{Deserialize, Serialize};
use simthing_core::{
    cost_band_quantize, CostBandAdmissionError, CostBandDraw, CostBandRegistrationMarker,
    CostBandResourceMarker, DecayBehavior, DimensionRegistry, Direction, ReductionRule,
    SimPropertyId, SimThing, SimThingId, SoftAggregateGuard, SubFieldRole, admit_cost_band_marker,
};
use simthing_feeder::{
    CapabilityUnlockEvent, CapabilityUnlockRegistration, ScriptedEventTriggerEvent,
    ScriptedEventTriggerRegistration,
};
use simthing_gpu::{
    SlotAllocator, ThresholdEvent, ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD,
    THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
};

use crate::fission::FissionLineageRecord;
use crate::sim_runtime_tree::SimRuntimeTree;

// ── Semantic action ───────────────────────────────────────────────────────────

/// What a fired `ThresholdEvent` means to the CPU boundary protocol.
/// Indexed by `ThresholdEvent::event_kind` in the `ThresholdRegistry`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThresholdSemantic {
    /// A `FissionThreshold` fired. The boundary must check the secondary
    /// condition (if any) and, if satisfied, spawn a new SimThing child.
    FissionTrigger {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        /// Index into `SimProperty::fission_templates`.
        template_idx: usize,
    },

    /// A `FusionThreshold` fired on a previously-fissioned child. The
    /// boundary merges the child back into the parent, applies the scar
    /// coefficient, and removes the child's slot.
    FusionTrigger {
        /// The child SimThing to dissolve.
        child_id: SimThingId,
        /// The parent that receives the scar.
        parent_id: SimThingId,
        property_id: SimPropertyId,
        /// Index into `SimProperty::fusion_templates`.
        template_idx: usize,
    },

    /// A `DecayBehavior` threshold fired. The boundary removes this property
    /// from the SimThing's `properties` map and tombstones the registry
    /// column if no other live SimThing carries it.
    PropertyExpiry {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
    },

    /// Velocity alert (AI-registered). Scans per-slot `values`.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },

    /// Aggregate alert (AI-registered). Scans post-reduction `output_vectors`
    /// on inner nodes (e.g. location instability from cohort children).
    AggregateAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },

    /// A capability tree progress threshold fired. Emitted from a
    /// `CapabilityUnlockRegistration` (one per `ActivationMode::Threshold`
    /// entry compiled by `simthing-spec::CapabilityTreeBuilder`). PR 5's
    /// `CapabilityTreeBoundaryHandler` reads the `sim_thing_id` to find
    /// the faction's `CapabilityTreeInstance`, then looks up the entry via
    /// `definition.by_threshold[(property_id, sub_field)]`.
    CapabilityUnlock {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },

    /// A scripted event's threshold trigger fired. Emitted from a
    /// `ScriptedEventTriggerRegistration` (one per `CompiledTrigger::Threshold`
    /// scripted event registered with the session). The spec layer's
    /// `ScriptedEventBoundaryHandler::handle_tick` matches `event_id` against
    /// its `ScriptedEventDefinition` slice and fires the corresponding effects
    /// under standard cooldown/priority gating.
    ScriptedEventTrigger {
        /// Matches `simthing_spec::EventKey.0`.
        event_id: String,
    },
}

impl ThresholdSemantic {
    /// Short, stable tag for observability/log lines. Display-only — never
    /// used for runtime branching (semantic-free sim doctrine, core §1.2).
    pub fn debug_kind(&self) -> &'static str {
        match self {
            ThresholdSemantic::FissionTrigger { .. } => "fission_trigger",
            ThresholdSemantic::FusionTrigger { .. } => "fusion_trigger",
            ThresholdSemantic::PropertyExpiry { .. } => "property_expiry",
            ThresholdSemantic::VelocityAlert { .. } => "velocity_alert",
            ThresholdSemantic::AggregateAlert { .. } => "aggregate_alert",
            ThresholdSemantic::CapabilityUnlock { .. } => "capability_unlock",
            ThresholdSemantic::ScriptedEventTrigger { .. } => "scripted_event_trigger",
        }
    }
}

/// AI-facing threshold registration for a rate/trajectory column on `values`.
///
/// CostBand admission rides this existing registration channel (keyed by the
/// assigned `event_kind`) — observation by default; sinks carry authored
/// throttle from the stored-hydrated `throttle_hint_max_per_tick` source.
#[derive(Clone, Debug)]
pub struct VelocityAlertRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
    /// Authored CostBand / sink semantics for this registration.
    pub cost_band: CostBandSemantic,
}

/// AI-facing threshold on post-reduction `output_vectors` (parent aggregates).
#[derive(Clone, Debug)]
pub struct AggregateAlertRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
    /// Authored CostBand / sink semantics for this registration.
    pub cost_band: CostBandSemantic,
}

/// Fired aggregate alert surfaced by the boundary protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct AggregateAlertEvent {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub value: f32,
}

/// Fired velocity alert surfaced by the boundary protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct VelocityAlertEvent {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub value: f32,
}

// ── CostBand admission (event_kind-parallel; never GPU POD) ───────────────────

/// Authored CostBand / sink semantics for one `event_kind` registration.
/// Observation is the default (`is_sink = false`). Sinks carry optional
/// `throttle_hint_max_per_tick` enforced at draw resolution.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostBandSemantic {
    pub is_sink: bool,
    pub throttle_hint_max_per_tick: Option<u32>,
}

impl CostBandSemantic {
    pub const fn observation() -> Self {
        Self {
            is_sink: false,
            throttle_hint_max_per_tick: None,
        }
    }

    /// Admit a sink registration. Optional per-resource marker: per-registration
    /// wins; ambiguity hard-errors.
    pub fn admit_sink(
        throttle_hint_max_per_tick: Option<u32>,
        resource: Option<CostBandResourceMarker>,
    ) -> Result<Self, CostBandAdmissionError> {
        if let Some(0) = throttle_hint_max_per_tick {
            return Err(CostBandAdmissionError::InvalidThrottle);
        }
        let is_sink = admit_cost_band_marker(
            Some(CostBandRegistrationMarker { is_sink: true }),
            resource,
        )?;
        Ok(Self {
            is_sink,
            throttle_hint_max_per_tick,
        })
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────

/// Parallel-indexed companion to the GPU threshold_registry buffer.
/// `registry[event_kind]` → `ThresholdSemantic`.
/// CostBand admission is parallel by the same `event_kind` index (CPU semantic
/// table — never a second transport or GPU POD field).
/// Rebuilt from scratch at each boundary by `ThresholdBuilder`.
#[derive(Clone, Debug, Default)]
pub struct ThresholdRegistry {
    entries: Vec<ThresholdSemantic>,
    cost_bands: Vec<CostBandSemantic>,
    /// Production resolve invocations (live-wiring referee; not sim authority).
    pub cost_band_resolve_invocations: u64,
}

impl ThresholdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up the semantic action for a fired event.
    pub fn get(&self, event_kind: u32) -> Option<&ThresholdSemantic> {
        self.entries.get(event_kind as usize)
    }

    /// CostBand admission for `event_kind` (observation default if missing).
    pub fn cost_band(&self, event_kind: u32) -> CostBandSemantic {
        self.cost_bands
            .get(event_kind as usize)
            .copied()
            .unwrap_or_else(CostBandSemantic::observation)
    }

    /// Push a new observation-default entry and return the event_kind assigned.
    pub fn push(&mut self, sem: ThresholdSemantic) -> u32 {
        self.push_with_cost_band(sem, CostBandSemantic::observation())
    }

    /// Push with explicit CostBand admission (production sink path).
    pub fn push_with_cost_band(&mut self, sem: ThresholdSemantic, cost_band: CostBandSemantic) -> u32 {
        debug_assert_eq!(self.entries.len(), self.cost_bands.len());
        let idx = self.entries.len() as u32;
        self.entries.push(sem);
        self.cost_bands.push(cost_band);
        idx
    }


    /// Production CostBand door: resolve draw from sealed delta operands +
    /// admitted `event_kind` semantics. Callers cannot opt out a sink via
    /// ad-hoc `is_sink=false` or substitute throttle.
    pub fn resolve_cost_band_draw(
        &mut self,
        event_kind: u32,
        available_v: f32,
        unit_cost_c: f32,
    ) -> Result<CostBandDraw, CostBandAdmissionError> {
        self.cost_band_resolve_invocations =
            self.cost_band_resolve_invocations.saturating_add(1);
        let cb = self.cost_band(event_kind);
        cost_band_quantize(
            available_v,
            unit_cost_c,
            cb.is_sink,
            cb.throttle_hint_max_per_tick,
        )
    }

    /// Resolve from a sealed band-crossing delta (V=post_value, C=threshold).
    pub fn resolve_cost_band_draw_from_delta(
        &mut self,
        delta: &simthing_gpu::BandCrossingDelta,
    ) -> Result<CostBandDraw, CostBandAdmissionError> {
        let (v, c) = (delta.post_value(), delta.threshold());
        self.resolve_cost_band_draw(delta.event_kind(), v, c)
    }

    /// Ordinary production crossing path: resolve CostBand draws for sealed
    /// deltas through the `event_kind` semantic table. Boundary calls this;
    /// dead-coding / bypassing it must RED the live-wiring referee.
    pub fn resolve_cost_band_draws_for_deltas(
        &mut self,
        deltas: &[simthing_gpu::BandCrossingDelta],
    ) -> Vec<(u32, CostBandDraw)> {
        let mut out = Vec::with_capacity(deltas.len());
        for delta in deltas {
            match self.resolve_cost_band_draw_from_delta(delta) {
                Ok(draw) => out.push((delta.event_kind(), draw)),
                Err(_) => {
                    // Invalid V/C on a sealed delta is a non-authoritative skip;
                    // the resolve attempt still advances the live-wiring counter.
                }
            }
        }
        out
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filter a slice of GPU `ThresholdEvent`s down to just the
    /// `ThresholdSemantic::CapabilityUnlock` arms, resolved into
    /// `CapabilityUnlockEvent`s for `simthing-spec`'s boundary handler.
    ///
    /// This is the conversion bridge that lets the spec layer stay
    /// independent of `simthing-gpu` and `simthing-sim`: callers that hold
    /// raw threshold events call this, then hand the result to
    /// `CapabilityTreeBoundaryHandler::handle_capability_unlock_events`.
    /// Non-`CapabilityUnlock` arms and events with out-of-range `event_kind`
    /// are silently filtered out.
    pub fn extract_capability_unlocks(
        &self,
        events: &[ThresholdEvent],
    ) -> Vec<CapabilityUnlockEvent> {
        events
            .iter()
            .filter_map(|event| match self.get(event.event_kind())? {
                ThresholdSemantic::CapabilityUnlock {
                    sim_thing_id,
                    property_id,
                    sub_field,
                } => Some(CapabilityUnlockEvent {
                    sim_thing_id: *sim_thing_id,
                    property_id: *property_id,
                    sub_field: sub_field.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    /// Filter a slice of GPU `ThresholdEvent`s down to just the
    /// `ThresholdSemantic::ScriptedEventTrigger` arms, resolved into
    /// `ScriptedEventTriggerEvent`s for `simthing-spec`'s scripted-event
    /// boundary handler. Sibling to `extract_capability_unlocks` — same
    /// pattern, different semantic arm. Non-`ScriptedEventTrigger` arms and
    /// out-of-range `event_kind`s are silently filtered out.
    pub fn extract_scripted_event_triggers(
        &self,
        events: &[ThresholdEvent],
    ) -> Vec<ScriptedEventTriggerEvent> {
        events
            .iter()
            .filter_map(|event| match self.get(event.event_kind())? {
                ThresholdSemantic::ScriptedEventTrigger { event_id } => {
                    Some(ScriptedEventTriggerEvent {
                        event_id: event_id.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }

    /// Zero-alloc predicate: any event in `events` resolves to a
    /// `CapabilityUnlock` registration. Used by the boundary-skip path
    /// (B3) so we don't allocate a Vec when we only need a bool. Matches
    /// the semantics of `extract_capability_unlocks(events).is_empty().not()`.
    pub fn has_capability_unlock_in(&self, events: &[ThresholdEvent]) -> bool {
        events.iter().any(|event| {
            matches!(
                self.get(event.event_kind()),
                Some(ThresholdSemantic::CapabilityUnlock { .. })
            )
        })
    }

    /// Zero-alloc predicate sibling for scripted-event triggers. See
    /// `has_capability_unlock_in` — same rationale, different semantic arm.
    pub fn has_scripted_event_trigger_in(&self, events: &[ThresholdEvent]) -> bool {
        events.iter().any(|event| {
            matches!(
                self.get(event.event_kind()),
                Some(ThresholdSemantic::ScriptedEventTrigger { .. })
            )
        })
    }
}

// ── Soft-aggregate guard validator ────────────────────────────────────────────

/// Failure variant produced by `assert_no_hard_trigger_on_soft_aggregate`.
/// The four classifying conditions are documented in
/// `docs/workshop/soft_aggregate_tolerance_audit.md` §3.4.
#[derive(Clone, Debug, PartialEq)]
pub enum SoftAggregateViolation {
    /// A hard structural trigger registration targets the post-reduction
    /// buffer on a sub-field whose resolved reduction is `Mean` or
    /// `WeightedMean` and that has no `SoftAggregateGuard` (or has
    /// `Some(Unguarded)`, which is the explicit "field is informational"
    /// opt-out and is not valid for hard triggers).
    HardTriggerOnUnguardedSoftAggregate {
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
        rule: ReductionRule,
        /// Static label of the violating semantic arm. Used for diagnostics.
        semantic_kind: &'static str,
    },
}

impl std::fmt::Display for SoftAggregateViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoftAggregateViolation::HardTriggerOnUnguardedSoftAggregate {
                property_id,
                sub_field,
                rule,
                semantic_kind,
            } => write!(
                f,
                "soft-aggregate column registered as hard structural trigger \
                 without guard: semantic={semantic_kind}, property={property_id:?}, \
                 sub_field={sub_field:?}, rule={rule:?}. Set \
                 SubFieldSpec::soft_aggregate_guard to \
                 Some(Quantized {{ step }}) or Some(Hysteresis {{ band }}) \
                 — see docs/workshop/soft_aggregate_tolerance_audit.md."
            ),
        }
    }
}

impl std::error::Error for SoftAggregateViolation {}

/// Classify a `ThresholdSemantic` as a hard structural trigger.
///
/// Hard structural triggers are arms whose firing mutates the SimThing tree
/// (`FissionTrigger`, `FusionTrigger`, `PropertyExpiry`) or durably mutates
/// spec session state (`CapabilityUnlock`). `VelocityAlert`,
/// `AggregateAlert`, and `ScriptedEventTrigger` are not in this class:
/// they surface informationally to AI/observability/spec layers, which
/// apply their own confirmation logic before any mutation.
fn is_hard_structural_trigger(s: &ThresholdSemantic) -> bool {
    matches!(
        s,
        ThresholdSemantic::FissionTrigger { .. }
            | ThresholdSemantic::FusionTrigger { .. }
            | ThresholdSemantic::PropertyExpiry { .. }
            | ThresholdSemantic::CapabilityUnlock { .. }
    )
}

fn semantic_kind(s: &ThresholdSemantic) -> &'static str {
    match s {
        ThresholdSemantic::FissionTrigger { .. } => "FissionTrigger",
        ThresholdSemantic::FusionTrigger { .. } => "FusionTrigger",
        ThresholdSemantic::PropertyExpiry { .. } => "PropertyExpiry",
        ThresholdSemantic::CapabilityUnlock { .. } => "CapabilityUnlock",
        ThresholdSemantic::VelocityAlert { .. } => "VelocityAlert",
        ThresholdSemantic::AggregateAlert { .. } => "AggregateAlert",
        ThresholdSemantic::ScriptedEventTrigger { .. } => "ScriptedEventTrigger",
    }
}

fn is_soft_aggregate_rule(r: &ReductionRule) -> bool {
    matches!(r, ReductionRule::Mean | ReductionRule::WeightedMean { .. })
}

/// Validate that `(semantic, property, sub_field, buffer)` does not
/// constitute an un-guarded hard structural trigger on a soft-aggregate
/// column. Returns `Ok(())` for safe registrations and for non-hard-trigger
/// semantics.
///
/// The validator panics-by-policy when called via `.expect(...)` from the
/// `ThresholdBuilder` push helpers, matching the production-plan wording
/// ("panics if a `WeightedMean`-reduced column is registered as a fission
/// or structural threshold without a guard"). The `Result`-returning form
/// is preserved so the spec layer or pre-flight tooling can validate before
/// commit.
///
/// Today no production threshold path satisfies the "post-reduction + hard
/// trigger" combination, so this validator returns `Ok(())` for every
/// existing registration. It is forward-protecting: when C-5 (WeightedMean
/// migration) or E0 (economic transfers) introduces registrations that
/// target `THRESH_BUF_OUTPUT` for a hard trigger, the validator fails fast
/// at registration time.
///
/// See `docs/workshop/soft_aggregate_tolerance_audit.md`.
pub fn assert_no_hard_trigger_on_soft_aggregate(
    semantic: &ThresholdSemantic,
    property_id: SimPropertyId,
    sub_field: &SubFieldRole,
    buffer: u32,
    dim_reg: &DimensionRegistry,
) -> Result<(), SoftAggregateViolation> {
    // Condition 1: hard structural trigger.
    if !is_hard_structural_trigger(semantic) {
        return Ok(());
    }
    // Condition 2: post-reduction buffer.
    if buffer != THRESH_BUF_OUTPUT {
        return Ok(());
    }
    // Condition 3: resolved reduction is soft.
    if !dim_reg.is_active(property_id) {
        return Ok(());
    }
    let prop = dim_reg.property(property_id);
    let Some(spec) = prop
        .layout
        .sub_fields
        .iter()
        .find(|sf| &sf.role == sub_field)
    else {
        return Ok(());
    };
    let rule = spec.resolved_reduction();
    if !is_soft_aggregate_rule(&rule) {
        return Ok(());
    }
    // Condition 4: no real guard. `None` and `Some(Unguarded)` both fail
    // — `Unguarded` is documented as "only valid for fields that never feed
    // threshold registrations" (see `SoftAggregateGuard` docstring).
    match spec.soft_aggregate_guard {
        Some(SoftAggregateGuard::Quantized { .. })
        | Some(SoftAggregateGuard::Hysteresis { .. }) => Ok(()),
        None | Some(SoftAggregateGuard::Unguarded) => Err(
            SoftAggregateViolation::HardTriggerOnUnguardedSoftAggregate {
                property_id,
                sub_field: sub_field.clone(),
                rule,
                semantic_kind: semantic_kind(semantic),
            },
        ),
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Walks the SimThing tree and derives GPU + CPU threshold registrations.
/// Call `build()` at each day boundary after structural mutations complete.
/// Output: `(gpu_regs, cpu_registry)` ready for upload + lookup.
pub struct ThresholdBuilder;

impl ThresholdBuilder {
    /// Walk the tree and build both the GPU registration vec and the parallel
    /// CPU registry. The two vecs are index-aligned: `gpu_regs[i]` fires with
    /// `event_kind = i`, which the boundary resolves via `cpu_registry.get(i)`.
    pub fn build(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_velocity_alerts(root, dim_reg, allocator, &[])
    }

    pub fn build_with_velocity_alerts(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_alerts(root, dim_reg, allocator, velocity_alerts, &[])
    }

    pub fn build_with_alerts(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        aggregate_alerts: &[AggregateAlertRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_lineage(
            root,
            dim_reg,
            allocator,
            velocity_alerts,
            aggregate_alerts,
            &[],
        )
    }

    /// Build with fusion lineage. Each `FissionLineageRecord` produces one
    /// `FusionTrigger` registration watching the child's activating-property
    /// Intensity column, threshold = template.fusion_intensity_threshold,
    /// direction = Upward (intensity climbs back up as the schism dissolves).
    ///
    /// Lineage entries whose property has been tombstoned, whose template
    /// index is out of range, whose child slot can't be resolved, or whose
    /// property has no Intensity sub-field are silently skipped.
    pub fn build_with_lineage(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        aggregate_alerts: &[AggregateAlertRegistration],
        lineage: &[FissionLineageRecord],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(
            root.inner(),
            dim_reg,
            allocator,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_fusion_lineage(dim_reg, allocator, lineage, &mut gpu_regs, &mut cpu_reg);
        Self::push_velocity_alerts(
            dim_reg,
            allocator,
            velocity_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_aggregate_alerts(
            dim_reg,
            allocator,
            aggregate_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
    }

    /// Build with capability unlocks. Each `CapabilityUnlockRegistration`
    /// produces one upward-direction Pass 7 watch on its `(slot, col)` pair
    /// with the matching `ThresholdSemantic::CapabilityUnlock` entry on the
    /// CPU registry. Capability unlocks watch the `values` buffer (per-slot
    /// progress columns), not `output_vectors`.
    ///
    /// Registrations whose property has been tombstoned, whose `sim_thing_id`
    /// can't be resolved to a slot, or whose `sub_field` is not present in
    /// the property layout are silently skipped — mirrors the velocity-alert
    /// and lineage skipping behavior.
    ///
    /// Full-rebuild path only. B2 append-only integration is a future PR;
    /// the first fission boundary after a capability tree is initialized
    /// takes the full-rebuild path regardless.
    pub fn build_with_capability_unlocks(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        capability_unlocks: &[CapabilityUnlockRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(
            root.inner(),
            dim_reg,
            allocator,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_velocity_alerts(
            dim_reg,
            allocator,
            velocity_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_capability_unlocks(
            dim_reg,
            allocator,
            capability_unlocks,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
    }

    /// Build with scripted event triggers. Each `ScriptedEventTriggerRegistration`
    /// produces one Pass 7 watch on its `(slot, col)` pair with the matching
    /// `ThresholdSemantic::ScriptedEventTrigger` entry on the CPU registry.
    /// Scripted event triggers watch the `values` buffer; selection
    /// `output_vectors` aggregates is a future extension.
    ///
    /// Registrations whose `slot` exceeds the allocator's current capacity are
    /// kept verbatim — the caller is responsible for resolving `ScopeRef`
    /// targets to live slots before registration. (We don't have a property_id
    /// to validate against on this path, unlike capability unlocks.)
    ///
    /// Full-rebuild path only; B2 append-only integration is a future PR.
    pub fn build_with_scripted_event_triggers(
        root: &SimRuntimeTree,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(
            root.inner(),
            dim_reg,
            allocator,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_velocity_alerts(
            dim_reg,
            allocator,
            velocity_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_scripted_event_triggers(scripted_event_triggers, &mut gpu_regs, &mut cpu_reg);
        (gpu_regs, cpu_reg)
    }

    fn push_scripted_event_triggers(
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for trigger in scripted_event_triggers {
            let event_kind = cpu_reg.push(ThresholdSemantic::ScriptedEventTrigger {
                event_id: trigger.event_id.clone(),
            });
            gpu_regs.push(ThresholdRegistration {
                slot: trigger.slot,
                col: trigger.col,
                threshold: trigger.threshold,
                direction: direction_to_u32(&trigger.direction),
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
    }

    fn push_capability_unlocks(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        capability_unlocks: &[CapabilityUnlockRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for unlock in capability_unlocks {
            if !dim_reg.is_active(unlock.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(unlock.sim_thing_id) else {
                continue;
            };
            let range = dim_reg.column_range(unlock.property_id);
            let layout = &dim_reg.property(unlock.property_id).layout;
            let Some(col) = range.col_for_role(&unlock.sub_field, layout) else {
                continue;
            };

            let semantic = ThresholdSemantic::CapabilityUnlock {
                sim_thing_id: unlock.sim_thing_id,
                property_id: unlock.property_id,
                sub_field: unlock.sub_field.clone(),
            };
            // A-4 guard: hard structural triggers may not target a soft-aggregate
            // post-reduction column without a SoftAggregateGuard. CapabilityUnlock
            // registers on THRESH_BUF_VALUES today so this is a no-op; the call
            // future-proofs us against C-5/E0 paths that may route through OUTPUT.
            assert_no_hard_trigger_on_soft_aggregate(
                &semantic,
                unlock.property_id,
                &unlock.sub_field,
                THRESH_BUF_VALUES,
                dim_reg,
            )
            .expect("assert_no_hard_trigger_on_soft_aggregate (CapabilityUnlock)");

            let event_kind = cpu_reg.push(semantic);
            gpu_regs.push(ThresholdRegistration {
                slot: slot.raw(),
                col: col.raw_u32(),
                threshold: unlock.threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
    }

    /// Append registrations for a subtree to the caller's existing `gpu_regs`
    /// and `cpu_reg` instead of building fresh ones. Used by B2 Approach B
    /// (append-only threshold rebuild on pure-fission growth boundaries):
    /// the boundary already holds the existing threshold registry, so we only
    /// need to derive registrations for the newly-spawned SimThings. The
    /// event_kinds assigned to the new entries are `cpu_reg.len()` and onwards.
    pub(crate) fn append_subtree(
        node: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::walk(node, dim_reg, allocator, gpu_regs, cpu_reg);
    }

    /// Append the FusionTrigger registrations for the given lineage records
    /// to the caller's existing `gpu_regs` and `cpu_reg`. Companion to
    /// `append_subtree` for B2 Approach B.
    pub fn append_lineage(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        lineage: &[FissionLineageRecord],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::push_fusion_lineage(dim_reg, allocator, lineage, gpu_regs, cpu_reg);
    }

    /// Append capability unlock registrations to the caller's existing
    /// `gpu_regs` and `cpu_reg`. Companion to `append_subtree` for B2
    /// append-only threshold rebuild on growth boundaries.
    pub fn append_capability_unlocks(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        capability_unlocks: &[CapabilityUnlockRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::push_capability_unlocks(dim_reg, allocator, capability_unlocks, gpu_regs, cpu_reg);
    }

    /// Append scripted-event trigger registrations to the caller's existing
    /// `gpu_regs` and `cpu_reg`. Companion to `append_subtree` for B2
    /// append-only threshold rebuild on growth boundaries.
    pub fn append_scripted_event_triggers(
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::push_scripted_event_triggers(scripted_event_triggers, gpu_regs, cpu_reg);
    }

    fn push_fusion_lineage(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        lineage: &[FissionLineageRecord],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for record in lineage {
            if !dim_reg.is_active(record.property_id) {
                continue;
            }
            let prop = dim_reg.property(record.property_id);
            if record.template_idx >= prop.fission_templates.len() {
                continue;
            }
            let ft = &prop.fission_templates[record.template_idx];

            let Some(child_slot) = allocator.slot_of(record.child_id) else {
                continue;
            };
            let range = dim_reg.column_range(record.property_id);
            let layout = &prop.layout;
            let Some(col) = range.col_for_role(&SubFieldRole::Intensity, layout) else {
                continue;
            };

            let semantic = ThresholdSemantic::FusionTrigger {
                child_id: record.child_id,
                parent_id: record.parent_id,
                property_id: record.property_id,
                template_idx: record.template_idx,
            };
            // A-4 guard: FusionTrigger always reads child Intensity (default
            // ReductionRule::Max — exact, not soft), so this is a no-op today
            // but forward-protects designer-overridden layouts.
            assert_no_hard_trigger_on_soft_aggregate(
                &semantic,
                record.property_id,
                &SubFieldRole::Intensity,
                THRESH_BUF_VALUES,
                dim_reg,
            )
            .expect("assert_no_hard_trigger_on_soft_aggregate (FusionTrigger)");

            let event_kind = cpu_reg.push(semantic);
            gpu_regs.push(ThresholdRegistration {
                slot: child_slot.raw(),
                col: col.raw_u32(),
                threshold: ft.template.fusion_intensity_threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
    }

    fn walk(
        node: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        if let Some(slot) = allocator.slot_of(node.id) {
            for (pid, _pval) in &node.properties {
                if !dim_reg.is_active(*pid) {
                    continue;
                }
                let prop = dim_reg.property(*pid);
                let range = dim_reg.column_range(*pid);
                let layout = &prop.layout;

                // 1. Fission thresholds from FissionThreshold list.
                for (idx, ft) in prop.fission_templates.iter().enumerate() {
                    if let Some(col) = range.col_for_role(&ft.sub_field, layout) {
                        let semantic = ThresholdSemantic::FissionTrigger {
                            sim_thing_id: node.id,
                            property_id: *pid,
                            template_idx: idx,
                        };
                        // A-4 guard: forward-protects against future PRs that
                        // might register fission triggers on THRESH_BUF_OUTPUT.
                        assert_no_hard_trigger_on_soft_aggregate(
                            &semantic,
                            *pid,
                            &ft.sub_field,
                            THRESH_BUF_VALUES,
                            dim_reg,
                        )
                        .expect("assert_no_hard_trigger_on_soft_aggregate (FissionTrigger)");

                        let event_kind = cpu_reg.push(semantic);
                        gpu_regs.push(ThresholdRegistration {
                            slot: slot.raw(),
                            col: col.raw_u32(),
                            threshold: ft.threshold,
                            direction: direction_to_u32(&ft.direction),
                            event_kind,
                            buffer: THRESH_BUF_VALUES,
                        });
                    }
                }

                // 2. Decay thresholds (OnThreshold, IntensityGated, WhenProperty).
                //    These emit PropertyExpiry.
                Self::push_decay_thresholds(
                    node.id,
                    *pid,
                    slot.raw(),
                    prop.decay.as_ref(),
                    range.start,
                    layout,
                    dim_reg,
                    gpu_regs,
                    cpu_reg,
                );
            }
        }

        for child in &node.children {
            Self::walk(child, dim_reg, allocator, gpu_regs, cpu_reg);
        }
    }

    fn push_decay_thresholds(
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        slot: u32,
        decay: Option<&DecayBehavior>,
        prop_col_start: usize,
        layout: &simthing_core::PropertyLayout,
        dim_reg: &DimensionRegistry,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        let semantic = ThresholdSemantic::PropertyExpiry {
            sim_thing_id,
            property_id,
        };
        // A-4 guard: PropertyExpiry is a hard structural trigger. Validate
        // the watched (property, sub_field) per decay variant before pushing.
        // All three variants register on THRESH_BUF_VALUES today; the validator
        // returns Ok(()) in all current cases and is a forward gate.
        match decay {
            Some(DecayBehavior::OnThreshold {
                threshold,
                direction,
            }) => {
                // Amount col for this property.
                if let Some(col) = layout.offset_of(&SubFieldRole::Amount) {
                    assert_no_hard_trigger_on_soft_aggregate(
                        &semantic,
                        property_id,
                        &SubFieldRole::Amount,
                        THRESH_BUF_VALUES,
                        dim_reg,
                    )
                    .expect(
                        "assert_no_hard_trigger_on_soft_aggregate \
                         (PropertyExpiry::OnThreshold)",
                    );
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: (prop_col_start + col.lane()) as u32,
                        threshold: *threshold,
                        direction: direction_to_u32(direction),
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            Some(DecayBehavior::IntensityGated { intensity_floor }) => {
                if let Some(col) = layout.offset_of(&SubFieldRole::Intensity) {
                    assert_no_hard_trigger_on_soft_aggregate(
                        &semantic,
                        property_id,
                        &SubFieldRole::Intensity,
                        THRESH_BUF_VALUES,
                        dim_reg,
                    )
                    .expect(
                        "assert_no_hard_trigger_on_soft_aggregate \
                         (PropertyExpiry::IntensityGated)",
                    );
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: (prop_col_start + col.lane()) as u32,
                        threshold: *intensity_floor,
                        direction: DIR_DOWNWARD,
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            Some(DecayBehavior::WhenProperty { other, threshold }) => {
                // Threshold is on `other`'s Amount column.
                if !dim_reg.is_active(*other) {
                    return;
                }
                let other_range = dim_reg.column_range(*other);
                let other_layout = &dim_reg.property(*other).layout;
                if let Some(col) = other_range.col_for_role(&SubFieldRole::Amount, other_layout) {
                    assert_no_hard_trigger_on_soft_aggregate(
                        &semantic,
                        *other,
                        &SubFieldRole::Amount,
                        THRESH_BUF_VALUES,
                        dim_reg,
                    )
                    .expect(
                        "assert_no_hard_trigger_on_soft_aggregate \
                         (PropertyExpiry::WhenProperty)",
                    );
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: col.raw_u32(),
                        threshold: *threshold,
                        direction: DIR_EITHER,
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            // TowardZero and AfterTicks are handled by the overlay lifecycle
            // manager and the property expiry step on the CPU, not via GPU
            // thresholds. No GPU registration needed for those.
            _ => {}
        }
    }

    fn push_velocity_alerts(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for alert in velocity_alerts {
            if !dim_reg.is_active(alert.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(alert.sim_thing_id) else {
                continue;
            };
            let range = dim_reg.column_range(alert.property_id);
            let layout = &dim_reg.property(alert.property_id).layout;
            let Some(col) = range.col_for_role(&alert.sub_field, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push_with_cost_band(
                ThresholdSemantic::VelocityAlert {
                    sim_thing_id: alert.sim_thing_id,
                    property_id: alert.property_id,
                    sub_field: alert.sub_field.clone(),
                },
                alert.cost_band,
            );
            gpu_regs.push(ThresholdRegistration {
                slot: slot.raw(),
                col: col.raw_u32(),
                threshold: alert.threshold,
                direction: direction_to_u32(&alert.direction),
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
    }

    fn push_aggregate_alerts(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        aggregate_alerts: &[AggregateAlertRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for alert in aggregate_alerts {
            if !dim_reg.is_active(alert.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(alert.sim_thing_id) else {
                continue;
            };
            let range = dim_reg.column_range(alert.property_id);
            let layout = &dim_reg.property(alert.property_id).layout;
            let Some(col) = range.col_for_role(&alert.sub_field, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push_with_cost_band(
                ThresholdSemantic::AggregateAlert {
                    sim_thing_id: alert.sim_thing_id,
                    property_id: alert.property_id,
                    sub_field: alert.sub_field.clone(),
                },
                alert.cost_band,
            );
            gpu_regs.push(ThresholdRegistration {
                slot: slot.raw(),
                col: col.raw_u32(),
                threshold: alert.threshold,
                direction: direction_to_u32(&alert.direction),
                event_kind,
                buffer: THRESH_BUF_OUTPUT,
            });
        }
    }
}

fn direction_to_u32(dir: &Direction) -> u32 {
    match dir {
        Direction::Rising => DIR_UPWARD,
        Direction::Falling => DIR_DOWNWARD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim_runtime_tree::SimRuntimeTree;
    use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
    use simthing_gpu::SlotAllocator;

    // ── PR 4: capability unlock ───────────────────────────────────────────

    fn cap_property() -> SimProperty {
        // One Named sub-field at offset 0. Mirrors what
        // `CapabilityTreeBuilder::build` emits for a one-entry category.
        use simthing_core::{ClampBehavior, PropertyLayout, ReductionRule, SubFieldSpec};
        SimProperty {
            namespace: "tech".into(),
            name: "propulsion".into(),
            admission_disposition: Default::default(),
            layout: PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role: SubFieldRole::Named("chemical_drive".into()),
                    width: 1,
                    clamp: ClampBehavior::Floored { min: 0.0 },
                    velocity_max: None,
                    default: 0.0,
                    display_name: "Chemical Drive".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: Some(ReductionRule::Max),
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                }],
            },
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description: String::new(),
            intensity_labels: vec![],
        }
    }

    // ── A-4: soft-aggregate guard validator ──────────────────────────────

    /// Build a property whose `Amount` sub-field has the given
    /// reduction_override and soft_aggregate_guard. Used by the validator
    /// tests below.
    fn property_with_guard(
        namespace: &str,
        name: &str,
        reduction: Option<simthing_core::ReductionRule>,
        guard: Option<simthing_core::SoftAggregateGuard>,
    ) -> SimProperty {
        use simthing_core::{ClampBehavior, PropertyLayout, SubFieldSpec};
        SimProperty {
            namespace: namespace.into(),
            name: name.into(),
            admission_disposition: Default::default(),
            layout: PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role: SubFieldRole::Amount,
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "amount".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: reduction,
                    soft_aggregate_guard: guard,
                    accumulator_spec: None,
                }],
            },
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description: String::new(),
            intensity_labels: vec![],
        }
    }

    fn fission_semantic(stid: SimThingId, pid: SimPropertyId) -> ThresholdSemantic {
        ThresholdSemantic::FissionTrigger {
            sim_thing_id: stid,
            property_id: pid,
            template_idx: 0,
        }
    }

}
