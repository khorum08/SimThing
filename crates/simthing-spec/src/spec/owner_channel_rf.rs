//! OWNER-CHANNEL-INTRINSIC-0 (rung 6.0) — generalized owner-channel reduce-up.
//!
//! This surface is derived and reconstructible.  It never enters authored, wire, or replay
//! state.  Every active node/resource pair contributes one ordinary STEAD own-aggregate row.  Effective
//! ownership is retained only at ownership crossings, so retained owner-boundary state is
//! O(crossings), never O(nodes × owners × resources).
//!
//! EVENT-GENERATION-STAMP-0: reduce-up products are a **second stamp carrier**. Integrating an
//! unstamped product is a hard error. Stamp at the producing tree's generation; parents integrate
//! stamped products without waiting (async is ordinary).

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::owner_channel::{resolve_owner, resolve_owners_in_order, OwnerRef};
use simthing_core::{
    integrate_stamped_product, AncestorStandingPolicyView, AuthoredSeamStaleness, GenerationStamp,
    GenerationStamped, IntegrateError, IntegrationReceipt, IntegrationSchedule,
    IntegrationScheduleRowKind, SimThing, SimThingId, StandingViewDoubleBuffer,
};

#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};

use super::channel_key::{OwnerChannelScopeKey, ResourceKey, ScopeId};

/// Largest integer for which every smaller non-negative integer is exactly representable as f32.
const MAX_GPU_EXACT_INTEGER: u32 = 1 << 24;

/// One node's ordinary RF aggregate before inherited ownership is resolved.
///
/// Deliberately contains no owner or scope.  Stamping either here would materialize a resolved
/// owner at every node and recreate the flat owner channel this rung removes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnerChannelRfOwnAggregate {
    pub simthing_id: SimThingId,
    pub resource_key: ResourceKey,
    pub surplus: u32,
    pub deficit: u32,
}

/// Canonically ordered resource flow retained at one ownership crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfCrossingResourceFlow {
    pub resource_key: ResourceKey,
    pub participant_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
}

/// One retained ownership crossing on the ordinary STEAD tree surface.
///
/// There is exactly one row per crossing, even when the crossing carries several resource
/// flows.  Identity edges are absent; their owner and scope are reconstructed by inheritance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfCrossingFlow {
    pub boundary_simthing_id: SimThingId,
    pub parent_scope_id: ScopeId,
    pub scope_id: ScopeId,
    pub owner_ref: OwnerRef,
    pub resources: Vec<OwnerChannelRfCrossingResourceFlow>,
}

/// Minimal reconstructible STEAD observation for owner-channel RF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfSteadSurface {
    pub own_aggregates: Vec<OwnerChannelRfOwnAggregate>,
    pub crossing_flows: Vec<OwnerChannelRfCrossingFlow>,
}

/// One canonical `{owner, resource, ScopeId}` reduce-up bucket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfBucket {
    pub scope: OwnerChannelScopeKey,
    pub source_row_indices: Vec<usize>,
    pub participant_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub net_surplus: u32,
    pub net_deficit: u32,
}

/// Conserved reduce-up report. `buckets` is in `OwnerChannelScopeKey` order.
///
/// Internal aggregation shape. The **production seam egress** is
/// [`reduce_owner_channel_rf`], which returns a [`StampedReduceUpProduct`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfReduceUpReport {
    pub participant_count: u32,
    pub owner_count: u32,
    pub bucket_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub buckets: Vec<OwnerChannelRfBucket>,
    pub stead: OwnerChannelRfSteadSurface,
}

/// Reduce-up product stamped with the producing tree's generation.
/// This is the only shape that may cross a parent integration seam.
pub type StampedReduceUpProduct = GenerationStamped<OwnerChannelRfReduceUpReport>;

/// Stable product key derived from conserved totals (identity for the schedule log).
pub fn reduce_up_product_key(report: &OwnerChannelRfReduceUpReport) -> u64 {
    let mut h = 0u64;
    h ^= report.participant_count as u64;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= report.owner_count as u64;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= report.bucket_count as u64;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= report.surplus_total as u64;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= report.deficit_total as u64;
    h
}

/// Parent-side RF state after integrating stamped child products.
///
/// This is the integrated output the schedule must be able to replay bit-exactly.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParentRfIntegrationState {
    pub surplus_total: u64,
    pub deficit_total: u64,
    pub product_count: u64,
    /// Exact per-scope values after direct or queued integration.
    pub buckets: BTreeMap<OwnerChannelScopeKey, OwnerChannelRfConservedValue>,
    /// Fold of product keys in schedule order (bit-exact replay witness).
    pub schedule_fold: u64,
}

/// Integrate a stamped reduce-up product into parent RF state at the parent generation.
///
/// Async is ordinary: parent at N+3 integrating child gen-N completes with **no wait**.
/// There is no production freshness gate, toggle, or lagging-child reject path.
/// Records a **per-product** schedule row (full generation set; never per-bucket-latest).
pub fn integrate_stamped_reduce_up(
    parent_generation: GenerationStamp,
    product: &StampedReduceUpProduct,
    parent_state: &mut ParentRfIntegrationState,
    schedule: &mut IntegrationSchedule,
) -> Result<IntegrationReceipt, IntegrateError> {
    let report = product.product();
    let key = reduce_up_product_key(report);
    let mut next = parent_state.clone();
    apply_report_exact(&mut next, report)?;
    next.product_count = next
        .product_count
        .checked_add(1)
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    let receipt = integrate_stamped_product(parent_generation, product, key, schedule);
    next.schedule_fold = next
        .schedule_fold
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(key)
        .wrapping_add(product.generation().get() as u64)
        .wrapping_add(parent_generation.get() as u64);
    *parent_state = next;
    Ok(receipt)
}

/// Test-only make-the-parent-wait mutant. **Not linked into production builds.**
///
/// Planted defect for Remand 2: when enabled, lagged N+3 <- N REDs. Production
/// `integrate_stamped_reduce_up` never contains this branch.
#[cfg(test)]
static WAIT_FOR_FRESH_CHILD_MUTANT: AtomicBool = AtomicBool::new(false);

/// Enable/disable the make-the-parent-wait mutant (test-only; `cfg(test)`).
#[cfg(test)]
pub fn plant_wait_for_fresh_child_mutant(enabled: bool) {
    WAIT_FOR_FRESH_CHILD_MUTANT.store(enabled, Ordering::SeqCst);
}

/// Test-only integrate path that can plant the wait mutant. Production never calls this.
#[cfg(test)]
pub fn integrate_stamped_reduce_up_for_wait_mutant_proof(
    parent_generation: GenerationStamp,
    product: &StampedReduceUpProduct,
    parent_state: &mut ParentRfIntegrationState,
    schedule: &mut IntegrationSchedule,
) -> Result<IntegrationReceipt, IntegrateError> {
    if WAIT_FOR_FRESH_CHILD_MUTANT.load(Ordering::SeqCst)
        && product.generation() != parent_generation
    {
        return Err(IntegrateError::WouldWaitForLaggingChild {
            parent: parent_generation.get(),
            child: product.generation().get(),
        });
    }
    integrate_stamped_reduce_up(parent_generation, product, parent_state, schedule)
}

/// Replay a recorded schedule into parent RF state bit-exactly.
///
/// Products are selected by `(child_generation, product_key)` — never ambient order.
pub fn replay_reduce_up_schedule(
    schedule: &IntegrationSchedule,
    products: &[StampedReduceUpProduct],
) -> Result<ParentRfIntegrationState, IntegrateError> {
    if schedule.entries().is_empty() && !products.is_empty() {
        return Err(IntegrateError::MissingSchedule);
    }
    let mut state = ParentRfIntegrationState::default();
    let mut scratch = IntegrationSchedule::new();
    for entry in schedule.entries_of_kind(IntegrationScheduleRowKind::DirectReduceUp) {
        let found = products.iter().find(|p| {
            p.generation() == entry.child_generation
                && reduce_up_product_key(p.product()) == entry.product_key
        });
        let Some(product) = found else {
            continue;
        };
        integrate_stamped_reduce_up(entry.parent_generation, product, &mut state, &mut scratch)?;
    }
    Ok(state)
}

/// Reject unstamped products at the production integration door.
///
/// The production door only accepts [`StampedReduceUpProduct`]. This helper exists
/// so a planted attempt to feed a raw report is expressible and REDs.
pub fn integrate_raw_reduce_up_report_forbidden(
    _report: &OwnerChannelRfReduceUpReport,
    _parent_state: &mut ParentRfIntegrationState,
    _schedule: &mut IntegrationSchedule,
) -> Result<IntegrationReceipt, IntegrateError> {
    Err(IntegrateError::UnstampedProduct)
}

/// Exact conserved numeric surface of one canonical RF bucket after crossing a seam.
/// Values widen to `u64` because a burst may contain many individually admitted `u32` products;
/// coalescing must sum rather than saturate, overwrite, or throttle any field.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OwnerChannelRfConservedValue {
    pub participant_count: u64,
    pub surplus_total: u64,
    pub deficit_total: u64,
    pub net_surplus: u64,
    pub net_deficit: u64,
}

impl OwnerChannelRfConservedValue {
    fn from_bucket(bucket: &OwnerChannelRfBucket) -> Self {
        Self {
            participant_count: u64::from(bucket.participant_count),
            surplus_total: u64::from(bucket.surplus_total),
            deficit_total: u64::from(bucket.deficit_total),
            net_surplus: u64::from(bucket.net_surplus),
            net_deficit: u64::from(bucket.net_deficit),
        }
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            participant_count: self
                .participant_count
                .checked_add(other.participant_count)?,
            surplus_total: self.surplus_total.checked_add(other.surplus_total)?,
            deficit_total: self.deficit_total.checked_add(other.deficit_total)?,
            net_surplus: self.net_surplus.checked_add(other.net_surplus)?,
            net_deficit: self.net_deficit.checked_add(other.net_deficit)?,
        })
    }

    fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self {
            participant_count: self
                .participant_count
                .checked_sub(other.participant_count)?,
            surplus_total: self.surplus_total.checked_sub(other.surplus_total)?,
            deficit_total: self.deficit_total.checked_sub(other.deficit_total)?,
            net_surplus: self.net_surplus.checked_sub(other.net_surplus)?,
            net_deficit: self.net_deficit.checked_sub(other.net_deficit)?,
        })
    }
}

/// Observable accounting for one `{owner, resource, scope}` bucket.
/// `admitted` is the immutable emitted-product total against which the three live locations
/// are checked. It is an oracle total, not a fourth place in which product can reside.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OwnerChannelRfSeamBalance {
    child: OwnerChannelRfConservedValue,
    seam: OwnerChannelRfConservedValue,
    parent: OwnerChannelRfConservedValue,
    admitted: OwnerChannelRfConservedValue,
}

impl OwnerChannelRfSeamBalance {
    pub fn child(self) -> OwnerChannelRfConservedValue {
        self.child
    }

    pub fn seam(self) -> OwnerChannelRfConservedValue {
        self.seam
    }

    pub fn parent(self) -> OwnerChannelRfConservedValue {
        self.parent
    }

    pub fn admitted(self) -> OwnerChannelRfConservedValue {
        self.admitted
    }

    pub fn is_exact(self) -> bool {
        self.child
            .checked_add(self.seam)
            .and_then(|value| value.checked_add(self.parent))
            == Some(self.admitted)
    }

    /// Referee observation of the three live locations plus the admitted total.
    /// Does not transfer product; the 8.1 judge consumes this snapshot.
    pub fn observe(
        child: OwnerChannelRfConservedValue,
        seam: OwnerChannelRfConservedValue,
        parent: OwnerChannelRfConservedValue,
        admitted: OwnerChannelRfConservedValue,
    ) -> Self {
        Self {
            child,
            seam,
            parent,
            admitted,
        }
    }
}

/// One losslessly coalesced pending carrier. There is at most one carrier for each scope key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueuedOwnerChannelRfBucket {
    pub scope: OwnerChannelScopeKey,
    pub value: OwnerChannelRfConservedValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PendingContribution {
    generation: GenerationStamp,
    product_key: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PendingBucket {
    value: OwnerChannelRfConservedValue,
    newest_generation: GenerationStamp,
}

/// Result of one parent generation-barrier application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncQueueBarrierReceipt {
    pub distinct_bucket_count: usize,
    pub contributing_product_count: usize,
}

/// The single bidirectional CPU seam for one independently executing child tree.
///
/// Upward conserved products share one exact, scope-keyed holding queue. Downward standing state
/// shares the same authored tolerance and the same external [`IntegrationSchedule`], but is
/// published through a torn-free double buffer because it is read-only state rather than a
/// conserved value. No operation waits for the other tree.
#[derive(Debug, Clone)]
pub struct AsyncOwnerChannelRfSeam {
    tolerance: AuthoredSeamStaleness,
    pending: BTreeMap<OwnerChannelScopeKey, PendingBucket>,
    /// In-flight membership needed to append one row per source product at the barrier.
    /// Replay authority remains exclusively in the external `IntegrationSchedule`.
    pending_products: Vec<PendingContribution>,
    admitted_product_count: u64,
    applied_product_count: u64,
    balances: BTreeMap<OwnerChannelScopeKey, OwnerChannelRfSeamBalance>,
    standing: StandingViewDoubleBuffer,
}

impl AsyncOwnerChannelRfSeam {
    /// Admit one seam with an explicitly authored tolerance. There is no inferred/default value.
    pub fn admit(tolerance: AuthoredSeamStaleness) -> Self {
        Self {
            tolerance,
            pending: BTreeMap::new(),
            pending_products: Vec::new(),
            admitted_product_count: 0,
            applied_product_count: 0,
            balances: BTreeMap::new(),
            standing: StandingViewDoubleBuffer::new(),
        }
    }

    pub fn tolerance(&self) -> AuthoredSeamStaleness {
        self.tolerance
    }

    /// Queue one child product without blocking. Every bucket is transferred child -> seam
    /// inside this call, and a same-scope pending value is increased by exact sums.
    pub fn enqueue_reduce_up(
        &mut self,
        product: &StampedReduceUpProduct,
    ) -> Result<(), IntegrateError> {
        self.admitted_product_count
            .checked_add(1)
            .ok_or(IntegrateError::ArithmeticOverflow)?;
        let mut incoming =
            BTreeMap::<OwnerChannelScopeKey, (OwnerChannelRfConservedValue, GenerationStamp)>::new(
            );
        for bucket in &product.product().buckets {
            let value = OwnerChannelRfConservedValue::from_bucket(bucket);
            match incoming.entry(bucket.scope.clone()) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((value, product.generation()));
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    let current = entry.get().0;
                    let current_generation = entry.get().1;
                    entry.get_mut().0 = current
                        .checked_add(value)
                        .ok_or(IntegrateError::ArithmeticOverflow)?;
                    entry.get_mut().1 = current_generation.max(product.generation());
                }
            }
        }

        // Full preflight: no scope mutates unless every exact addition and transfer is valid.
        for (scope, (value, _)) in &incoming {
            if let Some(pending) = self.pending.get(scope) {
                pending
                    .value
                    .checked_add(*value)
                    .ok_or(IntegrateError::ArithmeticOverflow)?;
            }
            let balance = self.balances.get(scope).copied().unwrap_or_default();
            balance
                .admitted
                .checked_add(*value)
                .ok_or(IntegrateError::ArithmeticOverflow)?;
            balance
                .child
                .checked_add(*value)
                .ok_or(IntegrateError::ArithmeticOverflow)?;
            balance
                .seam
                .checked_add(*value)
                .ok_or(IntegrateError::ArithmeticOverflow)?;
        }

        for (scope, (value, newest_incoming)) in incoming {
            let balance = self.balances.entry(scope.clone()).or_default();
            balance.admitted = balance.admitted.checked_add(value).expect("preflight");
            balance.child = balance.child.checked_add(value).expect("preflight");
            debug_assert!(
                balance.is_exact(),
                "child receipt must conserve immediately"
            );
            balance.child = balance.child.checked_sub(value).expect("just credited");
            balance.seam = balance.seam.checked_add(value).expect("preflight");
            debug_assert!(balance.is_exact(), "seam holding transfer must conserve");

            match self.pending.entry(scope) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(PendingBucket {
                        value,
                        newest_generation: newest_incoming,
                    });
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    let pending = entry.get_mut();
                    pending.value = pending.value.checked_add(value).expect("preflight");
                    pending.newest_generation = pending.newest_generation.max(newest_incoming);
                }
            }
        }
        self.pending_products.push(PendingContribution {
            generation: product.generation(),
            product_key: queued_reduce_up_product_key(product.product()),
        });
        self.admitted_product_count += 1;
        self.check_conservation()
    }

    /// Number of pending carriers, exactly the number of distinct scope keys.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.pending_products.is_empty()
    }

    /// Read coalesced carriers in canonical scope order. Each carrier stamp is the newest/max
    /// contributing generation; the complete contributing set remains in the schedule rows.
    pub fn pending_carriers(&self) -> Vec<GenerationStamped<QueuedOwnerChannelRfBucket>> {
        self.pending
            .iter()
            .map(|(scope, pending)| {
                GenerationStamped::stamp(
                    pending.newest_generation,
                    QueuedOwnerChannelRfBucket {
                        scope: scope.clone(),
                        value: pending.value,
                    },
                )
            })
            .collect()
    }

    pub fn balance(&self, scope: &OwnerChannelScopeKey) -> Option<OwnerChannelRfSeamBalance> {
        self.balances.get(scope).copied()
    }

    pub fn check_conservation(&self) -> Result<(), IntegrateError> {
        if !self.balances.values().all(|balance| balance.is_exact()) {
            return Err(IntegrateError::ConservationViolation);
        }
        for (scope, balance) in &self.balances {
            let pending = self
                .pending
                .get(scope)
                .map(|bucket| bucket.value)
                .unwrap_or_default();
            if pending != balance.seam {
                return Err(IntegrateError::ConservationViolation);
            }
        }
        if self
            .pending
            .keys()
            .any(|scope| !self.balances.contains_key(scope))
        {
            return Err(IntegrateError::ConservationViolation);
        }
        let pending_count = u64::try_from(self.pending_products.len())
            .map_err(|_| IntegrateError::ConservationViolation)?;
        if self.applied_product_count.checked_add(pending_count)
            != Some(self.admitted_product_count)
        {
            return Err(IntegrateError::ConservationViolation);
        }
        Ok(())
    }

    /// Drain every pending bucket into the parent only at its generation barrier. Staleness is
    /// checked against each coalesced carrier's newest/max source stamp before any value or
    /// schedule row mutates; historical source stamps remain replay evidence and never become
    /// admission blockers. Lag never waits.
    pub fn apply_parent_generation_barrier(
        &mut self,
        parent_generation: GenerationStamp,
        parent_state: &mut ParentRfIntegrationState,
        schedule: &mut IntegrationSchedule,
    ) -> Result<AsyncQueueBarrierReceipt, IntegrateError> {
        for pending in self.pending.values() {
            self.tolerance
                .check(parent_generation, pending.newest_generation)?;
        }

        let contributing_product_count = self.pending_products.len();
        let contributing_product_count_u64 = u64::try_from(contributing_product_count)
            .map_err(|_| IntegrateError::ArithmeticOverflow)?;
        self.applied_product_count
            .checked_add(contributing_product_count_u64)
            .ok_or(IntegrateError::ArithmeticOverflow)?;
        parent_state
            .product_count
            .checked_add(contributing_product_count_u64)
            .ok_or(IntegrateError::ArithmeticOverflow)?;
        self.pending
            .values()
            .try_fold(
                (parent_state.surplus_total, parent_state.deficit_total),
                |(surplus, deficit), pending| {
                    Some((
                        surplus.checked_add(pending.value.surplus_total)?,
                        deficit.checked_add(pending.value.deficit_total)?,
                    ))
                },
            )
            .ok_or(IntegrateError::ArithmeticOverflow)?;
        for (scope, pending) in &self.pending {
            parent_state
                .buckets
                .get(scope)
                .copied()
                .unwrap_or_default()
                .checked_add(pending.value)
                .ok_or(IntegrateError::ArithmeticOverflow)?;
            let balance = self.balances.get(scope).copied().unwrap_or_default();
            balance
                .seam
                .checked_sub(pending.value)
                .ok_or(IntegrateError::ConservationViolation)?;
            balance
                .parent
                .checked_add(pending.value)
                .ok_or(IntegrateError::ArithmeticOverflow)?;
        }

        let distinct_bucket_count = self.pending.len();
        for (scope, pending) in &self.pending {
            add_queue_value_exact(parent_state, scope, pending.value)?;
            let balance = self
                .balances
                .get_mut(scope)
                .expect("queued scope has accounting");
            balance.seam = balance.seam.checked_sub(pending.value).expect("preflight");
            balance.parent = balance
                .parent
                .checked_add(pending.value)
                .expect("preflight");
            debug_assert!(balance.is_exact(), "seam -> parent transfer must conserve");
        }
        for contribution in &self.pending_products {
            schedule.record_kind(
                IntegrationScheduleRowKind::QueueInjection,
                parent_generation,
                contribution.generation,
                contribution.product_key,
            );
            fold_queue_schedule_row(
                parent_state,
                parent_generation,
                contribution.generation,
                contribution.product_key,
            );
        }
        parent_state.product_count += contributing_product_count_u64;
        self.applied_product_count += contributing_product_count_u64;
        self.pending.clear();
        self.pending_products.clear();
        self.check_conservation()?;
        Ok(AsyncQueueBarrierReceipt {
            distinct_bucket_count,
            contributing_product_count,
        })
    }

    /// Test-only planted defect: reintroduce historical-contributor staleness blocking.
    ///
    /// Production admission deliberately does not call this path. It exists so the combined
    /// coalescing/staleness proof can demonstrate that checking the oldest/every source product
    /// rejects a carrier whose canonical newest/max stamp is admissible.
    #[cfg(test)]
    fn check_historical_contributors_for_staleness_mutant(
        &self,
        parent_generation: GenerationStamp,
    ) -> Result<(), IntegrateError> {
        for contribution in &self.pending_products {
            self.tolerance
                .check(parent_generation, contribution.generation)?;
        }
        Ok(())
    }

    /// Test-only planted defect: make carrier freshness follow arrival order instead of max.
    ///
    /// The assignment below is the exact last-wins mutation the out-of-order referee exists to
    /// reject. Production ingress continues to use `pending.newest_generation.max(...)` only.
    #[cfg(test)]
    fn enqueue_reduce_up_last_wins_mutant(
        &mut self,
        product: &StampedReduceUpProduct,
    ) -> Result<(), IntegrateError> {
        let newest_incoming = product.generation();
        let scopes = product
            .product()
            .buckets
            .iter()
            .map(|bucket| bucket.scope.clone())
            .collect::<BTreeSet<_>>();
        self.enqueue_reduce_up(product)?;
        for scope in scopes {
            self.pending
                .get_mut(&scope)
                .expect("incoming scope was queued")
                .newest_generation = newest_incoming;
        }
        Ok(())
    }

    /// Stage a complete downward ancestor standing/policy view in the inactive buffer.
    pub fn stage_ancestor_standing_view(
        &mut self,
        view: GenerationStamped<AncestorStandingPolicyView>,
    ) {
        self.standing.stage(view);
    }

    /// Publish staged downward state at the child barrier through the same schedule recorder.
    pub fn apply_child_generation_barrier(
        &mut self,
        child_generation: GenerationStamp,
        schedule: &mut IntegrationSchedule,
    ) -> Result<Option<IntegrationReceipt>, IntegrateError> {
        self.standing
            .publish_at_generation_barrier(child_generation, self.tolerance, schedule)
    }

    pub fn standing_view(
        &self,
        child_generation: GenerationStamp,
    ) -> Result<&GenerationStamped<AncestorStandingPolicyView>, IntegrateError> {
        self.standing.read(child_generation, self.tolerance)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsyncOwnerChannelRfReplay {
    pub parent_state: ParentRfIntegrationState,
    pub standing_reads: Vec<GenerationStamped<AncestorStandingPolicyView>>,
}

/// Replay both directions from the one typed schedule. Missing rows hard-error; replay never
/// guesses from ambient arrival order or consults a second injection log.
pub fn replay_async_owner_channel_rf_seam(
    schedule: &IntegrationSchedule,
    reduce_up_products: &[StampedReduceUpProduct],
    standing_views: &[GenerationStamped<AncestorStandingPolicyView>],
) -> Result<AsyncOwnerChannelRfReplay, IntegrateError> {
    if schedule.entries().is_empty()
        && (!reduce_up_products.is_empty() || !standing_views.is_empty())
    {
        return Err(IntegrateError::MissingSchedule);
    }
    let mut parent_state = ParentRfIntegrationState::default();
    let mut standing_reads = Vec::new();
    for entry in schedule.entries() {
        match entry.row_kind() {
            IntegrationScheduleRowKind::DirectReduceUp => {
                let product = reduce_up_products.iter().find(|product| {
                    product.generation() == entry.child_generation
                        && reduce_up_product_key(product.product()) == entry.product_key
                });
                let Some(product) = product else {
                    return Err(IntegrateError::MissingRecordedProduct {
                        kind: entry.row_kind(),
                        source_generation: entry.child_generation.get(),
                        product_key: entry.product_key,
                    });
                };
                let mut scratch = IntegrationSchedule::new();
                integrate_stamped_reduce_up(
                    entry.parent_generation,
                    product,
                    &mut parent_state,
                    &mut scratch,
                )?;
            }
            IntegrationScheduleRowKind::QueueInjection => {
                let found = reduce_up_products.iter().find(|product| {
                    product.generation() == entry.child_generation
                        && queued_reduce_up_product_key(product.product()) == entry.product_key
                });
                let Some(product) = found else {
                    return Err(IntegrateError::MissingRecordedProduct {
                        kind: entry.row_kind(),
                        source_generation: entry.child_generation.get(),
                        product_key: entry.product_key,
                    });
                };
                apply_report_exact(&mut parent_state, product.product())?;
                parent_state.product_count = parent_state
                    .product_count
                    .checked_add(1)
                    .ok_or(IntegrateError::ArithmeticOverflow)?;
                fold_queue_schedule_row(
                    &mut parent_state,
                    entry.parent_generation,
                    entry.child_generation,
                    entry.product_key,
                );
            }
            IntegrationScheduleRowKind::StandingView => {
                let found = standing_views.iter().find(|view| {
                    view.generation() == entry.child_generation
                        && view.product().product_key().ok() == Some(entry.product_key)
                });
                let Some(view) = found else {
                    return Err(IntegrateError::MissingRecordedProduct {
                        kind: entry.row_kind(),
                        source_generation: entry.child_generation.get(),
                        product_key: entry.product_key,
                    });
                };
                standing_reads.push(view.clone());
            }
            IntegrationScheduleRowKind::ResidencyPlacementCommit
            | IntegrationScheduleRowKind::ResidencyPlacementRefusal
            | IntegrationScheduleRowKind::ResidencyRelocation
            | IntegrationScheduleRowKind::ResidencyCommittedCorruption
            | IntegrationScheduleRowKind::GrowthEntitlementRefusal
            | IntegrationScheduleRowKind::ResidentClearingProduct
            | IntegrationScheduleRowKind::GrantAccepted
            | IntegrationScheduleRowKind::GrantRenewed
            | IntegrationScheduleRowKind::GrantRevoked
            | IntegrationScheduleRowKind::GrantPartitioned
            | IntegrationScheduleRowKind::GrantTransferred
            | IntegrationScheduleRowKind::GrantReleased => {
                // The canonical schedule is shared across boundary products. Residency rows
                // carry no owner-channel RF product and therefore do not participate in this
                // seam's reduce-up/standing replay.
            }
        }
    }
    Ok(AsyncOwnerChannelRfReplay {
        parent_state,
        standing_reads,
    })
}

fn apply_report_exact(
    state: &mut ParentRfIntegrationState,
    report: &OwnerChannelRfReduceUpReport,
) -> Result<(), IntegrateError> {
    state.surplus_total = state
        .surplus_total
        .checked_add(u64::from(report.surplus_total))
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    state.deficit_total = state
        .deficit_total
        .checked_add(u64::from(report.deficit_total))
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    for bucket in &report.buckets {
        let value = OwnerChannelRfConservedValue::from_bucket(bucket);
        let entry = state.buckets.entry(bucket.scope.clone()).or_default();
        *entry = entry
            .checked_add(value)
            .ok_or(IntegrateError::ArithmeticOverflow)?;
    }
    Ok(())
}

fn add_queue_value_exact(
    state: &mut ParentRfIntegrationState,
    scope: &OwnerChannelScopeKey,
    value: OwnerChannelRfConservedValue,
) -> Result<(), IntegrateError> {
    state.surplus_total = state
        .surplus_total
        .checked_add(value.surplus_total)
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    state.deficit_total = state
        .deficit_total
        .checked_add(value.deficit_total)
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    let entry = state.buckets.entry(scope.clone()).or_default();
    *entry = entry
        .checked_add(value)
        .ok_or(IntegrateError::ArithmeticOverflow)?;
    Ok(())
}

fn fold_queue_schedule_row(
    state: &mut ParentRfIntegrationState,
    parent_generation: GenerationStamp,
    child_generation: GenerationStamp,
    product_key: u64,
) {
    state.schedule_fold = state
        .schedule_fold
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(0x5155_4555_4549_4E4A)
        .wrapping_add(product_key)
        .wrapping_add(child_generation.get() as u64)
        .wrapping_add(parent_generation.get() as u64);
}

fn queued_reduce_up_product_key(report: &OwnerChannelRfReduceUpReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for bucket in &report.buckets {
        for bytes in [
            bucket.scope.owner_ref.as_str().as_bytes(),
            bucket.scope.resource_key.as_str().as_bytes(),
            bucket.scope.scope_id.as_str().as_bytes(),
        ] {
            hash_u64(&mut hash, bytes.len() as u64);
            for byte in bytes {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        for value in [
            bucket.participant_count,
            bucket.surplus_total,
            bucket.deficit_total,
            bucket.net_surplus,
            bucket.net_deficit,
        ] {
            hash_u64(&mut hash, u64::from(value));
        }
    }
    hash
}

fn hash_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= u64::from(byte);
        *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerChannelRfErrorKind {
    InvalidOwnerAuthority,
    DuplicateOwnAggregate,
    UnknownSimThing,
    ArithmeticOverflow,
    GpuExactnessExceeded,
    DuplicateCrossing,
    InvalidCrossingSurface,
    ReconstructionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfError {
    pub kind: OwnerChannelRfErrorKind,
    pub simthing_id: Option<SimThingId>,
    pub resource_key: Option<ResourceKey>,
    pub message: String,
}

#[derive(Debug, Default)]
struct BucketAccumulator {
    source_row_indices: Vec<usize>,
    participant_count: u32,
    surplus_total: u32,
    deficit_total: u32,
}

/// Reduce arbitrary tree-local RF aggregates at intrinsic ownership boundaries
/// and **return a production-stamped product** for the producing tree's generation.
///
/// The ordered key performs all segregation.  There is no one-owner-per-container admission
/// rule and no owner-equality control-flow branch in aggregation.
///
/// EVENT-GENERATION-STAMP-0: the stamp rides this existing reduce-up product. Products
/// that leave this door for parent integration are always [`StampedReduceUpProduct`].
pub fn reduce_owner_channel_rf(
    root: &SimThing,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
    generation: GenerationStamp,
) -> Result<StampedReduceUpProduct, OwnerChannelRfError> {
    let report = reduce_owner_channel_rf_unstamped(root, own_aggregates)?;
    Ok(GenerationStamped::stamp(generation, report))
}

/// Internal unstamped aggregation. Prefer [`reduce_owner_channel_rf`] for any product
/// that will cross a parent seam.
pub fn reduce_owner_channel_rf_unstamped(
    root: &SimThing,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
) -> Result<OwnerChannelRfReduceUpReport, OwnerChannelRfError> {
    let own_aggregates = canonical_own_aggregates(own_aggregates)?;
    let rows_by_node = rows_by_node(&own_aggregates);
    let resolved: BTreeMap<SimThingId, OwnerRef> = resolve_owners_in_order(root)
        .map_err(owner_authority_error)?
        .into_iter()
        .collect();

    let mut visited_rows = BTreeSet::new();
    let mut bucket_map = BTreeMap::<OwnerChannelScopeKey, BucketAccumulator>::new();
    let mut crossing_flows = Vec::new();
    let root_owner = resolved.get(&root.id).cloned().ok_or_else(|| {
        error(
            OwnerChannelRfErrorKind::InvalidOwnerAuthority,
            Some(root.id),
            None,
            "owner resolution omitted the authority-tree root",
        )
    })?;
    let root_scope = ScopeId::from_boundary(root.id);

    reduce_tree(
        root,
        true,
        &root_owner,
        &root_scope,
        &resolved,
        &rows_by_node,
        &own_aggregates,
        &mut visited_rows,
        &mut bucket_map,
        &mut crossing_flows,
    )?;

    if visited_rows.len() != own_aggregates.len() {
        let (index, row) = own_aggregates
            .iter()
            .enumerate()
            .find(|(index, _)| !visited_rows.contains(index))
            .expect("an unvisited row must exist");
        let _ = index;
        return Err(error(
            OwnerChannelRfErrorKind::UnknownSimThing,
            Some(row.simthing_id),
            Some(row.resource_key.clone()),
            "own aggregate references a SimThing outside the tree",
        ));
    }

    let buckets = finish_buckets(bucket_map)?;
    attach_crossing_resource_flows(&buckets, &mut crossing_flows);
    crossing_flows.sort_by(|a, b| {
        (&a.scope_id, a.boundary_simthing_id).cmp(&(&b.scope_id, b.boundary_simthing_id))
    });

    let (surplus_total, deficit_total) = totals_from_own_aggregates(&own_aggregates)?;
    let participant_count = u32::try_from(
        own_aggregates
            .iter()
            .map(|row| row.simthing_id)
            .collect::<BTreeSet<_>>()
            .len(),
    )
    .map_err(|_| {
        error(
            OwnerChannelRfErrorKind::ArithmeticOverflow,
            None,
            None,
            "participant count exceeds u32",
        )
    })?;
    let owners = buckets
        .iter()
        .map(|bucket| bucket.scope.owner_ref.clone())
        .collect::<BTreeSet<_>>();
    let stead = OwnerChannelRfSteadSurface {
        own_aggregates,
        crossing_flows,
    };

    let reconstructed = reconstruct_owner_channel_rf_map(root, &stead)?;
    if reconstructed != buckets {
        return Err(error(
            OwnerChannelRfErrorKind::ReconstructionMismatch,
            None,
            None,
            "crossing-flow plus own-aggregate STEAD surface did not reconstruct reduce-up buckets",
        ));
    }

    Ok(OwnerChannelRfReduceUpReport {
        participant_count,
        owner_count: owners.len() as u32,
        bucket_count: buckets.len() as u32,
        surplus_total,
        deficit_total,
        buckets,
        stead,
    })
}

/// Reconstruct the complete owner/resource RF map from the bounded STEAD observation.
///
/// Only the root owner is resolved from the live tree.  Every descendant inherits it unless a
/// retained crossing row changes the execution boundary.  This is the independent proof that
/// identity-edge flow rows are unnecessary.
pub fn reconstruct_owner_channel_rf_map(
    root: &SimThing,
    stead: &OwnerChannelRfSteadSurface,
) -> Result<Vec<OwnerChannelRfBucket>, OwnerChannelRfError> {
    let own_aggregates = canonical_own_aggregates(&stead.own_aggregates)?;
    let rows_by_node = rows_by_node(&own_aggregates);
    let crossing_by_node = canonical_crossings(&stead.crossing_flows)?;
    let mut visited_rows = BTreeSet::new();
    let mut visited_crossings = BTreeSet::new();
    let mut bucket_map = BTreeMap::<OwnerChannelScopeKey, BucketAccumulator>::new();
    let root_owner = resolve_owner(root, root.id).map_err(owner_authority_error)?;
    let root_scope = ScopeId::from_boundary(root.id);

    reconstruct_tree(
        root,
        true,
        &root_owner,
        &root_scope,
        &rows_by_node,
        &own_aggregates,
        &crossing_by_node,
        &mut visited_rows,
        &mut visited_crossings,
        &mut bucket_map,
    )?;

    if visited_rows.len() != own_aggregates.len() {
        let row = own_aggregates
            .iter()
            .enumerate()
            .find(|(index, _)| !visited_rows.contains(index))
            .map(|(_, row)| row)
            .expect("an unvisited row must exist");
        return Err(error(
            OwnerChannelRfErrorKind::UnknownSimThing,
            Some(row.simthing_id),
            Some(row.resource_key.clone()),
            "STEAD own aggregate references a SimThing outside the tree",
        ));
    }
    if visited_crossings.len() != crossing_by_node.len() {
        return Err(error(
            OwnerChannelRfErrorKind::InvalidCrossingSurface,
            None,
            None,
            "STEAD crossing references a SimThing outside the tree",
        ));
    }

    let buckets = finish_buckets(bucket_map)?;
    validate_crossing_resource_flows(&buckets, &stead.crossing_flows)?;
    Ok(buckets)
}

#[allow(clippy::too_many_arguments)]
fn reduce_tree(
    node: &SimThing,
    is_root: bool,
    parent_owner: &OwnerRef,
    parent_scope: &ScopeId,
    resolved: &BTreeMap<SimThingId, OwnerRef>,
    rows_by_node: &BTreeMap<SimThingId, Vec<usize>>,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
    visited_rows: &mut BTreeSet<usize>,
    bucket_map: &mut BTreeMap<OwnerChannelScopeKey, BucketAccumulator>,
    crossing_flows: &mut Vec<OwnerChannelRfCrossingFlow>,
) -> Result<(), OwnerChannelRfError> {
    let owner = resolved.get(&node.id).ok_or_else(|| {
        error(
            OwnerChannelRfErrorKind::UnknownSimThing,
            Some(node.id),
            None,
            "owner resolution omitted a tree node",
        )
    })?;
    let crossing = !is_root && owner != parent_owner;
    let scope = if crossing {
        ScopeId::from_boundary(node.id)
    } else {
        parent_scope.clone()
    };

    if crossing {
        crossing_flows.push(OwnerChannelRfCrossingFlow {
            boundary_simthing_id: node.id,
            parent_scope_id: parent_scope.clone(),
            scope_id: scope.clone(),
            owner_ref: owner.clone(),
            resources: Vec::new(),
        });
    }

    add_node_rows(
        node.id,
        owner,
        &scope,
        rows_by_node,
        own_aggregates,
        visited_rows,
        bucket_map,
    )?;

    for child in &node.children {
        reduce_tree(
            child,
            false,
            owner,
            &scope,
            resolved,
            rows_by_node,
            own_aggregates,
            visited_rows,
            bucket_map,
            crossing_flows,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn reconstruct_tree(
    node: &SimThing,
    is_root: bool,
    inherited_owner: &OwnerRef,
    inherited_scope: &ScopeId,
    rows_by_node: &BTreeMap<SimThingId, Vec<usize>>,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
    crossing_by_node: &BTreeMap<SimThingId, &OwnerChannelRfCrossingFlow>,
    visited_rows: &mut BTreeSet<usize>,
    visited_crossings: &mut BTreeSet<SimThingId>,
    bucket_map: &mut BTreeMap<OwnerChannelScopeKey, BucketAccumulator>,
) -> Result<(), OwnerChannelRfError> {
    let (owner, scope) = if is_root {
        (inherited_owner.clone(), inherited_scope.clone())
    } else if let Some(crossing) = crossing_by_node.get(&node.id) {
        if crossing.parent_scope_id != *inherited_scope {
            return Err(error(
                OwnerChannelRfErrorKind::InvalidCrossingSurface,
                Some(node.id),
                None,
                "crossing parent scope does not match inherited execution scope",
            ));
        }
        if crossing.scope_id != ScopeId::from_boundary(node.id) {
            return Err(error(
                OwnerChannelRfErrorKind::InvalidCrossingSurface,
                Some(node.id),
                None,
                "crossing scope is not the canonical boundary-node execution scope",
            ));
        }
        visited_crossings.insert(node.id);
        (crossing.owner_ref.clone(), crossing.scope_id.clone())
    } else {
        (inherited_owner.clone(), inherited_scope.clone())
    };

    add_node_rows(
        node.id,
        &owner,
        &scope,
        rows_by_node,
        own_aggregates,
        visited_rows,
        bucket_map,
    )?;

    for child in &node.children {
        reconstruct_tree(
            child,
            false,
            &owner,
            &scope,
            rows_by_node,
            own_aggregates,
            crossing_by_node,
            visited_rows,
            visited_crossings,
            bucket_map,
        )?;
    }
    Ok(())
}

fn add_node_rows(
    node_id: SimThingId,
    owner: &OwnerRef,
    scope: &ScopeId,
    rows_by_node: &BTreeMap<SimThingId, Vec<usize>>,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
    visited_rows: &mut BTreeSet<usize>,
    bucket_map: &mut BTreeMap<OwnerChannelScopeKey, BucketAccumulator>,
) -> Result<(), OwnerChannelRfError> {
    let Some(indices) = rows_by_node.get(&node_id) else {
        return Ok(());
    };
    for &index in indices {
        let row = &own_aggregates[index];
        visited_rows.insert(index);
        let key = OwnerChannelScopeKey {
            owner_ref: owner.clone(),
            resource_key: row.resource_key.clone(),
            scope_id: scope.clone(),
        };
        let entry = bucket_map.entry(key).or_default();
        entry.source_row_indices.push(index);
        entry.participant_count =
            checked_add(entry.participant_count, 1, row, "bucket participant count")?;
        entry.surplus_total = checked_add(
            entry.surplus_total,
            row.surplus,
            row,
            "bucket surplus total",
        )?;
        entry.deficit_total = checked_add(
            entry.deficit_total,
            row.deficit,
            row,
            "bucket deficit total",
        )?;
        ensure_gpu_exact(entry.surplus_total, row, "bucket surplus total")?;
        ensure_gpu_exact(entry.deficit_total, row, "bucket deficit total")?;
    }
    Ok(())
}

fn canonical_own_aggregates(
    rows: &[OwnerChannelRfOwnAggregate],
) -> Result<Vec<OwnerChannelRfOwnAggregate>, OwnerChannelRfError> {
    let mut rows = rows.to_vec();
    rows.sort();
    for pair in rows.windows(2) {
        if pair[0].simthing_id == pair[1].simthing_id
            && pair[0].resource_key == pair[1].resource_key
        {
            return Err(error(
                OwnerChannelRfErrorKind::DuplicateOwnAggregate,
                Some(pair[1].simthing_id),
                Some(pair[1].resource_key.clone()),
                "a node may contribute only one own aggregate per resource",
            ));
        }
    }
    for row in &rows {
        ensure_gpu_exact(row.surplus, row, "own surplus")?;
        ensure_gpu_exact(row.deficit, row, "own deficit")?;
    }
    Ok(rows)
}

fn rows_by_node(rows: &[OwnerChannelRfOwnAggregate]) -> BTreeMap<SimThingId, Vec<usize>> {
    let mut out = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        out.entry(row.simthing_id)
            .or_insert_with(Vec::new)
            .push(index);
    }
    out
}

fn canonical_crossings<'a>(
    crossings: &'a [OwnerChannelRfCrossingFlow],
) -> Result<BTreeMap<SimThingId, &'a OwnerChannelRfCrossingFlow>, OwnerChannelRfError> {
    let mut out = BTreeMap::new();
    for crossing in crossings {
        if out
            .insert(crossing.boundary_simthing_id, crossing)
            .is_some()
        {
            return Err(error(
                OwnerChannelRfErrorKind::DuplicateCrossing,
                Some(crossing.boundary_simthing_id),
                None,
                "a boundary may retain only one ownership crossing",
            ));
        }
    }
    Ok(out)
}

fn finish_buckets(
    bucket_map: BTreeMap<OwnerChannelScopeKey, BucketAccumulator>,
) -> Result<Vec<OwnerChannelRfBucket>, OwnerChannelRfError> {
    bucket_map
        .into_iter()
        .map(|(scope, acc)| {
            let (net_surplus, net_deficit) = if acc.surplus_total >= acc.deficit_total {
                (acc.surplus_total - acc.deficit_total, 0)
            } else {
                (0, acc.deficit_total - acc.surplus_total)
            };
            ensure_gpu_exact_for_key(acc.surplus_total, &scope, "bucket surplus total")?;
            ensure_gpu_exact_for_key(acc.deficit_total, &scope, "bucket deficit total")?;
            Ok(OwnerChannelRfBucket {
                scope,
                source_row_indices: acc.source_row_indices,
                participant_count: acc.participant_count,
                surplus_total: acc.surplus_total,
                deficit_total: acc.deficit_total,
                net_surplus,
                net_deficit,
            })
        })
        .collect()
}

fn attach_crossing_resource_flows(
    buckets: &[OwnerChannelRfBucket],
    crossings: &mut [OwnerChannelRfCrossingFlow],
) {
    let mut by_boundary =
        BTreeMap::<(OwnerRef, ScopeId), Vec<OwnerChannelRfCrossingResourceFlow>>::new();
    for bucket in buckets {
        by_boundary
            .entry((
                bucket.scope.owner_ref.clone(),
                bucket.scope.scope_id.clone(),
            ))
            .or_default()
            .push(OwnerChannelRfCrossingResourceFlow {
                resource_key: bucket.scope.resource_key.clone(),
                participant_count: bucket.participant_count,
                surplus_total: bucket.surplus_total,
                deficit_total: bucket.deficit_total,
            });
    }
    for crossing in crossings {
        crossing.resources = by_boundary
            .get(&(crossing.owner_ref.clone(), crossing.scope_id.clone()))
            .cloned()
            .unwrap_or_default();
    }
}

fn validate_crossing_resource_flows(
    buckets: &[OwnerChannelRfBucket],
    crossings: &[OwnerChannelRfCrossingFlow],
) -> Result<(), OwnerChannelRfError> {
    let mut expected = crossings
        .iter()
        .map(|crossing| {
            (
                (crossing.owner_ref.clone(), crossing.scope_id.clone()),
                Vec::<OwnerChannelRfCrossingResourceFlow>::new(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for bucket in buckets {
        if let Some(resources) = expected.get_mut(&(
            bucket.scope.owner_ref.clone(),
            bucket.scope.scope_id.clone(),
        )) {
            resources.push(OwnerChannelRfCrossingResourceFlow {
                resource_key: bucket.scope.resource_key.clone(),
                participant_count: bucket.participant_count,
                surplus_total: bucket.surplus_total,
                deficit_total: bucket.deficit_total,
            });
        }
    }
    for crossing in crossings {
        let resource_flows = expected
            .get(&(crossing.owner_ref.clone(), crossing.scope_id.clone()))
            .expect("crossing seeded in expected map");
        if resource_flows != &crossing.resources {
            return Err(error(
                OwnerChannelRfErrorKind::InvalidCrossingSurface,
                Some(crossing.boundary_simthing_id),
                None,
                "retained crossing resource flow does not match reconstructed scope aggregate",
            ));
        }
    }
    Ok(())
}

fn totals_from_own_aggregates(
    rows: &[OwnerChannelRfOwnAggregate],
) -> Result<(u32, u32), OwnerChannelRfError> {
    let mut surplus = 0u32;
    let mut deficit = 0u32;
    for row in rows {
        surplus = checked_add(surplus, row.surplus, row, "conserved surplus total")?;
        deficit = checked_add(deficit, row.deficit, row, "conserved deficit total")?;
    }
    Ok((surplus, deficit))
}

fn checked_add(
    left: u32,
    right: u32,
    row: &OwnerChannelRfOwnAggregate,
    label: &str,
) -> Result<u32, OwnerChannelRfError> {
    left.checked_add(right).ok_or_else(|| {
        error(
            OwnerChannelRfErrorKind::ArithmeticOverflow,
            Some(row.simthing_id),
            Some(row.resource_key.clone()),
            format!("{label} overflow"),
        )
    })
}

fn ensure_gpu_exact(
    value: u32,
    row: &OwnerChannelRfOwnAggregate,
    label: &str,
) -> Result<(), OwnerChannelRfError> {
    if value > MAX_GPU_EXACT_INTEGER {
        return Err(error(
            OwnerChannelRfErrorKind::GpuExactnessExceeded,
            Some(row.simthing_id),
            Some(row.resource_key.clone()),
            format!("{label} exceeds the exact CPU/GPU integer range"),
        ));
    }
    Ok(())
}

fn ensure_gpu_exact_for_key(
    value: u32,
    key: &OwnerChannelScopeKey,
    label: &str,
) -> Result<(), OwnerChannelRfError> {
    if value > MAX_GPU_EXACT_INTEGER {
        return Err(error(
            OwnerChannelRfErrorKind::GpuExactnessExceeded,
            None,
            Some(key.resource_key.clone()),
            format!("{label} exceeds the exact CPU/GPU integer range"),
        ));
    }
    Ok(())
}

fn error(
    kind: OwnerChannelRfErrorKind,
    simthing_id: Option<SimThingId>,
    resource_key: Option<ResourceKey>,
    message: impl Into<String>,
) -> OwnerChannelRfError {
    OwnerChannelRfError {
        kind,
        simthing_id,
        resource_key,
        message: message.into(),
    }
}

fn owner_authority_error(
    error: simthing_core::owner_channel::OwnerResolutionError,
) -> OwnerChannelRfError {
    OwnerChannelRfError {
        kind: OwnerChannelRfErrorKind::InvalidOwnerAuthority,
        simthing_id: match &error {
            simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { target } => {
                Some(*target)
            }
            simthing_core::owner_channel::OwnerResolutionError::MalformedBinding {
                simthing_id,
                ..
            }
            | simthing_core::owner_channel::OwnerResolutionError::BlankBinding { simthing_id } => {
                Some(*simthing_id)
            }
        },
        resource_key: None,
        message: error.to_string(),
    }
}

#[cfg(test)]
mod wait_mutant_proof {
    use super::*;
    use simthing_core::owner_channel::{bind_owner, OwnerRef};
    use simthing_core::{SimThing, SimThingKind};

    fn node() -> SimThing {
        SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
    }

    #[test]
    fn make_the_parent_wait_mutant_reds_then_restores_green() {
        let mut root = node();
        bind_owner(&mut root, &OwnerRef::new("alpha"));
        let leaf = node();
        let leaf_id = leaf.id;
        root.add_child(leaf);
        let rows = vec![OwnerChannelRfOwnAggregate {
            simthing_id: leaf_id,
            resource_key: ResourceKey::new("ore"),
            surplus: 2,
            deficit: 0,
        }];
        let stamped =
            reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).expect("stamped");

        // Production path never waits.
        let mut schedule = IntegrationSchedule::new();
        let mut parent = ParentRfIntegrationState::default();
        integrate_stamped_reduce_up(
            GenerationStamp::new(4),
            &stamped,
            &mut parent,
            &mut schedule,
        )
        .expect("production integrate has no wait branch");

        // Test-only mutant REDs lagged integrate.
        plant_wait_for_fresh_child_mutant(true);
        let mut schedule2 = IntegrationSchedule::new();
        let mut parent2 = ParentRfIntegrationState::default();
        let err = integrate_stamped_reduce_up_for_wait_mutant_proof(
            GenerationStamp::new(4),
            &stamped,
            &mut parent2,
            &mut schedule2,
        )
        .expect_err("wait mutant must RED");
        assert!(matches!(
            err,
            IntegrateError::WouldWaitForLaggingChild {
                parent: 4,
                child: 1
            }
        ));
        plant_wait_for_fresh_child_mutant(false);
        integrate_stamped_reduce_up_for_wait_mutant_proof(
            GenerationStamp::new(4),
            &stamped,
            &mut parent2,
            &mut schedule2,
        )
        .expect("restored green");
    }
}

#[cfg(test)]
mod async_queue_accounting_mutant_proof {
    use super::*;

    fn same_key_product_at(generation: u32) -> StampedReduceUpProduct {
        GenerationStamped::stamp(
            GenerationStamp::new(generation),
            OwnerChannelRfReduceUpReport {
                participant_count: 1,
                owner_count: 1,
                bucket_count: 1,
                surplus_total: 2,
                deficit_total: 3,
                buckets: vec![OwnerChannelRfBucket {
                    scope: OwnerChannelScopeKey {
                        owner_ref: OwnerRef::new("synthetic-owner"),
                        resource_key: ResourceKey::new("synthetic-resource"),
                        scope_id: ScopeId::new("synthetic-scope"),
                    },
                    source_row_indices: vec![0],
                    participant_count: 1,
                    surplus_total: 2,
                    deficit_total: 3,
                    net_surplus: 0,
                    net_deficit: 1,
                }],
                stead: OwnerChannelRfSteadSurface {
                    own_aggregates: Vec::new(),
                    crossing_flows: Vec::new(),
                },
            },
        )
    }

    fn seeded_seam() -> (AsyncOwnerChannelRfSeam, OwnerChannelScopeKey) {
        let scope = OwnerChannelScopeKey {
            owner_ref: OwnerRef::new("synthetic-owner"),
            resource_key: ResourceKey::new("synthetic-resource"),
            scope_id: ScopeId::new("synthetic-scope"),
        };
        let value = OwnerChannelRfConservedValue {
            participant_count: 1,
            surplus_total: 2,
            deficit_total: 3,
            net_surplus: 0,
            net_deficit: 1,
        };
        let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(1));
        seam.pending.insert(
            scope.clone(),
            PendingBucket {
                value,
                newest_generation: GenerationStamp::new(1),
            },
        );
        seam.balances.insert(
            scope.clone(),
            OwnerChannelRfSeamBalance {
                child: OwnerChannelRfConservedValue::default(),
                seam: value,
                parent: OwnerChannelRfConservedValue::default(),
                admitted: value,
            },
        );
        seam.pending_products.push(PendingContribution {
            generation: GenerationStamp::new(1),
            product_key: 1,
        });
        seam.admitted_product_count = 1;
        (seam, scope)
    }

    #[test]
    fn dropped_pending_product_mutant_reds() {
        let (mut seam, _) = seeded_seam();
        seam.check_conservation().expect("seed is exact");
        seam.pending_products.clear();
        assert!(matches!(
            seam.check_conservation().unwrap_err(),
            IntegrateError::ConservationViolation
        ));
    }

    #[test]
    fn in_flight_escape_from_all_three_accounts_mutant_reds() {
        let (mut seam, scope) = seeded_seam();
        seam.balances.get_mut(&scope).unwrap().seam = OwnerChannelRfConservedValue::default();
        assert!(matches!(
            seam.check_conservation().unwrap_err(),
            IntegrateError::ConservationViolation
        ));
    }

    #[test]
    fn coalesced_staleness_mutants_red_against_newest_carrier_law() {
        let (mut seam, scope) = seeded_seam();
        seam.tolerance = AuthoredSeamStaleness::new(3);
        seam.pending.get_mut(&scope).unwrap().value = OwnerChannelRfConservedValue {
            participant_count: 5,
            surplus_total: 10,
            deficit_total: 15,
            net_surplus: 0,
            net_deficit: 5,
        };
        seam.pending.get_mut(&scope).unwrap().newest_generation = GenerationStamp::new(5);
        seam.pending_products = (1..=5)
            .map(|generation| PendingContribution {
                generation: GenerationStamp::new(generation),
                product_key: u64::from(generation),
            })
            .collect();
        seam.admitted_product_count = 5;
        seam.balances.get_mut(&scope).unwrap().seam = seam.pending[&scope].value;
        seam.balances.get_mut(&scope).unwrap().admitted = seam.pending[&scope].value;
        seam.check_conservation().expect("coalesced seed is exact");

        assert!(matches!(
            seam.check_historical_contributors_for_staleness_mutant(GenerationStamp::new(8))
                .unwrap_err(),
            IntegrateError::StalenessToleranceExceeded {
                integration: 8,
                source_generation: 1,
                observed: 7,
                allowed: 3,
            }
        ));

        let mut parent = ParentRfIntegrationState::default();
        let mut schedule = IntegrationSchedule::new();
        seam.apply_parent_generation_barrier(GenerationStamp::new(8), &mut parent, &mut schedule)
            .expect("canonical newest/max carrier stamp 5 is within authored tolerance 3");
        assert_eq!(
            schedule
                .entries_of_kind(IntegrationScheduleRowKind::QueueInjection)
                .map(|entry| entry.child_generation.get())
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5]
        );

        let newer = same_key_product_at(5);
        let older = same_key_product_at(3);
        let mut canonical = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(3));
        canonical.enqueue_reduce_up(&newer).unwrap();
        canonical.enqueue_reduce_up(&older).unwrap();
        assert_eq!(
            canonical.pending_carriers()[0].generation(),
            GenerationStamp::new(5)
        );
        canonical
            .apply_parent_generation_barrier(
                GenerationStamp::new(8),
                &mut ParentRfIntegrationState::default(),
                &mut IntegrationSchedule::new(),
            )
            .expect("max-stamp production path admits 8 <- 5 at tolerance 3");

        let mut last_wins = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(3));
        last_wins
            .enqueue_reduce_up_last_wins_mutant(&newer)
            .unwrap();
        last_wins
            .enqueue_reduce_up_last_wins_mutant(&older)
            .unwrap();
        assert_eq!(
            last_wins.pending_carriers()[0].generation(),
            GenerationStamp::new(3),
            "planted last-wins carrier must differ from canonical max"
        );
        assert!(matches!(
            last_wins
                .apply_parent_generation_barrier(
                    GenerationStamp::new(8),
                    &mut ParentRfIntegrationState::default(),
                    &mut IntegrationSchedule::new(),
                )
                .unwrap_err(),
            IntegrateError::StalenessToleranceExceeded {
                integration: 8,
                source_generation: 3,
                observed: 5,
                allowed: 3,
            }
        ));
    }
}
