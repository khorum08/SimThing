//! TP-COMBAT-ARENA-0 scenario-envelope combat arena authoring.
//!
//! Lowers co-located hostile ship contact into discrete `ResourceEconomySpec`
//! transfers (`SubtractFromSource`), optional `governed_by` hull recovery,
//! owner bonus overlays on weapon columns, and zero-HP threshold removal events.
//!
//! **Homing Boundary:** scenario-candidate combat hydrator landed in
//! `simthing-clausething` under a one-time owner-cleared exception for this rung
//! only. Not precedent for Phase 5+ or future engine-crate scenario services.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use simthing_core::{
    OverlayKind, OverlayLifecycle, OverlaySource, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_spec::spec::effect::EffectSpec;
use simthing_spec::spec::event::EventSpec;
use simthing_spec::spec::game_mode::GameModeSpec;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::resource_economy::{ResourceEconomyOptInMode, ResourceTransferSpec};
use simthing_spec::spec::script::{PropertyKey, ScopeRef};
use simthing_spec::spec::trigger::{TriggerDirection, TriggerSpec};
use simthing_spec::{
    apply_participant_owner_flow_metadata, is_galaxy_map_entity, is_surface_gridcell,
    scenario_metadata_string_value, OWNER_FLOW_OWNER_REF_PROPERTY_ID,
};

use crate::error::HydrateError;
use crate::hydrate_scenario::{HydratedFleetShipPayload, HydratedScenarioOwner};
use crate::hydrate_shipsize_decoder::{
    decode_ship_modifier_key_spanned, ShipModifierFamily, ShipModifierOp,
};
use crate::raw::RawProperty;

/// Scenario-envelope combat participant resolved from co-located contact fleets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydratedCombatShipEnrollment {
    pub id: String,
    pub owner: String,
    pub simthing_id: SimThingId,
    pub hull_property: String,
    pub weapon_property: String,
    pub hull_capacity: f32,
    pub weapon_damage: f32,
    pub hp_recovery_per_tick: f32,
}

/// TP-COMBAT-ARENA-0 combat arena payload profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydratedCombatArenaPayload {
    pub id: String,
    pub system_target: String,
    pub hull_capacity: f32,
    pub hp_recovery_per_tick: f32,
    pub ships_per_side: u32,
    pub terran_weapon_damage: f32,
    pub pirate_weapon_damage: f32,
    pub owner_bonus_owner: Option<String>,
    pub owner_bonus_mult: Option<f32>,
    pub enrollments: Vec<HydratedCombatShipEnrollment>,
    pub transfers: Vec<ResourceTransferSpec>,
}

#[derive(Debug, Clone)]
pub struct ParsedCombatArenaPayload {
    pub id: String,
    pub system_target: Option<String>,
    pub hull_capacity: Option<f32>,
    pub hp_recovery_per_tick: Option<f32>,
    pub ships_per_side: Option<u32>,
    pub terran_weapon_damage: Option<f32>,
    pub pirate_weapon_damage: Option<f32>,
    pub owner_bonus_owner: Option<String>,
    pub owner_bonus_mult: Option<f32>,
    pub modifier_entries: Vec<(String, f32, crate::raw::RawSpan)>,
    pub span: crate::raw::RawSpan,
}

const COMBAT_TRANSFER_CAP: f32 = 9_999.0;

pub fn parse_combat_arena_payload(property: &RawProperty) -> Result<ParsedCombatArenaPayload, HydrateError> {
    let (header_id, block) = crate::hydrate_scenario::header_or_block_body(property, "combat_arena_payload")?;
    let mut id = header_id;
    let mut system_target = None;
    let mut hull_capacity = None;
    let mut hp_recovery_per_tick = None;
    let mut ships_per_side = None;
    let mut terran_weapon_damage = None;
    let mut pirate_weapon_damage = None;
    let mut owner_bonus_owner = None;
    let mut owner_bonus_mult = None;
    let mut modifier_entries = Vec::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = crate::hydrate_scenario::read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "system_target" => system_target = Some(crate::hydrate_scenario::read_scalar_text(field, "system_target")?),
            "hull_capacity" => hull_capacity = Some(crate::hydrate_scenario::read_scalar_f32(field, "hull_capacity")?),
            "hp_recovery_per_tick" => {
                hp_recovery_per_tick = Some(crate::hydrate_scenario::read_scalar_f32(field, "hp_recovery_per_tick")?);
            }
            "ships_per_side" => ships_per_side = Some(crate::hydrate_scenario::read_scalar_u32(field, "ships_per_side")?),
            "terran_weapon_damage" => {
                terran_weapon_damage = Some(crate::hydrate_scenario::read_scalar_f32(field, "terran_weapon_damage")?);
            }
            "pirate_weapon_damage" => {
                pirate_weapon_damage = Some(crate::hydrate_scenario::read_scalar_f32(field, "pirate_weapon_damage")?);
            }
            "owner_bonus_owner" => {
                owner_bonus_owner = Some(crate::hydrate_scenario::read_scalar_text(field, "owner_bonus_owner")?);
            }
            "owner_bonus_mult" => {
                owner_bonus_mult = Some(crate::hydrate_scenario::read_scalar_f32(field, "owner_bonus_mult")?);
            }
            "modifier" => modifier_entries.extend(parse_combat_modifier_entries(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported combat_arena_payload field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`combat_arena_payload` requires an id",
            Some(property.key.span.clone()),
        ));
    }

    Ok(ParsedCombatArenaPayload {
        id,
        system_target,
        hull_capacity,
        hp_recovery_per_tick,
        ships_per_side,
        terran_weapon_damage,
        pirate_weapon_damage,
        owner_bonus_owner,
        owner_bonus_mult,
        modifier_entries,
        span: property.key.span.clone(),
    })
}

fn parse_combat_modifier_entries(
    property: &RawProperty,
) -> Result<Vec<(String, f32, crate::raw::RawSpan)>, HydrateError> {
    let block = crate::hydrate_scenario::require_block(property, "modifier")?;
    let mut entries = Vec::new();
    for field in &block.properties {
        let amount = crate::hydrate_scenario::read_scalar_f32(field, &field.key.text)?;
        entries.push((field.key.text.clone(), amount, field.key.span.clone()));
    }
    if entries.is_empty() {
        return Err(HydrateError::new_spanned(
            "`combat_arena_payload.modifier` requires at least one key",
            Some(property.key.span.clone()),
        ));
    }
    Ok(entries)
}

pub fn finalize_combat_arena_payload(
    draft: ParsedCombatArenaPayload,
    owners: &[HydratedScenarioOwner],
    fleet_payloads: &[HydratedFleetShipPayload],
) -> Result<HydratedCombatArenaPayload, HydrateError> {
    let system_target = draft.system_target.ok_or_else(|| {
        HydrateError::new_spanned(
            "`combat_arena_payload.system_target` is required",
            Some(draft.span.clone()),
        )
    })?;
    let hull_capacity = draft.hull_capacity.unwrap_or(100.0);
    if !hull_capacity.is_finite() || hull_capacity <= 0.0 {
        return Err(HydrateError::new_spanned(
            "`combat_arena_payload.hull_capacity` must be finite and > 0",
            Some(draft.span.clone()),
        ));
    }
    let hp_recovery_per_tick = draft.hp_recovery_per_tick.unwrap_or(0.0);
    let ships_per_side = draft.ships_per_side.unwrap_or(1).max(1);
    let terran_weapon_damage = draft.terran_weapon_damage.unwrap_or(25.0);
    let pirate_weapon_damage = draft.pirate_weapon_damage.unwrap_or(30.0);

    let mut owner_bonus_mult = draft.owner_bonus_mult;
    for (key, amount, span) in &draft.modifier_entries {
        if key != "ship_weapon_damage_mult" {
            return Err(HydrateError::new_spanned(
                format!("unsupported combat_arena_payload modifier key `{key}`"),
                Some(span.clone()),
            ));
        }
        if owner_bonus_mult.is_some() {
            return Err(HydrateError::new_spanned(
                "duplicate ship_weapon_damage_mult in combat_arena_payload",
                Some(span.clone()),
            ));
        }
        owner_bonus_mult = Some(*amount);
    }

    for owner in ["terran", "pirate"] {
        if !owners.iter().any(|entry| entry.owner_key == owner) {
            return Err(HydrateError::new_spanned(
                format!("combat_arena_payload requires owner `{owner}`"),
                Some(draft.span.clone()),
            ));
        }
        if !fleet_payloads.iter().any(|payload| payload.owner == owner) {
            return Err(HydrateError::new_spanned(
                format!("combat_arena_payload requires fleet_ship_payload for `{owner}`"),
                Some(draft.span.clone()),
            ));
        }
    }

    let owner_bonus_owner = draft
        .owner_bonus_owner
        .or_else(|| owners.first().map(|owner| owner.owner_key.clone()));

    Ok(HydratedCombatArenaPayload {
        id: draft.id,
        system_target,
        hull_capacity,
        hp_recovery_per_tick,
        ships_per_side,
        terran_weapon_damage,
        pirate_weapon_damage,
        owner_bonus_owner,
        owner_bonus_mult,
        enrollments: Vec::new(),
        transfers: Vec::new(),
    })
}

pub fn attach_combat_contact_fleets(
    authority_root: &mut SimThing,
    payload: &HydratedCombatArenaPayload,
    fleet_payloads: &[HydratedFleetShipPayload],
) -> Result<Vec<HydratedCombatShipEnrollment>, HydrateError> {
    let contact_system_id = {
        let session = game_session_child(authority_root).ok_or_else(|| {
            HydrateError::new("combat arena requires GameSession on authority root")
        })?;
        let galaxy_map = game_session_galaxy_map(session).ok_or_else(|| {
            HydrateError::new("combat arena requires GalaxyMap child")
        })?;
        galaxy_map
            .children
            .iter()
            .find(|system| system_matches_target(system, &payload.system_target))
            .ok_or_else(|| {
                HydrateError::new(format!(
                    "combat contact system `{}` not found",
                    payload.system_target
                ))
            })?
            .id
    };

    let mut enrollments = Vec::new();
    for (owner, weapon_damage) in [("terran", payload.terran_weapon_damage), ("pirate", payload.pirate_weapon_damage)] {
        let fleet_payload = fleet_payloads
            .iter()
            .find(|entry| entry.owner == owner)
            .ok_or_else(|| HydrateError::new(format!("missing fleet payload for `{owner}`")))?;
        let enrollment = place_combat_contact_fleet(
            authority_root,
            contact_system_id,
            owner,
            weapon_damage,
            payload,
            fleet_payload,
        )?;
        enrollments.extend(enrollment);
    }

    if enrollments.len() < 2 {
        return Err(HydrateError::new(
            "combat arena requires at least two co-located hostile ships",
        ));
    }

    Ok(enrollments)
}

fn place_combat_contact_fleet(
    authority_root: &mut SimThing,
    system_id: SimThingId,
    owner: &str,
    weapon_damage: f32,
    payload: &HydratedCombatArenaPayload,
    fleet_payload: &HydratedFleetShipPayload,
) -> Result<Vec<HydratedCombatShipEnrollment>, HydrateError> {
    let session = game_session_child_mut(authority_root).expect("session");
    let galaxy_map = game_session_galaxy_map_mut(session).expect("galaxy map");
    let star_system = galaxy_map
        .children
        .iter_mut()
        .find(|system| system.id == system_id)
        .expect("contact system");
    let surface = star_system
        .children
        .iter_mut()
        .find_map(|planet| planet.children.iter_mut().find(|child| is_surface_gridcell(child)))
        .ok_or_else(|| HydrateError::new("contact system missing surface gridcell"))?;

    let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
    fleet.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value(owner),
    );

    let mut enrollments = Vec::new();
    for ship_index in 0..payload.ships_per_side {
        let enrollment_id = format!("{owner}_ship_{ship_index}");
        let hull_property = format!("combat_{enrollment_id}_hull");
        let weapon_property = format!("combat_{enrollment_id}_weapon");
        let mut ship = SimThing::new(SimThingKind::Cohort, 0);
        apply_participant_owner_flow_metadata(&mut ship, owner, 0, fleet_payload.upkeep_per_ship);
        enrollments.push(HydratedCombatShipEnrollment {
            id: enrollment_id,
            owner: owner.to_string(),
            simthing_id: ship.id,
            hull_property,
            weapon_property,
            hull_capacity: payload.hull_capacity,
            weapon_damage,
            hp_recovery_per_tick: payload.hp_recovery_per_tick,
        });
        fleet.add_child(ship);
    }
    surface.add_child(fleet);
    Ok(enrollments)
}

fn system_matches_target(system: &SimThing, target: &str) -> bool {
    let local = system_contact_target_id(system);
    local == target || target.ends_with(&format!("::{local}"))
}

fn system_contact_target_id(system: &SimThing) -> String {
    let row = system
        .properties
        .get(&simthing_spec::SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    let col = system
        .properties
        .get(&simthing_spec::SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    format!("row{row}_col{col}")
}

fn build_combat_transfers(
    enrollments: &[HydratedCombatShipEnrollment],
) -> Result<Vec<ResourceTransferSpec>, HydrateError> {
    let mut transfers = Vec::new();
    for attacker in enrollments {
        for defender in enrollments {
            if attacker.owner == defender.owner {
                continue;
            }
            transfers.push(ResourceTransferSpec {
                id: format!("{}_to_{}_damage", attacker.id, defender.id),
                source: PropertyKey::new("tp", &attacker.weapon_property),
                source_role: SubFieldRole::Amount,
                target: PropertyKey::new("tp", &defender.hull_property),
                target_role: SubFieldRole::Amount,
                amount: COMBAT_TRANSFER_CAP,
                order_band: 0,
                source_host_entity: None,
                target_host_entity: None,
            });
        }
    }
    Ok(transfers)
}

pub fn apply_combat_arena_to_game_mode(
    game_mode: &mut GameModeSpec,
    payload: &HydratedCombatArenaPayload,
    scenario_id: &str,
    install_targets: &mut BTreeMap<String, Vec<SimThingId>>,
) -> Result<(), HydrateError> {
    for enrollment in &payload.enrollments {
        if game_mode
            .properties
            .iter()
            .any(|prop| prop.name == enrollment.hull_property)
        {
            continue;
        }
        game_mode.properties.push(combat_hull_property_spec(
            &enrollment.hull_property,
            enrollment.hp_recovery_per_tick,
        ));
        game_mode.properties.push(combat_weapon_property_spec(
            &enrollment.weapon_property,
            enrollment.weapon_damage,
        ));
    }

    let mut economy = game_mode.resource_economy.clone().unwrap_or_default();
    economy.opt_in_mode = ResourceEconomyOptInMode::TransferOnly;
    economy.transfers.extend(payload.transfers.clone());
    game_mode.resource_economy = Some(economy);

    for enrollment in &payload.enrollments {
        let install_key = format!("combat_ship_{}", enrollment.id);
        install_targets
            .entry(install_key.clone())
            .or_default()
            .push(enrollment.simthing_id);
        game_mode.events.push(EventSpec {
            id: format!("{}_zero_hull_removal", enrollment.id),
            trigger: TriggerSpec::Threshold {
                target: ScopeRef::Current,
                property: PropertyKey::new("tp", &enrollment.hull_property),
                role: SubFieldRole::Amount,
                threshold: enrollment.hull_capacity,
                direction: TriggerDirection::Rising,
            },
            effects: vec![EffectSpec::Remove {
                target: ScopeRef::Current,
            }],
            cooldown: None,
            priority: Default::default(),
            install: InstallTargetSpec::ScenarioListed { target_id: install_key },
        });
    }

    if let (Some(owner), Some(mult)) = (&payload.owner_bonus_owner, payload.owner_bonus_mult) {
        let decoded = decode_ship_modifier_key_spanned("ship_weapon_damage_mult", &[], None)?;
        if !matches!(decoded.family, ShipModifierFamily::Ship) || decoded.op != ShipModifierOp::Mult {
            return Err(HydrateError::new(
                "combat owner bonus must decode as ship_weapon_damage_mult",
            ));
        }
        for enrollment in payload
            .enrollments
            .iter()
            .filter(|ship| ship.owner == *owner)
        {
            let overlay_id = format!("{scenario_id}::{}::ship_weapon_damage_mult", enrollment.id);
            game_mode.overlays.push(OverlaySpec {
                id: overlay_id,
                display_name: format!("{} owner combat bonus", enrollment.id),
                targets_property: format!("tp::{}", enrollment.weapon_property),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.0 + mult))],
                lifecycle: OverlayLifecycle::Permanent,
                kind: OverlayKind::Policy,
                source: OverlaySource::Player,
                install: InstallTargetSpec::ScenarioListed {
                    target_id: format!("combat_ship_{}", enrollment.id),
                },
            });
        }
    }

    Ok(())
}

pub fn complete_combat_arena_payload(
    mut payload: HydratedCombatArenaPayload,
    authority_root: &mut SimThing,
    fleet_payloads: &[HydratedFleetShipPayload],
) -> Result<HydratedCombatArenaPayload, HydrateError> {
    let enrollments = attach_combat_contact_fleets(authority_root, &payload, fleet_payloads)?;
    payload.enrollments = enrollments;
    payload.transfers = build_combat_transfers(&payload.enrollments)?;
    Ok(payload)
}

/// Seed per-ship combat hull/weapon columns so live-slot transfer resolution
/// binds each unique property name to its owning ship node.
pub fn seed_combat_property_columns_on_tree(
    authority_root: &mut SimThing,
    registry: &simthing_core::DimensionRegistry,
    enrollments: &[HydratedCombatShipEnrollment],
) -> Result<(), HydrateError> {
    for enrollment in enrollments {
        let ship = find_simthing_mut(authority_root, enrollment.simthing_id).ok_or_else(|| {
            HydrateError::new(format!(
                "combat enrollment `{}` missing ship node",
                enrollment.id
            ))
        })?;
        for (name, amount) in [
            (enrollment.hull_property.as_str(), 0.0_f32),
            (enrollment.weapon_property.as_str(), enrollment.weapon_damage),
        ] {
            let property_id = registry
                .id_of("tp", name)
                .ok_or_else(|| HydrateError::new(format!("missing combat property `{name}`")))?;
            let layout = registry.property(property_id).layout.clone();
            let mut value = registry.property(property_id).default_value();
            value.set_role(&SubFieldRole::Amount, &layout, amount);
            ship.add_property(property_id, value);
        }
    }
    Ok(())
}

fn find_simthing_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
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

fn combat_hull_property_spec(name: &str, recovery_per_tick: f32) -> PropertySpec {
    let governed_by = if recovery_per_tick > 0.0 {
        Some(SubFieldRole::Named("recovery".into()))
    } else {
        None
    };
    let mut sub_fields = vec![simthing_core::SubFieldSpec {
        role: SubFieldRole::Amount,
        width: 1,
        clamp: simthing_core::ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "Damage".into(),
        display_range: None,
        governed_by,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }];
    if recovery_per_tick > 0.0 {
        sub_fields.push(simthing_core::SubFieldSpec {
            role: SubFieldRole::Named("recovery".into()),
            width: 1,
            clamp: simthing_core::ClampBehavior::Unbounded,
            velocity_max: None,
            default: recovery_per_tick,
            display_name: "Recovery".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        });
    }
    PropertySpec {
        id: format!("tp_{name}"),
        namespace: "tp".into(),
        name: name.into(),
        display_name: name.into(),
        description: String::new(),
        sub_fields,
    }
}

fn combat_weapon_property_spec(name: &str, weapon_damage: f32) -> PropertySpec {
    PropertySpec {
        id: format!("tp_{name}"),
        namespace: "tp".into(),
        name: name.into(),
        display_name: name.into(),
        description: String::new(),
        sub_fields: vec![simthing_core::SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: simthing_core::ClampBehavior::Unbounded,
            velocity_max: None,
            default: weapon_damage,
            display_name: "Weapon".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

fn game_session_child(root: &SimThing) -> Option<&SimThing> {
    root.children
        .iter()
        .find(|child| child.kind == SimThingKind::GameSession)
}

fn game_session_galaxy_map(session: &SimThing) -> Option<&SimThing> {
    session
        .children
        .iter()
        .find(|child| is_galaxy_map_entity(child))
}

fn game_session_child_mut(root: &mut SimThing) -> Option<&mut SimThing> {
    root.children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
}

fn game_session_galaxy_map_mut<'a>(session: &'a mut SimThing) -> Option<&'a mut SimThing> {
    session
        .children
        .iter_mut()
        .find(|child| is_galaxy_map_entity(child))
}