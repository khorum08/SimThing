// CI fixture: SPEC-LOWERER-KIND-READ - deleted generic marker must not suppress scanning.
use simthing_core::SimThingKind;

pub fn generic_role_resolution_label(kind: &SimThingKind) -> String {
    match kind { SimThingKind::Fleet => "fleet".into(), other => format!("{other:?}") } // role-resolution-exclude-site
}
