//! Admission-resolved fission clone-source markers.
//!
//! Static authoring intent ("which child subtrees are fission-clone sources") is
//! stamped as opaque property data before tick/boundary clone selection. Production
//! fission paths match labels only — never `SimThingKind`.

use crate::ids::SimPropertyId;
use crate::property::PropertyValue;
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
pub fn prep_fission_clone_source_labels(node: &mut SimThing, container_kinds: &[String]) {
    let label = match &node.kind {
        SimThingKind::Custom(name) if container_kinds.iter().any(|k| k == name) => {
            Some(name.clone())
        }
        _ => None,
    };
    if let Some(label) = label {
        stamp_fission_clone_source_label(node, &label);
    }
    for child in &mut node.children {
        prep_fission_clone_source_labels(child, container_kinds);
    }
}

/// Admission/prep only: resolve direct-child clone-source markers on a fission parent.
pub fn prep_fission_parent_clone_source_labels(parent: &mut SimThing, container_kinds: &[String]) {
    for child in &mut parent.children {
        prep_fission_clone_source_labels(child, container_kinds);
    }
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
}
