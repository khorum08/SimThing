use simthing_mapgenerator::{RegisteredShapeName, RegistryResolveError, ShapeRegistry};

#[test]
fn shape_registry_is_data_driven_descriptor_surface_not_fixed_enum() {
    let registry = ShapeRegistry::default();
    let names: Vec<_> = registry.registered_names_sorted();
    assert!(names.contains(&"elliptical".to_string()));
    assert!(names.contains(&"static".to_string()));
    assert!(names.contains(&"arbitrary_static".to_string()));
    assert!(names.len() >= 6);

    // Names are strings resolved at runtime — not a Rust enum of supported shapes.
    let dynamic = RegisteredShapeName("modded_custom_shape".into());
    assert_eq!(dynamic.as_str(), "modded_custom_shape");
    assert!(!registry.contains("modded_custom_shape"));
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
    assert!(desc.allows_key("coordinate_transform"));
}

#[test]
fn descriptor_only_shapes_error_with_executable_list() {
    let registry = ShapeRegistry::default();
    let err = match registry.resolve_strategy("ring") {
        Err(err) => err,
        Ok(_) => panic!("expected not-implemented error"),
    };
    assert!(matches!(
        err,
        RegistryResolveError::StrategyNotImplemented { .. }
    ));
    let msg = err.to_string();
    assert!(msg.contains("elliptical"));
    assert!(msg.contains("executable"));
}
