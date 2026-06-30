// CI fixture: SIM-KIND-READ — production-shaped .kind read (HEURISTIC).
pub enum SimThingKind {
    Owner,
}

pub struct SimThing {
    pub kind: SimThingKind,
}

pub fn kind_tag(thing: &SimThing) -> SimThingKind {
    match thing.kind {
        SimThingKind::Owner => SimThingKind::Owner,
    }
}
