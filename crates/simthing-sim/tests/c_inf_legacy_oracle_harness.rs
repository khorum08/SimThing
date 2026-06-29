//! C-INF-2 legacy oracle harness integration tests.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, PropertyTransformDelta, PropertyValue, SimProperty,
    SimThing, SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, DispatchCoordinator, FeederWork, PatchTransform, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::{
    apply_oracle_flags, assert_events_oracle, assert_values_oracle, run_family_oracle,
    BoundaryProtocol, OracleCapture, OracleExactness, OracleFamily, OracleScenario, SimRuntimeTree,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
fn c_inf2_intent_oracle_harness_single_add() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let run = run_family_oracle(
        OracleFamily::Intent,
        OracleScenario::Default,
        OracleExactness::BitExact,
        |use_accumulator| {
            let mut reg = DimensionRegistry::new();
            let mut p = SimProperty::simple("core", "loyalty", 0);
            p.intensity_behavior = Some(IntensityBehavior::default());
            let pid = reg.register(p);
            let mut alloc = SlotAllocator::new();
            let id = SimThing::new(SimThingKind::Cohort, 0).id;
            alloc.alloc(id);
            let n_dims = reg.total_columns as u32;
            let n_slots = alloc.capacity() as u32;

            let ctx = GpuContext::new_blocking().expect("gpu");
            let mut state = WorldGpuState::new(ctx, &reg, n_slots);
            let pipelines = Pipelines::new(&state.ctx);
            let mut patcher = TransformPatcher::new(n_slots as usize);
            let mut coord = DispatchCoordinator::new(n_slots, n_dims, 8);
            let (tx, rx) = feeder_channel();

            coord.shadow.fill(0.0);
            coord.shadow[0] = 0.5;
            coord.upload_full_shadow(&state);

            let mut world = SimThing::new(SimThingKind::World, 0);
            let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
            cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
            world.add_child(cohort);

            let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
            apply_oracle_flags(&mut proto.flags, OracleFamily::Intent, use_accumulator);
            proto.initial_gpu_sync(&coord, &mut state);

            tx.send(FeederWork::Patch(PatchTransform {
                target: id,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
                },
            }))
            .unwrap();

            let _ = coord.tick(
                &rx,
                &mut patcher,
                &proto.registry,
                &proto.allocator,
                &pipelines,
                &mut state,
                1.0,
            );

            OracleCapture {
                values: state.read_values(),
                events: Vec::new(),
                readback_bytes: 0,
                gpu_us: None,
            }
        },
    );

    assert_values_oracle(&run, "c_inf2_intent_single_add");
}

#[test]
fn c_inf2_threshold_oracle_harness_fission_stress_smoke() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    use simthing_core::{Direction, FissionTemplate, FissionThreshold, SubFieldRole};

    let run = run_family_oracle(
        OracleFamily::Threshold,
        OracleScenario::ThresholdFissionStress,
        OracleExactness::BitExact,
        |use_accumulator| {
            let mut reg = DimensionRegistry::new();
            let mut pressure = SimProperty::simple("stress", "pressure", 0);
            pressure.intensity_behavior = Some(IntensityBehavior::default());
            pressure.fission_templates = vec![FissionThreshold {
                sub_field: SubFieldRole::Amount,
                threshold: 0.3,
                direction: Direction::Falling,
                template: FissionTemplate {
                    child_kind: SimThingKindTag::Cohort,
                    fusion_intensity_threshold: 0.9,
                    fusion_scar_coefficient: 0.02,
                    resolution_label: "stress_resolved".into(),
                    clone_capability_children: false,
                    capability_container_kinds: Vec::new(),
                },
                secondary: None,
            }];
            let pid = reg.register(pressure);
            let layout = reg.property(pid).layout.clone();
            let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();

            let n_slots = 64u32;
            let mut world = SimThing::new(SimThingKind::World, 0);
            for i in 0..n_slots.saturating_sub(1) {
                let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
                let mut pv = PropertyValue::from_layout(&layout);
                pv.data[amount] = 0.31 + ((i % 5) as f32) * 0.001;
                cohort.add_property(pid, pv);
                world.add_child(cohort);
            }

            let mut alloc = SlotAllocator::new();
            alloc.populate_from_tree(&world);
            let n_dims = reg.total_columns as u32;
            let ctx = GpuContext::new_blocking().expect("gpu");
            let mut state = WorldGpuState::new(ctx, &reg, n_slots);
            let pipelines = Pipelines::new(&state.ctx);
            let mut patcher = TransformPatcher::new(n_slots as usize);
            let mut coord = DispatchCoordinator::new(n_slots, n_dims, 1);
            let (_tx, rx) = feeder_channel();

            let projected_len = alloc.capacity() * n_dims as usize;
            let mut projected = vec![0.0; projected_len];
            simthing_gpu::project_tree_to_values(
                &world,
                &reg,
                &alloc,
                n_dims as usize,
                &mut projected,
            );
            coord.shadow[..projected_len].copy_from_slice(&projected);

            let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
            apply_oracle_flags(&mut proto.flags, OracleFamily::Threshold, use_accumulator);
            proto.initial_gpu_sync(&coord, &mut state);

            let mut events = Vec::new();
            for _ in 0..5 {
                let out = coord.tick(
                    &rx,
                    &mut patcher,
                    &proto.registry,
                    &proto.allocator,
                    &pipelines,
                    &mut state,
                    1.0,
                );
                events.extend(out.events);
            }

            OracleCapture {
                values: Vec::new(),
                events,
                readback_bytes: 0,
                gpu_us: None,
            }
        },
    );

    assert_events_oracle(&run, "c_inf2_threshold_fission_stress_smoke");
}
