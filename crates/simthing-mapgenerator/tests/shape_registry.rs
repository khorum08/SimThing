use simthing_mapgenerator::{RegisteredShapeName, ShapeRegistry};

const REQUIRED_VANILLA_SHAPES: &[&str] = &[
    "elliptical",
    "static",
    "arbitrary_static",
    "spiral_2",
    "spiral_3",
    "spiral_4",
    "spiral_6",
    "ring",
    "bar",
    "starburst",
    "cartwheel",
    "spoked",
];

#[test]
fn registry_contains_required_vanilla_shape_names() {
    let registry = ShapeRegistry::default();
    for name in REQUIRED_VANILLA_SHAPES {
        assert!(registry.contains(name), "missing shape {name}");
    }
    assert_eq!(
        registry.registered_names_sorted().len(),
        REQUIRED_VANILLA_SHAPES.len()
    );
}

#[test]
fn shape_registry_is_data_driven_descriptor_surface_not_fixed_enum() {
    let registry = ShapeRegistry::default();
    let dynamic = RegisteredShapeName("modded_custom_shape".into());
    assert_eq!(dynamic.as_str(), "modded_custom_shape");
    assert!(!registry.contains("modded_custom_shape"));
}

#[test]
fn registry_descriptor_and_executable_names_are_single_sourced() {
    let registry = ShapeRegistry::default();
    let executable = registry.executable_names_sorted();
    for name in &executable {
        assert!(
            registry.is_executable(name),
            "{name} listed executable but entry has no strategy"
        );
        assert!(
            registry.get(name).expect("descriptor").executable,
            "{name} strategy present but descriptor.executable is false"
        );
    }
    for desc in registry.descriptors() {
        if desc.executable {
            assert!(
                registry.is_executable(desc.name.as_str()),
                "{} marked executable in descriptor but no strategy",
                desc.name.as_str()
            );
        }
    }
}

#[test]
fn executable_shape_names_are_derived_from_registry_entries() {
    let registry = ShapeRegistry::default();
    let derived = registry.executable_names_sorted();
    assert!(derived.contains(&"elliptical".to_string()));
    assert!(derived.contains(&"spiral_4".to_string()));
    assert!(derived.contains(&"ring".to_string()));
    assert!(!derived.contains(&"not_registered".to_string()));
}

#[test]
fn adding_descriptor_does_not_require_strategy_by_name_match_arm() {
    let registry = ShapeRegistry::default();
    // New executable shapes resolve via registry entries — no manual match ladder.
    registry.resolve_strategy("spiral_6").expect("spiral_6");
    registry.resolve_strategy("cartwheel").expect("cartwheel");
    registry.resolve_strategy("spoked").expect("spoked");
    assert!(registry.resolve_strategy("modded_descriptor_only").is_err());
}

#[test]
fn all_required_shapes_have_mode_and_parameter_metadata() {
    let registry = ShapeRegistry::default();
    for name in REQUIRED_VANILLA_SHAPES {
        let desc = registry.get(name).expect(name);
        assert!(!desc.display_name.is_empty());
        assert!(!desc.description.is_empty());
        if desc.requires_explicit_cells {
            assert!(desc.allows_key("coordinate_transform"));
        } else {
            assert!(desc.allows_key("jitter") || desc.allows_key("core_radius"));
        }
    }
}

#[test]
fn registry_descriptors_advertise_shape_params() {
    let registry = ShapeRegistry::default();
    let spiral = registry.get("spiral_4").expect("spiral_4 registered");
    assert!(spiral.allows_key("arm_tightness"));
    assert!(!spiral.allows_key("coordinate_transform"));
}

#[test]
fn arbitrary_static_descriptor_exists() {
    let registry = ShapeRegistry::default();
    let desc = registry
        .get("arbitrary_static")
        .expect("arbitrary_static entry");
    assert!(desc.requires_explicit_cells);
    assert!(desc.allows_key("coordinate_transform"));
}

#[test]
fn all_vanilla_shapes_are_executable_in_pr8() {
    let registry = ShapeRegistry::default();
    for name in REQUIRED_VANILLA_SHAPES {
        assert!(
            registry.is_executable(name),
            "{name} must be executable in PR8"
        );
        registry.resolve_strategy(name).expect(name);
    }
}
