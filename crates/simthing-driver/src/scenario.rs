//! Scenario loading — registry, tree, and shadow seeds for a runnable session.

use ron::de::from_str;
use serde::Deserialize;
use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, IntensityBehavior,
    PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingId, SimThingKind,
    SimThingKindTag, SubFieldRole, TransformOp,
};
use simthing_feeder::PatchTransform;
use simthing_gpu::SlotAllocator;
use std::collections::HashMap;
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
    pub thing_id: SimThingId,
    pub namespace: String,
    pub name: String,
    pub amount: f32,
    pub velocity: f32,
}

/// Everything needed to construct a `SimSession`.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: String,
    pub ticks_per_day: u32,
    pub max_days: u32,
    pub dt: f32,
    pub n_slots: u32,
    pub registry: DimensionRegistry,
    pub root: SimThing,
    pub shadow_seeds: Vec<ShadowSeed>,
    pub tick_patches: Vec<PatchTransform>,
    /// Authored install-target registry. Maps a logical target id (used by
    /// `InstallTargetSpec::ScenarioListed { target_id }` on the spec layer)
    /// to a list of `SimThingId`s present in `root`. Empty in builtin
    /// scenarios; populated by RON-loaded scenarios that pin capability
    /// trees or scripted events to specific owners.
    pub install_targets: HashMap<String, Vec<SimThingId>>,
}

#[derive(Debug, Deserialize)]
struct ScenarioSpec {
    name: String,
    ticks_per_day: u32,
    max_days: u32,
    dt: f32,
    n_slots: u32,
    builtin: String,
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
            "map_1m_light" => Ok(Self::map_light(
                spec.name,
                spec.ticks_per_day,
                spec.max_days,
                spec.dt,
                spec.n_slots,
            )),
            "pop_heavy" => Ok(Self::pop_heavy(
                spec.name,
                spec.ticks_per_day,
                spec.max_days,
                spec.dt,
                spec.n_slots,
            )),
            "intent_stress" => Ok(Self::intent_stress(
                spec.name,
                spec.ticks_per_day,
                spec.max_days,
                spec.dt,
                spec.n_slots,
            )),
            "fission_stress" => Ok(Self::fission_stress(
                spec.name,
                spec.ticks_per_day,
                spec.max_days,
                spec.dt,
                spec.n_slots,
            )),
            "threshold_stress" => Ok(Self::threshold_stress(
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
                clone_capability_children: false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(loyalty);

        let layout = reg.property(pid).layout.clone();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.set_role(&SubFieldRole::Amount, &layout, 0.5);
        pv.set_role(&SubFieldRole::Velocity, &layout, -0.21);
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
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        }
    }

    pub fn map_light(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("map", "stability", 0));
        let layout = reg.property(pid).layout.clone();

        let mut world = SimThing::new(SimThingKind::World, 0);
        for i in 0..n_slots.saturating_sub(1) {
            let mut cell = SimThing::new(SimThingKind::Location, 0);
            let mut pv = PropertyValue::from_layout(&layout);
            pv.set_role(
                &SubFieldRole::Amount,
                &layout,
                0.5 + ((i % 100) as f32) * 0.001,
            );
            pv.set_role(
                &SubFieldRole::Velocity,
                &layout,
                if i % 2 == 0 { 0.0001 } else { -0.0001 },
            );
            cell.add_property(pid, pv);
            world.add_child(cell);
        }

        Self {
            name,
            ticks_per_day,
            max_days,
            dt,
            n_slots,
            registry: reg,
            root: world,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        }
    }

    pub fn pop_heavy(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut reg = DimensionRegistry::new();
        let mut population = SimProperty::simple("pop", "cohort", 29);
        population.intensity_behavior = Some(IntensityBehavior::default());
        let pid = reg.register(population);
        let layout = reg.property(pid).layout.clone();

        let locations = ((n_slots / 100).max(1)).min(n_slots.saturating_sub(1).max(1));
        let cohorts_per_location = ((n_slots.saturating_sub(1)) / locations).max(1);
        let mut made = 1u32;
        let mut world = SimThing::new(SimThingKind::World, 0);
        for loc_i in 0..locations {
            if made >= n_slots {
                break;
            }
            let mut loc = SimThing::new(SimThingKind::Location, 0);
            made += 1;
            for cohort_i in 0..cohorts_per_location {
                if made >= n_slots {
                    break;
                }
                let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
                let mut pv = PropertyValue::from_layout(&layout);
                pv.set_role(
                    &SubFieldRole::Amount,
                    &layout,
                    0.4 + ((cohort_i % 50) as f32) * 0.002,
                );
                pv.set_role(
                    &SubFieldRole::Velocity,
                    &layout,
                    0.001 * ((loc_i % 3) as f32 - 1.0),
                );
                pv.set_role(
                    &SubFieldRole::Intensity,
                    &layout,
                    0.2 + ((cohort_i % 10) as f32) * 0.03,
                );
                cohort.add_property(pid, pv);
                loc.add_child(cohort);
                made += 1;
            }
            world.add_child(loc);
        }

        Self {
            name,
            ticks_per_day,
            max_days,
            dt,
            n_slots,
            registry: reg,
            root: world,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        }
    }

    pub fn intent_stress(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut scenario = Self::map_light(name, ticks_per_day, max_days, dt, n_slots);
        let pid = scenario.registry.id_of("map", "stability").unwrap();
        let mut patches = Vec::new();
        for child in scenario.root.children.iter().take(10_000) {
            patches.push(PatchTransform {
                target: child.id,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![
                        (SubFieldRole::Amount, TransformOp::Add(0.0001)),
                        (SubFieldRole::Velocity, TransformOp::Multiply(0.999)),
                    ],
                },
            });
        }
        scenario.tick_patches = patches;
        scenario
    }

    pub fn fission_stress(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut reg = DimensionRegistry::new();
        let mut pressure = SimProperty::simple("stress", "pressure", 0);
        pressure.intensity_behavior = Some(IntensityBehavior::default());
        pressure.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.9,
                fusion_scar_coefficient: 0.02,
                resolution_label: "stress_resolved".into(),
                clone_capability_children: false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(pressure);
        let layout = reg.property(pid).layout.clone();

        let mut world = SimThing::new(SimThingKind::World, 0);
        for i in 0..n_slots.saturating_sub(1) {
            let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
            let mut pv = PropertyValue::from_layout(&layout);
            pv.set_role(
                &SubFieldRole::Amount,
                &layout,
                0.31 + ((i % 5) as f32) * 0.001,
            );
            pv.set_role(&SubFieldRole::Velocity, &layout, -0.02);
            pv.set_role(&SubFieldRole::Intensity, &layout, 0.1);
            cohort.add_property(pid, pv);
            world.add_child(cohort);
        }

        Self {
            name,
            ticks_per_day,
            max_days,
            dt,
            n_slots,
            registry: reg,
            root: world,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        }
    }

    pub fn threshold_stress(
        name: String,
        ticks_per_day: u32,
        max_days: u32,
        dt: f32,
        n_slots: u32,
    ) -> Self {
        let mut scenario = Self::fission_stress(name, ticks_per_day, max_days, dt, n_slots);
        scenario.max_days = scenario.max_days.max(2);
        scenario
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
            shadow[base + amount_off.lane()] = seed.amount;
            shadow[base + vel_off.lane()] = seed.velocity;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_builtin_seed_values_survive_column_access_refactor() {
        let scenario = Scenario::rebellion_demo("rebellion_demo".into(), 1, 1, 0.5, 32);
        let pid = scenario.registry.id_of("core", "loyalty").unwrap();
        let layout = scenario.registry.property(pid).layout.clone();
        let cohort = scenario
            .root
            .children
            .first()
            .and_then(|loc| loc.children.first())
            .expect("cohort");
        let pv = cohort.properties.get(&pid).expect("loyalty property");
        assert!((pv.get_role(&SubFieldRole::Amount, &layout) - 0.5).abs() < f32::EPSILON);
        assert!((pv.get_role(&SubFieldRole::Velocity, &layout) - (-0.21)).abs() < f32::EPSILON);
        assert_eq!(scenario.shadow_seeds.len(), 1);
        assert!((scenario.shadow_seeds[0].amount - 0.5).abs() < f32::EPSILON);
        assert!((scenario.shadow_seeds[0].velocity - (-0.21)).abs() < f32::EPSILON);
    }

    #[test]
    fn rebellion_demo_ron_round_trips() {
        let ron = include_str!("../../../scenarios/rebellion_demo.ron");
        let scenario = Scenario::from_ron_str(ron).expect("parse");
        assert_eq!(scenario.name, "rebellion_demo");
        assert_eq!(scenario.ticks_per_day, 1);
        assert_eq!(scenario.root.subtree_size(), 3);
        assert!(scenario.registry.id_of("core", "loyalty").is_some());
    }

    #[test]
    fn stress_builtins_load_at_small_scale() {
        for builtin in [
            "map_1m_light",
            "pop_heavy",
            "intent_stress",
            "fission_stress",
            "threshold_stress",
        ] {
            let ron = format!(
                r#"(
                    name: "{builtin}",
                    builtin: "{builtin}",
                    ticks_per_day: 1,
                    max_days: 1,
                    dt: 0.5,
                    n_slots: 32,
                )"#
            );
            let scenario = Scenario::from_ron_str(&ron).expect("parse builtin");
            assert_eq!(scenario.name, builtin);
            assert!(scenario.root.subtree_size() > 1);
        }
    }

    #[test]
    fn intent_stress_queues_tick_patches() {
        let scenario = Scenario::intent_stress("intent_stress".into(), 1, 1, 0.5, 64);
        assert!(!scenario.tick_patches.is_empty());
    }
}
