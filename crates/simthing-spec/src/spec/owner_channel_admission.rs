//! OWNER-CHANNEL-INTRINSIC-0 — one-way compatibility admission into intrinsic ownership.
//!
//! Legacy owner-ref properties are authoring/serialization input only. This admission view clones
//! the scenario, converts those properties to the minimum explicit intrinsic boundary bindings,
//! removes the compatibility properties, and resolves the complete owner map once in canonical
//! tree order. RF consumers receive only this post-boundary view.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::owner_channel::{
    bind_owner, declared_owner, resolve_owners_in_order, unowned, OwnerRef, OwnerResolutionError,
};
use simthing_core::{SimPropertyId, SimThing, SimThingId};
use thiserror::Error;

use super::scenario::{
    game_session_owners, owner_entity_id, scenario_metadata_string,
    validate_session_owner_entities, ScenarioRootError, SimThingScenarioSpec,
    OWNER_FLOW_OWNER_REF_PROPERTY_ID, PLANET_OWNER_REF_PROPERTY_ID,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntrinsicOwnerChannelAdmissionStats {
    pub node_count: u32,
    pub admitted_owner_count: u32,
    pub compatibility_property_count: u32,
    pub inserted_boundary_count: u32,
    pub retained_intrinsic_binding_count: u32,
    pub legacy_owner_properties_remaining: u32,
}

/// Post-compatibility RF authority view.
///
/// The resolved map is transient admission/compile state, not authored, serialized, replayed, or
/// stamped back onto nodes. The cloned tree retains only explicit ownership-boundary bindings.
#[derive(Debug, Clone)]
pub struct IntrinsicOwnerChannelView {
    scenario: SimThingScenarioSpec,
    resolved_owners: BTreeMap<SimThingId, OwnerRef>,
    admitted_owners: BTreeSet<OwnerRef>,
    stats: IntrinsicOwnerChannelAdmissionStats,
}

impl IntrinsicOwnerChannelView {
    pub fn scenario(&self) -> &SimThingScenarioSpec {
        &self.scenario
    }

    pub fn owner_for(
        &self,
        simthing_id: SimThingId,
    ) -> Result<&OwnerRef, OwnerChannelAdmissionError> {
        self.resolved_owners
            .get(&simthing_id)
            .ok_or(OwnerChannelAdmissionError::ForeignTarget { simthing_id })
    }

    pub fn resolved_owners(&self) -> &BTreeMap<SimThingId, OwnerRef> {
        &self.resolved_owners
    }

    pub fn admitted_owners(&self) -> &BTreeSet<OwnerRef> {
        &self.admitted_owners
    }

    pub fn stats(&self) -> &IntrinsicOwnerChannelAdmissionStats {
        &self.stats
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OwnerChannelAdmissionError {
    #[error("scenario owner admission failed: {0}")]
    Scenario(String),
    #[error("SimThing {simthing_id:?} has malformed compatibility owner property {property_id:?}")]
    MalformedCompatibilityBinding {
        simthing_id: SimThingId,
        property_id: SimPropertyId,
    },
    #[error("SimThing {simthing_id:?} has a present but blank compatibility owner binding")]
    BlankCompatibilityBinding { simthing_id: SimThingId },
    #[error("SimThing {simthing_id:?} has conflicting compatibility owner bindings")]
    ConflictingCompatibilityBindings { simthing_id: SimThingId },
    #[error("SimThing {simthing_id:?} resolves to unknown Owner `{owner}`")]
    UnknownOwner {
        simthing_id: SimThingId,
        owner: String,
    },
    #[error(
        "Owner SimThing {simthing_id:?} intrinsic binding does not match its authored identity"
    )]
    OwnerIdentityBindingMismatch { simthing_id: SimThingId },
    #[error("intrinsic owner resolution failed: {0}")]
    IntrinsicResolution(String),
    #[error("SimThing {simthing_id:?} is outside the admitted owner authority tree")]
    ForeignTarget { simthing_id: SimThingId },
    #[error("owner-channel admission count exceeds u32")]
    CountOverflow,
}

/// Convert legacy owner references once, then expose intrinsic ownership as the only RF authority.
pub fn admit_intrinsic_owner_channels(
    source: &SimThingScenarioSpec,
) -> Result<IntrinsicOwnerChannelView, OwnerChannelAdmissionError> {
    validate_session_owner_entities(source).map_err(scenario_error)?;

    let authored_owner_nodes = game_session_owners(source)
        .map_err(scenario_error)?
        .into_iter()
        .map(|owner| {
            let owner_ref = owner_entity_id(owner)
                .map(OwnerRef::new)
                .ok_or_else(|| OwnerChannelAdmissionError::Scenario("Owner missing id".into()))?;
            Ok((owner.id, owner_ref))
        })
        .collect::<Result<BTreeMap<_, _>, OwnerChannelAdmissionError>>()?;
    let admitted_owners = authored_owner_nodes
        .values()
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut scenario = source.clone();
    let mut counts = AdmissionCounts::default();
    convert_tree(
        &mut scenario.root,
        &unowned(),
        &authored_owner_nodes,
        &admitted_owners,
        &mut counts,
    )?;

    let resolved = resolve_owners_in_order(&scenario.root).map_err(resolution_error)?;
    let resolved_owners = resolved.into_iter().collect::<BTreeMap<_, _>>();
    for (simthing_id, owner) in &resolved_owners {
        validate_referential_integrity(*simthing_id, owner, &admitted_owners)?;
    }

    let legacy_owner_properties_remaining = count_legacy_owner_properties(&scenario.root);
    if legacy_owner_properties_remaining != 0 {
        return Err(OwnerChannelAdmissionError::IntrinsicResolution(
            "compatibility owner properties survived the admission boundary".into(),
        ));
    }

    let stats = IntrinsicOwnerChannelAdmissionStats {
        node_count: checked_count(resolved_owners.len())?,
        admitted_owner_count: checked_count(admitted_owners.len())?,
        compatibility_property_count: checked_count(counts.compatibility_property_count)?,
        inserted_boundary_count: checked_count(counts.inserted_boundary_count)?,
        retained_intrinsic_binding_count: checked_count(counts.retained_intrinsic_binding_count)?,
        legacy_owner_properties_remaining: 0,
    };

    Ok(IntrinsicOwnerChannelView {
        scenario,
        resolved_owners,
        admitted_owners,
        stats,
    })
}

#[derive(Debug, Default)]
struct AdmissionCounts {
    compatibility_property_count: usize,
    inserted_boundary_count: usize,
    retained_intrinsic_binding_count: usize,
}

fn convert_tree(
    node: &mut SimThing,
    inherited_owner: &OwnerRef,
    authored_owner_nodes: &BTreeMap<SimThingId, OwnerRef>,
    admitted_owners: &BTreeSet<OwnerRef>,
    counts: &mut AdmissionCounts,
) -> Result<(), OwnerChannelAdmissionError> {
    let declared = declared_owner(node).map_err(resolution_error)?;
    if declared.is_some() {
        counts.retained_intrinsic_binding_count += 1;
    }
    let compatibility = compatibility_owner(node, counts)?;
    let owner_identity = authored_owner_nodes.get(&node.id).cloned();

    if let (Some(declared), Some(compatibility)) = (&declared, &compatibility) {
        if declared != compatibility {
            return Err(
                OwnerChannelAdmissionError::ConflictingCompatibilityBindings {
                    simthing_id: node.id,
                },
            );
        }
    }

    let requested = owner_identity
        .clone()
        .or_else(|| declared.clone())
        .or(compatibility);
    if let (Some(expected), Some(declared)) = (&owner_identity, &declared) {
        if expected != declared {
            return Err(OwnerChannelAdmissionError::OwnerIdentityBindingMismatch {
                simthing_id: node.id,
            });
        }
    }
    if let Some(owner) = &requested {
        validate_referential_integrity(node.id, owner, admitted_owners)?;
    }

    let effective = requested.unwrap_or_else(|| inherited_owner.clone());
    if declared.is_none() && effective != *inherited_owner {
        bind_owner(node, &effective);
        counts.inserted_boundary_count += 1;
    }

    node.remove_property(&OWNER_FLOW_OWNER_REF_PROPERTY_ID);
    node.remove_property(&PLANET_OWNER_REF_PROPERTY_ID);

    for child in &mut node.children {
        convert_tree(
            child,
            &effective,
            authored_owner_nodes,
            admitted_owners,
            counts,
        )?;
    }
    Ok(())
}

fn compatibility_owner(
    node: &SimThing,
    counts: &mut AdmissionCounts,
) -> Result<Option<OwnerRef>, OwnerChannelAdmissionError> {
    let mut owner = None;
    for property_id in [
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        PLANET_OWNER_REF_PROPERTY_ID,
    ] {
        if !node.properties.contains_key(&property_id) {
            continue;
        }
        counts.compatibility_property_count += 1;
        let value = scenario_metadata_string(node, property_id).ok_or(
            OwnerChannelAdmissionError::MalformedCompatibilityBinding {
                simthing_id: node.id,
                property_id,
            },
        )?;
        if value.trim().is_empty() {
            return Err(OwnerChannelAdmissionError::BlankCompatibilityBinding {
                simthing_id: node.id,
            });
        }
        let value = OwnerRef::new(value);
        if owner.as_ref().is_some_and(|existing| existing != &value) {
            return Err(
                OwnerChannelAdmissionError::ConflictingCompatibilityBindings {
                    simthing_id: node.id,
                },
            );
        }
        owner = Some(value);
    }
    Ok(owner)
}

fn validate_referential_integrity(
    simthing_id: SimThingId,
    owner: &OwnerRef,
    admitted_owners: &BTreeSet<OwnerRef>,
) -> Result<(), OwnerChannelAdmissionError> {
    if !owner.is_unowned() && !admitted_owners.contains(owner) {
        return Err(OwnerChannelAdmissionError::UnknownOwner {
            simthing_id,
            owner: owner.as_str().to_string(),
        });
    }
    Ok(())
}

fn count_legacy_owner_properties(node: &SimThing) -> u32 {
    let local = u32::from(
        node.properties
            .contains_key(&OWNER_FLOW_OWNER_REF_PROPERTY_ID),
    ) + u32::from(node.properties.contains_key(&PLANET_OWNER_REF_PROPERTY_ID));
    node.children.iter().fold(local, |count, child| {
        count.saturating_add(count_legacy_owner_properties(child))
    })
}

fn checked_count(value: usize) -> Result<u32, OwnerChannelAdmissionError> {
    u32::try_from(value).map_err(|_| OwnerChannelAdmissionError::CountOverflow)
}

fn scenario_error(error: ScenarioRootError) -> OwnerChannelAdmissionError {
    OwnerChannelAdmissionError::Scenario(error.to_string())
}

fn resolution_error(error: OwnerResolutionError) -> OwnerChannelAdmissionError {
    OwnerChannelAdmissionError::IntrinsicResolution(error.to_string())
}
