//! CONTINUOUS-POSTURE-SOAK-0 — N-generation forced-lag soak over landed 6.0–6.2 surfaces.
//!
//! Synthetic inline input only. No corpus, scenario tuning, or GPU device-loss path.

use std::collections::BTreeMap;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    cost_band_quantize, deliver_routed_overlay, eval_overlay_eml, AuthoredSeamStaleness,
    DimensionRegistry, ExecutionPosture, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SlotIndex, SubFieldRole, TransformOp,
};
use simthing_spec::{
    derive_staleness_f32, reconstruct_owner_channel_rf_map, reduce_owner_channel_rf,
    replay_async_owner_channel_rf_seam, AsyncOwnerChannelRfSeam, AsyncStalenessColumn,
    AuthoredStalenessHorizon, OwnerChannelRfOwnAggregate, OwnerChannelRfSteadSurface,
    ParentRfIntegrationState, ResourceKey,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn own(
    simthing_id: SimThingId,
    resource: &str,
    surplus: u32,
    deficit: u32,
) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new(resource),
        surplus,
        deficit,
    }
}

/// Scaling-matrix tree: chain of `nodes` with ownership flips every `crossing_every`.
fn scaling_tree(
    nodes: usize,
    crossing_every: usize,
    resources: &[&str],
) -> (SimThing, Vec<OwnerChannelRfOwnAggregate>, usize) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("owner-0"));
    let mut ids = vec![root.id];
    let mut cursor = &mut root;
    let mut owner_idx = 0u32;
    let mut crossing_count = 0usize;
    for depth in 1..nodes {
        let mut child = node();
        if crossing_every > 0 && depth % crossing_every == 0 {
            owner_idx += 1;
            bind_owner(&mut child, &OwnerRef::new(&format!("owner-{owner_idx}")));
            crossing_count += 1;
        }
        ids.push(child.id);
        cursor.add_child(child);
        cursor = cursor.children.last_mut().expect("child just added");
    }
    let rows = ids
        .iter()
        .flat_map(|&id| {
            resources
                .iter()
                .enumerate()
                .map(move |(ri, r)| own(id, r, ((ri as u32) + 1) % 5, ((ri as u32) * 2) % 3))
        })
        .collect();
    (root, rows, crossing_count)
}

fn slot_map(root: &SimThing) -> BTreeMap<SimThingId, SlotIndex> {
    let mut map = BTreeMap::new();
    let mut next = 0u32;
    fn walk(node: &SimThing, map: &mut BTreeMap<SimThingId, SlotIndex>, next: &mut u32) {
        map.insert(node.id, SlotIndex::new(*next));
        *next += 1;
        for child in &node.children {
            walk(child, map, next);
        }
    }
    walk(root, &mut map, &mut next);
    map
}

fn overlay_from_eml(origin: SimThingId, n: f32) -> Overlay {
    let op = TransformOp::add(n);
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(1),
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        },
        lifecycle: OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
        },
    }
}

