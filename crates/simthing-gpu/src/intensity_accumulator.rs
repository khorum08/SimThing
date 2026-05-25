//! C-8b intensity migration → AccumulatorOp EvalEML planner.

use simthing_core::{
    compile_intensity_behavior_to_eml, intensity_tree_id, AccumulatorOp, CombineFn, ConsumeMode,
    DimensionRegistry, EmlConsumerKind, GateSpec, ScaleSpec, SimPropertyId, SourceSpec,
    SubFieldRole,
};

#[derive(Clone, Debug, PartialEq)]
pub struct IntensityEmlEntry {
    pub tree_id: simthing_core::EmlTreeId,
    pub velocity_col: u32,
    pub intensity_col: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntensityEmlPlan {
    pub ops: Vec<AccumulatorOp>,
    pub n_bands: u32,
    pub entries: Vec<IntensityEmlEntry>,
}

/// Walk active properties and collect intensity EML registration metadata.
pub fn build_intensity_eml_entries(registry: &DimensionRegistry) -> Vec<IntensityEmlEntry> {
    let mut entries = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) {
            continue;
        }
        if prop.intensity_behavior.is_none() {
            continue;
        }
        let range = registry.column_range(id);
        let layout = &prop.layout;
        let Some(velocity_col) = range.col_for_role(&SubFieldRole::Velocity, layout) else {
            continue;
        };
        let Some(intensity_col) = range.col_for_role(&SubFieldRole::Intensity, layout) else {
            continue;
        };
        entries.push(IntensityEmlEntry {
            tree_id: intensity_tree_id(idx as u32),
            velocity_col: velocity_col as u32,
            intensity_col: intensity_col as u32,
        });
    }
    entries
}

/// One EvalEML AccumulatorOp per `(slot, intensity entry)` — same topology as legacy Pass 2.
pub fn plan_intensity_eml_ops(entries: &[IntensityEmlEntry], n_slots: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(entries.len() * n_slots as usize);
    for slot in 0..n_slots {
        for entry in entries {
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot,
                    col: entry.intensity_col,
                },
                combine: CombineFn::EvalEML {
                    tree_id: entry.tree_id.0,
                },
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(slot, entry.intensity_col)],
            });
        }
    }
    ops
}

/// Register intensity EML formulas into the given registry (replacing prior intensity trees).
pub fn register_intensity_eml_formulas(
    registry: &mut simthing_core::EmlExpressionRegistry,
    dimension_registry: &DimensionRegistry,
    previous_tree_ids: &[simthing_core::EmlTreeId],
) -> Result<Vec<IntensityEmlEntry>, simthing_core::EmlRegistryError> {
    let entries = build_intensity_eml_entries(dimension_registry);
    let new_ids: std::collections::HashSet<_> = entries.iter().map(|e| e.tree_id).collect();
    for tree_id in previous_tree_ids {
        if !new_ids.contains(tree_id) {
            registry.remove_tree(*tree_id);
        }
    }
    for (idx, prop) in dimension_registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !dimension_registry.is_active(id) {
            continue;
        }
        let Some(behavior) = &prop.intensity_behavior else {
            continue;
        };
        let entry = entries.iter().find(|e| e.tree_id == intensity_tree_id(idx as u32));
        let Some(entry) = entry else {
            continue;
        };
        let (meta, nodes) = compile_intensity_behavior_to_eml(
            behavior,
            entry.tree_id,
            entry.velocity_col,
            entry.intensity_col,
        );
        registry.replace_formula_if_changed(entry.tree_id, meta, nodes)?;
        registry.assert_consumer_admissible(entry.tree_id, EmlConsumerKind::Intensity)?;
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{IntensityBehavior, SimProperty};

    fn intensity_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "pressure", 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

    #[test]
    fn plan_emits_slot_entry_ops() {
        let mut reg = DimensionRegistry::new();
        reg.register(intensity_property());
        let entries = build_intensity_eml_entries(&reg);
        assert_eq!(entries.len(), 1);
        let ops = plan_intensity_eml_ops(&entries, 3);
        assert_eq!(ops.len(), 3);
        assert!(matches!(ops[0].combine, CombineFn::EvalEML { .. }));
    }
}
