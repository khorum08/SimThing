use crate::ids::{SimPropertyId, SimThingId};
use crate::overlay::Overlay;
use crate::property::PropertyValue;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimThingKind {
    World,
    Faction,
    StarSystem,
    Location,
    Cohort,
    Fleet,
    Station,
    Custom(String),
}

/// Every entity in the simulation. The spatial tree expresses physical ownership.
/// Political structures, factions, and all non-physical groupings are overlays,
/// not nodes in the tree.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimThing {
    pub id:         SimThingId,
    pub kind:       SimThingKind,
    /// Sparse map: only properties that are currently meaningful for this entity.
    /// Adding a new property dimension never changes this struct.
    /// Serialized as a list of pairs since JSON object keys must be strings.
    #[serde_as(as = "Vec<(_, _)>")]
    pub properties: HashMap<SimPropertyId, PropertyValue>,
    /// All overlays directly owned by this SimThing (policy, governance, instructions, etc.)
    pub overlays:   Vec<Overlay>,
    /// Physical spatial children (locations own cohorts; systems own locations; etc.)
    pub children:   Vec<SimThing>,
    /// Day this SimThing was created (set at spawn).
    pub spawned_day: u32,
}

impl SimThing {
    pub fn new(kind: SimThingKind, spawned_day: u32) -> Self {
        Self {
            id:          SimThingId::new(),
            kind,
            properties:  HashMap::new(),
            overlays:    Vec::new(),
            children:    Vec::new(),
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
        1 + self.children.iter().map(|c| c.subtree_size()).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtree_size() {
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc   = SimThing::new(SimThingKind::Location, 0);
        loc.add_child(SimThing::new(SimThingKind::Cohort, 0));
        loc.add_child(SimThing::new(SimThingKind::Cohort, 0));
        world.add_child(loc);
        // world + 1 location + 2 cohorts = 4
        assert_eq!(world.subtree_size(), 4);
    }
}
