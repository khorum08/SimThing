//! C-2 — Atlas Admission Relaxation tests (bounded algebraic-G=0 only).
//!
//! These tests validate that C-2 correctly admits only the narrow bounded algebraic G=0 case
//! while continuing to reject everything outside that scope.

use simthing_spec::designer_admission::{
    AtlasAdmissionProfile, AtlasAdmissionSpec, AtlasIsolationAdmissionMode,
};
use simthing_spec::V78AtlasVramBudget;
#[test]
fn c2_rejects_missing_protocol_oracle() {
    let mut spec = make_typical_huge_commodity_spec();
    spec.protocol_oracle_backed = false;
    let decision = spec.evaluate();
    assert!(!decision.admitted);
}


fn make_typical_huge_commodity_spec() -> AtlasAdmissionSpec {
    AtlasAdmissionSpec {
        request_atlas_batching: true,
        profile: AtlasAdmissionProfile::TypicalHugeCommodity,
        tile_width: 10,
        tile_height: 10,
        homogeneous_square_tiles: true,
        isolation: AtlasIsolationAdmissionMode::AlgebraicTileLocalMaskG0,
        protocol_oracle_backed: true,
        active_vram_budget: V78AtlasVramBudget::default_1p5_gib(),
        multiplier_reporting_required: true,
        production_runtime: false,
        default_on: false,
        active_mask: false,
        source_identity: false,
        physical_gutter: false,
    }
}

fn make_horizon_stress_spec() -> AtlasAdmissionSpec {
    AtlasAdmissionSpec {
        request_atlas_batching: true,
        profile: AtlasAdmissionProfile::HorizonDedicatedServerStress,
        tile_width: 10,
        tile_height: 10,
        homogeneous_square_tiles: true,
        isolation: AtlasIsolationAdmissionMode::AlgebraicTileLocalMaskG0,
        protocol_oracle_backed: true,
        active_vram_budget: V78AtlasVramBudget::default_1p5_gib(),
        multiplier_reporting_required: true,
        production_runtime: false,
        default_on: false,
        active_mask: false,
        source_identity: false,
        physical_gutter: false,
    }
}
