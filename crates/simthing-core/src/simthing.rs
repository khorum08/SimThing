use std::collections::BTreeSet;

use crate::ids::{
    advance_simthing_id_allocator_past, SimPropertyId, SimThingId, SimThingIdReservationError,
};
use crate::overlay::Overlay;
use crate::property::PropertyValue;
use crate::residency::{ObjectResidencyRelease, ObjectResidencyRequest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

/// A typed, non-spatial parent edge for one resource property.
///
/// The edge is symbolic because a [`SimThing`] may be populated before its
/// property registry is compiled. Admission resolves the key, verifies that
/// both endpoints possess the property, and derives the arena topology.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceParentEdge {
    pub property_namespace: String,
    pub property_name: String,
    pub parent: SimThingId,
    /// Front-end source token retained for admission diagnostics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_token: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimThingKind {
    /// Serializable scenario file root. Authority/serialization marker only — not a runtime engine.
    Scenario,
    /// Running game session root — sole direct child of [`Scenario`]. Authority marker only;
    /// not a runtime engine singleton. Owner entities and maps are future children of GameSession.
    GameSession,
    World,
    /// Owner entity — sibling child of the GameSession root (not an overlay, not a spatial parent).
    Owner,
    /// **DEPRECATED — legacy serialized alias for [`Owner`].** Retained so existing serialized
    /// trees and `AllOfKind { kind: "Faction" }` install targets keep working. New authoring
    /// must use [`Owner`] / `"Owner"`.
    #[deprecated(note = "Use Owner. Retained only for legacy serialized data compatibility.")]
    Faction,
    /// **DEPRECATED — DO NOT USE (design authority, 2026-06-03).** `StarSystem` was added
    /// without a consuming scenario and violates maximal SimThing conformance
    /// (`design_0_0_8_1.md` §0.1): a star system is a `Location` SimThing carrying the relevant
    /// properties / overlays / arena enrollments, not a privileged kind. Retained only so legacy
    /// serialized data and the exhaustive `kind_matches` / `kind_tag_to_kind` arms still compile.
    /// Do not author new entities of this kind.
    StarSystem,
    Location,
    Cohort,
    Fleet,
    /// **DEPRECATED — DO NOT USE (design authority, 2026-06-03).** Same disposition as
    /// `StarSystem`: model a station as a `Location` / `Cohort` SimThing with the appropriate
    /// properties / overlays. Retained only for compile-compatibility; do not author new entities
    /// of this kind.
    Station,
    Custom(String),
}

/// Every entity in the simulation is a [`SimThing`].
///
/// The running simulation is rooted in a GameSession / Session [`SimThing`]. Owner entities are
/// sibling children of the Session root — not overlays and not spatial parents. Policies, bonuses,
/// penalties, capability subtrees, and stockpiles may live on Owner [`SimThing`]s as properties,
/// overlays, and children.
///
/// The spatial subtree expresses physical containment only. Asset ownership is represented by owner
/// references, properties, and columns — never by spatial reparenting. Runtime simulation code must
/// not branch behavior on [`SimThingKind`].
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimThing {
    pub id: SimThingId,
    pub kind: SimThingKind,
    /// Sparse map: only properties that are currently meaningful for this entity.
    /// Adding a new property dimension never changes this struct.
    /// Serialized as a list of pairs since JSON object keys must be strings.
    #[serde_as(as = "Vec<(_, _)>")]
    pub properties: HashMap<SimPropertyId, PropertyValue>,
    /// Resource-channel parentage. This never expresses spatial containment;
    /// physical containment remains exclusively in [`Self::children`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_parent_edges: Vec<ResourceParentEdge>,
    /// All overlays directly owned by this SimThing (policy, governance, instructions, etc.)
    pub overlays: Vec<Overlay>,
    /// Physical spatial children (locations own cohorts; systems own locations; etc.)
    pub children: Vec<SimThing>,
    /// Generation this SimThing was created (set at spawn; P0 generation ruling).
    /// Serde alias preserves the historical wire field name for load compatibility.
    #[serde(alias = "spawned_day")]
    pub spawned_generation: u32,
    /// Explicitly declared specialization profiles (SPECIALIZATION-PROTOCOL-0,
    /// P3). Additive-only: legacy trees carry none and load unchanged; empty
    /// declarations serialize to nothing (wire-identical to pre-3.1 trees).
    /// Validated at admission — see [`crate::specialization`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_specializations: Vec<crate::specialization::DeclaredSpecialization>,
}

impl SimThing {
    pub fn new(kind: SimThingKind, spawned_generation: u32) -> Self {
        Self {
            id: SimThingId::new(),
            kind,
            properties: HashMap::new(),
            resource_parent_edges: Vec::new(),
            overlays: Vec::new(),
            children: Vec::new(),
            spawned_generation,
            declared_specializations: Vec::new(),
        }
    }

    pub fn add_property(&mut self, id: SimPropertyId, value: PropertyValue) {
        self.properties.insert(id, value);
    }

    pub fn remove_property(&mut self, id: &SimPropertyId) -> Option<PropertyValue> {
        self.properties.remove(id)
    }

    pub fn property(&self, id: SimPropertyId) -> Option<&PropertyValue> {
        self.properties.get(&id)
    }

    pub fn property_mut(&mut self, id: SimPropertyId) -> Option<&mut PropertyValue> {
        self.properties.get_mut(&id)
    }

    pub fn add_resource_parent_edge(
        &mut self,
        property_namespace: impl Into<String>,
        property_name: impl Into<String>,
        parent: SimThingId,
        source_span_token: Option<usize>,
    ) {
        self.resource_parent_edges.push(ResourceParentEdge {
            property_namespace: property_namespace.into(),
            property_name: property_name.into(),
            parent,
            source_span_token,
        });
    }

    pub fn add_overlay(&mut self, overlay: Overlay) {
        self.overlays.push(overlay);
    }

    pub fn add_child(&mut self, child: SimThing) {
        self.children.push(child);
    }

    /// Emit the root-side request for this object to enter kernel residency.
    ///
    /// The request contains object identity and relation only. The kernel
    /// assigns the ephemeral [`crate::SlotIndex`].
    pub fn root_residency_request(&self) -> ObjectResidencyRequest {
        ObjectResidencyRequest::root(self.id)
    }

    /// Emit the child-row request owned by this parent/child relation.
    pub fn child_residency_request(&self, child: &SimThing) -> ObjectResidencyRequest {
        ObjectResidencyRequest::child(child.id, self.id)
    }

    /// Emit the object-side request to retire this object's ephemeral row.
    pub fn residency_release_request(&self) -> ObjectResidencyRelease {
        ObjectResidencyRelease::new(self.id)
    }

    /// Total number of SimThings in this subtree (including self).
    pub fn subtree_size(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|c| c.subtree_size())
            .sum::<usize>()
    }

    pub fn max_id_in_subtree(&self) -> SimThingId {
        self.children
            .iter()
            .map(|child| child.max_id_in_subtree())
            .fold(self.id, |max, candidate| max.max(candidate))
    }
}

pub fn reserve_simthing_ids_from_tree(root: &SimThing) -> Result<(), SimThingIdReservationError> {
    let mut seen = BTreeSet::new();
    reserve_visit_simthings(root, &mut seen)?;
    advance_simthing_id_allocator_past(root.max_id_in_subtree())
}

fn reserve_visit_simthings(
    thing: &SimThing,
    seen: &mut BTreeSet<u32>,
) -> Result<(), SimThingIdReservationError> {
    if !seen.insert(thing.id.raw()) {
        return Err(SimThingIdReservationError::DuplicateId(thing.id.raw()));
    }
    for child in &thing.children {
        reserve_visit_simthings(child, seen)?;
    }
    Ok(())
}

/// Compare an authored kind string (from RON / spec layer) to a runtime
/// `SimThingKind`. Used by `InstallTargetSpec::AllOfKind` to match install
/// targets without exposing `SimThingKind` variants to the spec crate.
///
/// Built-in variant names match the enum identifier exactly (`"World"`,
/// `"Owner"`, …). Legacy `"Faction"` matches deprecated [`SimThingKind::Faction`] and canonical
/// [`SimThingKind::Owner`]. `Custom(name)` matches when `authored == name`.
pub fn kind_matches(authored: &str, sim: &SimThingKind) -> bool {
    match sim {
        SimThingKind::Scenario => authored == "Scenario",
        SimThingKind::GameSession => authored == "GameSession",
        SimThingKind::World => authored == "World",
        SimThingKind::Owner => authored == "Owner" || authored == "Faction",
        SimThingKind::Faction => authored == "Faction" || authored == "Owner",
        SimThingKind::StarSystem => authored == "StarSystem",
        SimThingKind::Location => authored == "Location",
        SimThingKind::Cohort => authored == "Cohort",
        SimThingKind::Fleet => authored == "Fleet",
        SimThingKind::Station => authored == "Station",
        SimThingKind::Custom(s) => s == authored,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SESSION-WIRING-KILL-SWEEP-0: historical wire key loads into spawned_generation.
    #[test]
    fn spawned_generation_deserializes_legacy_generation_wire_alias() {
        // Fixture retains the historical JSON key only; identifier is generation-vocabulary.
        let json = r#"{
            "id": 1,
            "kind": "World",
            "properties": [],
            "overlays": [],
            "children": [],
            "spawned_day": 42
        }"#;
        let thing: SimThing =
            serde_json::from_str(json).expect("legacy generation wire alias load");
        assert_eq!(thing.spawned_generation, 42);
    }
}
