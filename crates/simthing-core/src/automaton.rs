//! SIMTHING-AUTOMATON-INTRINSIC-0 reception/origination contract.
//!
//! No inbox, queue, listener registry, transport, or scheduler lives here. Delivery
//! terminates in the existing [`SimThing::overlays`] vector, standing reception uses
//! ordinary ancestor inheritance, and predicate broadcast is one paid tree walk.

use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::evaluate::{RoutedPredicate, TransformStack};
use crate::overlay::{Overlay, OverlayKind};
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
    #[error("runtime dispatch-minted overlay fails dissolve discipline: {detail}")]
    DispatchDissolveRequired { detail: String },
    #[error("overlay lifecycle admission failed: {detail}")]
    LifecycleAdmission { detail: String },
}

fn is_runtime_dispatch_mint(overlay: &Overlay) -> bool {
    use crate::overlay::{OverlayKind, OverlaySource};
    matches!(overlay.source, OverlaySource::Event | OverlaySource::System)
        && matches!(
            overlay.kind,
            OverlayKind::Instruction | OverlayKind::Custom(_)
        )
}

/// Deliver a consumed/deficit-driven directive through the existing tree path.
///
/// The overlay is stored only at `target`. The evaluator and GPU preparation
/// derive the origin -> common-ancestor -> target route from live tree state on
/// every pass, so policy changes affect an already-delivered directive without
/// copying policy state into the instruction.
pub fn deliver_deficit_directive(
    root: &mut SimThing,
    target: SimThingId,
    overlay: Overlay,
) -> Result<DirectiveDeliveryReceipt, OverlayDeliveryError> {
    deliver_routed_overlay(root, target, overlay)
}

/// Route an already-admitted overlay to its target. This is the common arrival
/// primitive used by structural ingress too; it does not claim that boundary
/// attachment itself is the conserved deficit transport.
pub fn deliver_routed_overlay(
    root: &mut SimThing,
    target: SimThingId,
    mut overlay: Overlay,
) -> Result<DirectiveDeliveryReceipt, OverlayDeliveryError> {
    crate::admit_overlay_lifecycle(&overlay.lifecycle).map_err(|error| {
        OverlayDeliveryError::LifecycleAdmission {
            detail: error.to_string(),
        }
    })?;
    // EVENT-GENERATION-STAMP-0 / Definable Horizon: runtime dispatch-minted
    // overlays (Event/System instruction-class) must carry UntilDissolvedWith
    // and an authored dissolve condition. Authored Policy/Governance unit
    // UntilDissolved remains admissible.
    if is_runtime_dispatch_mint(&overlay) {
        crate::generation_stamp::admit_dispatch_minted_overlay(&overlay).map_err(|e| {
            OverlayDeliveryError::DispatchDissolveRequired {
                detail: e.to_string(),
            }
        })?;
    }
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
    deliver_routed_overlay(root, subtree_root, overlay)
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
        let stack = inherit_active_overlays(node, ancestors);
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
    let stack = inherit_active_overlays(node, ancestors);
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

/// One step of the canonical standing-inheritance walk. Absence leaves the
/// inherited stack untouched, exactly like `resolve_owner`; active overlays
/// extend it without materializing anything on descendants.
pub fn inherit_active_overlays(node: &SimThing, ancestors: &TransformStack) -> TransformStack {
    node.overlays
        .iter()
        .filter(|overlay| overlay.is_active())
        .fold(ancestors.clone(), |stack, overlay| {
            stack.push_overlay(overlay)
        })
}

/// Capture the active standing policy inherited at `target`, in root-first order.
///
/// This is a site-local capture door. Independently executing descendants must receive the
/// returned value through a generation-stamped seam snapshot; they must not retain references
/// into this live tree or re-read it between generation barriers.
pub fn capture_ancestor_standing_policy(
    root: &SimThing,
    target: SimThingId,
) -> Result<Vec<Overlay>, OverlayDeliveryError> {
    let path = find_path(root, target).ok_or(OverlayDeliveryError::TargetNotInTree { target })?;
    Ok(path
        .iter()
        .take(path.len().saturating_sub(1))
        .flat_map(|indices| node_at_path(root, indices).into_iter())
        .flat_map(|node| node.overlays.iter())
        .filter(|overlay| overlay.is_active() && is_policy(overlay))
        .cloned()
        .collect())
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

fn is_policy(overlay: &Overlay) -> bool {
    matches!(overlay.kind, OverlayKind::Policy | OverlayKind::Governance)
}

/// Ephemeral view used by both CPU evaluation and GPU overlay preparation.
/// It stores only references and parent links for the duration of one pass;
/// SimThings retain no route cache or reception allocation.
pub struct LiveOverlayRoutes<'a> {
    root: SimThingId,
    nodes: HashMap<SimThingId, &'a SimThing>,
    parents: HashMap<SimThingId, SimThingId>,
    routed_targets: HashSet<SimThingId>,
}

impl<'a> LiveOverlayRoutes<'a> {
    /// Build a route view only when an active routed instruction exists.
    /// Inert and standing-only trees allocate no route state.
    pub fn for_tree(root: &'a SimThing) -> Option<Self> {
        fn has_routed(node: &SimThing) -> bool {
            node.overlays.iter().any(|overlay| {
                overlay.is_active()
                    && matches!(overlay.kind, OverlayKind::Instruction)
                    && overlay
                        .affects
                        .iter()
                        .any(|target| *target != overlay.origin)
            }) || node.children.iter().any(has_routed)
        }
        if !has_routed(root) {
            return None;
        }

        fn index<'a>(
            node: &'a SimThing,
            parent: Option<SimThingId>,
            nodes: &mut HashMap<SimThingId, &'a SimThing>,
            parents: &mut HashMap<SimThingId, SimThingId>,
            routed_targets: &mut HashSet<SimThingId>,
        ) {
            nodes.insert(node.id, node);
            if let Some(parent) = parent {
                parents.insert(node.id, parent);
            }
            for overlay in &node.overlays {
                if overlay.is_active() && matches!(overlay.kind, OverlayKind::Instruction) {
                    routed_targets.extend(
                        overlay
                            .affects
                            .iter()
                            .copied()
                            .filter(|target| *target != overlay.origin),
                    );
                }
            }
            for child in &node.children {
                index(child, Some(node.id), nodes, parents, routed_targets);
            }
        }

        let mut nodes = HashMap::new();
        let mut parents = HashMap::new();
        let mut routed_targets = HashSet::new();
        index(root, None, &mut nodes, &mut parents, &mut routed_targets);
        Some(Self {
            root: root.id,
            nodes,
            parents,
            routed_targets,
        })
    }

    /// Return the live overlay order for a target with routed instructions.
    /// Route policies are removed from their ordinary ancestor position and
    /// applied immediately after the instruction they filter. Suspended or
    /// dissolved policies disappear on the next pass without re-delivery.
    pub fn ordered_active_overlays(&self, target: SimThingId) -> Option<Vec<&'a Overlay>> {
        if !self.routed_targets.contains(&target) {
            return None;
        }
        let target_path = self.path_from_root(target)?;
        let ordinary: Vec<&Overlay> = target_path
            .iter()
            .flat_map(|id| {
                self.nodes[id]
                    .overlays
                    .iter()
                    .filter(|overlay| overlay.is_active())
            })
            .collect();

        let routed: Vec<(&Overlay, Vec<&Overlay>)> = ordinary
            .iter()
            .copied()
            .filter(|overlay| {
                matches!(overlay.kind, OverlayKind::Instruction)
                    && overlay.affects.contains(&target)
                    && overlay.origin != target
            })
            .map(|instruction| {
                let policies = self
                    .route(instruction.origin, target)
                    .into_iter()
                    .flatten()
                    .flat_map(|id| {
                        self.nodes[&id]
                            .overlays
                            .iter()
                            .filter(|overlay| overlay.is_active() && is_policy(overlay))
                    })
                    .collect();
                (instruction, policies)
            })
            .collect();
        let deferred: HashSet<OverlayId> = routed
            .iter()
            .flat_map(|(_, policies)| policies.iter().map(|overlay| overlay.id))
            .collect();

        let mut ordered = Vec::with_capacity(ordinary.len() + deferred.len());
        for overlay in ordinary {
            if is_policy(overlay) && deferred.contains(&overlay.id) {
                continue;
            }
            ordered.push(overlay);
            if let Some((_, policies)) = routed
                .iter()
                .find(|(instruction, _)| instruction.id == overlay.id)
            {
                ordered.extend(policies.iter().copied());
            }
        }
        Some(ordered)
    }

    fn path_from_root(&self, target: SimThingId) -> Option<Vec<SimThingId>> {
        self.nodes.get(&target)?;
        let mut path = vec![target];
        while *path.last()? != self.root {
            path.push(*self.parents.get(path.last()?)?);
        }
        path.reverse();
        Some(path)
    }

    fn route(&self, origin: SimThingId, target: SimThingId) -> Option<Vec<SimThingId>> {
        let origin_path = self.path_from_root(origin)?;
        let target_path = self.path_from_root(target)?;
        let common_len = origin_path
            .iter()
            .zip(&target_path)
            .take_while(|(left, right)| left == right)
            .count();
        let mut route: Vec<SimThingId> = origin_path[common_len - 1..]
            .iter()
            .rev()
            .copied()
            .collect();
        route.extend(target_path[common_len..].iter().copied());
        Some(route)
    }
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
