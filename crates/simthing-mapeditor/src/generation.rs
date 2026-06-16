//! Map generation profile and producer integration (typed library path — no stdout scraping).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use simthing_mapgenerator::{
    build_generation_report, generate_galaxy_with_structure, structure_options_from_params,
    write_generation_report_json, GalaxyGenerationResult, GenerationReport, MapGeneratorParams,
    ReportArtifacts, ReportError, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
    ValidationError, MAP_QUALITY_FAIL, MAP_QUALITY_PASS, MAP_QUALITY_WARN,
};

use crate::shape_params::{
    apply_editable_values_to_profile_fields, default_params_for_shape,
    editable_values_from_profile_fields, spiral_arm_params_active, store_dormant_shape_params,
};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationProfile {
    pub preset_id: String,
    pub shape: String,
    pub star_count: u32,
    pub lattice_edge: u32,
    pub seed: u64,
    pub target_hyperlanes: u32,
    pub max_hyperlane_distance: f64,
    pub ensure_connected: bool,
    pub allow_disconnected: bool,
    pub draw_core: bool,
    pub render_lanes: bool,
    pub arm_width: f64,
    pub arm_tightness: f64,
    pub jitter: f64,
    pub cluster_count: u32,
    pub cluster_radius: f64,
    pub no_partitions: bool,
    #[serde(default)]
    pub shape_params_by_shape: BTreeMap<String, BTreeMap<String, f64>>,
}

#[derive(Debug, Clone)]
pub struct GenerationRunOutput {
    pub result: GalaxyGenerationResult,
    pub report: GenerationReport,
    pub galaxy_display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationPreset {
    Spiral2Dense3000,
    Spiral4Visual1500,
    Disc1500Connected,
    Elliptical1000,
    StaticImport,
    ArbitraryStatic,
    ClausewitzUiImport,
}

impl GenerationPreset {
    pub fn all() -> &'static [GenerationPreset] {
        &[
            GenerationPreset::Spiral2Dense3000,
            GenerationPreset::Spiral4Visual1500,
            GenerationPreset::Disc1500Connected,
            GenerationPreset::Elliptical1000,
            GenerationPreset::StaticImport,
            GenerationPreset::ArbitraryStatic,
            GenerationPreset::ClausewitzUiImport,
        ]
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Spiral2Dense3000 => "spiral_2_dense_3000",
            Self::Spiral4Visual1500 => "spiral_4_visual_1500",
            Self::Disc1500Connected => "disc_1500_connected",
            Self::Elliptical1000 => "elliptical_1000",
            Self::StaticImport => "static_import",
            Self::ArbitraryStatic => "arbitrary_static",
            Self::ClausewitzUiImport => "clausewitz_ui_import",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Spiral2Dense3000 => "Spiral 2 Dense 3000",
            Self::Spiral4Visual1500 => "Spiral 4 Visual 1500",
            Self::Disc1500Connected => "Disc 1500 Connected",
            Self::Elliptical1000 => "Elliptical 1000",
            Self::ArbitraryStatic => "Arbitrary Static",
            Self::StaticImport => "Static Import",
            Self::ClausewitzUiImport => "Clausewitz UI Import (research)",
        }
    }

    pub fn is_active(self) -> bool {
        matches!(
            self,
            GenerationPreset::Spiral2Dense3000
                | GenerationPreset::Spiral4Visual1500
                | GenerationPreset::Disc1500Connected
                | GenerationPreset::Elliptical1000
        )
    }

    pub fn to_profile(self) -> GenerationProfile {
        match self {
            GenerationPreset::Spiral2Dense3000 => GenerationProfile::default_spiral_2_dense_3000(),
            GenerationPreset::Spiral4Visual1500 => {
                let mut p = GenerationProfile::default_spiral_2_dense_3000();
                p.preset_id = self.id().into();
                p.shape = "spiral_4".into();
                p.star_count = 1500;
                p.lattice_edge = 300;
                p.target_hyperlanes = 3000;
                p.max_hyperlane_distance = 4.0;
                p
            }
            GenerationPreset::Disc1500Connected => {
                let mut p = GenerationProfile::default_spiral_2_dense_3000();
                p.preset_id = self.id().into();
                p.shape = "elliptical".into();
                p.star_count = 1500;
                p.lattice_edge = 300;
                p.target_hyperlanes = 5000;
                p.max_hyperlane_distance = 3.0;
                p.arm_width = 14.0;
                p.arm_tightness = 0.6;
                p.jitter = 2.0;
                p.init_shape_param_storage();
                p.switch_shape("spiral_2", "elliptical");
                p
            }
            GenerationPreset::Elliptical1000 => {
                let mut p = GenerationProfile::default_spiral_2_dense_3000();
                p.preset_id = self.id().into();
                p.shape = "elliptical".into();
                p.star_count = 1000;
                p.lattice_edge = 200;
                p.target_hyperlanes = 2000;
                p.max_hyperlane_distance = 4.0;
                p.init_shape_param_storage();
                p.switch_shape("spiral_2", "elliptical");
                p
            }
            GenerationPreset::StaticImport | GenerationPreset::ArbitraryStatic => {
                let mut p = GenerationProfile::default_spiral_2_dense_3000();
                p.preset_id = self.id().into();
                p.shape = "static".into();
                p
            }
            GenerationPreset::ClausewitzUiImport => {
                let mut p = GenerationProfile::default_spiral_2_dense_3000();
                p.preset_id = self.id().into();
                p
            }
        }
    }
}

impl Default for GenerationProfile {
    fn default() -> Self {
        Self::default_spiral_2_dense_3000()
    }
}

impl GenerationProfile {
    pub fn default_spiral_2_dense_3000() -> Self {
        Self {
            preset_id: GenerationPreset::Spiral2Dense3000.id().into(),
            shape: "spiral_2".into(),
            star_count: 3000,
            lattice_edge: 300,
            seed: 42,
            target_hyperlanes: 6000,
            max_hyperlane_distance: 8.0,
            ensure_connected: true,
            allow_disconnected: false,
            draw_core: true,
            render_lanes: true,
            arm_width: 14.0,
            arm_tightness: 0.6,
            jitter: 2.0,
            cluster_count: 4,
            cluster_radius: 500.0,
            no_partitions: true,
            shape_params_by_shape: BTreeMap::new(),
        }
    }

    pub fn init_shape_param_storage(&mut self) {
        if !self.shape_params_by_shape.is_empty() {
            return;
        }
        let editable =
            editable_values_from_profile_fields(self.arm_width, self.arm_tightness, self.jitter);
        store_dormant_shape_params(&self.shape, &editable, &mut self.shape_params_by_shape);
    }

    pub fn sync_editable_fields_from_active_shape(&mut self) {
        let active =
            crate::shape_params::active_shape_params_for(&self.shape, &self.shape_params_by_shape);
        apply_editable_values_to_profile_fields(
            &active,
            &mut self.arm_width,
            &mut self.arm_tightness,
            &mut self.jitter,
        );
    }

    pub fn persist_editable_fields_for_active_shape(&mut self) {
        if !spiral_arm_params_active(&self.shape) {
            let mut editable = BTreeMap::new();
            if crate::shape_params::param_keys_for_shape(&self.shape).contains(&"jitter") {
                editable.insert("jitter".into(), self.jitter);
            }
            store_dormant_shape_params(&self.shape, &editable, &mut self.shape_params_by_shape);
            return;
        }
        let editable =
            editable_values_from_profile_fields(self.arm_width, self.arm_tightness, self.jitter);
        store_dormant_shape_params(&self.shape, &editable, &mut self.shape_params_by_shape);
    }

    pub fn switch_shape(&mut self, old_shape: &str, new_shape: &str) {
        if old_shape != new_shape {
            let editable = editable_values_from_profile_fields(
                self.arm_width,
                self.arm_tightness,
                self.jitter,
            );
            store_dormant_shape_params(old_shape, &editable, &mut self.shape_params_by_shape);
        }
        self.shape = new_shape.to_string();
        if !self.shape_params_by_shape.contains_key(new_shape) {
            self.shape_params_by_shape
                .insert(new_shape.to_string(), default_params_for_shape(new_shape));
        }
        self.sync_editable_fields_from_active_shape();
    }

    pub fn submission_shape_params(&self) -> BTreeMap<String, f64> {
        crate::shape_params::active_shape_params_for(&self.shape, &self.shape_params_by_shape)
    }

    pub fn matches_known_healthy_editor_prep_params(&self) -> bool {
        self.shape == "spiral_2"
            && self.star_count == 3000
            && self.lattice_edge == 300
            && self.seed == 42
            && self.target_hyperlanes == 6000
            && (self.max_hyperlane_distance - 8.0).abs() < f64::EPSILON
            && self.ensure_connected
            && !self.allow_disconnected
            && (self.arm_width - 14.0).abs() < f64::EPSILON
            && (self.arm_tightness - 0.6).abs() < f64::EPSILON
            && (self.jitter - 2.0).abs() < f64::EPSILON
    }

    pub fn galaxy_type_label(&self) -> String {
        match self.shape.as_str() {
            "spiral_2" => "Unnamed 2-Armed Spiral".into(),
            "spiral_3" => "Unnamed 3-Armed Spiral".into(),
            "spiral_4" => "Unnamed 4-Armed Spiral".into(),
            "spiral_6" => "Unnamed 6-Armed Spiral".into(),
            "elliptical" => "Unnamed Elliptical Galaxy".into(),
            "ring" => "Unnamed Ring Galaxy".into(),
            "bar" => "Unnamed Bar Galaxy".into(),
            other => format!("Unnamed {other} Galaxy"),
        }
    }

    pub fn to_map_generator_params(&self) -> MapGeneratorParams {
        let mut params = MapGeneratorParams::default();
        params.shape.shape = self.shape.clone();
        params.scale_core.num_stars = self.star_count;
        params.scale_core.lattice_size = Some(self.lattice_edge);
        params.seed = self.seed;
        params.hyperlane.num_hyperlanes_default = self.target_hyperlanes;
        params.hyperlane.num_hyperlanes_max = self.target_hyperlanes.max(3);
        params.hyperlane.num_hyperlanes_min = 1;
        params.hyperlane.max_hyperlane_distance = self.max_hyperlane_distance;
        params.hyperlane.ensure_connected = self.ensure_connected && !self.allow_disconnected;
        params.shape.shape_params = self.submission_shape_params();
        params.clustering.cluster_count = Some(self.cluster_count);
        params.clustering.cluster_radius = self.cluster_radius;
        if self.no_partitions {
            params.partitioning.home_system_partitions = 0;
            params.partitioning.open_space_partitions = 0;
        }
        params
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("generation failed: {0}")]
    Producer(#[from] simthing_mapgenerator::PlaceAndEmitError),
    #[error("topology options failed: {0}")]
    Topology(#[from] simthing_mapgenerator::HyperlaneError),
    #[error("report write failed: {0}")]
    Report(#[from] ReportError),
}

pub fn run_generation(profile: &GenerationProfile) -> Result<GenerationRunOutput, GenerationError> {
    let registry = ShapeRegistry::default();
    let mut profile = profile.clone();
    profile.persist_editable_fields_for_active_shape();
    let params = profile.to_map_generator_params();
    params.validate(&registry)?;
    let (hyperlane, special, partition, cluster) = structure_options_from_params(&params)?;
    let result = generate_galaxy_with_structure(
        &params,
        &registry,
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        if profile.no_partitions {
            None
        } else {
            Some(partition)
        },
        Some(cluster),
    )?;
    let report = build_generation_report(&params, &result, ReportArtifacts::new());
    Ok(GenerationRunOutput {
        galaxy_display_name: profile.galaxy_type_label(),
        result,
        report,
    })
}

pub fn write_report_json(
    report: &GenerationReport,
    path: &std::path::Path,
) -> Result<(), GenerationError> {
    write_generation_report_json(report, path)?;
    Ok(())
}

pub fn quality_panel_accepts_report(report: &GenerationReport) -> bool {
    report.output.map_quality_status == MAP_QUALITY_PASS
}

pub fn quality_panel_flags_report(report: &GenerationReport) -> bool {
    report.output.map_quality_status == MAP_QUALITY_WARN
        || report.output.map_quality_status == MAP_QUALITY_FAIL
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape_params::report_has_spiral_only_params;

    #[test]
    fn editor_default_generation_profile_matches_known_healthy_report_params() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        assert!(profile.matches_known_healthy_editor_prep_params());
    }

    #[test]
    fn editor_disc_generation_does_not_submit_spiral_params() {
        let mut profile = GenerationProfile::default_spiral_2_dense_3000();
        profile.init_shape_param_storage();
        profile.switch_shape("spiral_2", "elliptical");
        let params = profile.to_map_generator_params();
        assert!(!params.shape.shape_params.contains_key("arm_width"));
        assert!(!params.shape.shape_params.contains_key("arm_tightness"));
    }

    #[test]
    fn elliptical_generation_does_not_submit_spiral_params() {
        let profile = GenerationPreset::Elliptical1000.to_profile();
        let params = profile.to_map_generator_params();
        assert_eq!(params.shape.shape, "elliptical");
        assert!(!params.shape.shape_params.contains_key("arm_width"));
        assert!(!params.shape.shape_params.contains_key("arm_tightness"));
    }

    #[test]
    fn disc_generation_does_not_submit_spiral_params() {
        let profile = GenerationPreset::Disc1500Connected.to_profile();
        let params = profile.to_map_generator_params();
        assert_eq!(params.shape.shape, "elliptical");
        assert!(!params.shape.shape_params.contains_key("arm_width"));
        assert!(!params.shape.shape_params.contains_key("arm_tightness"));
    }

    #[test]
    fn editor_spiral_generation_submits_spiral_params() {
        let mut profile = GenerationProfile::default_spiral_2_dense_3000();
        profile.init_shape_param_storage();
        let params = profile.to_map_generator_params();
        assert_eq!(params.shape.shape_params.get("arm_width"), Some(&14.0));
        assert_eq!(params.shape.shape_params.get("arm_tightness"), Some(&0.6));
        assert_eq!(params.shape.shape_params.get("jitter"), Some(&2.0));
    }

    #[test]
    fn spiral_generation_still_submits_spiral_params() {
        let profile = GenerationPreset::Spiral4Visual1500.to_profile();
        let params = profile.to_map_generator_params();
        assert_eq!(params.shape.shape, "spiral_4");
        assert_eq!(params.shape.shape_params.get("arm_width"), Some(&14.0));
        assert_eq!(params.shape.shape_params.get("arm_tightness"), Some(&0.6));
        assert_eq!(params.shape.shape_params.get("jitter"), Some(&2.0));
    }

    #[test]
    fn shape_change_preserves_old_shape_params_as_dormant_state() {
        let mut profile = GenerationProfile::default_spiral_2_dense_3000();
        profile.init_shape_param_storage();
        profile.arm_width = 20.0;
        profile.persist_editable_fields_for_active_shape();
        profile.switch_shape("spiral_2", "elliptical");
        profile.switch_shape("elliptical", "spiral_2");
        assert!((profile.arm_width - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn inactive_shape_params_do_not_validate_or_block_generation() {
        let profile = GenerationPreset::Disc1500Connected.to_profile();
        run_generation(&profile).expect("disc generation must not fail on dormant spiral params");
    }

    #[test]
    fn disc_preset_clears_or_deactivates_spiral_params() {
        let profile = GenerationPreset::Disc1500Connected.to_profile();
        let params = profile.to_map_generator_params();
        assert!(!params.shape.shape_params.contains_key("arm_width"));
        assert!(!params.shape.shape_params.contains_key("arm_tightness"));
    }

    #[test]
    fn report_for_disc_has_no_spiral_only_params() {
        let profile = GenerationPreset::Disc1500Connected.to_profile();
        let output = run_generation(&profile).expect("disc generation");
        assert!(!report_has_spiral_only_params(
            &output.report.request.shape_params
        ));
    }

    #[test]
    fn inactive_shape_params_are_visible_but_not_submitted() {
        let mut profile = GenerationProfile::default_spiral_2_dense_3000();
        profile.init_shape_param_storage();
        profile.arm_width = 22.0;
        profile.arm_tightness = 0.25;
        profile.jitter = 3.0;
        profile.switch_shape("spiral_2", "elliptical");
        assert_eq!(profile.arm_width, 22.0);
        assert_eq!(profile.arm_tightness, 0.25);

        let params = profile.to_map_generator_params();
        assert!(!params.shape.shape_params.contains_key("arm_width"));
        assert!(!params.shape.shape_params.contains_key("arm_tightness"));
        assert_eq!(params.shape.shape_params.get("jitter"), Some(&2.0));
    }
}
