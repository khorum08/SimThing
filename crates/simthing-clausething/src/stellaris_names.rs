//! Narrow consumer for Stellaris' literal `star_names` ClauseScript pool.
//!
//! The catalog is authored vocabulary, not a phoneme generator. Assignment is a
//! SimThing compatibility policy: deterministic shuffle by galaxy seed, then
//! no replacement within each catalog cycle.

use std::collections::BTreeMap;

use thiserror::Error;

use crate::raw::{RawArray, RawValue};
use crate::{ParseError, parse_raw_document};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StellarisStarNameCatalog {
    names: Vec<String>,
}

impl StellarisStarNameCatalog {
    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Deterministically assign authored names to sorted system ids.
    ///
    /// Authored duplicates are preserved because Stellaris pools may use them
    /// as implicit weighting. When systems outnumber entries, a newly shuffled
    /// cycle is used; names therefore repeat only after a complete pool cycle,
    /// aside from authored duplicates.
    pub fn assign_to_systems<I>(&self, seed: u64, system_ids: I) -> BTreeMap<u32, String>
    where
        I: IntoIterator<Item = u32>,
    {
        let mut ids: Vec<u32> = system_ids.into_iter().collect();
        ids.sort_unstable();
        ids.dedup();

        let mut assigned = BTreeMap::new();
        if self.names.is_empty() {
            return assigned;
        }

        let mut cycle = 0_u64;
        let mut order = shuffled_indices(self.names.len(), seed, cycle);
        for (index, system_id) in ids.into_iter().enumerate() {
            let offset = index % self.names.len();
            if offset == 0 && index != 0 {
                cycle += 1;
                order = shuffled_indices(self.names.len(), seed, cycle);
            }
            assigned.insert(system_id, self.names[order[offset]].clone());
        }
        assigned
    }
}

#[derive(Debug, Error)]
pub enum StellarisStarNameError {
    #[error("ClauseScript parse failed: {0}")]
    Parse(#[from] ParseError),
    #[error("Stellaris name corpus has no top-level `star_names` pool")]
    MissingPool,
    #[error("Stellaris name corpus has more than one top-level `star_names` pool")]
    DuplicatePool,
    #[error("Stellaris `star_names` must be a braced scalar array")]
    InvalidPoolShape,
    #[error("Stellaris `star_names` contains a non-scalar entry")]
    NonScalarEntry,
    #[error("Stellaris `star_names` contains no non-empty names")]
    EmptyPool,
}

pub fn parse_stellaris_star_name_catalog(
    source: &[u8],
) -> Result<StellarisStarNameCatalog, StellarisStarNameError> {
    let document = parse_raw_document(source)?;
    let RawValue::Block(root) = document.root else {
        return Err(StellarisStarNameError::MissingPool);
    };
    let mut pools = root
        .properties
        .iter()
        .filter(|property| property.key.text == "star_names");
    let pool = pools.next().ok_or(StellarisStarNameError::MissingPool)?;
    if pools.next().is_some() {
        return Err(StellarisStarNameError::DuplicatePool);
    }

    let array = match &pool.value {
        RawValue::Array(array) => array,
        RawValue::Block(block) if block.properties.is_empty() => block
            .tail
            .as_ref()
            .ok_or(StellarisStarNameError::InvalidPoolShape)?,
        _ => return Err(StellarisStarNameError::InvalidPoolShape),
    };
    let names = scalar_names(array)?;
    if names.is_empty() {
        return Err(StellarisStarNameError::EmptyPool);
    }
    Ok(StellarisStarNameCatalog { names })
}

fn scalar_names(array: &RawArray) -> Result<Vec<String>, StellarisStarNameError> {
    array
        .items
        .iter()
        .map(|item| match item {
            RawValue::Scalar(scalar) if !scalar.text.trim().is_empty() => Ok(scalar.text.clone()),
            RawValue::Scalar(_) => Err(StellarisStarNameError::EmptyPool),
            _ => Err(StellarisStarNameError::NonScalarEntry),
        })
        .collect()
}

fn shuffled_indices(len: usize, seed: u64, cycle: u64) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..len).collect();
    let mut state = seed ^ cycle.wrapping_mul(0xD1B5_4A32_D192_ED03);
    for cursor in (1..len).rev() {
        let random = splitmix64(&mut state);
        let swap_with = (random % (cursor as u64 + 1)) as usize;
        indices.swap(cursor, swap_with);
    }
    indices
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_and_unquoted_stellaris_star_names_in_authored_order() {
        let source = br#"
            other_pool = { ignored }
            star_names = {
                Sol
                "Alpha Centauri"
                Sirius # comments are ignored by the ClauseScript parser
            }
        "#;
        let catalog = parse_stellaris_star_name_catalog(source).expect("catalog");
        assert_eq!(catalog.names(), ["Sol", "Alpha Centauri", "Sirius"]);
    }

    #[test]
    fn assignment_is_seeded_and_exhausts_a_cycle_before_reuse() {
        let catalog =
            parse_stellaris_star_name_catalog(b"star_names = { A B C }").expect("catalog");
        let first = catalog.assign_to_systems(42, 0..3);
        let repeat = catalog.assign_to_systems(42, 0..3);
        assert_eq!(first, repeat);
        let mut values: Vec<_> = first.values().cloned().collect();
        values.sort();
        assert_eq!(values, ["A", "B", "C"]);

        let overflow = catalog.assign_to_systems(42, 0..4);
        assert_eq!(overflow.len(), 4);
        assert!(["A", "B", "C"].contains(&overflow[&3].as_str()));
    }
}
