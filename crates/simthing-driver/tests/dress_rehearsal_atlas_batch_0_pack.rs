#[path = "../src/dress_rehearsal_atlas_batch_0_pack.rs"]
mod dress_rehearsal_atlas_batch_0_pack;

use dress_rehearsal_atlas_batch_0_pack::{
    channel_set_has_kind, channel_set_has_owner_indexed, channel_set_matches, g_zero_sample,
    pack_coord, unpack_coord, AtlasBatchPlan, ChannelKind, LocationMaterialization, LocationRole,
    Owner, CLASS_GALACTIC_20X20, CLASS_PLANET_SURFACE_10X10, CLASS_STAR_SYSTEM_10X10,
    DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID, DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS,
    PACKING_STRATEGY, V78_ATLAS_VRAM_BUDGET_BYTES,
};

#[test]
fn docs_status_matches_gate() {
    assert_eq!(DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID, "ATLAS-BATCH-0-PACK");
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS;
    assert!(status.contains("IMPLEMENTED / PASS"));
    assert!(status.contains("EC-A2a"));
    assert!(status.contains("EC-A2b") || status.contains("PACK-GPU"));
    assert!(
        !status.contains("batched dispatch") || status.contains("deferred"),
        "must not claim EC-A2b GPU dispatch as proven"
    );
}

#[test]
fn pack_plan_is_deterministic() {
    let materialization = LocationMaterialization::canonical();
    let left = AtlasBatchPlan::from_materialization(&materialization);
    let right = AtlasBatchPlan::from_materialization(&materialization);
    assert_eq!(left, right);
}

#[test]
fn locations_group_into_expected_tile_classes() {
    let plan = AtlasBatchPlan::canonical();
    assert_eq!(plan.classes.len(), 3);
    assert_eq!(plan.tiles.len(), 27);

    let galactic = plan.class(CLASS_GALACTIC_20X20).expect("galactic class");
    assert_eq!(galactic.role, LocationRole::Galactic);
    assert_eq!(galactic.tile_width, 20);
    assert_eq!(galactic.tile_height, 20);
    assert_eq!(galactic.source_location_ids.len(), 1);
    assert_eq!(plan.tiles_in_class(CLASS_GALACTIC_20X20).len(), 1);

    let systems = plan
        .class(CLASS_STAR_SYSTEM_10X10)
        .expect("star-system class");
    assert_eq!(systems.role, LocationRole::StarSystem);
    assert_eq!(systems.source_location_ids.len(), 13);
    assert_eq!(plan.tiles_in_class(CLASS_STAR_SYSTEM_10X10).len(), 13);

    let surfaces = plan
        .class(CLASS_PLANET_SURFACE_10X10)
        .expect("surface class");
    assert_eq!(surfaces.role, LocationRole::PlanetSurface);
    assert_eq!(surfaces.source_location_ids.len(), 13);
    assert_eq!(plan.tiles_in_class(CLASS_PLANET_SURFACE_10X10).len(), 13);

    let materialization = LocationMaterialization::canonical();
    for tile in &plan.tiles {
        let class = plan.class(&tile.class_id).expect("class");
        assert_eq!(tile.source_role, class.role);
        let location = materialization
            .location(tile.source_location_id)
            .expect("location");
        assert_eq!(location.role, tile.source_role);
        assert_eq!(location.id, tile.source_location_id);
    }
}

#[test]
fn tile_origins_are_contiguous_and_non_overlapping() {
    let plan = AtlasBatchPlan::canonical();

    for class in &plan.classes {
        let mut occupied = std::collections::HashSet::new();
        for tile in plan.tiles_in_class(&class.class_id) {
            let (ox, oy) = tile.atlas_origin;
            let (tw, th) = tile.tile_dims;
            for y in oy..oy + th {
                for x in ox..ox + tw {
                    assert!(
                        occupied.insert((x, y)),
                        "overlap at ({x},{y}) in class {}",
                        class.class_id
                    );
                }
            }
        }
        let used_cells = occupied.len() as u64;
        let atlas_cells = u64::from(class.atlas_width) * u64::from(class.atlas_height);
        assert_eq!(
            used_cells,
            u64::from(class.tile_width)
                * u64::from(class.tile_height)
                * class.source_location_ids.len() as u64,
            "row-major packing should use every atlas cell in {}",
            class.class_id
        );
        assert_eq!(used_cells, atlas_cells, "no padding in {}", class.class_id);
    }
}

#[test]
fn tile_local_coordinates_round_trip() {
    let plan = AtlasBatchPlan::canonical();
    let materialization = LocationMaterialization::canonical();

    for location in &materialization.locations {
        let class_id = match location.role {
            LocationRole::Galactic => CLASS_GALACTIC_20X20,
            LocationRole::StarSystem => CLASS_STAR_SYSTEM_10X10,
            LocationRole::PlanetSurface => CLASS_PLANET_SURFACE_10X10,
        };
        let samples = [(0, 0), (1, 0), (location.width / 2, location.height / 2)];
        for (x, y) in samples {
            if x >= location.width || y >= location.height {
                continue;
            }
            let packed = pack_coord(&plan, location.id, x, y).expect("pack");
            let (round_id, lx, ly) =
                unpack_coord(&plan, class_id, packed.0, packed.1).expect("unpack");
            assert_eq!(round_id, location.id);
            assert_eq!((lx, ly), (x, y));
        }
    }
}

#[test]
fn g_zero_mask_blocks_inter_tile_bleed() {
    let plan = AtlasBatchPlan::canonical();
    let class = plan
        .class(CLASS_STAR_SYSTEM_10X10)
        .expect("star-system class");
    let atlas_len = (class.atlas_width * class.atlas_height) as usize;
    let mut field = vec![0.0f32; atlas_len];
    for index in 0..atlas_len {
        field[index] = (index as f32 + 1.0) * 0.001;
    }

    let tile_a = plan.tiles_in_class(CLASS_STAR_SYSTEM_10X10)[0];
    let tile_b = plan.tiles_in_class(CLASS_STAR_SYSTEM_10X10)[1];
    let (ax, ay) = (tile_a.atlas_origin.0 + 5, tile_a.atlas_origin.1 + 5);
    let (bx, by) = (tile_b.atlas_origin.0 + 5, tile_b.atlas_origin.1 + 5);

    let in_tile = g_zero_sample(&plan, CLASS_STAR_SYSTEM_10X10, ax, ay, (ax, ay), &field);
    assert!(in_tile > 0.0, "in-tile sample passes through");

    let across_boundary = g_zero_sample(&plan, CLASS_STAR_SYSTEM_10X10, ax, ay, (bx, by), &field);
    assert_eq!(across_boundary, 0.0, "cross-tile neighbor must be zeroed");
}

#[test]
fn vram_multiplier_report_is_numeric_and_budgeted() {
    let plan = AtlasBatchPlan::canonical();
    let report = &plan.vram;

    assert_eq!(report.unpacked_cell_count, 3000);
    assert_eq!(report.packed_cell_count, 3000);
    assert_eq!(report.mask_or_gutter_overhead_cells, 0);
    assert_eq!(report.bytes_per_cell_assumption, 4);
    assert!(report.unpacked_bytes_estimate > 0);
    assert!(report.packed_bytes_estimate > 0);
    assert!((report.vram_multiplier - 1.0).abs() < f64::EPSILON);
    assert_eq!(report.budget_name, "V78AtlasVramBudget");
    assert!(report.budget_pass);
    assert!(report.packed_bytes_estimate <= V78_ATLAS_VRAM_BUDGET_BYTES);
    assert_eq!(PACKING_STRATEGY.len() > 0, true);
}

#[test]
fn channel_metadata_survives_pack() {
    let plan = AtlasBatchPlan::canonical();
    let materialization = LocationMaterialization::canonical();

    for class in &plan.classes {
        for location_id in &class.source_location_ids {
            let location = materialization.location(*location_id).expect("location");
            assert!(
                channel_set_matches(&class.channels, &location.channels),
                "class {} must preserve channels for {:?}",
                class.class_id,
                location_id
            );
        }
    }

    let galactic = plan.class(CLASS_GALACTIC_20X20).unwrap();
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::Disruption
    ));
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::FleetStrength(Owner::Terran)
    ));

    let surface = plan.class(CLASS_PLANET_SURFACE_10X10).unwrap();
    assert!(channel_set_has_kind(&surface.channels, ChannelKind::Labor));
    assert!(channel_set_has_kind(
        &surface.channels,
        ChannelKind::Production
    ));
}

#[test]
fn owner_metadata_survives_pack_without_owner_runtime() {
    let plan = AtlasBatchPlan::canonical();
    let galactic = plan.class(CLASS_GALACTIC_20X20).unwrap();
    assert!(channel_set_has_owner_indexed(&galactic.channels));
    assert!(galactic
        .channels
        .channels
        .iter()
        .any(|channel| matches!(channel.kind, ChannelKind::FleetStrength(Owner::Pirate))));
}
