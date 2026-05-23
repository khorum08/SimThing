use crate::diagnostics::{SpecDiagnostic, SpecDiagnostics};
use crate::error::SpecError;
use crate::spec::capability::{
    ActivationMode, CapabilityTreeSpec, MaxActivePolicy, ReplacementPolicy,
};

/// Lightweight authored-spec validation for PR 1 (no runtime compilation).
pub fn validate_capability_tree(spec: &CapabilityTreeSpec) -> Result<SpecDiagnostics, SpecError> {
    let mut diagnostics = SpecDiagnostics::default();
    let mut seen_categories = std::collections::HashSet::new();
    let mut seen_entries = std::collections::HashSet::new();

    for category in &spec.categories {
        let category_key = format!(
            "{}::{}",
            category.property_namespace, category.property_name
        );
        if !seen_categories.insert(category_key.clone()) {
            return Err(SpecError::DuplicateCategory(
                category_key,
                spec.tree_id.clone(),
            ));
        }

        // v0 max_active policy: Unlimited or Limited(1, SuspendOldest).
        if let Some(MaxActivePolicy::Limited { count, replacement }) = &category.max_active {
            if *count != 1 || *replacement != ReplacementPolicy::SuspendOldest {
                return Err(SpecError::UnsupportedMaxActive {
                    in_tree: spec.tree_id.clone(),
                    category: category_key.clone(),
                    count: *count,
                });
            }
        }

        for entry in &category.entries {
            let entry_key = format!("{category_key}::{}", entry.id);
            if !seen_entries.insert(entry_key.clone()) {
                return Err(SpecError::DuplicateEntry(
                    entry.id.clone(),
                    spec.tree_id.clone(),
                ));
            }

            if entry.activation == ActivationMode::OnPrereqMet {
                return Err(SpecError::OnPrereqMetAuthoredDefault(entry.id.clone()));
            }

            if entry.research_cost < 0.0 {
                return Err(SpecError::NegativeResearchCost(entry.id.clone()));
            }

            if entry.activation == ActivationMode::Threshold && entry.research_cost <= 0.0 {
                return Err(SpecError::ThresholdRequiresPositiveCost(entry.id.clone()));
            }

            // Self-referential prereq detection (single-entry cycle).
            for pre in &entry.prereqs {
                if pre.category == category_key && pre.entry_id == entry.id {
                    return Err(SpecError::SelfReferentialPrereq(entry.id.clone()));
                }
            }

            if entry.effects.is_empty() {
                diagnostics.push(SpecDiagnostic::warning(
                    "capability.empty_effects",
                    format!("entry `{}` has no effects", entry.id),
                ));
            }

            if category.max_active.is_some()
                && !category
                    .entries
                    .iter()
                    .any(|e| e.activation == ActivationMode::PlayerSelection)
            {
                diagnostics.push(SpecDiagnostic::warning(
                    "capability.max_active_without_player_selection",
                    format!(
                        "category `{category_key}` sets max_active but has no PlayerSelection entries"
                    ),
                ));
            }
        }
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::capability::{CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec};
    use simthing_core::{OverlayLifecycle, SubFieldRole, TransformOp};

    fn minimal_tree() -> CapabilityTreeSpec {
        CapabilityTreeSpec {
            tree_id: "test".into(),
            tree_kind: "tech_tree".into(),
            owner_kind: "Faction".into(),
            categories: vec![CapabilityCategorySpec {
                property_namespace: "tech".into(),
                property_name: "propulsion".into(),
                display_name: "Propulsion".into(),
                tier: 0,
                max_active: None,
                entries: vec![CapabilitySpec {
                    id: "drive".into(),
                    display_name: "Drive".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 100.0,
                    activation: ActivationMode::Threshold,
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: vec![],
                    unlocks_ship_components: vec![],
                    unlocks_buildings: vec![],
                    unlocks_units: vec![],
                    unlocks_weapons: vec![],
                    effects: vec![CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
                        when_activated: OverlayLifecycle::Permanent,
                    }],
                }],
            }],
        }
    }

    #[test]
    fn validate_accepts_minimal_tree() {
        let spec = minimal_tree();
        let diag = validate_capability_tree(&spec).unwrap();
        assert!(diag.diagnostics.is_empty());
    }

    #[test]
    fn validate_rejects_threshold_with_zero_cost() {
        let mut spec = minimal_tree();
        spec.categories[0].entries[0].research_cost = 0.0;
        assert!(matches!(
            validate_capability_tree(&spec),
            Err(SpecError::ThresholdRequiresPositiveCost(_))
        ));
    }
}
