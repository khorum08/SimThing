//! E-8R — deterministic arena-internal plumbing columns for arena-bound properties.
//!
//! Unmarked columns are auto-derived at compile/register time. They carry no
//! `accumulator_spec` metadata and do not add `AccumulatorRole` variants.

use crate::accumulator_spec::AccumulatorRole;
use crate::property::{ClampBehavior, PropertyLayout, SubFieldRole, SubFieldSpec};

/// Arena-internal column roles appended in deterministic order (E-8R).
pub const ARENA_INTERNAL_COLUMN_ROLES: &[&str] = &[
    "intrinsic_flow_sum",
    "weight_sum",
    "propagated_intrinsic_flow",
    "propagated_allocated_flow",
    "propagated_weight_sum",
    "hosted_simthing_id",
];

/// RF-5A staged projection cells on the participant flow property (role pathway).
///
/// Per-tick Identity AccumulatorOps copy authored source cells here before
/// slot-local EvalEML. Not a second ledger — refreshed every RF tick on-device.
pub const NEED_STAGE_MAX_PAIRS: usize = 4;

/// Named roles `need_stage_in_{i}` / `need_stage_w_{i}` for i in 0..NEED_STAGE_MAX_PAIRS.
pub fn need_stage_role_names() -> impl Iterator<Item = String> {
    (0..NEED_STAGE_MAX_PAIRS).flat_map(|i| {
        [
            format!("need_stage_in_{i}"),
            format!("need_stage_w_{i}"),
        ]
    })
}

fn named(role: &str) -> SubFieldRole {
    SubFieldRole::Named(role.into())
}

fn internal_plumbing_subfield(role_name: &str) -> SubFieldSpec {
    SubFieldSpec {
        role: named(role_name),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: role_name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

/// True when any sub-field carries resource-flow arena participation metadata.
pub fn property_needs_arena_internal_columns(layout: &PropertyLayout) -> bool {
    layout.sub_fields.iter().any(|sf| {
        sf.accumulator_spec
            .as_ref()
            .is_some_and(|spec| role_is_arena_bound(&spec.role))
    })
}

fn role_is_arena_bound(role: &AccumulatorRole) -> bool {
    matches!(
        role,
        AccumulatorRole::IntrinsicFlow
            | AccumulatorRole::AllocatedFlow { .. }
            | AccumulatorRole::Balance(_)
            | AccumulatorRole::AllocatorWeight { .. }
    )
}

/// Append missing arena-internal plumbing columns in canonical order.
pub fn expand_arena_internal_columns(layout: PropertyLayout) -> PropertyLayout {
    if !property_needs_arena_internal_columns(&layout) {
        return layout;
    }
    let mut out = layout;
    for role_name in ARENA_INTERNAL_COLUMN_ROLES {
        let role = named(role_name);
        if out.offset_of(&role).is_some() {
            continue;
        }
        out.sub_fields.push(internal_plumbing_subfield(role_name));
    }
    // RF-5A: participant-local staged input/weight range for cross-row projection.
    for role_name in need_stage_role_names() {
        let role = named(&role_name);
        if out.offset_of(&role).is_some() {
            continue;
        }
        out.sub_fields
            .push(internal_plumbing_subfield(&role_name));
    }
    out
}

/// Roles added by [`expand_arena_internal_columns`] (for expansion reports).
pub fn arena_internal_columns_present(layout: &PropertyLayout) -> Vec<String> {
    ARENA_INTERNAL_COLUMN_ROLES
        .iter()
        .filter(|name| layout.offset_of(&named(name)).is_some())
        .map(|s| (*s).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accumulator_spec::{AccumulatorSpec, LogTier};

    fn flow_subfield(role_name: &str, spec: AccumulatorSpec) -> SubFieldSpec {
        SubFieldSpec {
            role: named(role_name),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: role_name.into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: Some(spec),
        }
    }

    fn arena_bound_layout() -> PropertyLayout {
        PropertyLayout {
            sub_fields: vec![
                flow_subfield(
                    "flow",
                    AccumulatorSpec {
                        role: AccumulatorRole::IntrinsicFlow,
                        log_tier: LogTier::Summary,
                    },
                ),
                flow_subfield(
                    "allocated",
                    AccumulatorSpec {
                        role: AccumulatorRole::AllocatedFlow {
                            arena: "food".into(),
                        },
                        log_tier: LogTier::Summary,
                    },
                ),
            ],
        }
    }

    #[test]
    fn e8r_layout_expansion_is_deterministic() {
        let a = expand_arena_internal_columns(arena_bound_layout());
        let b = expand_arena_internal_columns(arena_bound_layout());
        assert_eq!(a, b);
        assert_eq!(
            arena_internal_columns_present(&a),
            ARENA_INTERNAL_COLUMN_ROLES
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }

}
