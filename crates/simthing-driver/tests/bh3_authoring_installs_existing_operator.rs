//! BH-3-AUTHORING-0 — install path uses existing BH/PALMA operator surfaces.

use simthing_clausething::{hydrate_field_operator_pack, parse_raw_document};
use simthing_driver::{
    compiled_stress_compose_to_gpu_config, compiled_w_impedance_compose_to_gpu_config,
    composed_w_min_plus_stencil_config,
};
use simthing_gpu::MIN_PLUS_INF;
use simthing_spec::{
    compile_region_field_preview, compile_stress_compose_preview,
    compile_w_impedance_compose_preview, MappingExecutionProfile,
};

const FIXTURE: &str =
    include_str!("../../simthing-clausething/tests/fixtures/bh3_field_operator.clause");

const FORBIDDEN_HOT_PATH: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "pathfinding",
    "movement_engine",
    "predecessor",
    "ClauseThing",
];

#[test]
fn bh3_authoring_installs_existing_operator_surfaces_without_runtime_action() {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse fixture");
    let pack = hydrate_field_operator_pack(&document).expect("hydrate fixture");
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(!pack.game_mode.mapping_execution_profile.enables_execution());

    let field = &pack.game_mode.region_fields[0];
    let region = compile_region_field_preview(field).expect("region field admission");
    assert!(matches!(
        region.stencil.operator,
        simthing_spec::CompiledRegionFieldOperator::SaturatingFlux { .. }
    ));

    let w_spec = pack
        .w_impedance_compose
        .as_ref()
        .expect("authored w compose");
    let w_compiled = compile_w_impedance_compose_preview(w_spec).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let _palma = composed_w_min_plus_stencil_config(&w_gpu, 0, 5, (0, 0), MIN_PLUS_INF);

    let stress_spec = pack
        .stress_compose
        .as_ref()
        .expect("authored stress compose");
    let stress_compiled = compile_stress_compose_preview(stress_spec).expect("stress admission");
    let _stress_gpu = compiled_stress_compose_to_gpu_config(&stress_compiled);

    let bridge_src = include_str!("../src/w_impedance_compose_bridge.rs");
    for token in FORBIDDEN_HOT_PATH {
        assert!(
            !bridge_src.contains(token),
            "bridge must not contain forbidden token `{token}`"
        );
    }
}
