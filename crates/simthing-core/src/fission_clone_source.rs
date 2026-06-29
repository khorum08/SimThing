//! Admission-resolved fission clone-source markers.
//!
//! Static authoring intent ("which child subtrees are fission-clone sources") is
//! stamped as opaque property data before tick/boundary clone selection. Production
//! fission paths match labels only — never `SimThingKind`.

use crate::ids::SimPropertyId;
use crate::property::PropertyValue;
use crate::registry::DimensionRegistry;
use crate::simthing::{SimThing, SimThingKind};

/// Well-known structural property: opaque clone-source label resolved at admission/prep.
pub const FISSION_CLONE_SOURCE_PROPERTY_ID: SimPropertyId = SimPropertyId(0xF155_0A01);

/// Opaque clone-source label stored on a child SimThing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FissionCloneSourceLabel(String);

impl FissionCloneSourceLabel {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stamp `label` onto `node` as the fission clone-source marker property.
pub fn stamp_fission_clone_source_label(node: &mut SimThing, label: &str) {
    node.add_property(
        FISSION_CLONE_SOURCE_PROPERTY_ID,
        encode_label_property(label),
    );
}

/// Read the resolved clone-source label from `node`, if present.
pub fn fission_clone_source_label(node: &SimThing) -> Option<FissionCloneSourceLabel> {
    decode_label_property(node.property(FISSION_CLONE_SOURCE_PROPERTY_ID)?)
}

/// Production clone-eligibility predicate — property data only, never kind.
pub fn is_fission_clone_source(node: &SimThing, container_kinds: &[String]) -> bool {
    let Some(label) = fission_clone_source_label(node) else {
        return false;
    };
    container_kinds.iter().any(|k| k == label.as_str())
}

/// Admission/prep only: resolve clone-source markers from authored Custom labels.
/// Idempotent — nodes that already carry a marker are not restamped.
pub fn prep_fission_clone_source_labels(node: &mut SimThing, container_kinds: &[String]) {
    if node.property(FISSION_CLONE_SOURCE_PROPERTY_ID).is_none() {
        let label = match &node.kind {
            SimThingKind::Custom(name) if container_kinds.iter().any(|k| k == name) => {
                Some(name.clone())
            }
            _ => None,
        };
        if let Some(label) = label {
            stamp_fission_clone_source_label(node, &label);
        }
    }
    for child in &mut node.children {
        prep_fission_clone_source_labels(child, container_kinds);
    }
}

/// Collect clone-source container labels from active fission templates in `registry`.
pub fn fission_clone_source_container_kinds_for_registry(
    registry: &DimensionRegistry,
) -> Vec<String> {
    let mut kinds = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let pid = SimPropertyId(idx as u32);
        if !registry.is_active(pid) {
            continue;
        }
        for ft in &prop.fission_templates {
            if !ft.template.clone_capability_children {
                continue;
            }
            for label in &ft.template.capability_container_kinds {
                if !kinds.iter().any(|existing| existing == label) {
                    kinds.push(label.clone());
                }
            }
        }
    }
    kinds
}

/// Admission/prep only: stamp clone-source markers for every label declared on active
/// fission templates. Safe to call repeatedly — already-stamped nodes are skipped.
pub fn prepare_fission_clone_sources_for_registry(
    root: &mut SimThing,
    registry: &DimensionRegistry,
) {
    let kinds = fission_clone_source_container_kinds_for_registry(registry);
    if kinds.is_empty() {
        return;
    }
    prep_fission_clone_source_labels(root, &kinds);
}

/// Admission/prep only: stamp clone-source markers on an incoming subtree (e.g. AddChild).
pub fn prepare_fission_clone_sources_subtree(subtree: &mut SimThing, registry: &DimensionRegistry) {
    let kinds = fission_clone_source_container_kinds_for_registry(registry);
    if kinds.is_empty() {
        return;
    }
    prep_fission_clone_source_labels(subtree, &kinds);
}

fn encode_label_property(label: &str) -> PropertyValue {
    let mut lanes = vec![label.len() as f32];
    for chunk in label.as_bytes().chunks(4) {
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        lanes.push(f32::from_bits(u32::from_le_bytes(bytes)));
    }
    PropertyValue::from_raw_lanes(lanes)
}

fn decode_label_property(value: &PropertyValue) -> Option<FissionCloneSourceLabel> {
    let lanes = value.raw_lanes_for_serialization();
    if lanes.is_empty() {
        return None;
    }
    let len = lanes[0] as usize;
    let mut bytes = Vec::with_capacity(len);
    for lane in lanes.iter().skip(1) {
        bytes.extend_from_slice(&lane.to_bits().to_le_bytes());
    }
    bytes.truncate(len);
    String::from_utf8(bytes).ok().map(FissionCloneSourceLabel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fission_clone_source_label_roundtrip() {
        let mut node = SimThing::new(SimThingKind::Location, 0);
        stamp_fission_clone_source_label(&mut node, "tech_tree");
        let label = fission_clone_source_label(&node).expect("label");
        assert_eq!(label.as_str(), "tech_tree");
        assert!(is_fission_clone_source(
            &node,
            &["tech_tree".into(), "other".into()]
        ));
        assert!(!is_fission_clone_source(&node, &["other".into()]));
    }

    #[test]
    fn prepare_fission_clone_sources_for_registry_is_idempotent() {
        let mut reg = DimensionRegistry::new();
        let mut prop = crate::property::SimProperty::simple("core", "loyalty", 0);
        prop.fission_templates = vec![crate::property::FissionThreshold {
            sub_field: crate::property::SubFieldRole::Amount,
            threshold: 0.3,
            direction: crate::property::Direction::Falling,
            template: crate::property::FissionTemplate {
                child_kind: crate::property::SimThingKindTag::Custom("tech_tree".into()),
                fusion_intensity_threshold: 0.8,
                fusion_scar_coefficient: 0.05,
                resolution_label: "resolved".into(),
                clone_capability_children: true,
                capability_container_kinds: vec!["tech_tree".into()],
            },
            secondary: None,
        }];
        reg.register(prop);

        let mut faction = SimThing::new(SimThingKind::Faction, 0);
        faction.add_child(SimThing::new(SimThingKind::Custom("tech_tree".into()), 0));
        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(faction);

        prepare_fission_clone_sources_for_registry(&mut root, &reg);
        let first = root.children[0].children[0]
            .property(FISSION_CLONE_SOURCE_PROPERTY_ID)
            .expect("marker")
            .raw_lanes_for_serialization()
            .to_vec();
        prepare_fission_clone_sources_for_registry(&mut root, &reg);
        let second = root.children[0].children[0]
            .property(FISSION_CLONE_SOURCE_PROPERTY_ID)
            .expect("marker")
            .raw_lanes_for_serialization()
            .to_vec();
        assert_eq!(first, second);
    }
}
