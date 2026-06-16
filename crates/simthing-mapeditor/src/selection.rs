//! Editor selection model — view state only; never mutates structural galaxy data.

use crate::view_model::{StudioGalaxyViewModel, StudioHyperlaneView, StudioSystemRenderAnchor};

pub const DEFAULT_PICK_RADIUS_PX: f32 = 18.0;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StudioSelectionState {
    pub hovered_system_id: Option<u32>,
    pub selected_system_id: Option<u32>,
}

impl StudioSelectionState {
    pub fn clear(&mut self) {
        self.hovered_system_id = None;
        self.selected_system_id = None;
    }

    pub fn select(&mut self, system_id: u32) {
        self.selected_system_id = Some(system_id);
    }

    pub fn set_hover(&mut self, system_id: Option<u32>) {
        self.hovered_system_id = system_id;
    }
}

pub fn apply_star_click(selection: &mut StudioSelectionState, system_id: u32) {
    selection.select(system_id);
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedSystemDetails {
    pub system_id: u32,
    pub structural_col: u32,
    pub structural_row: u32,
    pub render_height: f32,
    pub degree: usize,
    pub incident_hyperlanes: Vec<(String, String)>,
    pub incident_neighbor_ids: Vec<String>,
    pub render_only_note: &'static str,
}

pub fn system_id_string(system_id: u32) -> String {
    system_id.to_string()
}

pub fn incident_hyperlanes_for_system(
    hyperlanes: &[StudioHyperlaneView],
    system_id: u32,
) -> Vec<(String, String)> {
    let id = system_id_string(system_id);
    hyperlanes
        .iter()
        .filter(|lane| lane.from_system_id == id || lane.to_system_id == id)
        .map(|lane| (lane.from_system_id.clone(), lane.to_system_id.clone()))
        .collect()
}

pub fn selected_system_details(
    vm: &StudioGalaxyViewModel,
    system_id: u32,
) -> Option<SelectedSystemDetails> {
    let star = vm.stars.iter().find(|s| s.system_id == system_id)?;
    let incident = incident_hyperlanes_for_system(&vm.hyperlanes, system_id);
    let mut neighbor_ids: Vec<String> = incident
        .iter()
        .flat_map(|(from, to)| {
            if from == &system_id_string(system_id) {
                vec![to.clone()]
            } else {
                vec![from.clone()]
            }
        })
        .collect();
    neighbor_ids.sort();
    neighbor_ids.dedup();
    Some(SelectedSystemDetails {
        system_id: star.system_id,
        structural_col: star.structural_col,
        structural_row: star.structural_row,
        render_height: star.render_height,
        degree: incident.len(),
        incident_hyperlanes: incident,
        incident_neighbor_ids: neighbor_ids,
        render_only_note: StudioGalaxyViewModel::RENDER_ONLY_NOTE,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenStarProjection {
    pub system_id: u32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub depth: f32,
}

pub fn screen_star_projection_from_anchor(
    anchor: &StudioSystemRenderAnchor,
    screen_x: f32,
    screen_y: f32,
    camera_position: [f32; 3],
) -> ScreenStarProjection {
    let dx = camera_position[0] - anchor.world_position[0];
    let dy = camera_position[1] - anchor.world_position[1];
    let dz = camera_position[2] - anchor.world_position[2];
    ScreenStarProjection {
        system_id: anchor.system_id,
        screen_x,
        screen_y,
        depth: (dx * dx + dy * dy + dz * dz).sqrt(),
    }
}

pub fn pick_nearest_star_screen(
    cursor_x: f32,
    cursor_y: f32,
    pick_radius_px: f32,
    projections: &[ScreenStarProjection],
) -> Option<u32> {
    let radius_sq = pick_radius_px * pick_radius_px;
    let mut best: Option<(u32, f32, f32)> = None;
    for proj in projections {
        let dx = proj.screen_x - cursor_x;
        let dy = proj.screen_y - cursor_y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq > radius_sq {
            continue;
        }
        let replace = match best {
            None => true,
            Some((_, best_depth, best_dist)) => {
                proj.depth < best_depth
                    || (proj.depth - best_depth).abs() < f32::EPSILON && dist_sq < best_dist
            }
        };
        if replace {
            best = Some((proj.system_id, proj.depth, dist_sq));
        }
    }
    best.map(|(id, _, _)| id)
}

pub fn selected_incident_hyperlane_keys(
    vm: &StudioGalaxyViewModel,
    selected_system_id: u32,
) -> Vec<(String, String)> {
    incident_hyperlanes_for_system(&vm.hyperlanes, selected_system_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::{run_generation, GenerationProfile};

    #[test]
    fn selection_state_defaults_empty() {
        let state = StudioSelectionState::default();
        assert!(state.hovered_system_id.is_none());
        assert!(state.selected_system_id.is_none());
    }

    #[test]
    fn selection_state_updates_on_click() {
        let mut state = StudioSelectionState::default();
        apply_star_click(&mut state, 42);
        assert_eq!(state.selected_system_id, Some(42));
    }

    #[test]
    fn selection_state_clears() {
        let mut state = StudioSelectionState {
            hovered_system_id: Some(1),
            selected_system_id: Some(2),
        };
        state.clear();
        assert!(state.hovered_system_id.is_none());
        assert!(state.selected_system_id.is_none());
    }

    #[test]
    fn selection_clear_removes_selected_system() {
        let mut state = StudioSelectionState::default();
        apply_star_click(&mut state, 7);
        state.clear();
        assert!(state.selected_system_id.is_none());
    }

    #[test]
    fn nearest_star_pick_prefers_closest_projected_star() {
        let projections = [
            ScreenStarProjection {
                system_id: 1,
                screen_x: 100.0,
                screen_y: 100.0,
                depth: 0.5,
            },
            ScreenStarProjection {
                system_id: 2,
                screen_x: 102.0,
                screen_y: 100.0,
                depth: 0.4,
            },
        ];
        assert_eq!(
            pick_nearest_star_screen(101.0, 100.0, 20.0, &projections),
            Some(2)
        );
    }

    #[test]
    fn pick_miss_returns_none() {
        let projections = [ScreenStarProjection {
            system_id: 1,
            screen_x: 100.0,
            screen_y: 100.0,
            depth: 0.5,
        }];
        assert!(pick_nearest_star_screen(200.0, 200.0, 10.0, &projections).is_none());
    }

    #[test]
    fn selected_system_details_uses_structural_coords() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm.stars.first().expect("star");
        let details = selected_system_details(&vm, star.system_id).expect("details");
        assert_eq!(details.structural_col, star.structural_col);
        assert_eq!(details.structural_row, star.structural_row);
    }

    #[test]
    fn selected_system_details_marks_render_height_render_only() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm.stars.first().expect("star");
        let details = selected_system_details(&vm, star.system_id).expect("details");
        assert_eq!(details.render_height, star.render_height);
        assert_eq!(
            details.render_only_note,
            StudioGalaxyViewModel::RENDER_ONLY_NOTE
        );
    }

    #[test]
    fn picking_uses_render_anchor_position() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let anchor = vm.render_anchors.first().expect("anchor");
        let projection = screen_star_projection_from_anchor(
            anchor,
            20.0,
            30.0,
            [anchor.world_position[0], 10.0, anchor.world_position[2]],
        );
        assert_eq!(projection.system_id, anchor.system_id);
        assert_eq!(projection.depth, (10.0 - anchor.world_position[1]).abs());
    }

    #[test]
    fn selected_system_anchor_matches_inspector_system_id() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let anchor = vm.render_anchors.first().expect("anchor");
        let details = selected_system_details(&vm, anchor.system_id).expect("details");
        assert_eq!(details.system_id, anchor.system_id);
        assert_eq!(details.structural_col, anchor.structural_col);
        assert_eq!(details.structural_row, anchor.structural_row);
        assert_eq!(details.render_height, anchor.render_height);
    }

    #[test]
    fn selected_system_details_reports_degree() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm.stars.first().expect("star");
        let details = selected_system_details(&vm, star.system_id).expect("details");
        assert_eq!(
            details.degree,
            incident_hyperlanes_for_system(&vm.hyperlanes, star.system_id).len()
        );
    }

    #[test]
    fn selected_system_details_lists_incident_neighbors() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm
            .stars
            .iter()
            .find(|s| !incident_hyperlanes_for_system(&vm.hyperlanes, s.system_id).is_empty())
            .expect("connected star");
        let details = selected_system_details(&vm, star.system_id).expect("details");
        assert!(!details.incident_neighbor_ids.is_empty());
    }

    #[test]
    fn selected_incident_hyperlanes_match_generated_edges() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm.stars.first().expect("star");
        let keys = selected_incident_hyperlane_keys(&vm, star.system_id);
        for (from, to) in &keys {
            assert!(vm
                .hyperlanes
                .iter()
                .any(|lane| { lane.from_system_id == *from && lane.to_system_id == *to }));
        }
    }

    #[test]
    fn non_selected_star_does_not_mutate_view_model() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm_before = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let vm_after = vm_before.clone();
        let _ = selected_system_details(&vm_before, vm_before.stars[0].system_id);
        assert_eq!(vm_before, vm_after);
    }
}
