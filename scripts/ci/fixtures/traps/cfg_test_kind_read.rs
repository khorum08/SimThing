// CI trap: .kind read inside #[cfg(test)] — excluded by cfg(test) heuristic filter.

pub enum SimThingKind {
    Owner,
}

pub struct SimThing {
    pub kind: SimThingKind,
}

#[cfg(test)]
mod tests {
    use super::{SimThing, SimThingKind};

    pub fn read_kind(thing: &SimThing) -> SimThingKind {
        thing.kind
    }
}

pub fn production_ok() -> u32 {
    0
}
