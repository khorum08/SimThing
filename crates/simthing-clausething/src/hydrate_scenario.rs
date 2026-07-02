//! PR2/PR3/PR4/PR5/PR6 scenario-container hydration over existing generic authoring surfaces.
//!
//! This module composes a ClauseScript `scenario` document into a root
//! `SimThing` tree plus `GameModeSpec` property/overlay declarations, bounded
//! PR3 grid placement/link metadata, and PR4 scenario-contained field operators.
//! It does not add driver/runtime semantics, arbitrary graph topology, movement,
//! routes, or pathfinding.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    OverlayKind, OverlayLifecycle, OverlaySource, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_spec::spec::game_mode::GameModeSpec;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::region_field::{CommitmentEffectSpec, MappingExecutionProfile};
use simthing_spec::spec::scenario::{
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, OWNER_COLOR_INDEX_PROPERTY_ID,
    OWNER_FLOW_OWNER_REF_PROPERTY_ID, SCENARIO_SCHEMA_VERSION, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, SimThingScenarioGrid, SimThingScenarioProvenance,
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_scenario_metadata_to_root,
    deserialize_scenario_authority, make_galaxy_map, make_owner_entity,
    scenario_metadata_string_value, scenario_metadata_u32_value, structural_property_value_u32,
};
use simthing_spec::spec::stress_compose::StressComposeSpec;
use simthing_spec::spec::w_impedance_compose::WImpedanceComposeSpec;

use crate::error::HydrateError;
use crate::hydrate_field_operator::hydrate_field_operator_property;
use crate::hydrate_palma_feedstock::{
    HydratedScenarioPalmaFeedstock, PR5_MAX_SCENARIO_PALMA_FEEDSTOCK, finalize_palma_feedstock,
    parse_palma_feedstock_property,
};
use crate::hydrate_scenario_commitment::{
    HydratedScenarioCommitment, PR6_MAX_SCENARIO_COMMITMENT, ParsedCommitmentEffectDraft,
    finalize_scenario_commitment, parse_commitment_property,
};
use crate::raw::{RawBlock, RawDocument, RawHeaderValue, RawProperty, RawSpan, RawValue};

pub const PR3_MAX_LINK_FANOUT: usize = 4;
/// PR4 admits one scenario-contained SaturatingFlux field operator per document.
pub const PR4_MAX_SCENARIO_FIELD_OPERATORS: usize = 1;

const FORBIDDEN_SCENARIO_FIELDS: &[&str] = &[
    "adjacency",
    "edge",
    "route",
    "path",
    "predecessor",
    "movement",
    "movement_order",
    "waypoint",
    "destination",
    "frontline",
    "border",
    "pathfinding",
    "arbitrary_graph",
    "non_grid_topology",
];

const FORBIDDEN_NODE_FIELDS: &[&str] = &[
    "link",
    "adjacency",
    "edge",
    "route",
    "path",
    "predecessor",
    "movement",
    "movement_order",
    "waypoint",
    "destination",
    "frontline",
    "border",
    "pathfinding",
    "arbitrary_graph",
    "non_grid_topology",
];

/// Scenario-container hydration result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedScenarioPack {
    pub scenario_id: String,
    pub metadata: BTreeMap<String, String>,
    pub game_mode: GameModeSpec,
    pub root: SimThing,
    /// Authored node tree preserving ids and per-node declarations that the
    /// runtime `SimThing` tree cannot hold before spec admission/registry compile.
    pub root_node: HydratedScenarioNode,
    /// Existing driver `Scenario` surface consumes this shape; no driver
    /// dependency is required at the ClauseThing front-end layer.
    pub install_targets: BTreeMap<String, Vec<SimThingId>>,
    /// PR3 bounded grid metadata for scenario locations. This is authoring /
    /// admission feedstock shaped like RegionField pressure placements, not a
    /// runtime topology object.
    pub grid_metadata: HydratedScenarioGridMetadata,
    /// PR4 optional W impedance compose lowered from a scenario field operator.
    pub w_impedance_compose: Option<WImpedanceComposeSpec>,
    /// PR4 optional stress compose lowered from a scenario field operator.
    pub stress_compose: Option<StressComposeSpec>,
    /// PR5 optional PALMA W/D feedstock metadata for later driver/admission consumption.
    pub palma_feedstock: Option<HydratedScenarioPalmaFeedstock>,
    /// PR6 optional FIELD_POLICY threshold / commitment feedstock metadata.
    pub commitment: Option<HydratedScenarioCommitment>,
    /// TP-BASE-EMBED-0 embedded producer-owned base scenarios consumed through
    /// the scenario-container grammar. Runtime ownership remains with this pack.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embedded_static_galaxy_scenarios: Vec<HydratedEmbeddedStaticGalaxyScenario>,
    /// TP-OWNER-SIBLINGS-0 canonical authoring tree:
    /// Scenario -> GameSession -> {Owner..., GalaxyMap}. The legacy `root`
    /// remains unchanged for existing scenario-container consumers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_root: Option<SimThing>,
    /// Authored owner declarations lowered into direct GameSession children.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<HydratedScenarioOwner>,
    /// TP-OWNERSHIP-COLUMNS-0 deterministic ownership-volume assignments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ownership_volumes: Vec<HydratedOwnershipVolume>,
}

/// Authored scenario node declaration paired with its generated `SimThingId`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedScenarioNode {
    pub id: String,
    pub simthing_id: SimThingId,
    pub kind: SimThingKind,
    pub display_name: String,
    pub properties: Vec<PropertySpec>,
    pub overlays: Vec<OverlaySpec>,
    pub children: Vec<HydratedScenarioNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedScenarioGridMetadata {
    pub grid_size: u32,
    pub max_fanout: usize,
    pub placements: Vec<HydratedScenarioGridPlacement>,
    pub links: Vec<HydratedScenarioLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedScenarioGridPlacement {
    pub location_id: String,
    pub target_id: String,
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HydratedScenarioLink {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedEmbeddedStaticGalaxyScenario {
    pub id: String,
    pub namespace: String,
    pub scenario_id: String,
    pub map_quality_status: String,
    pub provenance: SimThingScenarioProvenance,
    pub source_structural_grid: SimThingScenarioGrid,
    pub namespaced_placements: Vec<HydratedScenarioGridPlacement>,
    pub namespaced_links: Vec<HydratedScenarioLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedScenarioOwner {
    pub id: String,
    pub owner_key: String,
    pub display_name: String,
    pub archetype: String,
    pub color_index: Option<u32>,
    pub stockpile_seed: Option<u32>,
    pub stockpile_capacity: Option<u32>,
    pub policy_profile: Option<String>,
    pub personality_profile: Option<String>,
    pub capability_profile: Option<String>,
    pub simthing_id: SimThingId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedOwnershipVolume {
    pub id: String,
    pub owner: String,
    pub count: u32,
    pub selection: String,
    pub seed: Option<u64>,
    pub adjacent_to: Option<String>,
    pub anchor_row: u32,
    pub anchor_col: u32,
    pub assigned_systems: Vec<HydratedOwnedSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedOwnedSystem {
    pub location_id: String,
    pub target_id: String,
    pub row: u32,
    pub col: u32,
    pub owner_ref: String,
}

#[derive(Debug, Clone)]
struct ParsedOwnershipVolume {
    id: String,
    owner: String,
    count: u32,
    selection: String,
    seed: Option<u64>,
    anchor_row: Option<u32>,
    anchor_col: Option<u32>,
    adjacent_to: Option<String>,
    span: RawSpan,
}

pub fn hydrate_scenario(document: &RawDocument) -> Result<HydratedScenarioPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "scenario hydration expects exactly one top-level `scenario` block",
        ));
    }

    let scenario = &root.properties[0];
    if scenario.key.text != "scenario" {
        return Err(HydrateError::new_spanned(
            "top-level block must be `scenario`",
            Some(scenario.key.span.clone()),
        ));
    }

    let (scenario_id, body) = header_or_block_body(scenario, "scenario")?;
    if scenario_id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`scenario` requires an id",
            Some(scenario.key.span.clone()),
        ));
    }
    let mut metadata = BTreeMap::new();
    let mut locations = Vec::new();
    let mut seen_node_ids = BTreeSet::new();
    seen_node_ids.insert(scenario_id.clone());
    let mut seen_property_ids = BTreeSet::new();
    let mut seen_overlay_ids = BTreeSet::new();
    let mut raw_links = Vec::new();
    let mut owners = Vec::new();
    let mut seen_owner_keys = BTreeSet::new();
    let mut ownership_volume_drafts = Vec::new();
    let mut seen_ownership_volume_ids = BTreeSet::new();
    let mut embedded_static_galaxy_scenarios = Vec::new();
    let mut embedded_placements = Vec::new();
    let mut embedded_grid_size = None;
    let mut seen_location_targets = BTreeSet::new();
    let mut field_operator_count = 0_usize;
    let mut field_operator_pack = None;
    let mut palma_feedstock_count = 0_usize;
    let mut palma_feedstock_draft = None;
    let mut commitment_count = 0_usize;
    let mut commitment_draft = None;

    for field in &body.properties {
        reject_forbidden_scenario_field(field)?;
        match field.key.text.as_str() {
            "id" => {}
            "metadata" => metadata = parse_metadata_block(field)?,
            "location" => {
                let node = parse_node(
                    field,
                    Some(SimThingKind::Location),
                    &mut seen_node_ids,
                    &mut seen_property_ids,
                    &mut seen_overlay_ids,
                )?;
                if !seen_location_targets.insert(node.id.clone()) {
                    return Err(HydrateError::new_spanned(
                        format!("duplicate scenario location-target id `{}`", node.id),
                        Some(field.key.span.clone()),
                    ));
                }
                locations.push(node);
            }
            "static_galaxy_scenario" => {
                let embedded = parse_static_galaxy_scenario(field, &mut seen_location_targets)?;
                let frame_edge = embedded
                    .source_structural_grid
                    .frame
                    .width
                    .max(embedded.source_structural_grid.frame.height);
                embedded_grid_size = Some(embedded_grid_size.unwrap_or(0).max(frame_edge));
                embedded_placements.extend(embedded.namespaced_placements.iter().cloned());
                embedded_static_galaxy_scenarios.push(embedded);
            }
            "owner" => {
                let owner = parse_owner(field)?;
                if !seen_owner_keys.insert(owner.owner_key.clone()) {
                    return Err(HydrateError::new_spanned(
                        format!("duplicate scenario owner id `{}`", owner.owner_key),
                        Some(field.key.span.clone()),
                    ));
                }
                owners.push(owner);
            }
            "ownership_volume" => {
                let volume = parse_ownership_volume(field)?;
                if !seen_ownership_volume_ids.insert(volume.id.clone()) {
                    return Err(HydrateError::new_spanned(
                        format!("duplicate ownership_volume id `{}`", volume.id),
                        Some(field.key.span.clone()),
                    ));
                }
                ownership_volume_drafts.push(volume);
            }
            "link" => raw_links.push(parse_link(field)?),
            "field_operator" => {
                field_operator_count += 1;
                if field_operator_count > PR4_MAX_SCENARIO_FIELD_OPERATORS {
                    return Err(HydrateError::new_spanned(
                        format!(
                            "scenario admits at most {PR4_MAX_SCENARIO_FIELD_OPERATORS} field_operator block"
                        ),
                        Some(field.key.span.clone()),
                    ));
                }
                if field_operator_pack.is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate scenario field_operator block",
                        Some(field.key.span.clone()),
                    ));
                }
                field_operator_pack = Some(hydrate_field_operator_property(field)?);
            }
            "palma_feedstock" => {
                palma_feedstock_count += 1;
                if palma_feedstock_count > PR5_MAX_SCENARIO_PALMA_FEEDSTOCK {
                    return Err(HydrateError::new_spanned(
                        format!(
                            "scenario admits at most {PR5_MAX_SCENARIO_PALMA_FEEDSTOCK} palma_feedstock block"
                        ),
                        Some(field.key.span.clone()),
                    ));
                }
                if palma_feedstock_draft.is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate scenario palma_feedstock block",
                        Some(field.key.span.clone()),
                    ));
                }
                palma_feedstock_draft = Some(parse_palma_feedstock_property(field)?);
            }
            "commitment" => {
                commitment_count += 1;
                if commitment_count > PR6_MAX_SCENARIO_COMMITMENT {
                    return Err(HydrateError::new_spanned(
                        format!(
                            "scenario admits at most {PR6_MAX_SCENARIO_COMMITMENT} commitment block"
                        ),
                        Some(field.key.span.clone()),
                    ));
                }
                if commitment_draft.is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate scenario commitment block",
                        Some(field.key.span.clone()),
                    ));
                }
                commitment_draft = Some(parse_commitment_property(field)?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported scenario field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if locations.is_empty() && embedded_static_galaxy_scenarios.is_empty() {
        return Err(HydrateError::new(
            "scenario requires at least one `location` or `static_galaxy_scenario` block",
        ));
    }

    let grid_metadata = build_grid_metadata(
        &locations,
        raw_links,
        embedded_placements,
        embedded_grid_size,
    )?;

    let display_name = metadata
        .get("display_name")
        .cloned()
        .unwrap_or_else(|| scenario_id.clone());
    let description = metadata.get("description").cloned().unwrap_or_default();

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut root_node = HydratedScenarioNode {
        id: scenario_id.clone(),
        simthing_id: root.id,
        kind: SimThingKind::World,
        display_name: display_name.clone(),
        properties: Vec::new(),
        overlays: Vec::new(),
        children: locations,
    };

    let mut properties = Vec::new();
    let mut overlays = Vec::new();
    let mut install_targets = BTreeMap::new();
    for child in &root_node.children {
        flatten_node(child, &mut properties, &mut overlays, &mut install_targets);
        root.add_child(simthing_from_node(child));
    }
    install_targets.insert(root_node.id.clone(), vec![root_node.simthing_id]);

    let mut game_mode = GameModeSpec {
        id: scenario_id.clone(),
        display_name,
        description,
        ..GameModeSpec::default()
    };
    game_mode.metadata.description = metadata.get("description").cloned().unwrap_or_default();
    game_mode.metadata.icon = metadata.get("icon").cloned();
    if let Some(tags) = metadata.get("tags") {
        game_mode.metadata.tags = tags
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(ToOwned::to_owned)
            .collect();
    }
    game_mode.properties = properties;
    game_mode.overlays = overlays;

    let palma_feedstock = if let Some(draft) = palma_feedstock_draft {
        let operator = field_operator_pack.as_ref().ok_or_else(|| {
            HydrateError::new("palma_feedstock requires a scenario field_operator block")
        })?;
        Some(finalize_palma_feedstock(draft, operator)?)
    } else {
        None
    };

    let commitment = if let Some(mut draft) = commitment_draft {
        let operator = field_operator_pack.as_ref().ok_or_else(|| {
            HydrateError::new("commitment requires a scenario field_operator block")
        })?;
        let effect = resolve_commitment_effect(draft.effect.take(), &root_node)?;
        Some(finalize_scenario_commitment(draft, operator, effect)?)
    } else {
        None
    };

    let mut w_impedance_compose = None;
    let mut stress_compose = None;
    if let Some(mut operator) = field_operator_pack {
        if let Some(finalized) = commitment.as_ref() {
            if let Some(field) = operator.game_mode.region_fields.first_mut() {
                field.parent_formula = Some(finalized.parent_formula.clone());
                field.reduction = Some(finalized.reduction.clone());
                field.commitment = Some(finalized.metadata.commitment.clone());
            }
        }
        game_mode
            .region_fields
            .extend(operator.game_mode.region_fields);
        game_mode.mapping_execution_profile = MappingExecutionProfile::Disabled;
        w_impedance_compose = operator.w_impedance_compose;
        stress_compose = operator.stress_compose;
    }

    root_node.simthing_id = root.id;
    let ownership_volumes = finalize_ownership_volumes(
        &ownership_volume_drafts,
        &owners,
        embedded_static_galaxy_scenarios.first(),
    )?;
    let authority_root = if owners.is_empty() {
        None
    } else {
        Some(build_authority_root(
            &scenario_id,
            &metadata,
            &owners,
            embedded_static_galaxy_scenarios.first(),
            &ownership_volumes,
        ))
    };
    Ok(HydratedScenarioPack {
        scenario_id,
        metadata,
        game_mode,
        root,
        root_node,
        install_targets,
        grid_metadata,
        w_impedance_compose,
        stress_compose,
        palma_feedstock,
        commitment: commitment.map(|finalized| finalized.metadata),
        embedded_static_galaxy_scenarios,
        authority_root,
        owners,
        ownership_volumes,
    })
}

fn parse_owner(property: &RawProperty) -> Result<HydratedScenarioOwner, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "owner")?;
    let mut id = header_id;
    let mut owner_key = None;
    let mut display_name = None;
    let mut archetype = None;
    let mut stockpile_seed = None;
    let mut stockpile_capacity = None;
    let mut color_index = None;
    let mut policy_profile = None;
    let mut personality_profile = None;
    let mut capability_profile = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "owner_key" => owner_key = Some(read_scalar_text(field, "owner_key")?),
            "display_name" | "name" => {
                display_name = Some(read_scalar_text(field, &field.key.text)?);
            }
            "archetype" => archetype = Some(read_scalar_text(field, "archetype")?),
            "stockpile_seed" | "stockpile_current" => {
                stockpile_seed = Some(read_scalar_u32(field, &field.key.text)?);
            }
            "stockpile_capacity" => {
                stockpile_capacity = Some(read_scalar_u32(field, "stockpile_capacity")?);
            }
            "color_index" => color_index = Some(read_scalar_u32(field, "color_index")?),
            "policy_profile" => policy_profile = Some(read_scalar_text(field, "policy_profile")?),
            "personality_profile" => {
                personality_profile = Some(read_scalar_text(field, "personality_profile")?);
            }
            "capability_profile" => {
                capability_profile = Some(read_scalar_text(field, "capability_profile")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported owner field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`owner` requires an id",
            Some(property.key.span.clone()),
        ));
    }
    let owner_key = owner_key.unwrap_or_else(|| id.clone());
    if owner_key.trim().is_empty() {
        return Err(HydrateError::new_spanned(
            "`owner.owner_key` must be non-empty",
            Some(property.key.span.clone()),
        ));
    }
    let display_name = display_name.unwrap_or_else(|| id.clone());
    let archetype = archetype.unwrap_or_else(|| owner_key.clone());
    let owner = HydratedScenarioOwner {
        id,
        owner_key,
        display_name,
        archetype,
        color_index,
        stockpile_seed,
        stockpile_capacity,
        policy_profile,
        personality_profile,
        capability_profile,
        simthing_id: SimThingId::new(),
    };
    Ok(owner)
}

fn build_authority_root(
    scenario_id: &str,
    metadata: &BTreeMap<String, String>,
    owners: &[HydratedScenarioOwner],
    embedded: Option<&HydratedEmbeddedStaticGalaxyScenario>,
    ownership_volumes: &[HydratedOwnershipVolume],
) -> SimThing {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = embedded
        .map(|embedded| embedded.provenance.clone())
        .unwrap_or_else(|| SimThingScenarioProvenance {
            source: "ClauseThingScenarioContainer".to_string(),
            generator_shape: "scenario_container".to_string(),
            ..SimThingScenarioProvenance::default()
        });
    apply_scenario_metadata_to_root(&mut root, scenario_id, &provenance, SCENARIO_SCHEMA_VERSION);

    let mut session = SimThing::new(SimThingKind::GameSession, 0);
    for owner in owners {
        session.add_child(owner_simthing(owner));
    }
    let map_id = embedded
        .map(|embedded| {
            namespace_id(
                &embedded.namespace,
                &embedded.source_structural_grid.map_container_id,
            )
        })
        .unwrap_or_else(|| format!("{scenario_id}::galaxy_map"));
    let display_name = metadata
        .get("display_name")
        .map(String::as_str)
        .unwrap_or(scenario_id);
    let mut galaxy_map = make_galaxy_map(&map_id, display_name);
    if let Some(embedded) = embedded {
        attach_embedded_gridcells(&mut galaxy_map, embedded, ownership_volumes);
    }
    session.add_child(galaxy_map);
    root.add_child(session);
    root
}

fn attach_embedded_gridcells(
    galaxy_map: &mut SimThing,
    embedded: &HydratedEmbeddedStaticGalaxyScenario,
    ownership_volumes: &[HydratedOwnershipVolume],
) {
    let mut owner_by_target = BTreeMap::new();
    for volume in ownership_volumes {
        for system in &volume.assigned_systems {
            owner_by_target.insert(system.target_id.clone(), system.owner_ref.clone());
        }
    }
    for placement in &embedded.namespaced_placements {
        let mut gridcell = SimThing::new(SimThingKind::Location, 0);
        apply_gridcell_role_metadata(&mut gridcell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
        gridcell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(placement.col),
        );
        gridcell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(placement.row),
        );
        if let Some(owner_ref) = owner_by_target.get(&placement.target_id) {
            gridcell.add_property(
                OWNER_FLOW_OWNER_REF_PROPERTY_ID,
                scenario_metadata_string_value(owner_ref),
            );
        }
        galaxy_map.add_child(gridcell);
    }
}

fn owner_simthing(owner: &HydratedScenarioOwner) -> SimThing {
    let mut simthing = make_owner_entity(&owner.owner_key, &owner.display_name, &owner.archetype);
    simthing.id = owner.simthing_id;
    if let Some(color_index) = owner.color_index {
        simthing.add_property(
            OWNER_COLOR_INDEX_PROPERTY_ID,
            scenario_metadata_u32_value(color_index),
        );
    }
    if let Some(stockpile_seed) = owner.stockpile_seed {
        apply_owner_silo_metadata(&mut simthing, stockpile_seed, owner.stockpile_capacity);
    }
    simthing
}

fn parse_ownership_volume(property: &RawProperty) -> Result<ParsedOwnershipVolume, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "ownership_volume")?;
    let mut id = header_id;
    let mut owner = None;
    let mut count = None;
    let mut selection = None;
    let mut seed = None;
    let mut anchor_row = None;
    let mut anchor_col = None;
    let mut adjacent_to = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "owner" => owner = Some(read_scalar_text(field, "owner")?),
            "count" => count = Some(read_scalar_u32(field, "count")?),
            "selection" => selection = Some(read_scalar_text(field, "selection")?),
            "seed" => seed = Some(read_scalar_u64(field, "seed")?),
            "anchor_row" => anchor_row = Some(read_scalar_u32(field, "anchor_row")?),
            "anchor_col" => anchor_col = Some(read_scalar_u32(field, "anchor_col")?),
            "adjacent_to" => adjacent_to = Some(read_scalar_text(field, "adjacent_to")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported ownership_volume field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`ownership_volume` requires an id",
            Some(property.key.span.clone()),
        ));
    }
    let owner = require_field(owner, "owner", property)?;
    if owner.trim().is_empty() {
        return Err(HydrateError::new_spanned(
            "`ownership_volume.owner` must be non-empty",
            Some(property.key.span.clone()),
        ));
    }
    let count = require_field(count, "count", property)?;
    if count == 0 {
        return Err(HydrateError::new_spanned(
            "`ownership_volume.count` must be greater than zero",
            Some(property.key.span.clone()),
        ));
    }
    let selection = selection.unwrap_or_else(|| "chebyshev_contiguous".to_string());
    if selection != "chebyshev_contiguous" {
        return Err(HydrateError::new_spanned(
            format!("unsupported ownership_volume selection `{selection}`"),
            Some(property.key.span.clone()),
        ));
    }

    Ok(ParsedOwnershipVolume {
        id,
        owner,
        count,
        selection,
        seed,
        anchor_row,
        anchor_col,
        adjacent_to,
        span: property.key.span.clone(),
    })
}

fn finalize_ownership_volumes(
    drafts: &[ParsedOwnershipVolume],
    owners: &[HydratedScenarioOwner],
    embedded: Option<&HydratedEmbeddedStaticGalaxyScenario>,
) -> Result<Vec<HydratedOwnershipVolume>, HydrateError> {
    if drafts.is_empty() {
        return Ok(Vec::new());
    }
    let embedded = embedded.ok_or_else(|| {
        HydrateError::new_spanned(
            "ownership_volume requires an embedded static_galaxy_scenario",
            drafts.first().map(|draft| draft.span.clone()),
        )
    })?;
    let owner_keys: BTreeSet<_> = owners
        .iter()
        .map(|owner| owner.owner_key.as_str())
        .collect();
    let mut assigned_by_target: BTreeMap<String, String> = BTreeMap::new();
    let mut finalized = Vec::new();

    for draft in drafts {
        if !owner_keys.contains(draft.owner.as_str()) {
            return Err(HydrateError::new_spanned(
                format!(
                    "ownership_volume `{}` references unknown owner `{}`",
                    draft.id, draft.owner
                ),
                Some(draft.span.clone()),
            ));
        }
        let (selected, anchor_row, anchor_col) =
            select_ownership_systems(draft, embedded, &finalized, &assigned_by_target)?;
        let mut assigned_systems = Vec::new();
        for placement in selected {
            if let Some(previous_owner) = assigned_by_target.get(&placement.target_id) {
                return Err(HydrateError::new_spanned(
                    format!(
                        "ownership_volume `{}` overlaps `{}` already owned by `{previous_owner}`",
                        draft.id, placement.target_id
                    ),
                    Some(draft.span.clone()),
                ));
            }
            assigned_by_target.insert(placement.target_id.clone(), draft.owner.clone());
            assigned_systems.push(HydratedOwnedSystem {
                location_id: placement.location_id.clone(),
                target_id: placement.target_id.clone(),
                row: placement.row,
                col: placement.col,
                owner_ref: draft.owner.clone(),
            });
        }
        finalized.push(HydratedOwnershipVolume {
            id: draft.id.clone(),
            owner: draft.owner.clone(),
            count: draft.count,
            selection: draft.selection.clone(),
            seed: draft.seed,
            adjacent_to: draft.adjacent_to.clone(),
            anchor_row,
            anchor_col,
            assigned_systems,
        });
    }

    Ok(finalized)
}

fn select_ownership_systems(
    draft: &ParsedOwnershipVolume,
    embedded: &HydratedEmbeddedStaticGalaxyScenario,
    finalized: &[HydratedOwnershipVolume],
    assigned_by_target: &BTreeMap<String, String>,
) -> Result<(Vec<HydratedScenarioGridPlacement>, u32, u32), HydrateError> {
    let placements = sorted_placements(&embedded.namespaced_placements);
    let count = draft.count as usize;
    if count > placements.len() {
        return Err(HydrateError::new_spanned(
            format!(
                "ownership_volume `{}` requests {} systems but only {} placements exist",
                draft.id,
                draft.count,
                placements.len()
            ),
            Some(draft.span.clone()),
        ));
    }
    let reference_coords = if let Some(reference) = draft.adjacent_to.as_ref() {
        let volume = finalized
            .iter()
            .find(|volume| &volume.id == reference)
            .ok_or_else(|| {
                HydrateError::new_spanned(
                    format!(
                        "ownership_volume `{}` references unknown adjacent_to volume `{reference}`",
                        draft.id
                    ),
                    Some(draft.span.clone()),
                )
            })?;
        Some(
            volume
                .assigned_systems
                .iter()
                .map(|system| (system.row, system.col))
                .collect::<BTreeSet<_>>(),
        )
    } else {
        None
    };
    if let Some(reference_coords) = reference_coords.as_ref() {
        let mut candidates = placements
            .into_iter()
            .filter(|placement| !assigned_by_target.contains_key(&placement.target_id))
            .map(|placement| {
                let distance = reference_coords
                    .iter()
                    .map(|coord| chebyshev_distance((placement.row, placement.col), *coord))
                    .min()
                    .unwrap_or(u32::MAX);
                (distance, placement)
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            (left.0, left.1.row, left.1.col, left.1.target_id.as_str()).cmp(&(
                right.0,
                right.1.row,
                right.1.col,
                right.1.target_id.as_str(),
            ))
        });
        if candidates.len() >= count {
            let selected = candidates
                .into_iter()
                .take(count)
                .map(|(_, placement)| placement)
                .collect::<Vec<_>>();
            let anchor = selected[0].clone();
            return Ok((selected, anchor.row, anchor.col));
        }
    } else {
        let anchors: Vec<_> = if let (Some(anchor_row), Some(anchor_col)) =
            (draft.anchor_row, draft.anchor_col)
        {
            vec![placements
                    .iter()
                    .position(|placement| {
                        placement.row == anchor_row && placement.col == anchor_col
                    })
                    .ok_or_else(|| {
                        HydrateError::new_spanned(
                            format!(
                                "ownership_volume `{}` anchor ({anchor_row},{anchor_col}) is not an embedded placement",
                                draft.id
                            ),
                            Some(draft.span.clone()),
                        )
                    })?]
        } else {
            let mut anchors: Vec<_> = placements
                .iter()
                .enumerate()
                .map(|(index, _)| index)
                .collect();
            rotate_by_seed(&mut anchors, draft.seed);
            anchors
        };
        for anchor_index in anchors {
            let anchor = placements[anchor_index].clone();
            let mut candidates = placements.clone();
            candidates.sort_by(|left, right| {
                (
                    chebyshev_distance((left.row, left.col), (anchor.row, anchor.col)),
                    left.row,
                    left.col,
                    left.target_id.as_str(),
                )
                    .cmp(&(
                        chebyshev_distance((right.row, right.col), (anchor.row, anchor.col)),
                        right.row,
                        right.col,
                        right.target_id.as_str(),
                    ))
            });
            if candidates.len() >= count {
                return Ok((
                    candidates.into_iter().take(count).collect(),
                    anchor.row,
                    anchor.col,
                ));
            }
        }
    }
    Err(HydrateError::new_spanned(
        format!(
            "ownership_volume `{}` could not select {} Chebyshev-contiguous systems",
            draft.id, draft.count
        ),
        Some(draft.span.clone()),
    ))
}

fn sorted_placements(
    placements: &[HydratedScenarioGridPlacement],
) -> Vec<HydratedScenarioGridPlacement> {
    let mut sorted = placements.to_vec();
    sorted.sort_by(|left, right| {
        (left.row, left.col, &left.target_id).cmp(&(right.row, right.col, &right.target_id))
    });
    sorted
}

fn rotate_by_seed(indices: &mut Vec<usize>, seed: Option<u64>) {
    if indices.is_empty() {
        return;
    }
    let offset = seed.unwrap_or(0) as usize % indices.len();
    indices.rotate_left(offset);
}

fn chebyshev_distance(left: (u32, u32), right: (u32, u32)) -> u32 {
    left.0.abs_diff(right.0).max(left.1.abs_diff(right.1))
}

fn parse_static_galaxy_scenario(
    property: &RawProperty,
    seen_location_targets: &mut BTreeSet<String>,
) -> Result<HydratedEmbeddedStaticGalaxyScenario, HydrateError> {
    let (header_id, block) = header_or_block_body(property, "static_galaxy_scenario")?;
    let mut id = header_id;
    let mut namespace = None;
    let mut source_json = None;
    let mut map_quality_status = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "source_json" | "include_json" => {
                source_json = Some(read_scalar_text(field, &field.key.text)?);
            }
            "map_quality_status" => {
                map_quality_status = Some(read_scalar_text(field, "map_quality_status")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported static_galaxy_scenario field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            "`static_galaxy_scenario` requires an id",
            Some(property.key.span.clone()),
        ));
    }
    let namespace = require_field(namespace, "namespace", property)?;
    if namespace.trim().is_empty() {
        return Err(HydrateError::new_spanned(
            "`static_galaxy_scenario.namespace` must be non-empty",
            Some(property.key.span.clone()),
        ));
    }
    let map_quality_status = require_field(map_quality_status, "map_quality_status", property)?;
    if map_quality_status != "PASS" {
        return Err(HydrateError::new_spanned(
            format!(
                "static_galaxy_scenario map_quality_status is `{map_quality_status}`, expected `PASS`"
            ),
            Some(property.key.span.clone()),
        ));
    }
    let source_json = require_field(source_json, "source_json", property)?;
    let source = std::fs::read_to_string(&source_json).map_err(|err| {
        HydrateError::new_spanned(
            format!("failed to read static_galaxy_scenario source `{source_json}`: {err}"),
            Some(property.key.span.clone()),
        )
    })?;
    let scenario = deserialize_scenario_authority(&source).map_err(|err| {
        HydrateError::new_spanned(
            format!("failed to parse static_galaxy_scenario source `{source_json}`: {err}"),
            Some(property.key.span.clone()),
        )
    })?;

    let mut target_by_system_id = BTreeMap::new();
    let mut namespaced_placements = Vec::with_capacity(scenario.structural_grid.placements.len());
    for placement in &scenario.structural_grid.placements {
        let namespaced_location_id = namespace_id(&namespace, &placement.location_id);
        let namespaced_target_id = namespace_id(&namespace, &placement.target_id);
        if !seen_location_targets.insert(namespaced_target_id.clone()) {
            return Err(HydrateError::new_spanned(
                format!(
                    "duplicate scenario location-target id `{namespaced_target_id}` from static_galaxy_scenario `{id}`"
                ),
                Some(property.key.span.clone()),
            ));
        }
        target_by_system_id.insert(
            placement.system_id.to_string(),
            namespaced_target_id.clone(),
        );
        namespaced_placements.push(HydratedScenarioGridPlacement {
            location_id: namespaced_location_id,
            target_id: namespaced_target_id,
            row: placement.row,
            col: placement.col,
        });
    }

    let mut namespaced_links = BTreeSet::new();
    for link in &scenario.links {
        let from = target_by_system_id
            .get(&link.from_system_id)
            .ok_or_else(|| {
                HydrateError::new_spanned(
                    format!(
                        "static_galaxy_scenario link endpoint `{}` has no structural placement",
                        link.from_system_id
                    ),
                    Some(property.key.span.clone()),
                )
            })?;
        let to = target_by_system_id.get(&link.to_system_id).ok_or_else(|| {
            HydrateError::new_spanned(
                format!(
                    "static_galaxy_scenario link endpoint `{}` has no structural placement",
                    link.to_system_id
                ),
                Some(property.key.span.clone()),
            )
        })?;
        namespaced_links.insert(canonical_namespaced_link(from.clone(), to.clone()));
    }

    Ok(HydratedEmbeddedStaticGalaxyScenario {
        id,
        namespace,
        scenario_id: scenario.scenario_id,
        map_quality_status,
        provenance: scenario.provenance,
        source_structural_grid: scenario.structural_grid,
        namespaced_placements,
        namespaced_links: namespaced_links.into_iter().collect(),
    })
}

fn parse_node(
    property: &RawProperty,
    forced_kind: Option<SimThingKind>,
    seen_node_ids: &mut BTreeSet<String>,
    seen_property_ids: &mut BTreeSet<String>,
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<HydratedScenarioNode, HydrateError> {
    let (header_id, block) = header_or_block_body(property, &property.key.text)?;
    let mut id = header_id;
    let kind_is_forced = forced_kind.is_some();
    let mut kind = forced_kind;
    let mut display_name = String::new();
    let mut properties = Vec::new();
    let mut overlays = Vec::new();
    let mut children = Vec::new();

    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        match field.key.text.as_str() {
            "id" => {
                let explicit_id = read_scalar_text(field, "id")?;
                if !id.is_empty() && id != explicit_id {
                    return Err(HydrateError::new_spanned(
                        format!("header id `{id}` does not match explicit id `{explicit_id}`"),
                        Some(field.key.span.clone()),
                    ));
                }
                id = explicit_id;
            }
            "display_name" | "name" => display_name = read_scalar_text(field, &field.key.text)?,
            "kind" => {
                if kind_is_forced {
                    return Err(HydrateError::new_spanned(
                        "`location` kind is fixed to existing SimThingKind::Location",
                        Some(field.key.span.clone()),
                    ));
                }
                kind = Some(parse_kind(field)?);
            }
            "properties" => {
                properties = parse_properties_block(field, seen_property_ids)?;
            }
            "overlays" => {
                overlays = parse_overlays_block(field, &id, seen_overlay_ids)?;
            }
            "children" => {
                children = parse_children_block(
                    field,
                    seen_node_ids,
                    seen_property_ids,
                    seen_overlay_ids,
                )?;
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported scenario node field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if id.is_empty() {
        return Err(HydrateError::new_spanned(
            format!("`{}` requires an id", property.key.text),
            Some(property.key.span.clone()),
        ));
    }
    if !seen_node_ids.insert(id.clone()) {
        return Err(HydrateError::new_spanned(
            format!("duplicate scenario node id `{id}`"),
            Some(property.key.span.clone()),
        ));
    }
    if display_name.is_empty() {
        display_name = id.clone();
    }

    Ok(HydratedScenarioNode {
        id,
        simthing_id: SimThingId::new(),
        kind: kind.ok_or_else(|| {
            HydrateError::new_spanned(
                "`child` requires an existing SimThing kind",
                Some(property.key.span.clone()),
            )
        })?,
        display_name,
        properties,
        overlays,
        children,
    })
}

fn parse_metadata_block(property: &RawProperty) -> Result<BTreeMap<String, String>, HydrateError> {
    let block = require_block(property, "metadata")?;
    let mut metadata = BTreeMap::new();
    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        if metadata
            .insert(
                field.key.text.clone(),
                read_scalar_text(field, &field.key.text)?,
            )
            .is_some()
        {
            return Err(HydrateError::new_spanned(
                format!("duplicate metadata key `{}`", field.key.text),
                Some(field.key.span.clone()),
            ));
        }
    }
    Ok(metadata)
}

fn parse_properties_block(
    property: &RawProperty,
    seen_property_ids: &mut BTreeSet<String>,
) -> Result<Vec<PropertySpec>, HydrateError> {
    let block = require_block(property, "properties")?;
    let mut properties = Vec::new();
    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        if field.key.text != "property" {
            return Err(HydrateError::new_spanned(
                format!("unsupported properties field `{}`", field.key.text),
                Some(field.key.span.clone()),
            ));
        }
        let property = parse_property_spec(field)?;
        if !seen_property_ids.insert(property.id.clone()) {
            return Err(HydrateError::new_spanned(
                format!("duplicate property id `{}`", property.id),
                Some(field.key.span.clone()),
            ));
        }
        properties.push(property);
    }
    Ok(properties)
}

fn parse_property_spec(property: &RawProperty) -> Result<PropertySpec, HydrateError> {
    let block = require_block(property, "property")?;
    let mut id = None;
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    let mut description = String::new();

    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "description" => description = read_scalar_text(field, "description")?,
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(PropertySpec {
        id: require_field(id, "id", property)?,
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
        description,
        sub_fields: Vec::new(),
    })
}

fn parse_overlays_block(
    property: &RawProperty,
    target_id: &str,
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<Vec<OverlaySpec>, HydrateError> {
    let block = require_block(property, "overlays")?;
    let mut overlays = Vec::new();
    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        if field.key.text != "modifier" {
            return Err(HydrateError::new_spanned(
                format!("unsupported overlays field `{}`", field.key.text),
                Some(field.key.span.clone()),
            ));
        }
        let overlay = parse_modifier_spec(field, target_id)?;
        if !seen_overlay_ids.insert(overlay.id.clone()) {
            return Err(HydrateError::new_spanned(
                format!("duplicate overlay id `{}`", overlay.id),
                Some(field.key.span.clone()),
            ));
        }
        overlays.push(overlay);
    }
    Ok(overlays)
}

fn parse_modifier_spec(
    property: &RawProperty,
    target_id: &str,
) -> Result<OverlaySpec, HydrateError> {
    let block = require_block(property, "modifier")?;
    let mut id = None;
    let mut display_name = String::new();
    let mut targets_property = None;
    let mut amount_mult = None;
    let mut amount_add = None;

    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "targets_property" => {
                targets_property = Some(read_scalar_text(field, "targets_property")?);
            }
            "amount_mult" => amount_mult = Some(read_scalar_f32(field, "amount_mult")?),
            "amount_add" => amount_add = Some(read_scalar_f32(field, "amount_add")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported modifier field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let transform = match (amount_mult, amount_add) {
        (Some(mult), None) => TransformOp::Multiply(mult),
        (None, Some(add)) => TransformOp::Add(add),
        (Some(_), Some(_)) => {
            return Err(HydrateError::new_spanned(
                "modifier cannot specify both amount_mult and amount_add",
                Some(property.key.span.clone()),
            ));
        }
        (None, None) => {
            return Err(HydrateError::new_spanned(
                "modifier requires amount_mult or amount_add",
                Some(property.key.span.clone()),
            ));
        }
    };

    Ok(OverlaySpec {
        id: require_field(id, "id", property)?,
        display_name,
        targets_property: require_field(targets_property, "targets_property", property)?,
        sub_field_deltas: vec![(SubFieldRole::Amount, transform)],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: InstallTargetSpec::ScenarioListed {
            target_id: target_id.to_string(),
        },
    })
}

fn parse_children_block(
    property: &RawProperty,
    seen_node_ids: &mut BTreeSet<String>,
    seen_property_ids: &mut BTreeSet<String>,
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<Vec<HydratedScenarioNode>, HydrateError> {
    let block = require_block(property, "children")?;
    let mut children = Vec::new();
    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        if field.key.text != "child" {
            return Err(HydrateError::new_spanned(
                format!("unsupported children field `{}`", field.key.text),
                Some(field.key.span.clone()),
            ));
        }
        children.push(parse_node(
            field,
            None,
            seen_node_ids,
            seen_property_ids,
            seen_overlay_ids,
        )?);
    }
    Ok(children)
}

fn parse_link(property: &RawProperty) -> Result<HydratedScenarioLink, HydrateError> {
    let block = require_block(property, "link")?;
    let mut from = None;
    let mut to = None;

    for field in &block.properties {
        reject_forbidden_node_field(field)?;
        match field.key.text.as_str() {
            "from" => from = Some(read_scalar_text(field, "from")?),
            "to" => to = Some(read_scalar_text(field, "to")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported link field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(canonical_link(
        require_field(from, "from", property)?,
        require_field(to, "to", property)?,
        property,
    )?)
}

fn build_grid_metadata(
    locations: &[HydratedScenarioNode],
    raw_links: Vec<HydratedScenarioLink>,
    embedded_placements: Vec<HydratedScenarioGridPlacement>,
    embedded_grid_size: Option<u32>,
) -> Result<HydratedScenarioGridMetadata, HydrateError> {
    let grid_size = embedded_grid_size.unwrap_or_else(|| {
        embedded_placements
            .iter()
            .map(|placement| placement.row.max(placement.col))
            .max()
            .map(|max_coord| max_coord.saturating_add(1))
            .unwrap_or_else(|| smallest_square_edge(locations.len()))
    });
    let mut location_ids = BTreeSet::new();
    let mut placements = embedded_placements;
    let mut placement_by_id = BTreeMap::new();
    for placement in &placements {
        location_ids.insert(placement.target_id.clone());
        placement_by_id.insert(placement.target_id.clone(), (placement.row, placement.col));
    }

    for (index, location) in locations.iter().enumerate() {
        location_ids.insert(location.id.clone());
        let index = index as u32;
        let placement = HydratedScenarioGridPlacement {
            location_id: location.id.clone(),
            target_id: location.id.clone(),
            row: index / grid_size,
            col: index % grid_size,
        };
        placement_by_id.insert(location.id.clone(), (placement.row, placement.col));
        placements.push(placement);
    }

    let mut links = BTreeSet::new();
    let mut fanout: BTreeMap<String, usize> = BTreeMap::new();
    for link in raw_links {
        if !location_ids.contains(&link.from) {
            return Err(HydrateError::new(format!(
                "link endpoint `{}` is not a scenario location",
                link.from
            )));
        }
        if !location_ids.contains(&link.to) {
            return Err(HydrateError::new(format!(
                "link endpoint `{}` is not a scenario location",
                link.to
            )));
        }
        if links.insert(link.clone()) {
            *fanout.entry(link.from.clone()).or_default() += 1;
            *fanout.entry(link.to.clone()).or_default() += 1;
        }
    }

    for (location_id, count) in fanout {
        if count > PR3_MAX_LINK_FANOUT {
            return Err(HydrateError::new(format!(
                "link fanout for `{location_id}` is {count}, above PR3 N4 cap {PR3_MAX_LINK_FANOUT}"
            )));
        }
    }

    let links: Vec<_> = links.into_iter().collect();
    for link in &links {
        let from = placement_by_id
            .get(&link.from)
            .expect("validated link endpoint has a placement");
        let to = placement_by_id
            .get(&link.to)
            .expect("validated link endpoint has a placement");
        if !is_n4_neighbor(*from, *to) {
            return Err(HydrateError::new(format!(
                "link `{}` -> `{}` is outside PR3 row-major N4 grid adjacency",
                link.from, link.to
            )));
        }
    }

    Ok(HydratedScenarioGridMetadata {
        grid_size,
        max_fanout: PR3_MAX_LINK_FANOUT,
        placements,
        links,
    })
}

fn namespace_id(namespace: &str, id: &str) -> String {
    format!("{namespace}::{id}")
}

fn canonical_namespaced_link(from: String, to: String) -> HydratedScenarioLink {
    if from < to {
        HydratedScenarioLink { from, to }
    } else {
        HydratedScenarioLink { from: to, to: from }
    }
}

fn smallest_square_edge(count: usize) -> u32 {
    let count = count as u32;
    let mut edge: u32 = 1;
    while edge.saturating_mul(edge) < count {
        edge += 1;
    }
    edge
}

fn is_n4_neighbor(left: (u32, u32), right: (u32, u32)) -> bool {
    (left.0 == right.0 && left.1.abs_diff(right.1) == 1)
        || (left.1 == right.1 && left.0.abs_diff(right.0) == 1)
}

fn canonical_link(
    from: String,
    to: String,
    property: &RawProperty,
) -> Result<HydratedScenarioLink, HydrateError> {
    if from == to {
        return Err(HydrateError::new_spanned(
            "link endpoints must be distinct scenario locations",
            Some(property.key.span.clone()),
        ));
    }
    if from < to {
        Ok(HydratedScenarioLink { from, to })
    } else {
        Ok(HydratedScenarioLink { from: to, to: from })
    }
}

fn flatten_node(
    node: &HydratedScenarioNode,
    properties: &mut Vec<PropertySpec>,
    overlays: &mut Vec<OverlaySpec>,
    install_targets: &mut BTreeMap<String, Vec<SimThingId>>,
) {
    install_targets.insert(node.id.clone(), vec![node.simthing_id]);
    properties.extend(node.properties.iter().cloned());
    overlays.extend(node.overlays.iter().cloned());
    for child in &node.children {
        flatten_node(child, properties, overlays, install_targets);
    }
}

fn simthing_from_node(node: &HydratedScenarioNode) -> SimThing {
    let mut simthing = SimThing::new(node.kind.clone(), 0);
    simthing.id = node.simthing_id;
    for child in &node.children {
        simthing.add_child(simthing_from_node(child));
    }
    simthing
}

fn parse_kind(property: &RawProperty) -> Result<SimThingKind, HydrateError> {
    let text = read_scalar_text(property, "kind")?;
    match text.as_str() {
        "Location" => Ok(SimThingKind::Location),
        "Cohort" => Ok(SimThingKind::Cohort),
        "Faction" => Ok(SimThingKind::Faction),
        "Fleet" => Ok(SimThingKind::Fleet),
        "ArenaParticipant" => Ok(SimThingKind::ArenaParticipant),
        "World" | "StarSystem" | "Station" | "Custom" => Err(HydrateError::new_spanned(
            format!("scenario child kind `{text}` is not admitted for PR2"),
            Some(property.key.span.clone()),
        )),
        other => Err(HydrateError::new_spanned(
            format!("unknown existing SimThing kind `{other}`"),
            Some(property.key.span.clone()),
        )),
    }
}

fn header_or_block_body<'a>(
    property: &'a RawProperty,
    field: &str,
) -> Result<(String, &'a RawBlock), HydrateError> {
    match &property.value {
        RawValue::Header(RawHeaderValue { header, payload }) => {
            let RawValue::Block(block) = payload.as_ref() else {
                return Err(HydrateError::new_spanned(
                    format!("`{field}` header payload must be a block"),
                    Some(header.span.clone()),
                ));
            };
            Ok((header.text.clone(), block))
        }
        RawValue::Block(block) => Ok((read_optional_id(block)?, block)),
        _ => Err(HydrateError::new_spanned(
            format!("`{field}` must be a block or header block"),
            Some(property.key.span.clone()),
        )),
    }
}

fn read_optional_id(block: &RawBlock) -> Result<String, HydrateError> {
    let mut id = String::new();
    for field in &block.properties {
        if field.key.text == "id" {
            if !id.is_empty() {
                return Err(HydrateError::new_spanned(
                    "duplicate `id` field",
                    Some(field.key.span.clone()),
                ));
            }
            id = read_scalar_text(field, "id")?;
        }
    }
    Ok(id)
}

fn require_block<'a>(property: &'a RawProperty, field: &str) -> Result<&'a RawBlock, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            format!("`{field}` must be a block"),
            Some(property.key.span.clone()),
        ));
    };
    Ok(block)
}

fn reject_forbidden_scenario_field(property: &RawProperty) -> Result<(), HydrateError> {
    let key = property.key.text.as_str();
    if FORBIDDEN_SCENARIO_FIELDS.contains(&key) {
        return Err(HydrateError::new_spanned(
            format!("`{key}` is outside PR3 scenario-container grammar"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(())
}

fn reject_forbidden_node_field(property: &RawProperty) -> Result<(), HydrateError> {
    let key = property.key.text.as_str();
    if FORBIDDEN_NODE_FIELDS.contains(&key) {
        return Err(HydrateError::new_spanned(
            format!("`{key}` is outside PR3 scenario-container grammar"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(())
}

fn read_scalar_text(property: &RawProperty, field: &str) -> Result<String, HydrateError> {
    match &property.value {
        RawValue::Scalar(scalar) => Ok(scalar.text.clone()),
        _ => Err(HydrateError::new_spanned(
            format!("`{field}` must be a scalar"),
            Some(property.key.span.clone()),
        )),
    }
}

fn read_scalar_f32(property: &RawProperty, field: &str) -> Result<f32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    let value = text.parse::<f32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a numeric literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })?;
    if !value.is_finite() {
        return Err(HydrateError::new_spanned(
            format!("`{field}` must be finite"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(value)
}

fn read_scalar_u32(property: &RawProperty, field: &str) -> Result<u32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<u32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a non-negative integer literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn read_scalar_u64(property: &RawProperty, field: &str) -> Result<u64, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<u64>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a non-negative integer literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn require_field<T>(
    value: Option<T>,
    field: &str,
    property: &RawProperty,
) -> Result<T, HydrateError> {
    value.ok_or_else(|| {
        HydrateError::new_spanned(
            format!("missing required field `{field}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn resolve_commitment_effect(
    effect: Option<ParsedCommitmentEffectDraft>,
    root_node: &HydratedScenarioNode,
) -> Result<Option<CommitmentEffectSpec>, HydrateError> {
    match effect {
        None => Ok(None),
        Some(ParsedCommitmentEffectDraft::Resolved(spec)) => {
            if find_node_by_id(root_node, &spec.target_id).is_none() {
                return Err(HydrateError::new(format!(
                    "commitment effect target `{}` is not a scenario node id",
                    spec.target_id
                )));
            }
            Ok(Some(spec))
        }
        Some(ParsedCommitmentEffectDraft::AttachOverlay {
            target_id,
            overlay_id,
        }) => {
            let node = find_node_by_id(root_node, &target_id).ok_or_else(|| {
                HydrateError::new(format!(
                    "commitment effect target `{target_id}` is not a scenario node id"
                ))
            })?;
            let overlay = node
                .overlays
                .iter()
                .find(|overlay| overlay.id == overlay_id)
                .ok_or_else(|| {
                    HydrateError::new(format!(
                        "commitment effect attach_overlay `{overlay_id}` is not declared on target `{target_id}`"
                    ))
                })?;
            Ok(Some(CommitmentEffectSpec {
                target_id,
                targets_property: overlay.targets_property.clone(),
                sub_field_deltas: overlay.sub_field_deltas.clone(),
                lifecycle: Default::default(),
                once: true,
            }))
        }
    }
}

fn find_node_by_id<'a>(
    node: &'a HydratedScenarioNode,
    id: &str,
) -> Option<&'a HydratedScenarioNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node_by_id(child, id) {
            return Some(found);
        }
    }
    None
}
