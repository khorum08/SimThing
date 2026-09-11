//! Generation remains producer data and enters Studio through spec admission.
use std::collections::BTreeMap;

use simthing_mapeditor::generation::{run_generation, GenerationPreset};
use simthing_mapeditor::hydration::{
    generate_simthing_spec_scenario, generate_simthing_spec_scenario_with_star_names,
    hydrate_generation_into_studio_grid, hydrate_generation_result_into_studio_grid,
    hydrate_mapgen_result_into_simthing_spec,
    hydrate_mapgen_result_into_simthing_spec_with_star_names,
};
use simthing_mapeditor::StudioSession;
use simthing_mapgenerator::ShapeRegistry;
use simthing_spec::{
    load_scenario_spec_from_json_str, serialize_scenario_authority, SimThingScenarioSpec,
};

fn geometry(spec: &SimThingScenarioSpec) -> (Vec<(u32, u32, u32)>, Vec<(String, String)>) {
    let mut cells: Vec<_> = spec
        .structural_grid
        .placements
        .iter()
        .map(|cell| (cell.system_id, cell.row, cell.col))
        .collect();
    let mut links: Vec<_> = spec
        .links
        .iter()
        .map(|link| (link.from_system_id.clone(), link.to_system_id.clone()))
        .collect();
    cells.sort();
    links.sort();
    (cells, links)
}

#[test]
fn rehearsal_ingress_generation_presets_submit_current_parameter_data() {
    for preset in GenerationPreset::all()
        .iter()
        .filter(|preset| preset.is_active())
    {
        let mut profile = preset.to_profile();
        profile.star_count = 16;
        profile.lattice_edge = 32;
        profile.target_hyperlanes = 3;
        profile.cluster_count = 1;
        profile.init_shape_param_storage();
        profile.jitter = 0.25;
        let params = profile.to_map_generator_params();
        assert_eq!(
            params.shape.shape_params["jitter"], 0.25,
            "public parameter conversion must consume the current editor value"
        );
        params.validate(&ShapeRegistry::default()).unwrap();
        let output = run_generation(&profile).unwrap();
        assert_eq!(
            output.report.request.shape_params,
            params.shape.shape_params
        );
        assert_eq!(output.report.request.shape, params.shape.shape);
        assert_eq!(output.report.generator.seed, params.seed);
        assert_eq!(
            output.report.request.star_count,
            params.scale_core.num_stars
        );
        assert_eq!(output.result.placement.systems.len(), 16);
    }
}

#[test]
fn rehearsal_ingress_generation_adapters_preserve_admitted_structural_records() {
    let mut profile = GenerationPreset::Spiral4Visual1500.to_profile();
    profile.star_count = 16;
    profile.lattice_edge = 32;
    profile.target_hyperlanes = 3;
    profile.cluster_count = 1;
    let output = run_generation(&profile).unwrap();
    let mut expected_cells: Vec<_> = output
        .result
        .placement
        .systems
        .iter()
        .map(|system| (system.id, system.coord.row, system.coord.col))
        .collect();
    let mut expected_links: Vec<_> = output
        .result
        .base_hyperlane_edges
        .iter()
        .map(|edge| (edge.from.clone(), edge.to.clone()))
        .collect();
    expected_cells.sort();
    expected_links.sort();
    let expected = (expected_cells, expected_links);
    let names: BTreeMap<_, _> = output
        .result
        .placement
        .systems
        .iter()
        .map(|system| (system.id, format!("Authored star {}", system.id)))
        .collect();
    let scenarios = [
        generate_simthing_spec_scenario(&output).unwrap(),
        hydrate_mapgen_result_into_simthing_spec(&output.result, &output.report).unwrap(),
        generate_simthing_spec_scenario_with_star_names(&output, &names).unwrap(),
        hydrate_mapgen_result_into_simthing_spec_with_star_names(
            &output.result,
            &output.report,
            Some(&names),
        )
        .unwrap(),
    ];
    for (index, spec) in scenarios.iter().enumerate() {
        assert_eq!(geometry(spec), expected);
        let (loaded, report) = load_scenario_spec_from_json_str(
            "generated record",
            &serialize_scenario_authority(spec).unwrap(),
        )
        .unwrap();
        assert!(report.ingestion_ready);
        assert_eq!(geometry(&loaded), expected);
        let session =
            StudioSession::from_loaded_scenario(loaded, "generated.json".into(), None).unwrap();
        assert!(
            session.admission_summary.legacy_world_root,
            "a structural producer record must not invent authored ownership"
        );
        assert!(session.authored_live_profile.is_none());
        if index >= 2 {
            for cell in &session.hydration.grid.gridcells {
                assert_eq!(cell.display_name, names[&cell.system_id]);
            }
        }
    }
    for projection in [
        hydrate_generation_into_studio_grid(&output).unwrap(),
        hydrate_generation_result_into_studio_grid(&output.result, &output.report).unwrap(),
    ] {
        let mut cells: Vec<_> = projection
            .grid
            .gridcells
            .iter()
            .map(|cell| (cell.system_id, cell.structural_row, cell.structural_col))
            .collect();
        cells.sort();
        assert_eq!(cells, expected.0);
        assert_eq!(projection.grid.hyperlanes.len(), expected.1.len());
    }
}
