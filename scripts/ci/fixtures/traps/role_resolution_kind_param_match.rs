// CI trap: role-resolution via DA-authored named symbol, not a generic self-service marker.
use simthing_core::SimThingKind;

pub fn planet_non_grid_child_kind_label(kind: &SimThingKind) -> String { match kind { SimThingKind::Fleet => "fleet".into(), other => format!("{other:?}") } }
