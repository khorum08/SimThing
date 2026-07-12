//! STUDIO-FACTION-NAMEPLATES-0 / STUDIO-OWNED-STAR-SELECT-BRIGHTEN-0
//!
//! Presentation projection over ScenarioSpec Owner `color_rgb` + spatial `owner_flow_owner_ref`.
//! - 11.5: nameplate display name + faction RGBA
//! - 11.6: owned-set highlight system ids from selected star's owner_flow_owner_ref
//!
//! Does not mutate authority, invent ownership, or change selection-model authority.

use std::collections::{BTreeSet, HashMap};

use simthing_core::SimThing;
use simthing_spec::{
    game_session_owners, owner_entity_id, owner_faction_color_rgb, owner_flow_owner_ref,
    resolve_map_container, star_system_display_name, SimThingScenarioSpec,
    SimThingStructuralGridPlacement,
};

/// Neutral (unowned) nameplate RGBA — matches prior fixed star nameplate tint.
pub const NEUTRAL_NAMEPLATE_RGBA: [f32; 4] = [0.92, 0.96, 1.0, 1.0];

/// Convert authority `color_rgb` channels to StudioTypeface label RGBA.
pub fn nameplate_rgba_from_color_rgb(rgb: (u8, u8, u8)) -> [f32; 4] {
    [
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
        1.0,
    ]
}

/// Fallback id label when star_system_display_name is absent (presentation only).
pub fn fallback_simthing_nameplate_id(raw_id: u32) -> String {
    format!("#{raw_id:08X}")
}

/// Map owner_key / owner_id → faction color_rgb from authority Owner children.
pub fn owner_color_rgb_map_from_authority(
    spec: &SimThingScenarioSpec,
) -> HashMap<String, (u8, u8, u8)> {
    let mut map = HashMap::new();
    let Ok(owners) = game_session_owners(spec) else {
        return map;
    };
    for owner in owners {
        let Some(id) = owner_entity_id(owner) else {
            continue;
        };
        if let Some(rgb) = owner_faction_color_rgb(owner) {
            map.insert(id, rgb);
        }
    }
    map
}

/// Per-star presentation projection: name, color, optional owner id (11.5 + 11.6).
#[derive(Debug, Clone, PartialEq)]
pub struct StarOwnershipPresentation {
    pub system_id: u32,
    pub display_name: String,
    pub rgba: [f32; 4],
    pub owner_id: Option<String>,
}

/// Resolve owner_flow_owner_ref for a structural placement (star system cell).
pub fn star_owner_id_for_placement(
    spec: &SimThingScenarioSpec,
    placement: &SimThingStructuralGridPlacement,
) -> Option<String> {
    let map = resolve_map_container(spec).ok()?;
    let cell = map
        .children
        .iter()
        .find(|child| child.id.raw() == placement.simthing_id_raw)?;
    owner_flow_owner_ref(cell)
}

/// Resolve nameplate color for a structural placement (star system cell).
///
/// Uses `OWNER_FLOW_OWNER_REF` on the gridcell SimThing when present; otherwise neutral.
pub fn star_nameplate_rgba_for_placement(
    spec: &SimThingScenarioSpec,
    placement: &SimThingStructuralGridPlacement,
    owner_colors: &HashMap<String, (u8, u8, u8)>,
) -> [f32; 4] {
    let Some(map) = resolve_map_container(spec).ok() else {
        return NEUTRAL_NAMEPLATE_RGBA;
    };
    let Some(cell) = map
        .children
        .iter()
        .find(|child| child.id.raw() == placement.simthing_id_raw)
    else {
        return NEUTRAL_NAMEPLATE_RGBA;
    };
    star_nameplate_rgba_for_gridcell(cell, owner_colors)
}

/// Presentation color for a star-system gridcell SimThing (tests / render).
pub fn star_nameplate_rgba_for_gridcell(
    cell: &SimThing,
    owner_colors: &HashMap<String, (u8, u8, u8)>,
) -> [f32; 4] {
    match owner_flow_owner_ref(cell) {
        Some(owner_id) => owner_colors
            .get(&owner_id)
            .copied()
            .map(nameplate_rgba_from_color_rgb)
            .unwrap_or(NEUTRAL_NAMEPLATE_RGBA),
        None => NEUTRAL_NAMEPLATE_RGBA,
    }
}

/// Full ownership presentation map: system_id → name + rgba + owner_id.
pub fn star_ownership_presentations(
    spec: &SimThingScenarioSpec,
) -> HashMap<u32, StarOwnershipPresentation> {
    let owner_colors = owner_color_rgb_map_from_authority(spec);
    let map_container = resolve_map_container(spec).ok();
    let mut out = HashMap::new();
    for placement in &spec.structural_grid.placements {
        let cell = map_container.and_then(|map| {
            map.children
                .iter()
                .find(|child| child.id.raw() == placement.simthing_id_raw)
        });
        let semantic_name = cell.and_then(star_system_display_name);
        let name = semantic_name
            .unwrap_or_else(|| fallback_simthing_nameplate_id(placement.simthing_id_raw));
        let owner_id = cell.and_then(owner_flow_owner_ref);
        let rgba = match owner_id.as_ref() {
            Some(id) => owner_colors
                .get(id)
                .copied()
                .map(nameplate_rgba_from_color_rgb)
                .unwrap_or(NEUTRAL_NAMEPLATE_RGBA),
            None => NEUTRAL_NAMEPLATE_RGBA,
        };
        out.insert(
            placement.system_id,
            StarOwnershipPresentation {
                system_id: placement.system_id,
                display_name: name,
                rgba,
                owner_id,
            },
        );
    }
    out
}

/// Build system_id → (display_name, rgba) for star nameplate spawn (11.5).
pub fn star_nameplate_presentations(
    spec: &SimThingScenarioSpec,
) -> HashMap<u32, (String, [f32; 4])> {
    star_ownership_presentations(spec)
        .into_iter()
        .map(|(id, p)| (id, (p.display_name, p.rgba)))
        .collect()
}

/// system_id → owner_flow_owner_ref string (only systems with an owner).
pub fn star_owner_id_by_system_id(spec: &SimThingScenarioSpec) -> HashMap<u32, String> {
    star_ownership_presentations(spec)
        .into_iter()
        .filter_map(|(id, p)| p.owner_id.map(|owner| (id, owner)))
        .collect()
}

/// Owner id for the currently selected system, if that system has owner_flow_owner_ref.
///
/// Unowned selected star → None (no owned-set highlight group).
pub fn selected_owner_id_for_system(
    spec: &SimThingScenarioSpec,
    selected_system_id: Option<u32>,
) -> Option<String> {
    let system_id = selected_system_id?;
    star_ownership_presentations(spec)
        .get(&system_id)?
        .owner_id
        .clone()
}

/// System ids that share the selected star's owner_flow_owner_ref (11.6 render-only highlight).
///
/// Empty when nothing selected, selected system is unowned, or owner unknown.
/// Does **not** invent a "None owner" group for unowned stars.
pub fn owned_star_highlight_system_ids(
    spec: &SimThingScenarioSpec,
    selected_system_id: Option<u32>,
) -> BTreeSet<u32> {
    let Some(owner) = selected_owner_id_for_system(spec, selected_system_id) else {
        return BTreeSet::new();
    };
    star_ownership_presentations(spec)
        .into_iter()
        .filter_map(|(id, p)| {
            if p.owner_id.as_deref() == Some(owner.as_str()) {
                Some(id)
            } else {
                None
            }
        })
        .collect()
}

/// Whether a star should use selected-star *visual* brightness for owned-set highlight (11.6).
///
/// `actual_selected` remains the single selected system id; this is render-only OR.
pub fn star_visual_selected_for_owned_set(
    system_id: u32,
    actual_selected_system_id: Option<u32>,
    owned_highlight: &BTreeSet<u32>,
) -> bool {
    actual_selected_system_id == Some(system_id) || owned_highlight.contains(&system_id)
}
