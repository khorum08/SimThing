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
    integrate_stamped_product, IntegrateError, GenerationStamp, GenerationStamped,
    IntegrationReceipt, IntegrationSchedule, SimThing, SimThingId,
};
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
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub product_count: u32,
    /// Fold of product keys in schedule order (bit-exact replay witness).
    pub schedule_fold: u64,
}

/// Planted wait mutant: when true, integration rejects any child generation that
/// differs from the parent (synchronous freshness gate). The ordinary path never
/// sets this; tests enable it to prove N+3 <- N turns RED.
static WAIT_FOR_FRESH_CHILD_MUTANT: AtomicBool = AtomicBool::new(false);

/// Enable/disable the make-the-parent-wait mutant (test-only plant).
pub fn plant_wait_for_fresh_child_mutant(enabled: bool) {
    WAIT_FOR_FRESH_CHILD_MUTANT.store(enabled, Ordering::SeqCst);
}

/// Integrate a stamped reduce-up product into parent RF state at the parent generation.
///
/// Async is ordinary: parent at N+3 integrating child gen-N completes with no wait
/// (unless the planted wait mutant is enabled). Records the schedule for bit-exact
/// replay of integrated state. Staleness is visible.
pub fn integrate_stamped_reduce_up(
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
    let report = product.product();
    let key = reduce_up_product_key(report);
    let receipt = integrate_stamped_product(parent_generation, product, key, schedule);
    // Apply RF product into parent state (production integration semantics).
    parent_state.surplus_total = parent_state
        .surplus_total
        .saturating_add(report.surplus_total);
    parent_state.deficit_total = parent_state
        .deficit_total
        .saturating_add(report.deficit_total);
    parent_state.product_count = parent_state.product_count.saturating_add(1);
    parent_state.schedule_fold = parent_state
        .schedule_fold
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(key)
        .wrapping_add(product.generation().get() as u64)
        .wrapping_add(parent_generation.get() as u64);
    Ok(receipt)
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
    for entry in schedule.entries() {
        let found = products.iter().find(|p| {
            p.generation() == entry.child_generation
                && reduce_up_product_key(p.product()) == entry.product_key
        });
        let Some(product) = found else {
            continue;
        };
        integrate_stamped_reduce_up(
            entry.parent_generation,
            product,
            &mut state,
            &mut scratch,
        )?;
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
