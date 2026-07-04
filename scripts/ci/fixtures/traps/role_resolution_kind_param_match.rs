// CI trap: role-resolution via kind-parameter match (not drift-shaped .kind field branching).
use simthing_core::SimThingKind;

pub fn planet_non_grid_child_kind_label(kind: &SimThingKind) -> String {
    match kind {
        SimThingKind::Cohort => "cohort".into(),
        SimThingKind::Fleet => "fleet".into(),
        SimThingKind::Station => "station".into(),
        SimThingKind::Custom(name) => name.clone(),
        other => format!("{other:?}"),
    }
}