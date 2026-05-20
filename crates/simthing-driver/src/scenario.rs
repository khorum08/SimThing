//! Scenario loading — registry, tree, and shadow seeds for a runnable session.

use ron::de::from_str;
use serde::Deserialize;
use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, IntensityBehavior,
    PropertyValue, SimProperty, SimThing, SimThingId, SimThingKind,
    SimThingKindTag, SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScenarioError {
    #[error("ron parse: {0}")]
    Ron(#[from] ron::error::SpannedError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown builtin scenario {0:?}")]
    UnknownBuiltin(String),
    #[error("property {namespace}:{name} not found after registry build")]
    PropertyNotFound { namespace: String, name: String },
    #[error("sim thing {0:?} has no slot")]
    NoSlot(SimThingId),
}

/// CPU shadow seeds applied before the first GPU upload.
#[derive(Clone, Debug)]
pub struct ShadowSeed {
    pub thing_id:  SimThingId,
    pub namespace: String,
    pub name:      String,
    pub amount:    f32,
    pub velocity:  f32,
}

/// Everything needed to construct a `SimSession`.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name:          String,
    pub ticks_per_day: u32,
    pub max_days:      u32,
    pub dt:            f32,
    pub n_slots:       u32,
    pub registry:      DimensionRegistry,
    pub root:          SimThing,
    pub shadow_seeds:  Vec<ShadowSeed>,
}

#[derive(Debug, Deserialize)]
struct ScenarioSpec {
    name:          String,
    ticks_per_day: u32,
    max_days:      u32,
    dt:            f32,
    n_slots:       u32,
    builtin:       String,
}

impl Scenario {
    pub fn from_ron_str(ron: &str) -> Result<Self, ScenarioError> {
        let spec: ScenarioSpec = from_str(ron)?;
        match spec.builtin.as_str() {
            "rebellion_demo" => Ok(Self::rebellion_demo(
                spec.name,
                spec.ticks_per_day,
                spec.max_days,
                spec.dt,
                spec.n_slots,
            )),
            other => Err(ScenarioError::UnknownBuiltin(other.to_string())),
        }
    }

    pub fn from_ron_path(path: &std::path::Path) -> Result<Self, ScenarioError> {
        let text = std::fs::read_to_string(path)?;
        Self::from_ron_str(&text)
    }

    /// World → Location → Cohort with fission-on-loyalty and integrating velocity.
    pub fn rebellion_demo(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut reg = DimensionRegistry::new();
        let mut loyalty = SimProperty::simple("core", "loyalty", 0);
        loyalty.intensity_behavior = Some(IntensityBehavior::default());
        loyalty.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.8,
                fusion_scar_coefficient: 0.05,
                resolution_label: "rebellion_settled".into(),
            },
            secondary: None,
        }];
        let pid = reg.register(loyalty);

        let layout = reg.property(pid).layout.clone();
        let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[amount_off] = 0.5;
        pv.data[vel_off] = -0.21;
        cohort.add_property(pid, pv);
        let cohort_id = cohort.id;

        let mut loc = SimThing::new(SimThingKind::Location, 0);
        loc.add_child(cohort);
        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_child(loc);

        Self {
            name,
            ticks_per_day,
            max_days,
            dt,
            n_slots,
            registry: reg,
            root: world,
            shadow_seeds: vec![ShadowSeed {
                thing_id: cohort_id,
                namespace: "core".into(),
                name: "loyalty".into(),
                amount: 0.5,
                velocity: -0.21,
            }],
        }
    }

    pub fn apply_shadow_seeds(
        &self,
        allocator: &SlotAllocator,
        shadow: &mut [f32],
        n_dims: usize,
    ) -> Result<(), ScenarioError> {
        for seed in &self.shadow_seeds {
            let slot = allocator
                .slot_of(seed.thing_id)
                .ok_or(ScenarioError::NoSlot(seed.thing_id))? as usize;
            let pid = self
                .registry
                .id_of(&seed.namespace, &seed.name)
                .ok_or_else(|| ScenarioError::PropertyNotFound {
                    namespace: seed.namespace.clone(),
                    name: seed.name.clone(),
                })?;
            let layout = self.registry.property(pid).layout.clone();
            let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
            let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
            let base = slot * n_dims;
            shadow[base + amount_off] = seed.amount;
            shadow[base + vel_off] = seed.velocity;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebellion_demo_ron_round_trips() {
        let ron = include_str!("../../../scenarios/rebellion_demo.ron");
        let scenario = Scenario::from_ron_str(ron).expect("parse");
        assert_eq!(scenario.name, "rebellion_demo");
        assert_eq!(scenario.ticks_per_day, 1);
        assert_eq!(scenario.root.subtree_size(), 3);
        assert!(scenario.registry.id_of("core", "loyalty").is_some());
    }
}
