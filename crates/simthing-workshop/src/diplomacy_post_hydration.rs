//! TP-DIPLOMACY-FLOW-0 — workshop-homed post-hydration diplomacy RF lanes.
//!
//! Applied after generic `hydrate_scenario`; scenario candidate code lives here,
//! not in `simthing-clausething/src`.

use std::collections::BTreeSet;

use simthing_clausething::HydratedScenarioPack;
use simthing_core::{ClampBehavior, SimThing, SubFieldRole, SubFieldSpec};
use simthing_spec::spec::resource_economy::{EmitOnThresholdSpec, ResourceEconomyOptInMode};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::spec::trigger::TriggerDirection;
use simthing_spec::spec::PropertySpec;
use simthing_spec::{
    apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_participant_owner_flow_resource_key_metadata, is_admitted_planet_non_grid_child,
    is_galaxy_map_entity, is_owner_entity_kind, is_surface_gridcell, owner_entity_id,
    owner_silo_capacity, owner_silo_current,
    owner_flow_deficit, owner_flow_owner_ref, owner_flow_surplus,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

/// Scenario-local distrust resource channel key stamped on touched assets.
pub const TP_DISTRUST_RESOURCE_KEY: &str = "tp_distrust";

/// Authored baseline distrust surplus per touched border asset participant.
pub const BASELINE_BORDER_DISTRUST_SURPLUS: u32 = 60;

/// Reduced owner distrust column threshold for hostility commitment.
pub const HOSTILITY_DISTRUST_THRESHOLD: f32 = 50.0;

/// Threshold emission kind for Terran↔Pirate hostility commitment (workshop-local).
pub const HOSTILITY_COMMITMENT_EVENT_KIND: u32 = 0x484F_5354; // "HOST"

#[derive(Debug, thiserror::Error)]
pub enum DiplomacyHydrationError {
    #[error("diplomacy post-hydration requires authority_root")]
    MissingAuthorityRoot,
    #[error("diplomacy post-hydration requires terran and pirate owners")]
    MissingOwners,
    #[error("diplomacy post-hydration requires ownership volumes")]
    MissingOwnershipVolumes,
    #[error("{0}")]
    Message(String),
}

/// Workshop-side diplomacy RF application over a generic hydrated TP pack.
pub fn apply_diplomacy_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<(), DiplomacyHydrationError> {
    if pack.authority_root.is_none() {
        return Err(DiplomacyHydrationError::MissingAuthorityRoot);
    }
    if pack.owners.len() < 2 || pack.ownership_volumes.is_empty() {
        return Err(DiplomacyHydrationError::MissingOwnershipVolumes);
    }

    let touched_coords = border_touch_system_coords(pack);
    let owner_targets = owner_install_targets(pack)?;
    let root = pack
        .authority_root
        .as_mut()
        .expect("authority root checked above");
    stamp_distrust_on_touched_participants(root, &touched_coords)?;
    seed_owner_silos(root, &owner_targets)?;
    install_owner_distrust_game_mode(pack, &owner_targets)?;
    Ok(())
}

fn border_touch_system_coords(pack: &HydratedScenarioPack) -> BTreeSet<(u32, u32)> {
    pack.ownership_volumes
        .iter()
        .filter(|volume| volume.id == "terran_core" || volume.id == "pirate_border")
        .flat_map(|volume| {
            volume
                .assigned_systems
                .iter()
                .map(|system| (system.row, system.col))
        })
        .collect()
}

fn system_structural_coord(system: &SimThing) -> (u32, u32) {
    let row = system
        .properties
        .get(&SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    let col = system
        .properties
        .get(&SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    (row, col)
}

fn stamp_distrust_on_touched_participants(
    root: &mut SimThing,
    touched_coords: &BTreeSet<(u32, u32)>,
) -> Result<(), DiplomacyHydrationError> {
    let session = game_session_child_mut(root).ok_or_else(|| {
        DiplomacyHydrationError::Message("GameSession missing on authority root".into())
    })?;
    let galaxy_map = game_session_galaxy_map_mut(session).ok_or_else(|| {
        DiplomacyHydrationError::Message("GalaxyMap missing under GameSession".into())
    })?;

    let mut stamped = 0u32;
    for star_system in &mut galaxy_map.children {
        if !touched_coords.contains(&system_structural_coord(star_system)) {
            continue;
        }
        for planet in &mut star_system.children {
            for surface in &mut planet.children {
                if !is_surface_gridcell(surface) {
                    continue;
                }
                for child in &mut surface.children {
                    if !is_admitted_planet_non_grid_child(&child.kind) {
                        continue;
                    }
                    let Some(owner) = owner_flow_owner_ref(child) else {
                        continue;
                    };
                    bump_participant_distrust_surplus(child, &owner);
                    stamped = stamped.saturating_add(1);
                }
            }
        }
    }

    if stamped == 0 {
        return Err(DiplomacyHydrationError::Message(
            "no touched RF participants found for diplomacy distrust seeding".into(),
        ));
    }
    Ok(())
}

fn bump_participant_distrust_surplus(participant: &mut SimThing, owner: &str) {
    let surplus = owner_flow_surplus(participant).unwrap_or(0);
    let deficit = owner_flow_deficit(participant).unwrap_or(0);
    apply_participant_owner_flow_metadata(
        participant,
        owner,
        surplus.saturating_add(BASELINE_BORDER_DISTRUST_SURPLUS),
        deficit,
    );
    apply_participant_owner_flow_resource_key_metadata(participant, TP_DISTRUST_RESOURCE_KEY);
}

fn owner_install_targets(
    pack: &HydratedScenarioPack,
) -> Result<Vec<(simthing_core::SimThingId, String)>, DiplomacyHydrationError> {
    let root = pack
        .authority_root
        .as_ref()
        .ok_or(DiplomacyHydrationError::MissingAuthorityRoot)?;
    let owners = owner_entities_under_root(root);
    if owners.len() < 2 {
        return Err(DiplomacyHydrationError::MissingOwners);
    }
    Ok(owners)
}

fn owner_entities_under_root(root: &SimThing) -> Vec<(simthing_core::SimThingId, String)> {
    let Some(session) = game_session_child(root) else {
        return Vec::new();
    };
    session
        .children
        .iter()
        .filter(|child| is_owner_entity_kind(&child.kind))
        .filter_map(|child| owner_entity_id(child).map(|owner_key| (child.id, owner_key)))
        .collect()
}

fn game_session_child(root: &SimThing) -> Option<&SimThing> {
    root.children
        .iter()
        .find(|child| child.kind == simthing_core::SimThingKind::GameSession)
}

fn seed_owner_silos(
    root: &mut SimThing,
    owner_targets: &[(simthing_core::SimThingId, String)],
) -> Result<(), DiplomacyHydrationError> {
    for &(owner_id, _) in owner_targets {
        let Some(owner_mut) = find_simthing_mut(root, owner_id) else {
            continue;
        };
        if owner_silo_current(owner_mut).is_none() {
            apply_owner_silo_metadata(
                owner_mut,
                0,
                owner_silo_capacity(owner_mut).or(Some(1_000)),
            );
        }
    }
    Ok(())
}

fn install_owner_distrust_game_mode(
    pack: &mut HydratedScenarioPack,
    owner_targets: &[(simthing_core::SimThingId, String)],
) -> Result<(), DiplomacyHydrationError> {
    if pack
        .game_mode
        .properties
        .iter()
        .any(|prop| prop.namespace == "tp" && prop.name == "owner_distrust")
    {
        return Ok(());
    }

    pack.game_mode.properties.push(owner_distrust_property_spec());

    let mut economy = pack
        .game_mode
        .resource_economy
        .clone()
        .unwrap_or_default();
    economy.opt_in_mode = ResourceEconomyOptInMode::EmissionOnly;
    economy.emit_on_threshold.push(EmitOnThresholdSpec {
        id: "tp_hostility_commitment".into(),
        source: PropertyKey::new("tp", "owner_distrust"),
        source_role: SubFieldRole::Amount,
        threshold: HOSTILITY_DISTRUST_THRESHOLD,
        direction: TriggerDirection::Rising,
        event_kind: HOSTILITY_COMMITMENT_EVENT_KIND,
        buffer: Default::default(),
    });
    pack.game_mode.resource_economy = Some(economy);

    for &(owner_id, ref owner_key) in owner_targets {
        pack.install_targets
            .entry(format!("diplomacy_owner_{owner_key}"))
            .or_default()
            .push(owner_id);
    }

    Ok(())
}

fn owner_distrust_property_spec() -> PropertySpec {
    PropertySpec {
        id: "tp_owner_distrust".into(),
        namespace: "tp".into(),
        name: "owner_distrust".into(),
        display_name: "Owner Distrust".into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "Distrust".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

fn game_session_child_mut(root: &mut SimThing) -> Option<&mut SimThing> {
    root.children
        .iter_mut()
        .find(|child| child.kind == simthing_core::SimThingKind::GameSession)
}

fn game_session_galaxy_map_mut<'a>(session: &'a mut SimThing) -> Option<&'a mut SimThing> {
    session
        .children
        .iter_mut()
        .find(|child| is_galaxy_map_entity(child))
}

fn find_simthing_mut(root: &mut SimThing, id: simthing_core::SimThingId) -> Option<&mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(found) = find_simthing_mut(child, id) {
            return Some(found);
        }
    }
    None
}