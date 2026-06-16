//! Shape-parameter scoping for editor generation — only active shape params are submitted.

use std::collections::BTreeMap;

use simthing_mapgenerator::shape_param_specs;

pub const SPIRAL_ARM_KEYS: [&str; 3] = ["arm_width", "arm_tightness", "jitter"];

pub fn spiral_arm_params_active(shape: &str) -> bool {
    matches!(shape, "spiral_2" | "spiral_3" | "spiral_4" | "spiral_6")
}

pub fn param_keys_for_shape(shape: &str) -> Vec<&'static str> {
    shape_param_specs(shape)
        .iter()
        .map(|spec| spec.key)
        .collect()
}

pub fn default_params_for_shape(shape: &str) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    match shape {
        "spiral_2" | "spiral_3" | "spiral_4" | "spiral_6" => {
            out.insert("arm_width".into(), 14.0);
            out.insert("arm_tightness".into(), 0.6);
            out.insert("jitter".into(), 2.0);
        }
        "elliptical" => {
            out.insert("jitter".into(), 2.0);
        }
        _ => {}
    }
    for spec in shape_param_specs(shape) {
        if let Some(default) = spec.default {
            out.entry(spec.key.to_string()).or_insert(default);
        }
    }
    out
}

pub fn active_shape_params_for(
    shape: &str,
    shape_params_by_shape: &BTreeMap<String, BTreeMap<String, f64>>,
) -> BTreeMap<String, f64> {
    let allowed: Vec<&str> = param_keys_for_shape(shape)
        .into_iter()
        .map(|s| s as &str)
        .collect();
    let mut out = BTreeMap::new();
    if let Some(stored) = shape_params_by_shape.get(shape) {
        for (key, value) in stored {
            if allowed.contains(&key.as_str()) {
                out.insert(key.clone(), *value);
            }
        }
    }
    if out.is_empty() {
        for (key, value) in default_params_for_shape(shape) {
            if allowed.contains(&key.as_str()) {
                out.insert(key, value);
            }
        }
    }
    out
}

pub fn store_dormant_shape_params(
    shape: &str,
    editable: &BTreeMap<String, f64>,
    shape_params_by_shape: &mut BTreeMap<String, BTreeMap<String, f64>>,
) {
    let allowed = param_keys_for_shape(shape);
    let mut map = BTreeMap::new();
    for key in allowed {
        if let Some(value) = editable.get(key) {
            map.insert(key.to_string(), *value);
        }
    }
    if !map.is_empty() {
        shape_params_by_shape.insert(shape.to_string(), map);
    }
}

pub fn editable_values_from_profile_fields(
    arm_width: f64,
    arm_tightness: f64,
    jitter: f64,
) -> BTreeMap<String, f64> {
    BTreeMap::from([
        ("arm_width".into(), arm_width),
        ("arm_tightness".into(), arm_tightness),
        ("jitter".into(), jitter),
    ])
}

pub fn apply_editable_values_to_profile_fields(
    values: &BTreeMap<String, f64>,
    arm_width: &mut f64,
    arm_tightness: &mut f64,
    jitter: &mut f64,
) {
    if let Some(v) = values.get("arm_width") {
        *arm_width = *v;
    }
    if let Some(v) = values.get("arm_tightness") {
        *arm_tightness = *v;
    }
    if let Some(v) = values.get("jitter") {
        *jitter = *v;
    }
}

pub fn report_has_spiral_only_params(report_shape_params: &BTreeMap<String, f64>) -> bool {
    report_shape_params.contains_key("arm_width")
        || report_shape_params.contains_key("arm_tightness")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_disc_generation_does_not_submit_spiral_params() {
        let mut by_shape = BTreeMap::new();
        by_shape.insert(
            "spiral_2".into(),
            BTreeMap::from([
                ("arm_width".into(), 14.0),
                ("arm_tightness".into(), 0.6),
                ("jitter".into(), 2.0),
            ]),
        );
        by_shape.insert(
            "elliptical".into(),
            BTreeMap::from([("jitter".into(), 1.5)]),
        );
        let active = active_shape_params_for("elliptical", &by_shape);
        assert!(!active.contains_key("arm_width"));
        assert!(!active.contains_key("arm_tightness"));
        assert_eq!(active.get("jitter"), Some(&1.5));
    }

    #[test]
    fn editor_spiral_generation_submits_spiral_params() {
        let mut by_shape = BTreeMap::new();
        by_shape.insert(
            "spiral_2".into(),
            BTreeMap::from([
                ("arm_width".into(), 14.0),
                ("arm_tightness".into(), 0.6),
                ("jitter".into(), 2.0),
            ]),
        );
        let active = active_shape_params_for("spiral_2", &by_shape);
        assert_eq!(active.get("arm_width"), Some(&14.0));
        assert_eq!(active.get("arm_tightness"), Some(&0.6));
        assert_eq!(active.get("jitter"), Some(&2.0));
    }

    #[test]
    fn shape_change_preserves_old_shape_params_as_dormant_state() {
        let mut by_shape = BTreeMap::new();
        let spiral_vals = BTreeMap::from([
            ("arm_width".into(), 14.0),
            ("arm_tightness".into(), 0.6),
            ("jitter".into(), 2.0),
        ]);
        store_dormant_shape_params("spiral_2", &spiral_vals, &mut by_shape);
        store_dormant_shape_params(
            "elliptical",
            &BTreeMap::from([("jitter".into(), 0.5)]),
            &mut by_shape,
        );
        let restored = active_shape_params_for("spiral_2", &by_shape);
        assert_eq!(restored.get("arm_width"), Some(&14.0));
        let elliptical = active_shape_params_for("elliptical", &by_shape);
        assert!(!elliptical.contains_key("arm_width"));
    }

    #[test]
    fn inactive_shape_params_do_not_validate_or_block_generation() {
        let mut by_shape = BTreeMap::new();
        by_shape.insert(
            "spiral_2".into(),
            BTreeMap::from([
                ("arm_width".into(), 14.0),
                ("arm_tightness".into(), 0.6),
                ("jitter".into(), 2.0),
            ]),
        );
        let elliptical = active_shape_params_for("elliptical", &by_shape);
        assert!(elliptical.get("arm_tightness").is_none());
    }

    #[test]
    fn disc_preset_clears_or_deactivates_spiral_params() {
        let active = active_shape_params_for("elliptical", &BTreeMap::new());
        assert!(!active.contains_key("arm_width"));
        assert!(!active.contains_key("arm_tightness"));
    }

    #[test]
    fn report_for_disc_has_no_spiral_only_params() {
        let active = active_shape_params_for("elliptical", &BTreeMap::new());
        assert!(!report_has_spiral_only_params(&active));
    }
}
