// CI trap: semantic words inside #[cfg(test)] — excluded by cfg(test) heuristic filter.

#[cfg(test)]
mod tests {
    pub fn faction_fixture_trap() -> &'static str {
        "faction"
    }
}

pub fn production_ok() -> u32 {
    0
}
