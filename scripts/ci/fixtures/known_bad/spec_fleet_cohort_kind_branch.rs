// CI fixture: SPEC-LOWERER-KIND-READ — drift-shaped spec Fleet/Cohort traversal (HEURISTIC).
use simthing_core::SimThingKind;

pub struct SimThing {
    pub kind: SimThingKind,
    pub children: Vec<SimThing>,
}

pub fn drift_shaped_fleet_traversal(child: &SimThing) -> bool {
    if child.kind == SimThingKind::Fleet {
        for ship in &child.children {
            if ship.kind != SimThingKind::Cohort {
                return true;
            }
        }
    }
    false
}