use std::collections::BTreeSet;

use crate::ids::{
    advance_simthing_id_allocator_past, SimPropertyId, SimThingId, SimThingIdReservationError,
};
use crate::overlay::Overlay;
use crate::property::PropertyValue;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

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
    /// Driver/session topology marker for arena-participant wrapper nodes (E-10R2).
    /// Not a spatial entity; `simthing-sim` must not branch on this variant.
    ArenaParticipant,
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
    /// All overlays directly owned by this SimThing (policy, governance, instructions, etc.)
    pub overlays: Vec<Overlay>,
    /// Physical spatial children (locations own cohorts; systems own locations; etc.)
    pub children: Vec<SimThing>,
    /// Day this SimThing was created (set at spawn).
    pub spawned_day: u32,
}

impl SimThing {
    pub fn new(kind: SimThingKind, spawned_day: u32) -> Self {
        Self {
            id: SimThingId::new(),
            kind,
            properties: HashMap::new(),
            overlays: Vec::new(),
            children: Vec::new(),
            spawned_day,
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

    pub fn add_overlay(&mut self, overlay: Overlay) {
        self.overlays.push(overlay);
    }

    pub fn add_child(&mut self, child: SimThing) {
        self.children.push(child);
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
        SimThingKind::ArenaParticipant => authored == "ArenaParticipant",
        SimThingKind::Custom(s) => s == authored,
    }
}

/// Kind-free topology predicate for callers that must not name [`SimThingKind`].
pub fn is_arena_participant_node(node: &SimThing) -> bool {
    kind_matches("ArenaParticipant", &node.kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtree_size() {
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);
        loc.add_child(SimThing::new(SimThingKind::Cohort, 0));
        loc.add_child(SimThing::new(SimThingKind::Cohort, 0));
        world.add_child(loc);
        // world + 1 location + 2 cohorts = 4
        assert_eq!(world.subtree_size(), 4);
    }

    #[test]
    fn loaded_tree_reserves_existing_simthing_ids() {
        let mut world = SimThing::new(SimThingKind::World, 0);
        let loaded = SimThing {
            id: SimThingId::from_session_raw(1_000_000),
            kind: SimThingKind::Location,
            properties: HashMap::new(),
            overlays: Vec::new(),
            children: Vec::new(),
            spawned_day: 0,
        };
        world.add_child(loaded);

        reserve_simthing_ids_from_tree(&world).expect("reserve ids");

        let spawned = SimThing::new(SimThingKind::Cohort, 0);
        assert!(spawned.id.raw() > 1_000_000);
    }
}
