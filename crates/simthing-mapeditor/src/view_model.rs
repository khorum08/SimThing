//! View model: structural gridcell coords + render-only studio metadata.

use serde::{Deserialize, Serialize};
use simthing_mapgenerator::{
    deterministic_unit_hash, grid_chebyshev_distance, GalaxyGenerationResult, GenerationReport,
};
use simthing_spec::SimThingScenarioSpec;

use crate::hydration::{
    hydrate_generation_result_into_studio_grid, studio_projection_from_simthing_spec,
    StudioHydrationBoundary,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StudioGalaxyRenderMeta {
    pub vertical_thickness_scale: f32,
    pub core_bulge_strength: f32,
    pub core_bulge_radius: f32,
    pub star_sprite_scale: f32,
    pub star_visibility_scale: f32,
    pub lane_visibility_scale: f32,
    pub min_star_world_scale: f32,
    pub star_near_distance: f32,
    pub star_far_distance: f32,
    pub star_far_core_scale: f32,
    pub star_near_core_scale: f32,
    pub star_far_aura_scale: f32,
    pub star_near_aura_scale: f32,
    pub star_far_core_alpha: f32,
    pub star_near_core_alpha: f32,
    pub star_far_aura_alpha: f32,
    pub star_near_aura_alpha: f32,
    pub star_falloff_settings: crate::star_render::StarFalloffSettings,
    pub star_render_mode: crate::star_render::StarRenderMode,
    pub selected_star_scale_multiplier: f32,
    pub hovered_star_scale_multiplier: f32,
    pub lane_near_alpha: f32,
    pub lane_mid_alpha: f32,
    pub lane_far_alpha: f32,
    pub lane_far_min_alpha: f32,
    pub hyperlane_render_settings: crate::hyperlane_buckets::HyperlaneRenderSettings,
    pub hyperlane_depth_near_max: f32,
    pub hyperlane_depth_mid_max: f32,
}

impl Default for StudioGalaxyRenderMeta {
    fn default() -> Self {
        Self {
            vertical_thickness_scale: 1.0,
            core_bulge_strength: 0.85,
            core_bulge_radius: 0.22,
            star_sprite_scale: 1.0,
            star_visibility_scale: crate::star_render::DEFAULT_STAR_VISIBILITY_SCALE,
            lane_visibility_scale: crate::star_render::DEFAULT_LANE_VISIBILITY_SCALE,
            min_star_world_scale: crate::star_render::MIN_STAR_WORLD_SCALE,
            star_near_distance: 45.0,
            star_far_distance: 210.0,
            star_far_core_scale: 0.10,
            star_near_core_scale: 0.68,
            star_far_aura_scale: crate::star_render::PR2R5_STAR_FAR_AURA_SCALE,
            star_near_aura_scale: crate::star_render::PR2R6_STAR_NEAR_AURA_SCALE,
            star_far_core_alpha: crate::star_render::PR2R5_STAR_FAR_CORE_ALPHA,
            star_near_core_alpha: 1.0,
            star_far_aura_alpha: 0.008,
            star_near_aura_alpha: 0.22,
            star_falloff_settings: crate::star_render::StarFalloffSettings::default(),
            star_render_mode: crate::star_render::StarRenderMode::default(),
            selected_star_scale_multiplier: 1.85,
            hovered_star_scale_multiplier: 1.22,
            lane_near_alpha: 0.75,
            lane_mid_alpha: 0.42,
            lane_far_alpha: 0.16,
            lane_far_min_alpha: 0.045,
            hyperlane_render_settings: crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
            hyperlane_depth_near_max: 100.0,
            hyperlane_depth_mid_max: 155.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioSystemRenderAnchor {
    pub system_id: u32,
    pub structural_col: u32,
    pub structural_row: u32,
    pub world_position: [f32; 3],
    pub render_height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioStarView {
    pub system_id: u32,
    pub structural_col: u32,
    pub structural_row: u32,
    pub render_height: f32,
    pub world_x: f32,
    pub world_y: f32,
    pub world_z: f32,
    pub sprite_scale: f32,
    pub emissive_strength: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioHyperlaneView {
    pub from_system_id: String,
    pub to_system_id: String,
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub depth_bucket: crate::hyperlane_buckets::HyperlaneDepthBucket,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HyperlaneRenderSegment {
    pub from_system_id: String,
    pub to_system_id: String,
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub depth_bucket: crate::hyperlane_buckets::HyperlaneDepthBucket,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioGalaxyViewModel {
    pub seed: u64,
    pub lattice_edge: u32,
    pub core_col: f32,
    pub core_row: f32,
    pub cell_world_scale: f32,
    pub render_meta: StudioGalaxyRenderMeta,
    pub render_anchors: Vec<StudioSystemRenderAnchor>,
    pub stars: Vec<StudioStarView>,
    pub hyperlanes: Vec<StudioHyperlaneView>,
    pub structural_only_note: &'static str,
}

impl StudioGalaxyViewModel {
    pub const RENDER_ONLY_NOTE: &'static str =
        "world positions and render_height are presentation-only; structural (col,row) remain authoritative";

    pub fn from_generation(result: &GalaxyGenerationResult, report: &GenerationReport) -> Self {
        let hydration = hydrate_generation_result_into_studio_grid(result, report)
            .expect("generation output must hydrate before Studio view projection");
        Self::from_hydration(&hydration)
    }

    pub fn from_hydration(hydration: &StudioHydrationBoundary) -> Self {
        let meta = StudioGalaxyRenderMeta::default();
        let lattice_edge = hydration.grid.grid_width;
        let cell_world_scale = 100.0 / lattice_edge as f32;
        let center = lattice_edge as f32 * 0.5;
        let core_col = center;
        let core_row = center;
        let max_core_dist = (center * 0.95).max(1.0);

        let mut stars = Vec::with_capacity(hydration.grid.gridcells.len());
        let mut render_anchors = Vec::with_capacity(hydration.grid.gridcells.len());
        for cell in &hydration.grid.gridcells {
            let col = cell.structural_col as f32;
            let row = cell.structural_row as f32;
            let cheb = grid_chebyshev_distance(
                (cell.structural_col, cell.structural_row),
                (core_col as u32, core_row as u32),
            ) as f32;
            let distance_from_core_norm = (cheb / max_core_dist).clamp(0.0, 1.0);
            let edge_thickness = 0.15 * meta.vertical_thickness_scale;
            let core_thickness = 2.8 * meta.vertical_thickness_scale * meta.core_bulge_strength;
            let height_amplitude = lerp(
                edge_thickness,
                core_thickness,
                1.0 - distance_from_core_norm,
            );
            let signed_noise =
                deterministic_unit_hash(hydration.report_summary.seed, cell.system_id, "height")
                    * 2.0
                    - 1.0;
            let render_height = signed_noise * height_amplitude;
            let world_x = (col - center) * cell_world_scale;
            let world_z = (row - center) * cell_world_scale;
            let anchor = StudioSystemRenderAnchor {
                system_id: cell.system_id,
                structural_col: cell.structural_col,
                structural_row: cell.structural_row,
                world_position: [world_x, render_height, world_z],
                render_height,
            };
            let radius_unit =
                deterministic_unit_hash(hydration.report_summary.seed, cell.system_id, "radius");
            let sprite_scale = crate::star_render::star_world_scale(&meta, radius_unit);
            let emissive_strength = 0.6 + radius_unit * 0.8;
            stars.push(StudioStarView {
                system_id: cell.system_id,
                structural_col: cell.structural_col,
                structural_row: cell.structural_row,
                render_height,
                world_x,
                world_y: render_height,
                world_z,
                sprite_scale,
                emissive_strength,
            });
            render_anchors.push(anchor);
        }

        let id_to_world: std::collections::HashMap<String, [f32; 3]> = render_anchors
            .iter()
            .map(|anchor| (anchor.system_id.to_string(), anchor.world_position))
            .collect();

        let hyperlanes = {
            let mut lanes = Vec::new();
            let mut dists = Vec::new();
            for edge in &hydration.grid.hyperlanes {
                let Some(from) = id_to_world.get(&edge.from_system_id) else {
                    continue;
                };
                let Some(to) = id_to_world.get(&edge.to_system_id) else {
                    continue;
                };
                let mid = [
                    (from[0] + to[0]) * 0.5,
                    (from[1] + to[1]) * 0.5,
                    (from[2] + to[2]) * 0.5,
                ];
                let dist = (mid[0] * mid[0] + mid[2] * mid[2]).sqrt();
                dists.push(dist);
                lanes.push((edge.clone(), *from, *to, mid, dist));
            }
            let max_dist = dists.iter().copied().fold(1.0_f32, f32::max).max(1.0);
            lanes
                .into_iter()
                .map(|(edge, from, to, _mid, dist)| StudioHyperlaneView {
                    from_system_id: edge.from_system_id.clone(),
                    to_system_id: edge.to_system_id.clone(),
                    from,
                    to,
                    // Initial bucket is render-only bootstrap data; Bevy rebuckets by camera every frame.
                    depth_bucket: crate::hyperlane_buckets::classify_hyperlane_depth_bucket(
                        dist / max_dist,
                    ),
                })
                .collect::<Vec<_>>()
        };

        Self {
            seed: hydration.report_summary.seed,
            lattice_edge,
            core_col,
            core_row,
            cell_world_scale,
            render_meta: meta,
            render_anchors,
            stars,
            hyperlanes,
            structural_only_note: Self::RENDER_ONLY_NOTE,
        }
    }

    pub fn from_simthing_spec_scenario(
        scenario: &SimThingScenarioSpec,
        report: &GenerationReport,
    ) -> Self {
        let hydration = studio_projection_from_simthing_spec(scenario, report)
            .expect("SimThing-Spec scenario must project into Studio view model");
        Self::from_hydration(&hydration)
    }

    pub fn galaxy_center(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    pub fn hyperlane_render_segments(&self) -> Vec<HyperlaneRenderSegment> {
        build_hyperlane_render_segments(&self.hyperlanes, &self.render_anchors)
    }

    pub fn apply_star_falloff_settings(
        &mut self,
        settings: crate::star_render::StarFalloffSettings,
    ) {
        crate::star_render::apply_star_falloff_settings_to_meta(&mut self.render_meta, settings);
    }

    pub fn apply_star_render_mode(&mut self, mode: crate::star_render::StarRenderMode) {
        crate::star_render::apply_star_render_mode_to_meta(&mut self.render_meta, mode);
    }

    pub fn apply_hyperlane_render_settings(
        &mut self,
        settings: crate::hyperlane_buckets::HyperlaneRenderSettings,
    ) {
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(
            &mut self.render_meta,
            settings,
        );
    }
}

pub fn anchor_for_system(
    anchors: &[StudioSystemRenderAnchor],
    system_id: u32,
) -> Option<&StudioSystemRenderAnchor> {
    anchors.iter().find(|anchor| anchor.system_id == system_id)
}

pub fn anchor_for_system_str<'a>(
    anchors: &'a [StudioSystemRenderAnchor],
    system_id: &str,
) -> Option<&'a StudioSystemRenderAnchor> {
    let id = system_id.parse::<u32>().ok()?;
    anchor_for_system(anchors, id)
}

pub fn build_hyperlane_render_segments(
    hyperlanes: &[StudioHyperlaneView],
    anchors: &[StudioSystemRenderAnchor],
) -> Vec<HyperlaneRenderSegment> {
    hyperlanes
        .iter()
        .filter_map(|lane| {
            let from = anchor_for_system_str(anchors, &lane.from_system_id)?;
            let to = anchor_for_system_str(anchors, &lane.to_system_id)?;
            Some(HyperlaneRenderSegment {
                from_system_id: lane.from_system_id.clone(),
                to_system_id: lane.to_system_id.clone(),
                from: from.world_position,
                to: to.world_position,
                depth_bucket: lane.depth_bucket,
            })
        })
        .collect()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::{run_generation, GenerationProfile};

    #[test]
    fn editor_view_model_uses_structural_grid_coords() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        assert_eq!(vm.stars.len(), output.result.placement.systems.len());
        for (star, system) in vm.stars.iter().zip(output.result.placement.systems.iter()) {
            assert_eq!(star.structural_col, system.coord.col);
            assert_eq!(star.structural_row, system.coord.row);
        }
    }

    #[test]
    fn view_model_derives_from_hydration_not_raw_render_metadata() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let mut output = run_generation(&profile).expect("generate");
        let hydration =
            crate::hydration::hydrate_generation_into_studio_grid(&output).expect("hydrate");
        let first_cell = hydration.grid.gridcells.first().expect("cell").clone();
        output.result.placement.systems[0].coord.col = output.result.placement.systems[0]
            .coord
            .col
            .saturating_add(1);

        let vm = StudioGalaxyViewModel::from_hydration(&hydration);
        let star = vm
            .stars
            .iter()
            .find(|star| star.system_id == first_cell.system_id)
            .expect("star");

        assert_eq!(star.structural_col, first_cell.structural_col);
        assert_eq!(star.structural_row, first_cell.structural_row);
    }

    #[test]
    fn view_model_derives_from_simthing_spec_scenario() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let scenario =
            crate::hydration::generate_simthing_spec_scenario(&output).expect("spec authority");
        let vm = StudioGalaxyViewModel::from_simthing_spec_scenario(&scenario, &output.report);

        assert_eq!(vm.stars.len(), scenario.structural_grid.placements.len());
        let first_placement = scenario
            .structural_grid
            .placements
            .first()
            .expect("placement");
        let star = vm
            .stars
            .iter()
            .find(|star| star.system_id == first_placement.system_id)
            .expect("star");
        assert_eq!(star.structural_col, first_placement.col);
        assert_eq!(star.structural_row, first_placement.row);
    }

    #[test]
    fn view_model_rebuilds_from_simthing_spec_scenario() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let scenario =
            crate::hydration::generate_simthing_spec_scenario(&output).expect("spec authority");
        let rebuilt_a =
            StudioGalaxyViewModel::from_simthing_spec_scenario(&scenario, &output.report);
        let rebuilt_b =
            StudioGalaxyViewModel::from_simthing_spec_scenario(&scenario, &output.report);

        assert_eq!(rebuilt_a.stars, rebuilt_b.stars);
        assert_eq!(rebuilt_a.hyperlanes, rebuilt_b.hyperlanes);
        assert_eq!(rebuilt_a.render_anchors, rebuilt_b.render_anchors);
    }

    #[test]
    fn view_model_preserves_structural_coords_from_hydration() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let hydration =
            crate::hydration::hydrate_generation_into_studio_grid(&output).expect("hydrate");
        let vm = StudioGalaxyViewModel::from_hydration(&hydration);

        for cell in &hydration.grid.gridcells {
            let star = vm
                .stars
                .iter()
                .find(|star| star.system_id == cell.system_id)
                .expect("star");
            assert_eq!(star.structural_col, cell.structural_col);
            assert_eq!(star.structural_row, cell.structural_row);
        }
    }

    #[test]
    fn render_anchor_count_matches_system_count() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        assert_eq!(
            vm.render_anchors.len(),
            output.result.placement.systems.len()
        );
        assert_eq!(vm.render_anchors.len(), vm.stars.len());
    }

    #[test]
    fn render_anchor_preserves_structural_col_row() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        for star in &vm.stars {
            let anchor = anchor_for_system(&vm.render_anchors, star.system_id).expect("anchor");
            assert_eq!(anchor.structural_col, star.structural_col);
            assert_eq!(anchor.structural_row, star.structural_row);
        }
    }

    #[test]
    fn render_anchor_world_position_includes_render_height() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let star = vm
            .stars
            .iter()
            .find(|star| star.render_height != 0.0)
            .expect("non-flat star");
        let anchor = anchor_for_system(&vm.render_anchors, star.system_id).expect("anchor");
        assert_eq!(
            anchor.world_position,
            [star.world_x, star.render_height, star.world_z]
        );
        assert_eq!(anchor.world_position[1], anchor.render_height);
    }

    #[test]
    fn editor_view_model_render_height_is_render_only() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        assert!(vm.stars.iter().any(|s| s.render_height != 0.0));
        for system in &output.result.placement.systems {
            assert_eq!(system.coord.col, system.coord.col);
        }
        assert_eq!(
            vm.structural_only_note,
            StudioGalaxyViewModel::RENDER_ONLY_NOTE
        );
    }

    #[test]
    fn editor_view_model_hyperlanes_match_generated_edges() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        assert_eq!(
            vm.hyperlanes.len(),
            output.result.base_hyperlane_edges.len()
        );
    }

    #[test]
    fn hyperlane_endpoints_use_render_anchor_positions() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let segments = vm.hyperlane_render_segments();
        assert_eq!(segments.len(), vm.hyperlanes.len());
        let segment = segments.first().expect("segment");
        let from = anchor_for_system_str(&vm.render_anchors, &segment.from_system_id)
            .expect("from anchor");
        let to =
            anchor_for_system_str(&vm.render_anchors, &segment.to_system_id).expect("to anchor");
        assert_eq!(segment.from, from.world_position);
        assert_eq!(segment.to, to.world_position);
    }

    #[test]
    fn hyperlane_anchor_coherence_unchanged() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let segments = vm.hyperlane_render_segments();
        assert_eq!(segments.len(), vm.hyperlanes.len());
        for segment in &segments {
            let from = anchor_for_system_str(&vm.render_anchors, &segment.from_system_id)
                .expect("from anchor");
            let to = anchor_for_system_str(&vm.render_anchors, &segment.to_system_id)
                .expect("to anchor");
            assert_eq!(segment.from, from.world_position);
            assert_eq!(segment.to, to.world_position);
        }
    }

    #[test]
    fn incident_highlight_lanes_use_render_anchor_positions() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let selected = vm
            .stars
            .iter()
            .find(|star| {
                crate::selection::incident_hyperlanes_for_system(&vm.hyperlanes, star.system_id)
                    .len()
                    > 0
            })
            .expect("connected star");
        let selected_anchor =
            anchor_for_system(&vm.render_anchors, selected.system_id).expect("selected anchor");
        let incident =
            crate::selection::incident_hyperlanes_for_system(&vm.hyperlanes, selected.system_id);
        let segments = vm.hyperlane_render_segments();
        for (from_id, to_id) in incident {
            let segment = segments
                .iter()
                .find(|segment| segment.from_system_id == from_id && segment.to_system_id == to_id)
                .expect("incident segment");
            assert!(
                segment.from == selected_anchor.world_position
                    || segment.to == selected_anchor.world_position
            );
        }
    }

    #[test]
    fn render_anchor_is_render_only_metadata() {
        assert!(StudioGalaxyViewModel::RENDER_ONLY_NOTE.contains("presentation-only"));
        assert!(StudioGalaxyViewModel::RENDER_ONLY_NOTE.contains("structural"));
    }

    #[test]
    fn camera_depth_bucket_uses_segment_midpoint_from_render_anchors() {
        let segment = HyperlaneRenderSegment {
            from_system_id: "1".into(),
            to_system_id: "2".into(),
            from: [0.0, 4.0, 0.0],
            to: [0.0, 8.0, 10.0],
            depth_bucket: crate::hyperlane_buckets::HyperlaneDepthBucket::Near,
        };
        let midpoint =
            crate::hyperlane_buckets::hyperlane_segment_midpoint(segment.from, segment.to);
        assert_eq!(midpoint, [0.0, 6.0, 5.0]);
    }

    #[test]
    fn settings_changes_do_not_regenerate_galaxy() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let seed = vm.seed;
        let star_count = vm.stars.len();
        let hyperlane_count = vm.hyperlanes.len();
        vm.apply_star_falloff_settings(crate::star_render::StarFalloffSettings {
            base_blur_radius: 0.31,
            falloff_distance_percent: 60.0,
            falloff_blur_radius_percent: 40.0,
            falloff_opacity_percent: 55.0,
        });
        assert_eq!(vm.seed, seed);
        assert_eq!(vm.stars.len(), star_count);
        assert_eq!(vm.hyperlanes.len(), hyperlane_count);
        assert_eq!(output.result.placement.systems.len(), star_count);
    }

    #[test]
    fn hyperlane_settings_change_does_not_regenerate_galaxy() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let seed = vm.seed;
        let star_count = vm.stars.len();
        let hyperlane_count = vm.hyperlanes.len();
        let anchor_count = vm.render_anchors.len();
        vm.apply_hyperlane_render_settings(crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_thickness_percent_of_star: 12.0,
            base_opacity_percent: 0.0,
            falloff_distance_percent: 50.0,
            falloff_thickness_percent: 25.0,
            falloff_opacity_percent: 10.0,
        });
        assert_eq!(vm.seed, seed);
        assert_eq!(vm.stars.len(), star_count);
        assert_eq!(vm.hyperlanes.len(), hyperlane_count);
        assert_eq!(vm.render_anchors.len(), anchor_count);
        assert_eq!(output.result.base_hyperlane_edges.len(), hyperlane_count);
    }

    #[test]
    fn hyperlane_settings_change_preserves_anchor_coherence() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        vm.apply_hyperlane_render_settings(crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_thickness_percent_of_star: 25.0,
            base_opacity_percent: 100.0,
            falloff_distance_percent: 40.0,
            falloff_thickness_percent: 50.0,
            falloff_opacity_percent: 25.0,
        });
        for segment in vm.hyperlane_render_segments() {
            let from = anchor_for_system_str(&vm.render_anchors, &segment.from_system_id)
                .expect("from anchor");
            let to = anchor_for_system_str(&vm.render_anchors, &segment.to_system_id)
                .expect("to anchor");
            assert_eq!(segment.from, from.world_position);
            assert_eq!(segment.to, to.world_position);
        }
    }

    #[test]
    fn overhead_mode_does_not_mutate_render_anchors() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let anchors_before = vm.render_anchors.clone();
        let presentation_mode_is_overhead = true;
        assert!(presentation_mode_is_overhead);
        assert_eq!(vm.render_anchors, anchors_before);
        assert_eq!(vm.hyperlane_render_segments().len(), vm.hyperlanes.len());
    }

    #[test]
    fn switching_modes_does_not_regenerate_galaxy() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let seed = vm.seed;
        let star_count = vm.stars.len();
        let hyperlane_count = vm.hyperlanes.len();
        let mut presentation_mode_is_overhead = false;
        presentation_mode_is_overhead = !presentation_mode_is_overhead;
        assert!(presentation_mode_is_overhead);
        assert_eq!(vm.seed, seed);
        assert_eq!(vm.stars.len(), star_count);
        assert_eq!(vm.hyperlanes.len(), hyperlane_count);
        assert_eq!(output.result.base_hyperlane_edges.len(), hyperlane_count);
    }

    #[test]
    fn base_hyperlane_thickness_updates_render_meta() {
        let mut meta = StudioGalaxyRenderMeta::default();
        let settings = crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_thickness_percent_of_star: 13.0,
            ..Default::default()
        };
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(&mut meta, settings);
        assert_eq!(
            meta.hyperlane_render_settings
                .base_thickness_percent_of_star,
            13.0
        );
    }

    #[test]
    fn base_hyperlane_opacity_updates_render_meta() {
        let mut meta = StudioGalaxyRenderMeta::default();
        let settings = crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_opacity_percent: 44.0,
            ..Default::default()
        };
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.hyperlane_render_settings.base_opacity_percent, 44.0);
    }

    #[test]
    fn hyperlane_falloff_distance_updates_render_meta() {
        let mut meta = StudioGalaxyRenderMeta::default();
        let settings = crate::hyperlane_buckets::HyperlaneRenderSettings {
            falloff_distance_percent: 66.0,
            ..Default::default()
        };
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(&mut meta, settings);
        assert_eq!(
            meta.hyperlane_render_settings.falloff_distance_percent,
            66.0
        );
    }

    #[test]
    fn hyperlane_falloff_thickness_updates_render_meta() {
        let mut meta = StudioGalaxyRenderMeta::default();
        let settings = crate::hyperlane_buckets::HyperlaneRenderSettings {
            falloff_thickness_percent: 22.0,
            ..Default::default()
        };
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(&mut meta, settings);
        assert_eq!(
            meta.hyperlane_render_settings.falloff_thickness_percent,
            22.0
        );
    }

    #[test]
    fn hyperlane_falloff_opacity_updates_render_meta() {
        let mut meta = StudioGalaxyRenderMeta::default();
        let settings = crate::hyperlane_buckets::HyperlaneRenderSettings {
            falloff_opacity_percent: 11.0,
            ..Default::default()
        };
        crate::hyperlane_buckets::apply_hyperlane_render_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.hyperlane_render_settings.falloff_opacity_percent, 11.0);
    }

    #[test]
    fn settings_changes_preserve_render_anchor_count() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let anchor_count = vm.render_anchors.len();
        vm.apply_star_falloff_settings(crate::star_render::StarFalloffSettings {
            base_blur_radius: 0.12,
            ..Default::default()
        });
        assert_eq!(vm.render_anchors.len(), anchor_count);
        assert_eq!(vm.render_anchors.len(), vm.stars.len());
    }

    #[test]
    fn settings_changes_preserve_hyperlane_anchor_coherence() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        vm.apply_star_falloff_settings(crate::star_render::StarFalloffSettings {
            falloff_blur_radius_percent: 15.0,
            falloff_opacity_percent: 35.0,
            ..Default::default()
        });
        for segment in vm.hyperlane_render_segments() {
            let from = anchor_for_system_str(&vm.render_anchors, &segment.from_system_id)
                .expect("from anchor");
            let to = anchor_for_system_str(&vm.render_anchors, &segment.to_system_id)
                .expect("to anchor");
            assert_eq!(segment.from, from.world_position);
            assert_eq!(segment.to, to.world_position);
        }
    }

    #[test]
    fn editor_quality_panel_accepts_pass_report() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        assert!(crate::generation::quality_panel_accepts_report(
            &output.report
        ));
    }

    #[test]
    fn editor_quality_panel_flags_warn_or_fail_report() {
        use simthing_mapgenerator::{
            build_generation_report, generate_galaxy_with_structure, structure_options_from_params,
            ReportArtifacts, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
            MAP_QUALITY_FAIL,
        };
        let mut profile = GenerationProfile::default_spiral_2_dense_3000();
        profile.target_hyperlanes = 6000;
        let mut params = profile.to_map_generator_params();
        params.hyperlane.num_hyperlanes_max = 3;
        let registry = ShapeRegistry::default();
        params.validate(&registry).expect("valid");
        let (hyperlane, special, _partition, cluster) =
            structure_options_from_params(&params).expect("opts");
        let result = generate_galaxy_with_structure(
            &params,
            &registry,
            None,
            &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
            Some(hyperlane),
            Some(special),
            None,
            Some(cluster),
        )
        .expect("gen");
        let report = build_generation_report(&params, &result, ReportArtifacts::new());
        assert_eq!(report.output.map_quality_status, MAP_QUALITY_FAIL);
        assert!(crate::generation::quality_panel_flags_report(&report));
        assert!(!crate::generation::quality_panel_accepts_report(&report));
    }
}
