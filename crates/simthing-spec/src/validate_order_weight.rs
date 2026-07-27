//! ORDER-WEIGHT-CLASS-0 admission: finite class data + directive overlay shape.

use crate::error::SpecError;
use crate::spec::order_weight::OrderWeightClassSpec;
use crate::spec::overlay::OverlaySpec;
use simthing_core::{OverlaySource, TransformOp};

/// Validate order-weight class table + any directive overlays that reference it.
pub fn validate_order_weight_classes(
    classes: &[OrderWeightClassSpec],
) -> Result<(), SpecError> {
    let mut seen = std::collections::HashSet::new();
    for class in classes {
        if class.id.is_empty() {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: "class id must be non-empty".into(),
                source_span_token: class.source_span_token,
            });
        }
        if !seen.insert(class.id.clone()) {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: "duplicate class id".into(),
                source_span_token: class.source_span_token,
            });
        }
        if !class.magnitude.is_finite() {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: format!("magnitude must be finite (got {})", class.magnitude),
                source_span_token: class.source_span_token,
            });
        }
        if class.magnitude <= 0.0 {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: format!("magnitude must be > 0 (got {})", class.magnitude),
                source_span_token: class.source_span_token,
            });
        }
    }
    Ok(())
}

/// Validate a single overlay against the order-weight class table.
///
/// Rules:
/// - All transform magnitudes must be finite (no Inf/NaN).
/// - If `order_weight_class` is set: source must be Player; class must exist;
///   at least one transform magnitude must equal the class magnitude.
/// - If a Player overlay carries a finite dominant-looking Add/Set magnitude
///   (≥ any declared class min, or ≥ 1000 when no classes) without citing a
///   class → class-less dominant weight error.
pub fn validate_order_weight_overlay(
    overlay: &OverlaySpec,
    classes: &[OrderWeightClassSpec],
) -> Result<(), SpecError> {
    // Non-finite magnitudes are always rejected (RF-1 envelope).
    for (_, op) in &overlay.sub_field_deltas {
        let value = op_magnitude(op);
        if !value.is_finite() {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!("non-finite transform magnitude {value}"),
                source_span_token: overlay.source_span_token,
            });
        }
    }

    if let Some(class_id) = &overlay.order_weight_class {
        let class = classes.iter().find(|c| c.id == *class_id).ok_or_else(|| {
            SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!("unknown order_weight_class `{class_id}`"),
                source_span_token: overlay.source_span_token,
            }
        })?;
        if overlay.source != OverlaySource::Player {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: "order-weight directive source must be Player".into(),
                source_span_token: overlay.source_span_token,
            });
        }
        let uses_class_magnitude = overlay.sub_field_deltas.iter().any(|(_, op)| {
            let v = op_magnitude(op);
            v.is_finite() && (v - class.magnitude).abs() <= f32::EPSILON
        });
        if !uses_class_magnitude {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "directive transform magnitude must match class `{}` magnitude {}",
                    class.id, class.magnitude
                ),
                source_span_token: overlay.source_span_token,
            });
        }
        return Ok(());
    }

    // Class-less dominant weight: Player overlay with a magnitude that reaches
    // the sanctioned dominance band without citing a class.
    if overlay.source == OverlaySource::Player {
        let dominance_floor = classes
            .iter()
            .map(|c| c.magnitude)
            .fold(1000.0_f32, f32::min);
        for (_, op) in &overlay.sub_field_deltas {
            let v = op_magnitude(op).abs();
            if v.is_finite() && v >= dominance_floor {
                return Err(SpecError::OrderWeightDirectiveInvalid {
                    overlay_id: overlay.id.clone(),
                    reason: format!(
                        "class-less dominant weight {v} (dominance floor {dominance_floor}); cite order_weight_class"
                    ),
                    source_span_token: overlay.source_span_token,
                });
            }
        }
    }

    Ok(())
}

fn op_magnitude(op: &TransformOp) -> f32 {
    match op {
        TransformOp::Add(v) | TransformOp::Multiply(v) | TransformOp::Set(v) => *v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::install_target::InstallTargetSpec;
    use simthing_core::{OverlayKind, OverlayLifecycle, SubFieldRole};

    fn class(id: &str, magnitude: f32) -> OrderWeightClassSpec {
        OrderWeightClassSpec {
            id: id.into(),
            magnitude,
            source_span_token: Some(1),
        }
    }

    fn overlay(
        id: &str,
        source: OverlaySource,
        class_id: Option<&str>,
        magnitude: f32,
    ) -> OverlaySpec {
        OverlaySpec {
            id: id.into(),
            display_name: String::new(),
            targets_property: "order::fleet_need".into(),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(magnitude))],
            lifecycle: OverlayLifecycle::Transient {
                dissolution_conditions: vec![],
            },
            kind: OverlayKind::Instruction,
            source,
            install: InstallTargetSpec::SessionRoot,
            order_weight_class: class_id.map(|s| s.into()),
            source_span_token: Some(9),
        }
    }

    #[test]
    fn order_weight_admission_table() {
        const CASES: &[(&str, bool)] = &[
            ("class_ok", true),
            ("non_finite_class", false),
            ("non_finite_overlay", false),
            ("class_less_dominant", false),
            ("unknown_class", false),
            ("wrong_source", false),
            ("magnitude_mismatch", false),
            ("ambient_player_ok", true),
        ];
        for (label, expect_ok) in CASES {
            let result = match *label {
                "class_ok" => {
                    let classes = vec![class("destination_order", 10000.0)];
                    validate_order_weight_classes(&classes).and_then(|_| {
                        validate_order_weight_overlay(
                            &overlay("go", OverlaySource::Player, Some("destination_order"), 10000.0),
                            &classes,
                        )
                    })
                }
                "non_finite_class" => {
                    validate_order_weight_classes(&[class("bad", f32::INFINITY)])
                }
                "non_finite_overlay" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, None, f32::NAN),
                    &[],
                ),
                "class_less_dominant" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, None, 10000.0),
                    &[class("destination_order", 10000.0)],
                ),
                "unknown_class" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, Some("missing"), 10000.0),
                    &[class("destination_order", 10000.0)],
                ),
                "wrong_source" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Ai, Some("destination_order"), 10000.0),
                    &[class("destination_order", 10000.0)],
                ),
                "magnitude_mismatch" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, Some("destination_order"), 50.0),
                    &[class("destination_order", 10000.0)],
                ),
                "ambient_player_ok" => validate_order_weight_overlay(
                    &overlay("nudge", OverlaySource::Player, None, 1.5),
                    &[class("destination_order", 10000.0)],
                ),
                other => panic!("unknown case {other}"),
            };
            assert_eq!(
                result.is_ok(),
                *expect_ok,
                "{label}: expected ok={expect_ok}, got {result:?}"
            );
        }
    }
}
