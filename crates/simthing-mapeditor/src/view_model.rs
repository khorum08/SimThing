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
    pub display_name: String,
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
                display_name: cell.display_name.clone(),
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
        Self::from_scenario(scenario, report)
    }

    pub fn from_scenario(scenario: &SimThingScenarioSpec, report: &GenerationReport) -> Self {
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

}
