//! Canonical entity-name derivation for generated star systems.
//!
//! `assign_star_names` is the sole name authority for generated system IDs.
//! Structural MapGen RNG is not consumed: the catalog shuffle uses
//! `MapGenSeed(seed ^ NAMING_SEED_DOMAIN)` and a deterministic Fisher-Yates
//! walk over the 4096-entry prefix/core/suffix catalog. Inputs are sorted and
//! deduplicated before assignment. Future golden drift is a decision, not a
//! surprise: re-bless only through `UPDATE_STUDIO_STAR_NAMING_GOLDEN` after
//! naming the first divergent upstream commit. See
//! `crates/simthing-clausething/tests/studio_star_naming_pass_0.rs`.

use crate::rng::{MapGenRng, MapGenSeed};

const NAMING_SEED_DOMAIN: u64 = 0x5354_4152_4E41_4D45;
const PREFIXES: [&str; 16] = [
    "Al", "Be", "Ca", "Da", "El", "Fa", "Ga", "Ha", "Io", "Ja", "Ka", "Lu", "Ma", "Na", "Or", "Pa",
];
const CORES: [&str; 16] = [
    "ra", "se", "ti", "vo", "we", "xi", "yo", "zu", "an", "en", "in", "on", "ul", "ar", "er", "ir",
];
const SUFFIXES: [&str; 16] = [
    "ax", "el", "is", "or", "um", "yn", "ea", "os", "ix", "al", "et", "us", "on", "ar", "en", "ir",
];
const CATALOG_LEN: usize = PREFIXES.len() * CORES.len() * SUFFIXES.len();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarNameAssignment {
    pub system_id: u32,
    pub display_name: String,
}

/// Canonical entity-name derivation: unique names for sorted, deduplicated system IDs.
///
/// Does not consume structural generation RNG. Catalog index `i` is stable for a
/// given seed and sorted ID sequence; catalogue overflow uses a `" {cycle+1}"` suffix.
pub fn assign_star_names<I>(seed: u64, system_ids: I) -> Vec<StarNameAssignment>
where
    I: IntoIterator<Item = u32>,
{
    let mut system_ids: Vec<u32> = system_ids.into_iter().collect();
    system_ids.sort_unstable();
    system_ids.dedup();

    let order = shuffled_catalog_indices(seed);
    system_ids
        .into_iter()
        .enumerate()
        .map(|(index, system_id)| StarNameAssignment {
            system_id,
            display_name: name_from_order(&order, index),
        })
        .collect()
}

/// Resolve one stable assignment index using the same isolated naming sequence.
pub fn star_name_for_index_or_seed(seed: u64, assignment_index: usize) -> String {
    name_from_order(&shuffled_catalog_indices(seed), assignment_index)
}

fn shuffled_catalog_indices(seed: u64) -> Vec<usize> {
    let mut order: Vec<usize> = (0..CATALOG_LEN).collect();
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(seed ^ NAMING_SEED_DOMAIN));
    for cursor in (1..order.len()).rev() {
        let swap_with = rng.gen_index((cursor + 1) as u32) as usize;
        order.swap(cursor, swap_with);
    }
    order
}

fn name_from_order(order: &[usize], assignment_index: usize) -> String {
    let cycle = assignment_index / CATALOG_LEN;
    let catalog_index = order[assignment_index % CATALOG_LEN];
    let suffix_index = catalog_index % SUFFIXES.len();
    let core_index = (catalog_index / SUFFIXES.len()) % CORES.len();
    let prefix_index = catalog_index / (CORES.len() * SUFFIXES.len());
    let base = format!(
        "{}{}{}",
        PREFIXES[prefix_index], CORES[core_index], SUFFIXES[suffix_index]
    );
    if cycle == 0 {
        base
    } else {
        format!("{base} {}", cycle + 1)
    }
}
