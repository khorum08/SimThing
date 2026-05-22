//! `CapabilityTreeBuilder` — compiles a `CapabilityTreeSpec` into a live
//! capability tree `SimThing`, a shared `CapabilityTreeDefinition`, and the
//! `CapabilityUnlockRegistration`s that PR 4 will hand to the feeder.

use crate::diagnostics::SpecResult;
use crate::error::SpecError;
use crate::keys::{CapabilityEffectKey, CapabilityEntryKey, CategoryKey};
use crate::runtime::{
    CapabilityCategoryDefinition, CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityUnlockRegistration,
};
use crate::spec::capability::{ActivationMode, CapabilityTreeSpec};
use crate::validate::validate_capability_tree;
use simthing_core::{
    ClampBehavior, DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyLayout, PropertyTransformDelta, ReductionRule, SimProperty, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use std::collections::HashMap;

/// Aggregate of artifacts produced by `CapabilityTreeBuilder::build`.
#[derive(Debug)]
pub struct CapabilityTreeBuildOutput {
    /// Authored template tree. Cloned per faction instance at session-init time;
    /// the session coordinator is responsible for stamping faction-specific
    /// ownership / `affects` on the overlays at activation.
    pub tree: SimThing,
    pub definition: CapabilityTreeDefinition,
    /// One per `ActivationMode::Threshold` entry. `PlayerSelection` and
    /// `OnPrereqMet` entries produce no GPU registration.
    pub unlock_registrations: Vec<CapabilityUnlockRegistration>,
}

pub struct CapabilityTreeBuilder;

impl CapabilityTreeBuilder {
    /// Compile the spec end-to-end. Steps (per `simthing_spec_master_handoff.md` §PR3):
    ///
    /// 1. Always-on validation: duplicate ids, `OnPrereqMet` authored, etc.
    /// 2. Register one `SimProperty` per category — one sub-field per entry,
    ///    `SubFieldRole::Named(entry.id)`, `ReductionRule::Max` unconditionally.
    /// 3. Build the template tree `SimThing` (`SimThingKind::Custom(tree_kind)`),
    ///    seed all progress sub-fields to 0.0 by adding default `PropertyValue`s.
    /// 4. Compile each effect to a suspended `Overlay` attached to the tree.
    /// 5. Resolve every prereq (`property_id`, `role`, `col`, `min_value`).
    /// 6. Assemble `CapabilityTreeDefinition` with `by_threshold` + `by_overlay`.
    /// 7. Emit one `CapabilityUnlockRegistration` per Threshold entry.
    pub fn build(
        spec: &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
    ) -> SpecResult<CapabilityTreeBuildOutput> {
        // ── 1. Validate ───────────────────────────────────────────────────────
        let diagnostics = validate_capability_tree(spec)?;

        // ── 2. Register one SimProperty per category ──────────────────────────
        let mut category_prop: HashMap<CategoryKey, simthing_core::SimPropertyId> = HashMap::new();
        let mut categories: HashMap<CategoryKey, CapabilityCategoryDefinition> = HashMap::new();
        for cat in &spec.categories {
            let cat_key = CategoryKey::new(&cat.property_namespace, &cat.property_name);

            if registry
                .id_of(&cat.property_namespace, &cat.property_name)
                .is_some()
            {
                return Err(SpecError::DuplicateProperty {
                    namespace: cat.property_namespace.clone(),
                    name: cat.property_name.clone(),
                });
            }

            // One sub-field per entry. ReductionRule::Max forced.
            let sub_fields: Vec<SubFieldSpec> = cat
                .entries
                .iter()
                .map(|e| SubFieldSpec {
                    role: SubFieldRole::Named(e.id.clone()),
                    width: 1,
                    clamp: ClampBehavior::Floored { min: 0.0 },
                    velocity_max: None,
                    default: 0.0,
                    display_name: e.display_name.clone(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: Some(ReductionRule::Max),
                })
                .collect();

            let prop = SimProperty {
                namespace: cat.property_namespace.clone(),
                name: cat.property_name.clone(),
                layout: PropertyLayout { sub_fields },
                decay: None,
                intensity_behavior: None,
                fission_templates: vec![],
                fusion_templates: vec![],
                on_expire: None,
                description: cat.display_name.clone(),
                intensity_labels: vec![],
            };
            let prop_id = registry.register(prop);
            categories.insert(
                cat_key.clone(),
                CapabilityCategoryDefinition {
                    key: cat_key.clone(),
                    property_id: prop_id,
                    max_active: cat.max_active.clone(),
                    tier: cat.tier,
                },
            );
            category_prop.insert(cat_key, prop_id);
        }

        // ── 3. Construct the template SimThing ────────────────────────────────
        let mut tree = SimThing::new(SimThingKind::Custom(spec.tree_kind.clone()), 0);
        for (_cat_key, &prop_id) in &category_prop {
            // default_value() produces zeros because every sub-field default is 0.0.
            let value = registry.property(prop_id).default_value();
            tree.add_property(prop_id, value);
        }
        let tree_id = tree.id;

        // ── 4 + 6. Compile effects → overlays; build entry definitions ────────
        let mut entries: HashMap<CapabilityEntryKey, CapabilityDefinition> = HashMap::new();
        let mut by_threshold: HashMap<
            (simthing_core::SimPropertyId, SubFieldRole),
            CapabilityEntryKey,
        > = HashMap::new();
        let mut by_overlay: HashMap<OverlayId, CapabilityEntryKey> = HashMap::new();
        let mut unlock_registrations: Vec<CapabilityUnlockRegistration> = Vec::new();

        // Pre-pass: build a lookup of (CategoryKey, entry_id) → research_cost for
        // prereq min_value resolution in the second pass.
        let mut entry_cost: HashMap<(CategoryKey, String), f32> = HashMap::new();
        for cat in &spec.categories {
            let cat_key = CategoryKey::new(&cat.property_namespace, &cat.property_name);
            for entry in &cat.entries {
                entry_cost.insert((cat_key.clone(), entry.id.clone()), entry.research_cost);
            }
        }

        for cat in &spec.categories {
            let cat_key = CategoryKey::new(&cat.property_namespace, &cat.property_name);
            let cat_prop_id = category_prop[&cat_key];

            for entry in &cat.entries {
                let entry_key = CapabilityEntryKey::new(cat_key.clone(), &entry.id);
                let entry_role = SubFieldRole::Named(entry.id.clone());
                let cat_layout = &registry.property(cat_prop_id).layout;
                let cat_range = registry.column_range(cat_prop_id);
                let progress_col = cat_range
                    .col_for_role(&entry_role, cat_layout)
                    .expect("builder-created capability role must resolve");

                // ── 4. Effects → suspended overlays ───────────────────────────
                let mut overlay_ids: Vec<OverlayId> = Vec::new();
                let mut effect_keys: Vec<CapabilityEffectKey> = Vec::new();

                for (effect_index, effect) in entry.effects.iter().enumerate() {
                    let (target_ns, target_name) =
                        parse_property_ref(&entry.id, effect_index, &effect.targets_property)?;

                    let target_prop_id =
                        registry.id_of(target_ns, target_name).ok_or_else(|| {
                            SpecError::InvalidEffectTarget {
                                entry_id: entry.id.clone(),
                                effect_index,
                                targets_property: effect.targets_property.clone(),
                                reason: "target property not registered".into(),
                            }
                        })?;

                    let target_layout = &registry.property(target_prop_id).layout;
                    for (role, _op) in &effect.sub_field_deltas {
                        if target_layout.offset_of(role).is_none() {
                            return Err(SpecError::InvalidEffectTarget {
                                entry_id: entry.id.clone(),
                                effect_index,
                                targets_property: effect.targets_property.clone(),
                                reason: format!(
                                    "sub-field role `{}` not present in target layout",
                                    format_role(role),
                                ),
                            });
                        }
                    }

                    let overlay_id = OverlayId::new();
                    let overlay = Overlay {
                        id: overlay_id,
                        kind: OverlayKind::Custom("capability".into()),
                        source: OverlaySource::System,
                        // affects is filled in by the session coordinator at
                        // activation time — per CapabilityTreeBoundaryHandler.
                        affects: vec![],
                        transform: PropertyTransformDelta {
                            property_id: target_prop_id,
                            sub_field_deltas: effect.sub_field_deltas.clone(),
                        },
                        // V6: every capability effect starts Suspended.
                        lifecycle: OverlayLifecycle::Suspended {
                            when_activated: Box::new(effect.when_activated.clone()),
                        },
                    };
                    tree.add_overlay(overlay);

                    overlay_ids.push(overlay_id);
                    effect_keys.push(CapabilityEffectKey {
                        entry: entry_key.clone(),
                        effect_index,
                    });
                    by_overlay.insert(overlay_id, entry_key.clone());
                }

                // ── 5. Resolve prereqs ────────────────────────────────────────
                let mut prereqs: Vec<CapabilityPrereq> = Vec::new();
                for pre in &entry.prereqs {
                    let (pre_ns, pre_name) =
                        parse_category_ref(&spec.tree_id, &entry.id, &pre.category)?;
                    let pre_cat_key = CategoryKey::new(pre_ns, pre_name);

                    let &pre_prop_id = category_prop.get(&pre_cat_key).ok_or_else(|| {
                        SpecError::UnknownPrereqCategory {
                            in_tree: spec.tree_id.clone(),
                            entry_id: entry.id.clone(),
                            category: pre.category.clone(),
                        }
                    })?;

                    let pre_role = SubFieldRole::Named(pre.entry_id.clone());
                    let pre_layout = &registry.property(pre_prop_id).layout;
                    let range = registry.column_range(pre_prop_id);
                    let col = range.col_for_role(&pre_role, pre_layout).ok_or_else(|| {
                        SpecError::UnknownPrereqEntry {
                            in_tree: spec.tree_id.clone(),
                            entry_id: entry.id.clone(),
                            category: pre.category.clone(),
                            prereq_entry_id: pre.entry_id.clone(),
                        }
                    })?;

                    let min_value = entry_cost
                        .get(&(pre_cat_key.clone(), pre.entry_id.clone()))
                        .copied()
                        .ok_or_else(|| SpecError::UnknownPrereqEntry {
                            in_tree: spec.tree_id.clone(),
                            entry_id: entry.id.clone(),
                            category: pre.category.clone(),
                            prereq_entry_id: pre.entry_id.clone(),
                        })?;

                    prereqs.push(CapabilityPrereq {
                        property_id: pre_prop_id,
                        role: pre_role,
                        col,
                        min_value,
                    });
                }

                // ── 7. Threshold unlock registration ──────────────────────────
                if entry.activation == ActivationMode::Threshold {
                    by_threshold.insert((cat_prop_id, entry_role.clone()), entry_key.clone());
                    unlock_registrations.push(CapabilityUnlockRegistration {
                        sim_thing_id: tree_id,
                        property_id: cat_prop_id,
                        sub_field: entry_role.clone(),
                        threshold: entry.research_cost,
                    });
                }

                let definition = CapabilityDefinition {
                    key: entry_key.clone(),
                    display_name: entry.display_name.clone(),
                    description: entry.description.clone(),
                    flavor_text: if entry.flavor_text.is_empty() {
                        None
                    } else {
                        Some(entry.flavor_text.clone())
                    },
                    activation: entry.activation,
                    overlay_ids,
                    effect_keys,
                    prereqs,
                    progress_col,
                    research_cost: entry.research_cost,
                };
                entries.insert(entry_key, definition);
            }
        }

        let definition = CapabilityTreeDefinition {
            id: CapabilityTreeDefinitionId::new(),
            tree_id: spec.tree_id.clone(),
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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_property_ref<'a>(
    entry_id: &str,
    effect_index: usize,
    refstr: &'a str,
) -> Result<(&'a str, &'a str), SpecError> {
    let mut parts = refstr.splitn(2, "::");
    let ns = parts.next().unwrap_or("");
    let name = parts.next();
    match name {
        Some(n) if !ns.is_empty() && !n.is_empty() => Ok((ns, n)),
        _ => Err(SpecError::InvalidEffectTarget {
            entry_id: entry_id.to_owned(),
            effect_index,
            targets_property: refstr.to_owned(),
            reason: "expected `namespace::name`".into(),
        }),
    }
}

fn parse_category_ref<'a>(
    in_tree: &str,
    entry_id: &str,
    refstr: &'a str,
) -> Result<(&'a str, &'a str), SpecError> {
    let mut parts = refstr.splitn(2, "::");
    let ns = parts.next().unwrap_or("");
    let name = parts.next();
    match name {
        Some(n) if !ns.is_empty() && !n.is_empty() => Ok((ns, n)),
        _ => Err(SpecError::UnknownPrereqCategory {
            in_tree: in_tree.to_owned(),
            entry_id: entry_id.to_owned(),
            category: refstr.to_owned(),
        }),
    }
}

fn format_role(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(n) => format!("Named({n})"),
        SubFieldRole::Custom(n) => format!("Custom({n})"),
    }
}
