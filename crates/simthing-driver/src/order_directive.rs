//! ORDER-WEIGHT-CLASS-0 — typed class-bound operator directive submission.
//!
//! Resolves an authored order-weight class id into an ordinary
//! `OverlaySource::Player` Transient overlay and submits it through the
//! existing player-intent feeder path. No second queue or execution mechanism.

use serde::{Deserialize, Serialize};
use simthing_core::{
    AccumulatorRole, DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimPropertyId, SimThing, SimThingId,
    SubFieldRole, TransformOp,
};
use simthing_spec::{validate_runtime_player_overlay_magnitude, OrderWeightClassSpec, SpecError};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Request to submit a class-bound destination/order directive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDirectiveRequest {
    /// Authored class id from the admitted order-weight class table.
    pub class_id: String,
    /// SimThing issuing the directive. Usually an Owner SimThing whose
    /// Player/Ai will is aimed at one of its admitted participants.
    pub origin: SimThingId,
    /// Host that receives the Player overlay (e.g. ordered destination leaf).
    pub target: SimThingId,
    /// Weight/need property locus (must already exist on the target host).
    pub property_id: SimPropertyId,
    /// Sub-field role on the weight/need property (typically Named("weight")).
    pub sub_field: SubFieldRole,
    /// Declarative arrival (or other) dissolution condition at a generation boundary.
    pub dissolve: DissolveCondition,
}

/// Generation-stamped directive ingress stored in the existing replay frame.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDirectiveInjection {
    pub generation: u64,
    pub request: OrderDirectiveRequest,
}

pub fn order_directive_injections_from_frame(
    frame: &simthing_sim::ReplayFrame,
) -> Result<Vec<OrderDirectiveInjection>, serde_json::Error> {
    frame
        .injection_entries
        .iter()
        .cloned()
        .map(serde_json::from_value)
        .collect()
}

#[derive(Debug, Error)]
pub enum OrderDirectiveError {
    #[error("unknown order_weight_class `{class_id}`")]
    UnknownClass { class_id: String },
    #[error("order-weight directive does not match admitted class binding: {0}")]
    Binding(String),
    #[error("order-weight admission: {0}")]
    Admission(#[from] SpecError),
    #[error("feeder disconnected")]
    FeederDisconnected,
}

/// Install-resolved class binding. The arena participant set and actual
/// ambient normalization envelope are immutable admission artifacts.
#[derive(Clone, Debug)]
pub struct AdmittedOrderWeightClass {
    pub spec: OrderWeightClassSpec,
    pub arena_idx: u32,
    pub property_id: SimPropertyId,
    pub participants: HashSet<SimThingId>,
    pub ambient_weight_sum: f32,
}

#[derive(Debug, Default)]
pub struct OrderDirectiveGateState {
    pub resolved: HashMap<OverlayId, String>,
    ambient_player_additions: HashMap<String, f32>,
}

/// Resolve typed class loci against the admitted RF registry and prove the
/// finite dominance inequality `magnitude > sum(all ambient weights)`.
pub fn admit_order_weight_classes(
    classes: &[OrderWeightClassSpec],
    registry: &DimensionRegistry,
    root: &SimThing,
    arenas: &crate::arena_registry::ArenaRegistry,
) -> Result<Vec<AdmittedOrderWeightClass>, SpecError> {
    let snapshot = simthing_core::evaluate::Evaluator::new(registry, 0.0).evaluate(root, 0);
    let mut admitted = Vec::with_capacity(classes.len());
    for class in classes {
        let (arena_idx, arena) = arenas
            .arenas
            .iter()
            .enumerate()
            .find(|(_, arena)| arena.name == class.arena)
            .ok_or_else(|| malformed(class, format!("unknown RF arena `{}`", class.arena)))?;
        if arena.fission_policy != crate::arena_registry::FissionPolicy::Reject {
            return Err(malformed(
                class,
                format!(
                    "arena `{}` can change participation under {:?}; no finite authored ambient envelope is stable",
                    class.arena, arena.fission_policy
                ),
            ));
        }
        let property_id = registry
            .id_of(&class.property.namespace, &class.property.name)
            .ok_or_else(|| {
                malformed(
                    class,
                    format!(
                        "unknown property `{}::{}`",
                        class.property.namespace, class.property.name
                    ),
                )
            })?;
        if arena.flow_property_id != property_id {
            return Err(malformed(
                class,
                format!(
                    "arena `{}` normalizes property {:?}, not bound property {:?}",
                    class.arena, arena.flow_property_id, property_id
                ),
            ));
        }
        let property = registry.property(property_id);
        let sub_field = property
            .layout
            .sub_fields
            .iter()
            .find(|sf| sf.role == class.sub_field)
            .ok_or_else(|| malformed(class, format!("missing role {:?}", class.sub_field)))?;
        if !matches!(
            sub_field.accumulator_spec.as_ref().map(|a| &a.role),
            Some(AccumulatorRole::AllocatorWeight { arena }) if arena == &class.arena
        ) {
            return Err(malformed(
                class,
                format!(
                    "bound locus {:?} is not AccumulatorRole::AllocatorWeight for arena `{}`",
                    class.sub_field, class.arena
                ),
            ));
        }

        let (start, len) = arena.participant_range;
        let members = &arenas.participants[start as usize..(start + len) as usize];
        let participants: HashSet<_> = members.iter().map(|m| m.subtree_root).collect();
        let mut ambient_weight_sum = 0.0_f32;
        for participant in &participants {
            let value = snapshot
                .get(*participant)
                .and_then(|entity| entity.properties.get(&property_id))
                .ok_or_else(|| {
                    malformed(
                        class,
                        format!("arena participant {participant:?} lacks bound property"),
                    )
                })?
                .get_role(&class.sub_field, &property.layout);
            if !value.is_finite() || value < 0.0 {
                return Err(malformed(
                    class,
                    format!("arena participant {participant:?} has invalid ambient weight {value}"),
                ));
            }
            ambient_weight_sum += value;
        }
        if !ambient_weight_sum.is_finite() || class.magnitude <= ambient_weight_sum {
            return Err(malformed(
                class,
                format!(
                    "finite dominance fails: magnitude {} must exceed arena `{}` ambient normalization sum {} across {} admitted participants (declared max {})",
                    class.magnitude,
                    class.arena,
                    ambient_weight_sum,
                    participants.len(),
                    arena.max_participants,
                ),
            ));
        }
        admitted.push(AdmittedOrderWeightClass {
            spec: class.clone(),
            arena_idx: arena_idx as u32,
            property_id,
            participants,
            ambient_weight_sum,
        });
    }
    Ok(admitted)
}

fn malformed(class: &OrderWeightClassSpec, reason: String) -> SpecError {
    SpecError::MalformedOrderWeightClass {
        class_id: class.id.clone(),
        reason,
        source_span_token: class.source_span_token,
    }
}

/// Resolve `class_id` against the admitted class table and build the ordinary
/// Player Transient overlay (price injection). Does not submit.
pub fn build_order_directive_overlay(
    classes: &[AdmittedOrderWeightClass],
    req: &OrderDirectiveRequest,
) -> Result<(Overlay, f32), OrderDirectiveError> {
    let class = classes
        .iter()
        .find(|c| c.spec.id == req.class_id)
        .ok_or_else(|| OrderDirectiveError::UnknownClass {
            class_id: req.class_id.clone(),
        })?;
    if req.property_id != class.property_id
        || req.sub_field != class.spec.sub_field
        || !class.participants.contains(&req.target)
    {
        return Err(OrderDirectiveError::Binding(format!(
            "class `{}` is arena `{}` property {:?}/{:?}; target {:?} must be an admitted participant",
            class.spec.id,
            class.spec.arena,
            class.property_id,
            class.spec.sub_field,
            req.target,
        )));
    }

    let overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        origin: req.origin,
        affects: vec![req.target],
        transform: PropertyTransformDelta {
            property_id: req.property_id,
            sub_field_deltas: vec![(
                req.sub_field.clone(),
                TransformOp::Add(class.spec.magnitude),
            )],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![req.dissolve.clone()],
        },
    };
    Ok((overlay, class.spec.magnitude))
}

/// Gate a raw Player overlay against the class table (no class id on core Overlay).
/// Dominant magnitudes must use [`submit_order_directive`].
pub fn gate_raw_player_overlay(
    overlay: &Overlay,
    classes: &[AdmittedOrderWeightClass],
) -> Result<(), OrderDirectiveError> {
    let mags: Vec<f32> = overlay
        .transform
        .sub_field_deltas
        .iter()
        .map(|(_, op)| match op {
            TransformOp::Add(v) | TransformOp::Multiply(v) | TransformOp::Set(v) => *v,
            TransformOp::Eml(_) => 0.0,
        })
        .collect();
    let specs: Vec<_> = classes.iter().map(|class| class.spec.clone()).collect();
    validate_runtime_player_overlay_magnitude(overlay.source.clone(), &mags, &specs)?;
    Ok(())
}

/// Canonical feeder-drain gate. A dominant Player overlay is accepted only
/// once, when its id was registered by the session's class-resolving API and
/// its complete shape still matches that admitted class binding.
pub fn gate_ingested_player_intent(
    target: SimThingId,
    overlay: &Overlay,
    classes: &[AdmittedOrderWeightClass],
    gate_state: &mut OrderDirectiveGateState,
) -> Result<(), OrderDirectiveError> {
    let Some(class_id) = gate_state.resolved.remove(&overlay.id) else {
        gate_raw_player_overlay(overlay, classes)?;
        for class in classes {
            if !class.participants.contains(&target)
                || overlay.transform.property_id != class.property_id
                || !overlay
                    .transform
                    .sub_field_deltas
                    .iter()
                    .any(|(role, _)| role == &class.spec.sub_field)
            {
                continue;
            }
            let [(role, TransformOp::Add(value))] = overlay.transform.sub_field_deltas.as_slice()
            else {
                return Err(OrderDirectiveError::Binding(format!(
                    "raw Player transforms on class-bound arena `{}` weight locus must be one finite Add",
                    class.spec.arena
                )));
            };
            if role != &class.spec.sub_field || !value.is_finite() {
                return Err(OrderDirectiveError::Binding(format!(
                    "raw Player transform does not match arena `{}` weight role",
                    class.spec.arena
                )));
            }
            let accumulated = gate_state
                .ambient_player_additions
                .entry(class.spec.id.clone())
                .or_default();
            let next = *accumulated + value.max(0.0);
            if class.ambient_weight_sum + next >= class.spec.magnitude {
                return Err(OrderDirectiveError::Binding(format!(
                    "aggregate Player ambient envelope {} + {} would reach class `{}` magnitude {}",
                    class.ambient_weight_sum, next, class.spec.id, class.spec.magnitude
                )));
            }
            *accumulated = next;
        }
        return Ok(());
    };
    let class = classes
        .iter()
        .find(|class| class.spec.id == class_id)
        .ok_or_else(|| OrderDirectiveError::UnknownClass { class_id })?;
    if overlay.source != OverlaySource::Player
        || overlay.kind != OverlayKind::Instruction
        || !class.participants.contains(&target)
        || overlay.affects.as_slice() != [target]
        || overlay.transform.property_id != class.property_id
        || overlay.transform.sub_field_deltas.as_slice()
            != [(
                class.spec.sub_field.clone(),
                TransformOp::Add(class.spec.magnitude),
            )]
        || !matches!(
            &overlay.lifecycle,
            OverlayLifecycle::Transient { dissolution_conditions }
                if !dissolution_conditions.is_empty()
        )
    {
        return Err(OrderDirectiveError::Binding(format!(
            "resolved overlay {:?} no longer matches class `{}` arena `{}`",
            overlay.id, class.spec.id, class.spec.arena
        )));
    }
    Ok(())
}
