//! OWNER-CHANNEL-INTRINSIC-0 (rung 6.0) — deliverables (a) and (b).
//!
//! Input is constructed INLINE. Ownership resolution is a law over ARBITRARY trees, so the
//! smallest tree exhibiting each property is a complete witness; no scenario, corpus, or
//! shipped fixture is required or permitted (Transient Fixture Law).

use simthing_core::owner_channel::{
    bind_owner, declared_owner, is_ownership_crossing, resolve_owner, resolve_owners_in_order,
    unbind_owner, unowned, OwnerRef, OWNER_CHANNEL_PROPERTY_ID,
};
use simthing_core::simthing::{SimThing, SimThingKind};

use simthing_core::ids::SimThingId;

fn node() -> SimThing {
    // Ids are auto-assigned by SimThing::new; the u32 argument is the spawned generation.
    SimThing::new(SimThingKind::Location, 0)
}

/// root -> mid -> leaf, three levels, nothing bound. Ids returned in depth order.
fn chain() -> (SimThing, [SimThingId; 3]) {
    let mut root = node();
    let mut mid = node();
    let leaf = node();
    let ids = [root.id, mid.id, leaf.id];
    mid.add_child(leaf);
    root.add_child(mid);
    (root, ids)
}

// ---------------------------------------------------------------- (b) totality

#[test]
fn resolution_is_total_and_bottoms_out_at_the_neutral_owner() {
    let (root, ids) = chain();
    for (n, id) in ids.iter().enumerate() {
        let owner = resolve_owner(&root, *id);
        assert!(
            owner.is_unowned(),
            "unbound node {n} must resolve to the neutral owner, got {owner:?}"
        );
    }
}

#[test]
fn resolution_is_total_even_for_a_node_outside_the_tree() {
    // Totality is the point: no Option, no error path for RF callers to handle.
    let (root, _) = chain();
    let foreign = node().id;
    assert!(resolve_owner(&root, foreign).is_unowned());
}

// ------------------------------------------------------- (b) inheritance depth

#[test]
fn inheritance_reaches_through_three_levels_without_stamping_descendants() {
    let (mut root, ids) = chain();
    bind_owner(&mut root, &OwnerRef::new("alpha"));

    for (n, id) in ids.iter().enumerate() {
        assert_eq!(
            resolve_owner(&root, *id).as_str(),
            "alpha",
            "node {n} must inherit through the chain"
        );
    }

    // NEVER MATERIALIZED: only the bound root carries the property. If resolution ever
    // stamps its answer, this is the assertion that catches it.
    let mid = &root.children[0];
    let leaf = &mid.children[0];
    assert!(declared_owner(mid).is_none(), "mid must remain unbound");
    assert!(declared_owner(leaf).is_none(), "leaf must remain unbound");
}

#[test]
fn nearest_bound_ancestor_wins_over_a_more_distant_one() {
    let (mut root, ids) = chain();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    bind_owner(&mut root.children[0], &OwnerRef::new("beta"));

    assert_eq!(resolve_owner(&root, ids[0]).as_str(), "alpha");
    assert_eq!(resolve_owner(&root, ids[1]).as_str(), "beta");
    assert_eq!(
        resolve_owner(&root, ids[2]).as_str(),
        "beta",
        "leaf must follow its NEAREST bound ancestor, not the root"
    );
}

// ------------------------------------------------------------------- (b) fission

#[test]
fn fission_is_one_property_rebind_and_reparents_the_whole_subtree() {
    let (mut root, ids) = chain();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    assert_eq!(resolve_owner(&root, ids[2]).as_str(), "alpha");

    // ONE write at the subtree root.
    bind_owner(&mut root.children[0], &OwnerRef::new("beta"));

    assert_eq!(
        resolve_owner(&root, ids[2]).as_str(),
        "beta",
        "the descendant must re-parent with no descendant write"
    );
    assert!(
        declared_owner(&root.children[0].children[0]).is_none(),
        "fission must not touch descendants"
    );

    // And it is reversible in one write.
    unbind_owner(&mut root.children[0]);
    assert_eq!(resolve_owner(&root, ids[2]).as_str(), "alpha");
}

// ------------------------------------------------------------- (a) inert by default

#[test]
fn inert_simthings_store_zero_owner_bytes() {
    let (root, _) = chain();
    fn count_bound(n: &SimThing) -> usize {
        usize::from(n.property(OWNER_CHANNEL_PROPERTY_ID).is_some())
            + n.children.iter().map(count_bound).sum::<usize>()
    }
    assert_eq!(
        count_bound(&root),
        0,
        "an unbound tree must carry no owner property at all"
    );

    let (mut bound, _) = chain();
    bind_owner(&mut bound, &OwnerRef::new("alpha"));
    assert_eq!(
        count_bound(&bound),
        1,
        "binding a 3-node tree must cost exactly ONE property, not three"
    );
}

// ------------------------------------------- (c) single ownership, multi-owner containers

#[test]
fn one_container_admits_many_owners_among_its_children() {
    // The adversarial case: two differently-owned cohorts under one spatial container.
    let mut star = node();
    let mut ally = node();
    let mut enemy = node();
    bind_owner(&mut ally, &OwnerRef::new("alpha"));
    bind_owner(&mut enemy, &OwnerRef::new("beta"));
    let (star_id, ally_id, enemy_id) = (star.id, ally.id, enemy.id);
    star.add_child(ally);
    star.add_child(enemy);

    assert!(resolve_owner(&star, star_id).is_unowned());
    assert_eq!(resolve_owner(&star, ally_id).as_str(), "alpha");
    assert_eq!(resolve_owner(&star, enemy_id).as_str(), "beta");

    // Each SimThing resolves to exactly one owner; the CONTAINER hosts two. Nothing in
    // resolution rejects this, and nothing may: uniformity would make contention impossible.
    let resolved = resolve_owners_in_order(&star);
    let distinct: std::collections::BTreeSet<_> =
        resolved.iter().map(|(_, o)| o.as_str().to_string()).collect();
    assert_eq!(distinct.len(), 3, "unowned + alpha + beta all coexist");
}

// ----------------------------------------------------------------- (e) crossings

#[test]
fn crossings_are_exactly_the_edges_where_ownership_changes() {
    let mut star = node();
    let mut same = node();
    let mut other = node();
    bind_owner(&mut star, &OwnerRef::new("alpha"));
    bind_owner(&mut same, &OwnerRef::new("alpha"));
    bind_owner(&mut other, &OwnerRef::new("beta"));
    star.add_child(same);
    star.add_child(other);

    let star_owner = OwnerRef::new("alpha");
    assert!(
        !is_ownership_crossing(&star.children[0], &star_owner),
        "same-owner child is NOT a crossing and must not be recorded"
    );
    assert!(
        is_ownership_crossing(&star.children[1], &star_owner),
        "different-owner child IS a crossing"
    );
}

#[test]
fn ownership_flip_is_an_ordinary_rebind_not_a_none_transition() {
    // Neutral ground taken by an owned unit: unowned -> alpha, no special case.
    let mut star = node();
    assert!(resolve_owner(&star, star.id).is_unowned());
    bind_owner(&mut star, &OwnerRef::new("alpha"));
    assert_eq!(resolve_owner(&star, star.id).as_str(), "alpha");
    bind_owner(&mut star, &unowned());
    assert!(
        resolve_owner(&star, star.id).is_unowned(),
        "flipping back is the same operation in reverse"
    );
}
