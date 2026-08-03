//! SIMTHING-AUTOMATON-INTRINSIC-0 reception/origination contract.
//!
//! No inbox, queue, listener registry, transport, or scheduler lives here. Delivery
//! terminates in the existing [`SimThing::overlays`] vector, standing reception uses
//! ordinary ancestor inheritance, and predicate broadcast is one paid tree walk.

use std::collections::HashSet;

use thiserror::Error;

use crate::evaluate::{RoutedPredicate, TransformStack};
use crate::overlay::{Overlay, OverlayKind, PropertyTransformDelta};
use crate::property::{SubFieldRole, TransformOp};
use crate::{
    DimensionRegistry, OverlayId, PlacedParticipant, SimThing, SimThingId, StructuralCoord,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectiveDeliveryReceipt {
    pub overlay_id: OverlayId,
    pub origin: SimThingId,
    pub target: SimThingId,
    /// Ordered origin -> common ancestor -> target route.
    pub route: Vec<SimThingId>,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum OverlayDeliveryError {
    #[error("overlay origin {origin:?} is not a member of the supplied authority tree")]
    OriginNotInTree { origin: SimThingId },
    #[error("overlay target {target:?} is not a member of the supplied authority tree")]
    TargetNotInTree { target: SimThingId },
    #[error(
        "predicate-broadcast origin argument {argument:?} differs from overlay origin {overlay:?}"
    )]
    OriginMismatch {
        argument: SimThingId,
        overlay: SimThingId,
    },
    #[error("policy route for property role {role:?} cannot be represented by the existing transform stack")]
    NonRepresentablePolicyRoute { role: SubFieldRole },
}

/// Deliver a consumed/deficit-driven directive through the existing tree path.
///
/// The overlay is stored only at `target`. Policy/governance transforms on the
/// origin -> common-ancestor -> target route are algebraically moved after the
/// directive, so the existing ancestor-first evaluator filters the directive
/// without a second stack or transport.
pub fn deliver_deficit_directive(
    root: &mut SimThing,
    target: SimThingId,
    mut overlay: Overlay,
) -> Result<DirectiveDeliveryReceipt, OverlayDeliveryError> {
    let origin_path =
        find_path(root, overlay.origin).ok_or(OverlayDeliveryError::OriginNotInTree {
            origin: overlay.origin,
        })?;
    let target_path =
        find_path(root, target).ok_or(OverlayDeliveryError::TargetNotInTree { target })?;
    let common_len = origin_path
        .iter()
        .zip(&target_path)
        .take_while(|(left, right)| left == right)
        .count();
    debug_assert!(common_len > 0, "both members share the supplied root");

    let mut route_paths: Vec<Vec<usize>> = origin_path[common_len - 1..]
        .iter()
        .rev()
        .cloned()
        .collect();
    route_paths.extend(target_path[common_len..].iter().cloned());
    let route: Vec<SimThingId> = route_paths
        .iter()
        .map(|path| node_at_path(root, path).expect("path resolved above").id)
        .collect();

    if matches!(overlay.kind, OverlayKind::Instruction) {
        overlay.transform =
            route_filtered_transform(root, &target_path, &route_paths, &overlay.transform)?;
    }
    overlay.affects.clear();
    overlay.affects.push(target);
    let receipt = DirectiveDeliveryReceipt {
        overlay_id: overlay.id,
        origin: overlay.origin,
        target,
        route,
    };
    node_at_path_mut(root, target_path.last().expect("non-empty root path"))
        .expect("target path resolved above")
        .add_overlay(overlay);
    Ok(receipt)
}

/// Install a non-consumed standing directive at one subtree root.
///
/// Descendants receive it through the evaluator's existing inherited
/// `TransformStack`; no descendant copy is materialized and no conservation
/// claim is introduced for this read-only mode.
pub fn deliver_standing_directive(
    root: &mut SimThing,
    subtree_root: SimThingId,
    overlay: Overlay,
) -> Result<DirectiveDeliveryReceipt, OverlayDeliveryError> {
    deliver_deficit_directive(root, subtree_root, overlay)
}

/// Paid push-by-predicate mode. Searches the origin's descendants exactly once,
/// installs one attributable overlay on each admitted receiver, and returns the
/// receiver receipts in deterministic pre-order.
pub fn deliver_predicate_broadcast(
    root: &mut SimThing,
    origin: SimThingId,
    template: &Overlay,
    predicate: &RoutedPredicate,
    registry: &DimensionRegistry,
) -> Result<Vec<DirectiveDeliveryReceipt>, OverlayDeliveryError> {
    if template.origin != origin {
        return Err(OverlayDeliveryError::OriginMismatch {
            argument: origin,
            overlay: template.origin,
        });
    }

    fn seek(
        node: &mut SimThing,
        ancestors: &TransformStack,
        origin: SimThingId,
        template: &Overlay,
        predicate: &RoutedPredicate,
        registry: &DimensionRegistry,
        receipts: &mut Vec<DirectiveDeliveryReceipt>,
    ) -> Result<bool, OverlayDeliveryError> {
        let stack = active_stack(node, ancestors);
        if node.id == origin {
            let mut route = vec![node.id];
            for child in &mut node.children {
                broadcast_subtree(
                    child, &stack, &mut route, template, predicate, registry, receipts,
                )?;
            }
            return Ok(true);
        }
        for child in &mut node.children {
            if seek(
                child, &stack, origin, template, predicate, registry, receipts,
            )? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    let mut receipts = Vec::new();
    if !seek(
        root,
        &TransformStack::default(),
        origin,
        template,
        predicate,
        registry,
        &mut receipts,
    )? {
        return Err(OverlayDeliveryError::OriginNotInTree { origin });
    }
    Ok(receipts)
}

fn broadcast_subtree(
    node: &mut SimThing,
    ancestors: &TransformStack,
    route: &mut Vec<SimThingId>,
    template: &Overlay,
    predicate: &RoutedPredicate,
    registry: &DimensionRegistry,
    receipts: &mut Vec<DirectiveDeliveryReceipt>,
) -> Result<(), OverlayDeliveryError> {
    let stack = active_stack(node, ancestors);
    route.push(node.id);

    let admitted = node.property(predicate.property_id).is_some_and(|value| {
        registry
            .try_property(predicate.property_id)
            .is_some_and(|property| {
                stack.allows_routed_predicate(predicate, value, &property.layout)
            })
    });

    // Build the child stack before adding the delivered instruction: the
    // broadcast selector is evaluated against pre-delivery state everywhere.
    let child_stack = stack.clone();
    if admitted {
        let mut overlay = template.clone();
        overlay.id = OverlayId::new();
        overlay.affects.clear();
        overlay.affects.push(node.id);
        if matches!(overlay.kind, OverlayKind::Instruction) {
            overlay.transform = stack_filtered_transform(&stack, &overlay.transform)?;
        }
        receipts.push(DirectiveDeliveryReceipt {
            overlay_id: overlay.id,
            origin: overlay.origin,
            target: node.id,
            route: route.to_vec(),
        });
        node.add_overlay(overlay);
    }

    for child in &mut node.children {
        broadcast_subtree(
            child,
            &child_stack,
            route,
            template,
            predicate,
            registry,
            receipts,
        )?;
    }
    route.pop();
    Ok(())
}

fn active_stack(node: &SimThing, ancestors: &TransformStack) -> TransformStack {
    node.overlays
        .iter()
        .filter(|overlay| overlay.is_active())
        .fold(ancestors.clone(), |stack, overlay| {
            stack.push_overlay(overlay)
        })
}

/// Derive an event's structural location from its required origin and already
/// validated placement proofs. No coordinate is copied onto the overlay.
pub fn overlay_origin_structural_coord(
    overlay: &Overlay,
    placed: &[PlacedParticipant],
) -> Option<StructuralCoord> {
    placed
        .iter()
        .copied()
        .find(|participant| participant.participant() == overlay.origin)
        .map(PlacedParticipant::coord)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Affine {
    mul: f32,
    add: f32,
}

impl Affine {
    const IDENTITY: Self = Self { mul: 1.0, add: 0.0 };

    fn from_op(op: &TransformOp) -> Self {
        match *op {
            TransformOp::Add(add) => Self { mul: 1.0, add },
            TransformOp::Multiply(mul) => Self { mul, add: 0.0 },
            TransformOp::Set(add) => Self { mul: 0.0, add },
        }
    }

    /// Apply `next` after `self`.
    fn then(self, next: Self) -> Self {
        Self {
            mul: next.mul * self.mul,
            add: next.mul * self.add + next.add,
        }
    }
}

fn route_filtered_transform(
    root: &SimThing,
    target_path: &[Vec<usize>],
    route_paths: &[Vec<usize>],
    directive: &PropertyTransformDelta,
) -> Result<PropertyTransformDelta, OverlayDeliveryError> {
    let route_nodes: HashSet<SimThingId> = route_paths
        .iter()
        .map(|path| node_at_path(root, path).expect("route path resolved").id)
        .collect();
    let existing: Vec<(SimThingId, &Overlay)> = target_path
        .iter()
        .flat_map(|path| {
            let node = node_at_path(root, path).expect("target path resolved");
            node.overlays
                .iter()
                .filter(|overlay| overlay.is_active())
                .map(move |overlay| (node.id, overlay))
        })
        .collect();
    let route_policies: Vec<&Overlay> = route_paths
        .iter()
        .flat_map(|path| {
            node_at_path(root, path)
                .expect("route path resolved")
                .overlays
                .iter()
                .filter(|overlay| overlay.is_active() && is_policy(overlay))
        })
        .collect();

    rewrite_transform(directive, |role| {
        let all_existing = affine_for_overlays(
            existing.iter().map(|(_, overlay)| *overlay),
            directive,
            role,
        );
        let without_route_policies = affine_for_overlays(
            existing.iter().filter_map(|(host, overlay)| {
                (!(route_nodes.contains(host) && is_policy(overlay))).then_some(*overlay)
            }),
            directive,
            role,
        );
        let policies = affine_for_overlays(route_policies.iter().copied(), directive, role);
        let instruction = affine_for_delta(directive, role);
        let desired = without_route_policies.then(instruction).then(policies);
        solve_suffix(all_existing, desired, role)
    })
}

fn stack_filtered_transform(
    stack: &TransformStack,
    directive: &PropertyTransformDelta,
) -> Result<PropertyTransformDelta, OverlayDeliveryError> {
    rewrite_transform(directive, |role| {
        let existing = affine_for_deltas(stack.entries().map(|(delta, _)| delta), directive, role);
        let unrestricted = affine_for_deltas(
            stack
                .entries()
                .filter_map(|(delta, restriction)| (!restriction).then_some(delta)),
            directive,
            role,
        );
        let restrictions = affine_for_deltas(
            stack
                .entries()
                .filter_map(|(delta, restriction)| restriction.then_some(delta)),
            directive,
            role,
        );
        let instruction = affine_for_delta(directive, role);
        solve_suffix(
            existing,
            unrestricted.then(instruction).then(restrictions),
            role,
        )
    })
}

fn rewrite_transform(
    directive: &PropertyTransformDelta,
    mut solve: impl FnMut(&SubFieldRole) -> Result<Affine, OverlayDeliveryError>,
) -> Result<PropertyTransformDelta, OverlayDeliveryError> {
    let mut roles = Vec::new();
    for (role, _) in &directive.sub_field_deltas {
        if !roles.contains(role) {
            roles.push(role.clone());
        }
    }
    let mut sub_field_deltas = Vec::new();
    for role in roles {
        encode_affine(&mut sub_field_deltas, role.clone(), solve(&role)?);
    }
    Ok(PropertyTransformDelta {
        property_id: directive.property_id,
        sub_field_deltas,
    })
}

fn affine_for_overlays<'a>(
    overlays: impl Iterator<Item = &'a Overlay>,
    directive: &PropertyTransformDelta,
    role: &SubFieldRole,
) -> Affine {
    affine_for_deltas(overlays.map(|overlay| &overlay.transform), directive, role)
}

fn affine_for_deltas<'a>(
    deltas: impl Iterator<Item = &'a PropertyTransformDelta>,
    directive: &PropertyTransformDelta,
    role: &SubFieldRole,
) -> Affine {
    deltas
        .filter(|delta| delta.property_id == directive.property_id)
        .fold(Affine::IDENTITY, |affine, delta| {
            affine.then(affine_for_delta(delta, role))
        })
}

fn affine_for_delta(delta: &PropertyTransformDelta, role: &SubFieldRole) -> Affine {
    delta
        .sub_field_deltas
        .iter()
        .filter(|(candidate, _)| candidate == role)
        .fold(Affine::IDENTITY, |affine, (_, op)| {
            affine.then(Affine::from_op(op))
        })
}

fn solve_suffix(
    existing: Affine,
    desired: Affine,
    role: &SubFieldRole,
) -> Result<Affine, OverlayDeliveryError> {
    if existing.mul != 0.0 {
        let mul = desired.mul / existing.mul;
        return Ok(Affine {
            mul,
            add: desired.add - mul * existing.add,
        });
    }
    if desired.mul == 0.0 {
        return Ok(Affine {
            mul: 0.0,
            add: desired.add,
        });
    }
    Err(OverlayDeliveryError::NonRepresentablePolicyRoute { role: role.clone() })
}

fn encode_affine(out: &mut Vec<(SubFieldRole, TransformOp)>, role: SubFieldRole, affine: Affine) {
    if affine.mul == 0.0 {
        out.push((role, TransformOp::Set(affine.add)));
    } else {
        if affine.mul != 1.0 {
            out.push((role.clone(), TransformOp::Multiply(affine.mul)));
        }
        if affine.add != 0.0 {
            out.push((role, TransformOp::Add(affine.add)));
        }
    }
}

fn is_policy(overlay: &Overlay) -> bool {
    matches!(overlay.kind, OverlayKind::Policy | OverlayKind::Governance)
}

fn find_path(root: &SimThing, target: SimThingId) -> Option<Vec<Vec<usize>>> {
    fn walk(
        node: &SimThing,
        target: SimThingId,
        path: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) -> bool {
        out.push(path.clone());
        if node.id == target {
            return true;
        }
        for (index, child) in node.children.iter().enumerate() {
            path.push(index);
            if walk(child, target, path, out) {
                return true;
            }
            path.pop();
        }
        out.pop();
        false
    }

    let mut path = Vec::new();
    let mut out = Vec::new();
    walk(root, target, &mut path, &mut out).then_some(out)
}

fn node_at_path<'a>(root: &'a SimThing, path: &[usize]) -> Option<&'a SimThing> {
    let mut node = root;
    for index in path {
        node = node.children.get(*index)?;
    }
    Some(node)
}

fn node_at_path_mut<'a>(root: &'a mut SimThing, path: &[usize]) -> Option<&'a mut SimThing> {
    let mut node = root;
    for index in path {
        node = node.children.get_mut(*index)?;
    }
    Some(node)
}
