//! Leaf A ingress reproduction for Board 5675549349.
//! RED at ordinary session open is a qualification blocker, not a Model-1 falsification.
//! No skip, pin replacement, source restoration, or oracle fallback is permitted here.

use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::{
    GpuContext, ResidentClearingQualification, QUALIFIED_RESIDENT_CLEARING_FINGERPRINT,
};

#[test]
fn construction_leaf_a_requires_qualified_ordinary_session() {
    let gpu = GpuContext::new_blocking().expect("reference GPU required");
    let qualification = ResidentClearingQualification::capture(&gpu).expect("capture provenance");
    println!("adapter: {:?}", gpu.adapter.get_info());
    println!("qualification: {qualification:#?}");
    println!("required_fingerprint: {QUALIFIED_RESIDENT_CLEARING_FINGERPRINT:016x}");
    println!("observed_fingerprint: {:016x}", qualification.fingerprint());
    println!(
        "bundle: {:016x}",
        ResidentClearingQualification::semantic_kernel_bundle_hash()
    );

    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("leaf_a", "ingress", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut registry,
        &mut root,
    );
    let fixture = Scenario {
        name: "construction-leaf-a-ingress".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 8,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    // Actual production admission, not a copied qualification comparator.
    match SimSession::open(fixture) {
        Ok(_) => println!("ordinary session admitted; construction discriminator may proceed"),
        Err(error) => panic!("Leaf A stopped before economics: {error:?}"),
    }
}
