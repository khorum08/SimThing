// CI fixture: SPEC-LOWERER-KIND-READ — parameterized SimThingKind branch (HEURISTIC).
use simthing_core::SimThingKind;

pub fn drift_param_kind_branch(kind: &SimThingKind) -> bool {
    match kind {
        SimThingKind::Fleet => true,
        _ => false,
    }
}