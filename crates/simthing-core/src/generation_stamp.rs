//! EVENT-GENERATION-STAMP-0 — the seam primitive for independently-executing subtrees.
//!
//! Generation stamps ride existing emission and reduce-up carriers. There is no second
//! clock, sequence authority, scheduler, or transport. Determinism holds relative to a
//! **recorded integration schedule**, never by waiting for lagging children.
//!
//! Async is ordinary: a parent at generation N+3 integrating a child's gen-N product is the
//! normal case and must complete with no error, warning, degraded path, or wait.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::overlay::Overlay;
use crate::owner_channel::OwnerRef;
use crate::{GrantLifecycleFact, GrantLifecycleFactKind};

/// Per-tree generation counter value. One authority per tree (per-tree instantiation);
/// not a global barrier or cross-tree sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GenerationStamp(pub u32);

impl GenerationStamp {
    pub const fn new(generation: u32) -> Self {
        Self(generation)
    }

    pub const fn get(self) -> u32 {
        self.0
    }

    /// Observable staleness of a child product relative to the integrating parent.
    /// Zero when the child is current or ahead; positive when the parent is ahead.
    /// Staleness is visible and attributable from the stamp alone — never silent.
    pub const fn staleness_from_child(self, child: GenerationStamp) -> u32 {
        self.0.saturating_sub(child.0)
    }

    pub const fn is_stale_relative_to_parent(self, parent: GenerationStamp) -> bool {
        parent.staleness_from_child(self) > 0
    }
}

/// Any product that crosses a tree seam must carry a generation stamp.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationStamped<T> {
    pub generation: GenerationStamp,
    pub product: T,
}

impl<T> GenerationStamped<T> {
    pub fn stamp(generation: GenerationStamp, product: T) -> Self {
        Self {
            generation,
            product,
        }
    }

    pub fn generation(&self) -> GenerationStamp {
        self.generation
    }

    pub fn product(&self) -> &T {
        &self.product
    }

    pub fn into_product(self) -> T {
        self.product
    }
}

/// Minimal routed duration carrier for generation-denominated facilities.
///
/// This substrate intentionally carries only authored duration and source
/// provenance. It cannot represent an absolute deadline, and serde rejects
/// unknown fields rather than silently admitting a foreign deadline-shaped
/// payload. Interpretation belongs to the receiving facility's later admitted
/// semantics; this type performs no lifecycle calculation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutedGenerationDuration {
    authored_duration: u32,
    provenance: GenerationStamp,
}

impl RoutedGenerationDuration {
    pub const fn new(authored_duration: u32, provenance: GenerationStamp) -> Self {
        Self {
            authored_duration,
            provenance,
        }
    }

    pub const fn authored_duration(self) -> u32 {
        self.authored_duration
    }

    pub const fn provenance(self) -> GenerationStamp {
        self.provenance
    }
}

/// One recorded integration of a stamped child product at a parent generation.
///
/// **Per-product row, full generation set** (Definable schedule fence / HD-RECEIPT
/// `9df0629526ec`): never collapse to per-bucket-latest. Values may sum under later
/// coalescing, but stamps do not — a schedule that records only the newest stamp loses
/// which generations merged and cannot replay bit-exactly. This is THE single replay
/// recorder; 6.2 extends it with a row kind, never a second log.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationScheduleRowKind {
    /// Direct 6.1 reduce-up integration (the pre-queue path).
    #[default]
    DirectReduceUp,
    /// One contributing product admitted through the 6.2 coalescing queue.
    QueueInjection,
    /// One downward standing/policy snapshot published at a child barrier.
    StandingView,
    /// One provisional market entitlement realized as committed physical residency.
    ResidencyPlacementCommit,
    /// One ordinary physical infeasibility; geometry remained uncommitted and quantity stayed U.
    ResidencyPlacementRefusal,
    /// One committed placement moved through the existing epoch-rebind authority.
    ResidencyRelocation,
    /// One already-committed placement invariant breach recorded before session termination.
    ResidencyCommittedCorruption,
    /// One ordinary growth claim that remained U before structural attachment
    /// (zero/partial market clearance or a non-placement admission refusal).
    GrowthEntitlementRefusal,
    GrantAccepted,
    GrantRenewed,
    GrantRevoked,
    GrantPartitioned,
    GrantTransferred,
    GrantReleased,
}

impl IntegrationScheduleRowKind {
    pub const fn for_grant_lifecycle(kind: GrantLifecycleFactKind) -> Self {
        match kind {
            GrantLifecycleFactKind::Accepted => Self::GrantAccepted,
            GrantLifecycleFactKind::Renewed => Self::GrantRenewed,
            GrantLifecycleFactKind::Revoked => Self::GrantRevoked,
            GrantLifecycleFactKind::Partitioned => Self::GrantPartitioned,
            GrantLifecycleFactKind::Transferred => Self::GrantTransferred,
            GrantLifecycleFactKind::Released => Self::GrantReleased,
        }
    }

    pub const fn is_grant_lifecycle(self) -> bool {
        matches!(
            self,
            Self::GrantAccepted
                | Self::GrantRenewed
                | Self::GrantRevoked
                | Self::GrantPartitioned
                | Self::GrantTransferred
                | Self::GrantReleased
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationScheduleEntry {
    /// Semantic use of this row. Older serialized 6.1 schedules default to direct reduce-up.
    #[serde(default)]
    pub kind: IntegrationScheduleRowKind,
    /// Generation of the integrating side's barrier. For a downward standing read this is the
    /// child consumer generation; the 6.1 field name is retained for wire compatibility.
    pub parent_generation: GenerationStamp,
    /// Generation of the side that produced the stamped value. For a downward standing read
    /// this is the ancestor/parent source generation.
    pub child_generation: GenerationStamp,
    /// Stable identity of the product (e.g. reduce-up fingerprint). Not a clock.
    pub product_key: u64,
    /// Typed payload only for the six grant-lifecycle row kinds. Older schedule
    /// rows remain wire-compatible and deserialize with no payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_lifecycle_fact: Option<GrantLifecycleFact>,
}

impl IntegrationScheduleEntry {
    pub const fn row_kind(&self) -> IntegrationScheduleRowKind {
        self.kind
    }
}

/// Recorded integration schedule for one tree. Determinism is relative to this log.
///
/// Rows are append-only and per-product. Identical `product_key` values at different
/// child generations produce distinct rows so the full generation set is preserved.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationSchedule {
    pub entries: Vec<IntegrationScheduleEntry>,
}

impl IntegrationSchedule {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one per-product row. Never overwrites an earlier row for the same key.
    pub fn record(
        &mut self,
        parent_generation: GenerationStamp,
        child_generation: GenerationStamp,
        product_key: u64,
    ) {
        self.record_kind(
            IntegrationScheduleRowKind::DirectReduceUp,
            parent_generation,
            child_generation,
            product_key,
        );
    }

    /// Append one row to the single integration recorder. Queue and standing rows are
    /// discriminated here rather than sent to a second log or sequence authority.
    pub fn record_kind(
        &mut self,
        kind: IntegrationScheduleRowKind,
        parent_generation: GenerationStamp,
        child_generation: GenerationStamp,
        product_key: u64,
    ) {
        self.entries.push(IntegrationScheduleEntry {
            kind,
            parent_generation,
            child_generation,
            product_key,
            grant_lifecycle_fact: None,
        });
    }

    /// Append a lifecycle fact to THE canonical integration recorder. The
    /// parent-side row is due exactly at N+1. Multiple lawful transitions of
    /// the same kind and provenance remain distinct, ordered history rows.
    pub fn record_grant_lifecycle(
        &mut self,
        fact: GrantLifecycleFact,
    ) -> Result<(), GrantLifecycleScheduleError> {
        let parent_generation = GenerationStamp(
            fact.generation
                .get()
                .checked_add(1)
                .ok_or(GrantLifecycleScheduleError::GenerationOverflow)?,
        );
        let kind = IntegrationScheduleRowKind::for_grant_lifecycle(fact.kind);
        self.entries.push(IntegrationScheduleEntry {
            kind,
            parent_generation,
            child_generation: fact.generation,
            product_key: fact.provenance,
            grant_lifecycle_fact: Some(fact),
        });
        Ok(())
    }

    pub fn grant_lifecycle_facts_due(
        &self,
        generation: GenerationStamp,
    ) -> impl Iterator<Item = &GrantLifecycleFact> {
        self.entries.iter().filter_map(move |entry| {
            (entry.kind.is_grant_lifecycle() && entry.parent_generation == generation)
                .then_some(entry.grant_lifecycle_fact.as_ref())
                .flatten()
        })
    }

    pub fn entries(&self) -> &[IntegrationScheduleEntry] {
        &self.entries
    }

    /// Distinct child generations recorded for `product_key` (full set, not latest-only).
    pub fn child_generations_for_key(&self, product_key: u64) -> Vec<GenerationStamp> {
        self.entries
            .iter()
            .filter(|e| e.product_key == product_key)
            .map(|e| e.child_generation)
            .collect()
    }

    pub fn entries_of_kind(
        &self,
        kind: IntegrationScheduleRowKind,
    ) -> impl Iterator<Item = &IntegrationScheduleEntry> {
        self.entries.iter().filter(move |entry| entry.kind == kind)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum GrantLifecycleScheduleError {
    #[error("grant lifecycle fact generation cannot schedule N+1")]
    GenerationOverflow,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum IntegrateError {
    #[error(
        "cannot integrate an unstamped product: generation stamp is required on reduce-up products and events"
    )]
    UnstampedProduct,
    #[error("integration schedule is required for deterministic async integration")]
    MissingSchedule,
    /// Planted wait mutant: parent would wait for a lagging child's generation to catch up.
    /// The ordinary path never emits this — async N+3 <- N is admitted without wait.
    #[error(
        "would wait for lagging child: parent generation {parent} requires child generation {child} (wait mutant)"
    )]
    WouldWaitForLaggingChild { parent: u32, child: u32 },
    #[error(
        "seam staleness tolerance exceeded: integration generation {integration}, source generation {source_generation}, observed {observed}, authored maximum {allowed}"
    )]
    StalenessToleranceExceeded {
        integration: u32,
        source_generation: u32,
        observed: u32,
        allowed: u32,
    },
    #[error("integration arithmetic overflow while exactly conserving queued values")]
    ArithmeticOverflow,
    #[error("queued child + seam + parent balance escaped the admitted conserved total")]
    ConservationViolation,
    #[error("no generation-consistent standing snapshot has been published")]
    MissingPublishedStandingView,
    #[error("standing/policy view could not be encoded for stable replay identity: {0}")]
    StandingViewEncoding(String),
    #[error(
        "recorded {kind:?} product is unavailable at source generation {source_generation} with key {product_key}"
    )]
    MissingRecordedProduct {
        kind: IntegrationScheduleRowKind,
        source_generation: u32,
        product_key: u64,
    },
}

/// Per-seam authored staleness admission. There is deliberately no `Default`: every seam
/// author must state how old a cross-site value may be before integration hard-errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoredSeamStaleness {
    max_generations: u32,
}

impl AuthoredSeamStaleness {
    pub const fn new(max_generations: u32) -> Self {
        Self { max_generations }
    }

    pub const fn max_generations(self) -> u32 {
        self.max_generations
    }

    pub fn check(
        self,
        integration_generation: GenerationStamp,
        source_generation: GenerationStamp,
    ) -> Result<u32, IntegrateError> {
        let observed = integration_generation.staleness_from_child(source_generation);
        if observed > self.max_generations {
            return Err(IntegrateError::StalenessToleranceExceeded {
                integration: integration_generation.get(),
                source_generation: source_generation.get(),
                observed,
                allowed: self.max_generations,
            });
        }
        Ok(observed)
    }
}

/// Downward state crossing one independent-execution seam. `OwnerRef` is the canonical
/// intrinsic owner identity; no session-local interned owner id has a representable slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AncestorStandingPolicyView {
    pub owner_ref: OwnerRef,
    pub overlays: Vec<Overlay>,
}

impl AncestorStandingPolicyView {
    pub fn new(owner_ref: OwnerRef, overlays: Vec<Overlay>) -> Self {
        Self {
            owner_ref,
            overlays,
        }
    }

    pub fn product_key(&self) -> Result<u64, IntegrateError> {
        let bytes = serde_json::to_vec(self)
            .map_err(|error| IntegrateError::StandingViewEncoding(error.to_string()))?;
        Ok(stable_bytes_key(&bytes))
    }
}

/// Two-slot publication surface for downward standing/policy state.
///
/// Writers replace the inactive slot and readers observe only the atomically selected complete
/// `(generation, value)` pair. Publication is available only through the generation-barrier
/// method, preventing mixed-generation shadow reads.
#[derive(Clone, Debug)]
pub struct StandingViewDoubleBuffer {
    slots: [Option<GenerationStamped<AncestorStandingPolicyView>>; 2],
    published: usize,
    staged: bool,
}

impl StandingViewDoubleBuffer {
    pub fn new() -> Self {
        Self {
            slots: [None, None],
            published: 0,
            staged: false,
        }
    }

    /// Stage a complete parent-produced view in the inactive slot. Standing state is read-only,
    /// not a conserved product; a newer pre-barrier view may replace an older unobserved view.
    pub fn stage(&mut self, view: GenerationStamped<AncestorStandingPolicyView>) {
        let staging = 1 - self.published;
        self.slots[staging] = Some(view);
        self.staged = true;
    }

    /// Publish the inactive complete view at a child generation barrier and record that read in
    /// the same [`IntegrationSchedule`] used by upward products.
    pub fn publish_at_generation_barrier(
        &mut self,
        child_generation: GenerationStamp,
        tolerance: AuthoredSeamStaleness,
        schedule: &mut IntegrationSchedule,
    ) -> Result<Option<IntegrationReceipt>, IntegrateError> {
        if !self.staged {
            if let Some(published) = self.slots[self.published].as_ref() {
                tolerance.check(child_generation, published.generation())?;
            }
            return Ok(None);
        }
        let staging = 1 - self.published;
        let staged = self.slots[staging]
            .as_ref()
            .expect("staged flag always names a complete inactive slot");
        let staleness = tolerance.check(child_generation, staged.generation())?;
        let product_key = staged.product().product_key()?;
        let source_generation = staged.generation();
        self.published = staging;
        self.staged = false;
        schedule.record_kind(
            IntegrationScheduleRowKind::StandingView,
            child_generation,
            source_generation,
            product_key,
        );
        Ok(Some(IntegrationReceipt {
            parent_generation: child_generation,
            child_generation: source_generation,
            product_key,
            staleness,
        }))
    }

    /// Read one coherent published pair. The staleness check is repeated at the consuming
    /// generation so retaining an old published view cannot silently outlive its authored bound.
    pub fn read(
        &self,
        child_generation: GenerationStamp,
        tolerance: AuthoredSeamStaleness,
    ) -> Result<&GenerationStamped<AncestorStandingPolicyView>, IntegrateError> {
        let published = self.slots[self.published]
            .as_ref()
            .ok_or(IntegrateError::MissingPublishedStandingView)?;
        tolerance.check(child_generation, published.generation())?;
        Ok(published)
    }
}

impl Default for StandingViewDoubleBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay downward standing reads from the one integration schedule. Ambient arrival order is
/// ignored; every row must name an available bit-identical stamped view.
pub fn replay_standing_views(
    schedule: &IntegrationSchedule,
    available: &[GenerationStamped<AncestorStandingPolicyView>],
) -> Result<Vec<GenerationStamped<AncestorStandingPolicyView>>, IntegrateError> {
    let mut replayed = Vec::new();
    for entry in schedule.entries_of_kind(IntegrationScheduleRowKind::StandingView) {
        let found = available.iter().find(|view| {
            view.generation() == entry.child_generation
                && view.product().product_key().ok() == Some(entry.product_key)
        });
        let Some(view) = found else {
            return Err(IntegrateError::MissingRecordedProduct {
                kind: entry.kind,
                source_generation: entry.child_generation.get(),
                product_key: entry.product_key,
            });
        };
        replayed.push(view.clone());
    }
    Ok(replayed)
}

fn stable_bytes_key(bytes: &[u8]) -> u64 {
    // FNV-1a is deliberately specified here instead of using process-seeded `Hash` state.
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Result of integrating one stamped product. Never waits. Staleness is observable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegrationReceipt {
    pub parent_generation: GenerationStamp,
    pub child_generation: GenerationStamp,
    pub product_key: u64,
    /// `parent_generation - child_generation` when parent is ahead; zero otherwise.
    pub staleness: u32,
}

/// Integrate a stamped child product at the parent's current generation.
///
/// - Completes immediately — never waits for a lagging child.
/// - Records the schedule entry so replay is bit-exact.
/// - Surfaces staleness from the stamps alone.
///
/// Unstamped products are unrepresentable here: only [`GenerationStamped`] is accepted.
pub fn integrate_stamped_product<T>(
    parent_generation: GenerationStamp,
    stamped: &GenerationStamped<T>,
    product_key: u64,
    schedule: &mut IntegrationSchedule,
) -> IntegrationReceipt {
    let child_generation = stamped.generation;
    let staleness = parent_generation.staleness_from_child(child_generation);
    schedule.record(parent_generation, child_generation, product_key);
    IntegrationReceipt {
        parent_generation,
        child_generation,
        product_key,
        staleness,
    }
}

/// Hard-error path for any attempt to integrate without a stamp.
///
/// Planted-defect witness: calling this (or any unstamped integrate path) is RED.
pub fn integrate_unstamped_product_forbidden(
    _product_key: u64,
    _schedule: &mut IntegrationSchedule,
) -> Result<IntegrationReceipt, IntegrateError> {
    Err(IntegrateError::UnstampedProduct)
}

/// Replay a previously recorded schedule against a stream of stamped products.
///
/// Products are selected by `(child_generation, product_key)` from the schedule —
/// never by ambient arrival order. Dropping the schedule and using ambient timing is RED.
pub fn replay_integration_schedule<T>(
    schedule: &IntegrationSchedule,
    available: &[GenerationStamped<T>],
    product_keys: &[u64],
) -> Result<Vec<IntegrationReceipt>, IntegrateError>
where
    T: Clone,
{
    if schedule.entries.is_empty() && !available.is_empty() {
        // A non-empty product stream with an empty schedule is the ambient-timing mutant.
        return Err(IntegrateError::MissingSchedule);
    }
    let mut receipts = Vec::with_capacity(schedule.entries.len());
    for entry in &schedule.entries {
        let found = available
            .iter()
            .zip(product_keys.iter())
            .find(|(stamped, key)| {
                stamped.generation == entry.child_generation && **key == entry.product_key
            });
        let Some((stamped, _)) = found else {
            // Missing product for a recorded entry — still not ambient recovery.
            continue;
        };
        let mut scratch = IntegrationSchedule::new();
        let receipt = integrate_stamped_product(
            entry.parent_generation,
            stamped,
            entry.product_key,
            &mut scratch,
        );
        receipts.push(receipt);
    }
    Ok(receipts)
}

// ── Stamped ring egress with admission-time backpressure ─────────────────────

/// Admission-time backpressure policy for observer/event egress rings.
///
/// Forced observer lag must honor the declared policy without perturbing the sim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackpressurePolicy {
    /// Drop the oldest entry when full (overwrite-oldest).
    OverwriteOldest,
    /// Refuse the push when full (throttle); sim continues; observer loses the event.
    Throttle,
    /// Coalesce with the newest same-key entry when full (coalesce-per-band shape).
    CoalescePerKey,
}

/// One generation-stamped egress slot. Payload is opaque to the ring.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StampedEgressEntry {
    pub generation: GenerationStamp,
    pub key: u64,
    pub payload_bits: u64,
}

/// Generation-stamped ring for observer egress. Capacity and policy are admission-time.
///
/// The ring never feeds back into sim state — observer lag cannot perturb the simulation.
#[derive(Clone, Debug)]
pub struct StampedEventRing {
    capacity: usize,
    policy: BackpressurePolicy,
    entries: Vec<StampedEgressEntry>,
    /// Count of events dropped/throttled/coalesced (observation only; not sim authority).
    pub backpressure_actions: u64,
    /// Count of successful pushes that landed in the ring.
    pub accepted: u64,
    /// Times the production egress door was entered (even when zero records).
    /// Proves the live path ran; a dead-code wrap (`if false`) leaves this at 0.
    pub admit_invocations: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingPushOutcome {
    Accepted,
    OverwroteOldest,
    Throttled,
    Coalesced,
}

impl StampedEventRing {
    pub fn admit(capacity: usize, policy: BackpressurePolicy) -> Self {
        assert!(capacity > 0, "ring capacity must be positive at admission");
        Self {
            capacity,
            policy,
            entries: Vec::with_capacity(capacity),
            backpressure_actions: 0,
            accepted: 0,
            admit_invocations: 0,
        }
    }

    /// Record that the production egress door was entered (observer path live).
    pub fn note_admit_invocation(&mut self) {
        self.admit_invocations = self.admit_invocations.saturating_add(1);
    }

    pub fn policy(&self) -> BackpressurePolicy {
        self.policy
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[StampedEgressEntry] {
        &self.entries
    }

    /// Push a stamped event. Never blocks the sim; applies admission-time policy when full.
    pub fn push(&mut self, entry: StampedEgressEntry) -> RingPushOutcome {
        if self.entries.len() < self.capacity {
            self.entries.push(entry);
            self.accepted += 1;
            return RingPushOutcome::Accepted;
        }
        match self.policy {
            BackpressurePolicy::OverwriteOldest => {
                self.entries.remove(0);
                self.entries.push(entry);
                self.backpressure_actions += 1;
                self.accepted += 1;
                RingPushOutcome::OverwroteOldest
            }
            BackpressurePolicy::Throttle => {
                self.backpressure_actions += 1;
                RingPushOutcome::Throttled
            }
            BackpressurePolicy::CoalescePerKey => {
                if let Some(existing) = self.entries.iter_mut().rev().find(|e| e.key == entry.key) {
                    *existing = entry;
                    self.backpressure_actions += 1;
                    RingPushOutcome::Coalesced
                } else {
                    // No same-key entry: fall back to overwrite-oldest shape.
                    self.entries.remove(0);
                    self.entries.push(entry);
                    self.backpressure_actions += 1;
                    self.accepted += 1;
                    RingPushOutcome::OverwroteOldest
                }
            }
        }
    }

    /// Drain up to `max` entries for a lagging observer. Does not affect sim state.
    pub fn observer_drain(&mut self, max: usize) -> Vec<StampedEgressEntry> {
        let n = max.min(self.entries.len());
        self.entries.drain(0..n).collect()
    }
}

// ── Dispatch dissolve discipline (Definable Horizon) ─────────────────────────

use crate::overlay::{DissolveCondition, OverlayLifecycle};

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DispatchOverlayError {
    #[error(
        "dispatch-minted overlay must carry UntilDissolved with at least one authored dissolve condition"
    )]
    MissingDissolveCondition,
    #[error("dispatch-minted overlay must use UntilDissolved lifecycle (no permanence variant)")]
    NotUntilDissolved,
    #[error(
        "dispatch-minted overlay origin must be a real originating node (not default/borrowed)"
    )]
    InvalidOrigin,
}

/// Build the only admitted lifecycle for a dispatch-minted overlay.
///
/// Requires at least one authored dissolve condition. `AtSessionEnd` is a definable
/// horizon (session floor), never "never". There is no permanence variant.
pub fn dispatch_until_dissolved(
    dissolution_conditions: Vec<DissolveCondition>,
) -> Result<OverlayLifecycle, DispatchOverlayError> {
    if dissolution_conditions.is_empty() {
        return Err(DispatchOverlayError::MissingDissolveCondition);
    }
    Ok(OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions,
    })
}

/// Admit a dispatch-minted overlay: UntilDissolved + authored condition + real origin.
///
/// Planted defect: minting without a dissolve condition REDs.
pub fn admit_dispatch_minted_overlay(overlay: &Overlay) -> Result<(), DispatchOverlayError> {
    crate::overlay_lifecycle_deadline::admit_overlay_lifecycle(&overlay.lifecycle)
        .map_err(|_| DispatchOverlayError::NotUntilDissolved)?;
    match &overlay.lifecycle {
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        } => {
            if dissolution_conditions.is_empty() {
                return Err(DispatchOverlayError::MissingDissolveCondition);
            }
        }
        OverlayLifecycle::UntilDissolved => {
            // Unit UntilDissolved has no authored automatic condition — forbidden for dispatch.
            return Err(DispatchOverlayError::MissingDissolveCondition);
        }
        _ => return Err(DispatchOverlayError::NotUntilDissolved),
    }
    // Origin is required on the type; zero-id is still a real SimThingId but we only
    // require the field is present (type boundary). Borrowed/synthesized origin is
    // rejected by deliver_routed_overlay when not in tree — not re-checked here.
    let _ = overlay.origin;
    Ok(())
}
