//! ASYNC-COMMAND-QUEUE-0 — synthetic seam, conservation, and replay proofs.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    capture_ancestor_standing_policy, AncestorStandingPolicyView, AuthoredSeamStaleness,
    GenerationStamp, GenerationStamped, IntegrateError, IntegrationSchedule,
    IntegrationScheduleRowKind, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_spec::{
    reduce_owner_channel_rf, replay_async_owner_channel_rf_seam, AsyncOwnerChannelRfSeam,
    OwnerChannelRfOwnAggregate, ParentRfIntegrationState, ResourceKey,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn source_tree() -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let inherited = node();
    let inherited_id = inherited.id;
    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    root.add_child(inherited);
    root.add_child(crossing);
    let rows = vec![
        OwnerChannelRfOwnAggregate {
            simthing_id: inherited_id,
            resource_key: ResourceKey::new("resource-a"),
            surplus: 7,
            deficit: 2,
        },
        OwnerChannelRfOwnAggregate {
            simthing_id: crossing_id,
            resource_key: ResourceKey::new("resource-a"),
            surplus: 3,
            deficit: 5,
        },
    ];
    (root, rows)
}

fn product_at(generation: u32) -> GenerationStamped<simthing_spec::OwnerChannelRfReduceUpReport> {
    let (root, rows) = source_tree();
    reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(generation)).expect("reduce-up")
}

fn products_at(
    generations: impl IntoIterator<Item = u32>,
) -> Vec<GenerationStamped<simthing_spec::OwnerChannelRfReduceUpReport>> {
    let (root, rows) = source_tree();
    generations
        .into_iter()
        .map(|generation| {
            reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(generation))
                .expect("reduce-up")
        })
        .collect()
}

fn policy(origin: SimThingId, amount: f32) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(77),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(amount))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

