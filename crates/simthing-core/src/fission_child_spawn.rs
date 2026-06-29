//! Resolved fission child spawn — kind-tag resolution stays in `simthing-core`.
//!
//! Runtime-facing spawn blueprint does not expose authored kind metadata:
//!
//! ```compile_fail
//! use simthing_core::{FissionTemplate, ResolvedFissionChildBlueprint};
//!
//! fn peek_kind(v: ResolvedFissionChildBlueprint<'_>) {
//!     let _ = v.kind;
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::{FissionTemplate, ResolvedFissionChildBlueprint};
//!
//! fn peek_child_kind(v: ResolvedFissionChildBlueprint<'_>) {
//!     let _ = v.child_kind;
//! }
//! ```

use crate::property::{FissionTemplate, SimThingKindTag};
use crate::simthing::{SimThing, SimThingKind};

/// Admission-side spawn blueprint for a fission template child — no `.kind` / `.child_kind`.
pub struct ResolvedFissionChildBlueprint<'a> {
    template: &'a FissionTemplate,
}

impl<'a> ResolvedFissionChildBlueprint<'a> {
    pub fn from_template(template: &'a FissionTemplate) -> Self {
        Self { template }
    }

    pub fn spawn(self, current_day: u32) -> SimThing {
        SimThing::new(kind_tag_to_kind(&self.template.child_kind), current_day)
    }
}

impl FissionTemplate {
    pub fn spawn_child(&self, current_day: u32) -> SimThing {
        ResolvedFissionChildBlueprint::from_template(self).spawn(current_day)
    }
}

fn kind_tag_to_kind(tag: &SimThingKindTag) -> SimThingKind {
    match tag {
        SimThingKindTag::Scenario => SimThingKind::Scenario,
        SimThingKindTag::GameSession => SimThingKind::GameSession,
        SimThingKindTag::World => SimThingKind::World,
        SimThingKindTag::Owner => SimThingKind::Owner,
        SimThingKindTag::Faction => SimThingKind::Faction,
        SimThingKindTag::StarSystem => SimThingKind::StarSystem,
        SimThingKindTag::Location => SimThingKind::Location,
        SimThingKindTag::Cohort => SimThingKind::Cohort,
        SimThingKindTag::Fleet => SimThingKind::Fleet,
        SimThingKindTag::Station => SimThingKind::Station,
        SimThingKindTag::Custom(s) => SimThingKind::Custom(s.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::FissionTemplate;

    fn minimal_template(child_kind: SimThingKindTag) -> FissionTemplate {
        FissionTemplate {
            child_kind,
            fusion_intensity_threshold: 0.0,
            fusion_scar_coefficient: 0.0,
            resolution_label: "test".into(),
            clone_capability_children: false,
            capability_container_kinds: Vec::new(),
        }
    }

    #[test]
    fn spawn_child_resolves_builtin_kind_tags() {
        let cases = [
            (SimThingKindTag::Cohort, SimThingKind::Cohort),
            (SimThingKindTag::Location, SimThingKind::Location),
            (SimThingKindTag::Owner, SimThingKind::Owner),
            (
                SimThingKindTag::Custom("tech_tree".into()),
                SimThingKind::Custom("tech_tree".into()),
            ),
        ];
        for (tag, expected) in cases {
            let child = minimal_template(tag).spawn_child(7);
            assert_eq!(child.kind, expected);
            assert_eq!(child.spawned_day, 7);
        }
    }

    #[test]
    fn faction_tag_maps_to_deprecated_faction_kind() {
        #[allow(deprecated)]
        let child = minimal_template(SimThingKindTag::Faction).spawn_child(0);
        #[allow(deprecated)]
        assert_eq!(child.kind, SimThingKind::Faction);
    }
}
