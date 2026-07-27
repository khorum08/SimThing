//! ORDER-WEIGHT-CLASS-0 admission: finite class data + directive overlay shape.
//!
//! Dominance is arena-grounded: class magnitude must exceed the authored
//! ambient price envelope (`ambient_ceiling`). No magic 1000 floor.

use crate::error::SpecError;
use crate::spec::order_weight::OrderWeightClassSpec;
use crate::spec::overlay::OverlaySpec;
use simthing_core::{OverlayLifecycle, OverlaySource, SubFieldRole, TransformOp};

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
        if !class.ambient_ceiling.is_finite() {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: format!(
                    "ambient_ceiling must be finite (got {})",
                    class.ambient_ceiling
                ),
                source_span_token: class.source_span_token,
            });
        }
        if class.ambient_ceiling < 0.0 {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: format!(
                    "ambient_ceiling must be >= 0 (got {})",
                    class.ambient_ceiling
                ),
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
        // Arena-grounded dominance: magnitude must strictly exceed ambient
        // ceiling so proportional allocation selects the ordered locus.
        if class.magnitude <= class.ambient_ceiling {
            return Err(SpecError::MalformedOrderWeightClass {
                class_id: class.id.clone(),
                reason: format!(
                    "magnitude {} does not dominate ambient_ceiling {} under arena normalization",
                    class.magnitude, class.ambient_ceiling
                ),
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
/// - If `order_weight_class` is set:
///   - source must be Player
///   - class must exist
///   - lifecycle must be Transient with ≥1 dissolution condition
///   - exactly one sanctioned transform: `Add(class.magnitude)` on a weight/need locus
///   - no unrelated extra transforms may satisfy the class by one matching value
/// - If a Player overlay carries a magnitude that reaches any admitted class
///   magnitude without citing a class → class-less dominant weight error.
/// - No magic dominance floor when the class table is empty (only non-finite reject).
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
                reason: format!(
                    "non-finite transform magnitude {value}; target locus `{}`",
                    overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            });
        }
    }

    if let Some(class_id) = &overlay.order_weight_class {
        let class = classes.iter().find(|c| c.id == *class_id).ok_or_else(|| {
            SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "unknown order_weight_class `{class_id}`; target locus `{}`",
                    overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            }
        })?;
        if overlay.source != OverlaySource::Player {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "order-weight directive source must be Player (class `{}`, locus `{}`)",
                    class.id, overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            });
        }
        if !is_transient_with_dissolve(&overlay.lifecycle) {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "order-weight directive must be Transient with declarative dissolve (class `{}`, locus `{}`)",
                    class.id, overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            });
        }
        if !is_weight_or_need_locus(&overlay.targets_property, &overlay.sub_field_deltas) {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "order-weight directive must target admitted need/weight locus (class `{}`, got `{}`)",
                    class.id, overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            });
        }
        // Exactly one transform, class-derived Add magnitude — no extra ops.
        if overlay.sub_field_deltas.len() != 1 {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: overlay.id.clone(),
                reason: format!(
                    "order-weight directive must carry exactly one class-derived transform (class `{}` magnitude {}, got {} deltas on locus `{}`)",
                    class.id,
                    class.magnitude,
                    overlay.sub_field_deltas.len(),
                    overlay.targets_property
                ),
                source_span_token: overlay.source_span_token,
            });
        }
        let (role, op) = &overlay.sub_field_deltas[0];
        match op {
            TransformOp::Add(v)
                if v.is_finite() && (*v - class.magnitude).abs() <= f32::EPSILON => {}
            other => {
                return Err(SpecError::OrderWeightDirectiveInvalid {
                    overlay_id: overlay.id.clone(),
                    reason: format!(
                        "directive transform must be Add({}) for class `{}` (got {:?} on role {:?}, locus `{}`)",
                        class.magnitude, class.id, other, role, overlay.targets_property
                    ),
                    source_span_token: overlay.source_span_token,
                });
            }
        }
        return Ok(());
    }

    // Class-less dominant weight: Player overlay whose magnitude reaches an
    // admitted class magnitude without citing a class. No magic floor when
    // the table is empty.
    if overlay.source == OverlaySource::Player && !classes.is_empty() {
        let dominance_floor = classes
            .iter()
            .map(|c| c.magnitude)
            .fold(f32::INFINITY, f32::min);
        for (_, op) in &overlay.sub_field_deltas {
            let v = op_magnitude(op).abs();
            if v.is_finite() && v >= dominance_floor {
                return Err(SpecError::OrderWeightDirectiveInvalid {
                    overlay_id: overlay.id.clone(),
                    reason: format!(
                        "class-less dominant weight {v} on locus `{}` (class magnitude floor {dominance_floor}); cite order_weight_class",
                        overlay.targets_property
                    ),
                    source_span_token: overlay.source_span_token,
                });
            }
        }
    }

    Ok(())
}

/// Runtime gate for a hand-built core `Overlay` (player-intent path).
///
/// Rejects dominant Player magnitudes that bypass the class table. Class-bound
/// construction must go through the typed directive surface that resolves a
/// class id into the sanctioned transform.
pub fn validate_runtime_player_overlay_magnitude(
    source: OverlaySource,
    transform_magnitudes: &[f32],
    classes: &[OrderWeightClassSpec],
) -> Result<(), SpecError> {
    if source != OverlaySource::Player || classes.is_empty() {
        return Ok(());
    }
    let dominance_floor = classes
        .iter()
        .map(|c| c.magnitude)
        .fold(f32::INFINITY, f32::min);
    for &v in transform_magnitudes {
        if !v.is_finite() {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: "<runtime-player-intent>".into(),
                reason: format!("non-finite runtime Player magnitude {v}"),
                source_span_token: None,
            });
        }
        if v.abs() >= dominance_floor {
            return Err(SpecError::OrderWeightDirectiveInvalid {
                overlay_id: "<runtime-player-intent>".into(),
                reason: format!(
                    "class-less dominant runtime Player weight {v} (class magnitude floor {dominance_floor}); use submit_order_directive"
                ),
                source_span_token: None,
            });
        }
    }
    Ok(())
}

fn is_transient_with_dissolve(lifecycle: &OverlayLifecycle) -> bool {
    match lifecycle {
        OverlayLifecycle::Transient {
            dissolution_conditions,
        } => !dissolution_conditions.is_empty(),
        _ => false,
    }
}

fn is_weight_or_need_locus(
    targets_property: &str,
    deltas: &[(SubFieldRole, TransformOp)],
) -> bool {
    let prop = targets_property.to_ascii_lowercase();
    let prop_ok = prop.contains("weight") || prop.contains("need") || prop.contains("flow");
    let role_ok = deltas.iter().any(|(role, _)| match role {
        SubFieldRole::Amount => true,
        SubFieldRole::Named(name) => {
            let n = name.to_ascii_lowercase();
            n.contains("weight") || n.contains("need") || n == "amount"
        }
        _ => false,
    });
    prop_ok && role_ok
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
    use simthing_core::{DissolveCondition, OverlayKind, OverlayLifecycle, SubFieldRole};

    fn class(id: &str, magnitude: f32, ambient_ceiling: f32) -> OrderWeightClassSpec {
        OrderWeightClassSpec {
            id: id.into(),
            magnitude,
            ambient_ceiling,
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
            targets_property: "order::food_flow".into(),
            sub_field_deltas: vec![(
                SubFieldRole::Named("weight".into()),
                TransformOp::Add(magnitude),
            )],
            lifecycle: OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
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
            ("non_dominating_class", false),
            ("non_finite_overlay", false),
            ("class_less_dominant", false),
            ("unknown_class", false),
            ("wrong_source", false),
            ("magnitude_mismatch", false),
            ("extra_transform", false),
            ("permanent_lifecycle", false),
            ("ambient_player_ok", true),
            ("empty_table_no_magic_floor", true),
        ];
        for (label, expect_ok) in CASES {
            let result = match *label {
                "class_ok" => {
                    let classes = vec![class("destination_order", 20.0, 2.0)];
                    validate_order_weight_classes(&classes).and_then(|_| {
                        validate_order_weight_overlay(
                            &overlay(
                                "go",
                                OverlaySource::Player,
                                Some("destination_order"),
                                20.0,
                            ),
                            &classes,
                        )
                    })
                }
                "non_finite_class" => {
                    validate_order_weight_classes(&[class("bad", f32::INFINITY, 1.0)])
                }
                "non_dominating_class" => {
                    validate_order_weight_classes(&[class("bad", 2.0, 2.0)])
                }
                "non_finite_overlay" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, None, f32::NAN),
                    &[],
                ),
                "class_less_dominant" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, None, 20.0),
                    &[class("destination_order", 20.0, 2.0)],
                ),
                "unknown_class" => validate_order_weight_overlay(
                    &overlay("go", OverlaySource::Player, Some("missing"), 20.0),
                    &[class("destination_order", 20.0, 2.0)],
                ),
                "wrong_source" => validate_order_weight_overlay(
                    &overlay(
                        "go",
                        OverlaySource::Ai,
                        Some("destination_order"),
                        20.0,
                    ),
                    &[class("destination_order", 20.0, 2.0)],
                ),
                "magnitude_mismatch" => validate_order_weight_overlay(
                    &overlay(
                        "go",
                        OverlaySource::Player,
                        Some("destination_order"),
                        50.0,
                    ),
                    &[class("destination_order", 20.0, 2.0)],
                ),
                "extra_transform" => {
                    let classes = vec![class("destination_order", 20.0, 2.0)];
                    let mut o = overlay(
                        "go",
                        OverlaySource::Player,
                        Some("destination_order"),
                        20.0,
                    );
                    o.sub_field_deltas
                        .push((SubFieldRole::Amount, TransformOp::Add(1.0)));
                    validate_order_weight_overlay(&o, &classes)
                }
                "permanent_lifecycle" => {
                    let classes = vec![class("destination_order", 20.0, 2.0)];
                    let mut o = overlay(
                        "go",
                        OverlaySource::Player,
                        Some("destination_order"),
                        20.0,
                    );
                    o.lifecycle = OverlayLifecycle::Permanent;
                    validate_order_weight_overlay(&o, &classes)
                }
                "ambient_player_ok" => validate_order_weight_overlay(
                    &overlay("nudge", OverlaySource::Player, None, 1.5),
                    &[class("destination_order", 20.0, 2.0)],
                ),
                "empty_table_no_magic_floor" => validate_order_weight_overlay(
                    &overlay("big", OverlaySource::Player, None, 10000.0),
                    &[],
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
