use crate::diagnostics::{SpecDiagnostics, SpecResult, SpecWarning};
use crate::error::CapabilityTreeError;
use crate::keys::{
    CapabilityEffectKey, CapabilityEntryKey, CapabilityTreeDefinitionId, CategoryKey,
};
use crate::runtime::{
    CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition, CategoryDefinition,
};
use crate::spec::capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec,
    CapabilityTreeSpec, MaxActivePolicy,
};
use simthing_core::{
    ClampBehavior, DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource,     PropertyLayout, PropertyTransformDelta, ReductionRule,
    SimProperty, SimPropertyId, SimThing, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_feeder::CapabilityUnlockRegistration;
use std::collections::{HashMap, HashSet};

const PROGRESS_EPSILON: f32 = 0.001;

pub struct CapabilityTreeBuildOutput {
    pub tree:                   SimThing,
    pub definition:             CapabilityTreeDefinition,
    pub unlock_registrations:   Vec<CapabilityUnlockRegistration>,
}

pub struct CapabilityTreeBuilder;

impl CapabilityTreeBuilder {
    pub fn build(
        spec: &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
    ) -> SpecResult<CapabilityTreeBuildOutput> {
        validate_spec(spec)?;
        let mut diagnostics = SpecDiagnostics::default();

        let definition_id = CapabilityTreeDefinitionId::new(0);
        let mut tree = SimThing::new(SimThingKind::Custom(spec.tree_kind.clone()), 0);
        let mut categories = HashMap::new();
        let mut entries = HashMap::new();
        let mut by_threshold = HashMap::new();
        let mut by_overlay = HashMap::new();
        let mut unlock_registrations = Vec::new();

        for category_spec in &spec.categories {
            let category_key = CategoryKey::from_category_spec(category_spec);
            let max_active = match MaxActivePolicy::from_option(category_spec.max_active) {
                Ok(p) => p,
                Err(CapabilityTreeError::InvalidMaxActive(_, _)) => {
                    return Err(CapabilityTreeError::InvalidMaxActive(
                        category_spec.max_active.unwrap_or(0),
                        category_key.clone(),
                    ));
                }
                Err(e) => return Err(e),
            };

            if matches!(max_active, MaxActivePolicy::Limited { .. })
                && !category_spec
                    .entries
                    .iter()
                    .any(|e| e.activation == ActivationMode::PlayerSelection)
            {
                diagnostics.push(SpecWarning::MaxActiveWithoutPlayerSelection {
                    tree_id:      spec.tree_id.clone(),
                    category_key: category_key.to_string(),
                });
            }

            let property_id = register_category_property(registry, category_spec)?;
            let property_value = registry.property(property_id).default_value();
            tree.add_property(property_id, property_value);

            categories.insert(
                category_key.clone(),
                CategoryDefinition {
                    key:          category_key.clone(),
                    display_name: category_spec.display_name.clone(),
                    max_active,
                    property_id,
                },
            );

            for entry_spec in &category_spec.entries {
                let entry_key = CapabilityEntryKey::new(category_key.clone(), &entry_spec.id);
                Self::validate_entry(spec, &entry_key, entry_spec)?;

                if entry_spec.effects.is_empty() {
                    diagnostics.push(SpecWarning::EmptyEffects {
                        tree_id:  spec.tree_id.clone(),
                        entry_id: entry_spec.id.clone(),
                    });
                }

                let layout = &registry.property(property_id).layout;
                let progress_role = SubFieldRole::Named(entry_spec.id.clone());
                let progress_col = registry
                    .column_range(property_id)
                    .col_for_role(&progress_role, layout)
                    .ok_or_else(|| CapabilityTreeError::UnknownEffectSubField(
                        progress_role.clone(),
                        category_key.to_string(),
                    ))?;

                let rate_role = SubFieldRole::Named(format!("{}_rate", entry_spec.id));
                let rate_col = registry
                    .column_range(property_id)
                    .col_for_role(&rate_role, layout);

                let mut overlay_ids = Vec::new();
                let mut overlay_transforms = Vec::new();
                let mut effect_keys = Vec::new();
                for (effect_index, effect_spec) in entry_spec.effects.iter().enumerate() {
                    validate_effect(&entry_key, effect_index, effect_spec)?;
                    validate_effect_targets(registry, &entry_key, effect_index, effect_spec)?;

                    let overlay_id = OverlayId::new();
                    let overlay = build_suspended_overlay(overlay_id, effect_spec, registry)?;
                    overlay_transforms.push(overlay.transform.clone());
                    tree.add_overlay(overlay);
                    overlay_ids.push(overlay_id);
                    effect_keys.push(CapabilityEffectKey {
                        entry:        entry_key.clone(),
                        effect_index,
                    });
                    by_overlay.insert(overlay_id, entry_key.clone());
                }

                if entry_spec.activation == ActivationMode::Threshold {
                    if entry_spec.research_cost <= 0.0 {
                        return Err(CapabilityTreeError::ThresholdRequiresPositiveCost(
                            entry_key.clone(),
                        ));
                    }
                    unlock_registrations.push(CapabilityUnlockRegistration {
                        sim_thing_id: tree.id,
                        property_id,
                        sub_field:    progress_role.clone(),
                        threshold:    entry_spec.research_cost,
                    });
                    by_threshold.insert((property_id, progress_role.clone()), entry_key.clone());
                }

                entries.insert(
                    entry_key.clone(),
                    CapabilityDefinition {
                        key:                  entry_key,
                        display_name:         entry_spec.display_name.clone(),
                        research_cost:        entry_spec.research_cost,
                        default_activation:   entry_spec.activation,
                        progress_property_id: property_id,
                        progress_role,
                        progress_col,
                        rate_col,
                        overlay_ids,
                        overlay_transforms,
                        effect_keys,
                        prereqs:              Vec::new(),
                        category_key:         category_key.clone(),
                    },
                );
            }
        }

        resolve_prereqs(spec, registry, &mut entries)?;

        let definition = CapabilityTreeDefinition {
            id: definition_id,
            tree_id: spec.tree_id.clone(),
            tree_kind: spec.tree_kind.clone(),
            owner_kind: spec.owner_kind.clone(),
            categories,
            entries,
            by_threshold,
            by_overlay,
        };

        Ok((
            CapabilityTreeBuildOutput {
                tree,
                definition,
                unlock_registrations,
            },
            diagnostics,
        ))
    }
}

fn validate_spec(spec: &CapabilityTreeSpec) -> Result<(), CapabilityTreeError> {
    if spec.owner_kind != "Faction" {
        return Err(CapabilityTreeError::UnknownOwnerKind(spec.owner_kind.clone()));
    }
    let mut seen_categories = HashSet::new();
    let mut seen_entries = HashSet::new();
    for category in &spec.categories {
        let category_key = CategoryKey::from_category_spec(category);
        if !seen_categories.insert(category_key.clone()) {
            return Err(CapabilityTreeError::DuplicateCategory(
                category_key,
                spec.tree_id.clone(),
            ));
        }
        for entry in &category.entries {
            let entry_key = CapabilityEntryKey::new(category_key.clone(), &entry.id);
            if !seen_entries.insert(entry_key.clone()) {
                return Err(CapabilityTreeError::DuplicateEntry(
                    entry_key,
                    spec.tree_id.clone(),
                ));
            }
        }
    }
    Ok(())
}

impl CapabilityTreeBuilder {
    fn validate_entry(
        _spec: &CapabilityTreeSpec,
        entry_key: &CapabilityEntryKey,
        entry: &CapabilitySpec,
    ) -> Result<(), CapabilityTreeError> {
        if entry.research_cost < 0.0 {
            return Err(CapabilityTreeError::NegativeResearchCost(entry_key.clone()));
        }
        Ok(())
    }
}

fn validate_effect(
    entry_key: &CapabilityEntryKey,
    effect_index: usize,
    effect: &CapabilityEffectSpec,
) -> Result<(), CapabilityTreeError> {
    if matches!(effect.when_activated, OverlayLifecycle::Suspended { .. }) {
        return Err(CapabilityTreeError::InvalidLifecycle(
            entry_key.clone(),
            effect_index,
        ));
    }
    if effect.sub_field_deltas.is_empty() {
        return Err(CapabilityTreeError::InvalidLifecycle(
            entry_key.clone(),
            effect_index,
        ));
    }
    Ok(())
}

fn validate_effect_targets(
    registry: &DimensionRegistry,
    entry_key: &CapabilityEntryKey,
    effect_index: usize,
    effect: &CapabilityEffectSpec,
) -> Result<(), CapabilityTreeError> {
    let (namespace, name) = parse_property_key(&effect.targets_property)?;
    let property_id = registry
        .id_of(&namespace, &name)
        .ok_or_else(|| CapabilityTreeError::UnknownEffectProperty(effect.targets_property.clone()))?;
    let layout = &registry.property(property_id).layout;
    for (role, _) in &effect.sub_field_deltas {
        if layout.offset_of(role).is_none() {
            return Err(CapabilityTreeError::UnknownEffectSubField(
                role.clone(),
                effect.targets_property.clone(),
            ));
        }
    }
    let _ = (entry_key, effect_index);
    Ok(())
}

fn register_category_property(
    registry: &mut DimensionRegistry,
    category: &CapabilityCategorySpec,
) -> Result<SimPropertyId, CapabilityTreeError> {
    let mut sub_fields = Vec::new();
    for entry in &category.entries {
        let rate_role = SubFieldRole::Named(format!("{}_rate", entry.id));
        let progress_role = SubFieldRole::Named(entry.id.clone());
        sub_fields.push(capability_subfield(
            rate_role.clone(),
            format!("{} rate", entry.display_name),
            None,
        ));
        sub_fields.push(capability_subfield(
            progress_role,
            entry.display_name.clone(),
            Some(rate_role),
        ));
    }

    let prop = SimProperty {
        namespace:          category.property_namespace.clone(),
        name:               category.property_name.clone(),
        layout:             PropertyLayout { sub_fields },
        decay:              None,
        intensity_behavior: None,
        fission_templates:  vec![],
        fusion_templates:   vec![],
        on_expire:          None,
        description:        category.display_name.clone(),
        intensity_labels:   vec![],
    };

    Ok(registry.register(prop))
}

fn capability_subfield(
    role: SubFieldRole,
    display_name: String,
    governed_by: Option<SubFieldRole>,
) -> SubFieldSpec {
    SubFieldSpec {
        role,
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name,
        display_range: None,
        governed_by,
        reduction_override: Some(ReductionRule::Max),
    }
}

fn build_suspended_overlay(
    overlay_id: OverlayId,
    effect: &CapabilityEffectSpec,
    registry: &DimensionRegistry,
) -> Result<Overlay, CapabilityTreeError> {
    let (namespace, name) = parse_property_key(&effect.targets_property)?;
    let property_id = registry
        .id_of(&namespace, &name)
        .ok_or_else(|| CapabilityTreeError::UnknownEffectProperty(effect.targets_property.clone()))?;

    Ok(Overlay {
        id: overlay_id,
        kind: OverlayKind::Custom("capability_effect".into()),
        source: OverlaySource::System,
        affects: vec![],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: effect.sub_field_deltas.clone(),
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(effect.when_activated.clone()),
        },
    })
}

fn resolve_prereqs(
    spec: &CapabilityTreeSpec,
    registry: &DimensionRegistry,
    entries: &mut HashMap<CapabilityEntryKey, CapabilityDefinition>,
) -> Result<(), CapabilityTreeError> {
    for category in &spec.categories {
        let category_key = CategoryKey::from_category_spec(category);
        for entry_spec in &category.entries {
            let entry_key = CapabilityEntryKey::new(category_key.clone(), &entry_spec.id);
            let mut resolved = Vec::new();
            for prereq in &entry_spec.prereqs {
                if prereq.category == entry_spec.id && prereq.entry_id == entry_spec.id {
                    return Err(CapabilityTreeError::SelfPrereq(entry_key.clone()));
                }
                let prereq_category = find_category(spec, &prereq.category)?;
                let prereq_category_key = CategoryKey::from_category_spec(prereq_category);
                let prereq_entry_key =
                    CapabilityEntryKey::new(prereq_category_key.clone(), &prereq.entry_id);
                if !entries.contains_key(&prereq_entry_key) {
                    return Err(CapabilityTreeError::UnknownPrereqEntry(
                        prereq.entry_id.clone(),
                        prereq.category.clone(),
                    ));
                }
                let prereq_def = entries.get(&prereq_entry_key).unwrap();
                let progress_role = SubFieldRole::Named(prereq.entry_id.clone());
                let layout = &registry.property(prereq_def.progress_property_id).layout;
                let col = registry
                    .column_range(prereq_def.progress_property_id)
                    .col_for_role(&progress_role, layout)
                    .expect("prereq entry progress col");
                resolved.push(CapabilityPrereq {
                    property_id: prereq_def.progress_property_id,
                    role:        progress_role,
                    col,
                    min_value:   prereq_def.research_cost,
                });
            }
            entries.get_mut(&entry_key).unwrap().prereqs = resolved;
        }
    }
    Ok(())
}

fn find_category<'a>(
    spec: &'a CapabilityTreeSpec,
    category_ref: &str,
) -> Result<&'a CapabilityCategorySpec, CapabilityTreeError> {
    spec.categories
        .iter()
        .find(|c| c.property_name == category_ref || format!("{}::{}", c.property_namespace, c.property_name) == category_ref)
        .ok_or_else(|| CapabilityTreeError::UnknownPrereqCategory(
            category_ref.to_owned(),
            CapabilityEntryKey::new(CategoryKey::new("", ""), ""),
        ))
}

pub fn parse_property_key(key: &str) -> Result<(String, String), CapabilityTreeError> {
    key.split_once("::")
        .map(|(ns, name)| (ns.to_owned(), name.to_owned()))
        .ok_or_else(|| CapabilityTreeError::InvalidPropertyKey(key.to_owned()))
}

/// Patch empty overlay `affects` vecs to target the owning faction after attach.
pub fn set_overlay_affects(tree: &mut SimThing, owner_id: simthing_core::SimThingId) {
    for overlay in &mut tree.overlays {
        if overlay.affects.is_empty() {
            overlay.affects = vec![owner_id];
        }
    }
}

pub fn progress_reset_value(research_cost: f32) -> f32 {
    (research_cost - PROGRESS_EPSILON).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::capability::{CapabilityEffectSpec, ResearchRateSpec};
    use simthing_core::TransformOp;

    fn minimal_spec() -> CapabilityTreeSpec {
        CapabilityTreeSpec {
            tree_id:    "test_tree".into(),
            tree_kind:  "tech_tree".into(),
            owner_kind: "Faction".into(),
            categories: vec![CapabilityCategorySpec {
                property_namespace: "tech".into(),
                property_name:      "propulsion".into(),
                display_name:       "Propulsion".into(),
                tier:               0,
                max_active:         None,
                entries:            vec![CapabilitySpec {
                    id:            "ion_drive".into(),
                    display_name:  "Ion Drive".into(),
                    description:   String::new(),
                    flavor_text:   String::new(),
                    research_cost: 100.0,
                    activation:    ActivationMode::Threshold,
                    research_rate: ResearchRateSpec::Literal(1.0),
                    icon:          String::new(),
                    thumbnail:     String::new(),
                    card_image:    String::new(),
                    unlock_video:  None,
                    model_preview: None,
                    prereqs:       vec![],
                    unlocks_ship_components: vec![],
                    unlocks_buildings:       vec![],
                    unlocks_units:           vec![],
                    unlocks_weapons:         vec![],
                    effects: vec![CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.3))],
                        when_activated:   OverlayLifecycle::Permanent,
                    }],
                }],
            }],
        }
    }

    #[test]
    fn builder_registers_max_reduction_and_unlock_threshold() {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("military", "fleet_speed", 0));
        let spec = minimal_spec();

        let (output, _diag) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();

        assert_eq!(output.tree.kind, SimThingKind::Custom("tech_tree".into()));
        assert_eq!(output.tree.overlays.len(), 1);
        assert!(matches!(
            output.tree.overlays[0].lifecycle,
            OverlayLifecycle::Suspended { .. }
        ));
        assert_eq!(output.unlock_registrations.len(), 1);
        assert_eq!(output.unlock_registrations[0].threshold, 100.0);

        let prop = registry.property(output.definition.categories[&CategoryKey::new("tech", "propulsion")].property_id);
        for sf in &prop.layout.sub_fields {
            assert_eq!(sf.resolved_reduction(), ReductionRule::Max);
        }
    }

    #[test]
    fn player_selection_skips_threshold_registration() {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("military", "fleet_speed", 0));
        let mut spec = minimal_spec();
        spec.categories[0].entries[0].activation = ActivationMode::PlayerSelection;
        spec.categories[0].entries[0].research_cost = 0.0;

        let (output, _) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();
        assert!(output.unlock_registrations.is_empty());
    }
}
