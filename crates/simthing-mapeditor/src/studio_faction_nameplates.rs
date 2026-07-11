//! STUDIO-FACTION-NAMEPLATES-0 — presentation color for star nameplates from owner authority.
//!
//! Read-only projection over ScenarioSpec Owner `color_rgb` + spatial `owner_flow_owner_ref`.
//! Does not mutate authority, invent ownership, or brighten selection (11.6).

use std::collections::HashMap;

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

/// Build system_id → (display_name, rgba) for star nameplate spawn.
pub fn star_nameplate_presentations(
    spec: &SimThingScenarioSpec,
) -> HashMap<u32, (String, [f32; 4])> {
    let owner_colors = owner_color_rgb_map_from_authority(spec);
    let map_container = resolve_map_container(spec).ok();
    let mut out = HashMap::new();
    for placement in &spec.structural_grid.placements {
        let semantic_name = map_container
            .and_then(|map| {
                map.children
                    .iter()
                    .find(|child| child.id.raw() == placement.simthing_id_raw)
            })
            .and_then(star_system_display_name);
        let name = semantic_name
            .unwrap_or_else(|| fallback_simthing_nameplate_id(placement.simthing_id_raw));
        let rgba = star_nameplate_rgba_for_placement(spec, placement, &owner_colors);
        out.insert(placement.system_id, (name, rgba));
    }
    out
}
