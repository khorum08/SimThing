//! BAND-QUANTIZED-DRAW-0 Remand 2 — real ThresholdBuilder + BoundaryProtocol
//! CostBand admission / resolve referees.

use simthing_core::{
    cost_band_expected_n, cost_band_quantize, ConjunctiveRecipeRegistration, Direction,
    SimProperty, SimPropertyId, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::{
    BoundaryProtocol, CostBandSemantic, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertRegistration,
};

fn velocity_sem() -> ThresholdSemantic {
    ThresholdSemantic::VelocityAlert {
        sim_thing_id: SimThingId::new(),
        property_id: SimPropertyId(1),
        sub_field: SubFieldRole::Amount,
    }
}

fn simple_boundary() -> (BoundaryProtocol, SimThingId, SimPropertyId) {
    let mut reg = simthing_core::DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    child.add_property(pid, reg.property(pid).default_value());
    let child_id = child.id;
    root.add_child(child);
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&root);
    let proto = BoundaryProtocol::new(simthing_sim::SimRuntimeTree::admit(root), reg, alloc);
    (proto, child_id, pid)
}
