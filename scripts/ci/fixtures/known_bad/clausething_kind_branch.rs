// CI fixture: SPEC-LOWERER-KIND-READ — closed-lowerer kind branch shape (HEURISTIC).
use simthing_core::SimThingKind;

pub struct SimThing {
    pub kind: SimThingKind,
}

pub fn lowerer_kind_branch(node: &SimThing) -> bool {
    match &node.kind {
        SimThingKind::Fleet => true,
        _ => false,
    }
}