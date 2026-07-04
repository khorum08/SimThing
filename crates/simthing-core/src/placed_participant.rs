//! Sealed proof that a participant has a validated spatial binding at a structural coordinate.
//!
//! [`StructuralCoord`] is a coordinate value, not a placement proof. [`PlacedParticipant`] is minted
//! only after validation against a structural grid binding table (boundary admission path).

use crate::{SimThingId, StructuralCoord};

/// Sealed proof that a participant is placed in the structural tree at a validated coordinate.
///
/// External crates cannot construct this type directly:
///
/// ```compile_fail
/// use simthing_core::{PlacedParticipant, SimThingId, StructuralCoord};
/// let _ = PlacedParticipant {
///     participant: SimThingId::from_session_raw(1),
///     coord: StructuralCoord::new(0, 0),
///     _private: (),
/// };
/// ```
///
/// External crates cannot mint a placement proof via the internal minter:
///
/// ```compile_fail
/// use simthing_core::{PlacedParticipant, SimThingId, StructuralCoord};
/// let _ = PlacedParticipant::from_validated_spatial_binding(
///     SimThingId::from_session_raw(1),
///     StructuralCoord::new(0, 0),
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlacedParticipant {
    participant: SimThingId,
    coord: StructuralCoord,
    _private: (),
}

impl PlacedParticipant {
    pub fn participant(self) -> SimThingId {
        self.participant
    }

    pub fn coord(self) -> StructuralCoord {
        self.coord
    }

    pub(crate) fn from_validated_spatial_binding(
        participant: SimThingId,
        coord: StructuralCoord,
    ) -> Self {
        Self {
            participant,
            coord,
            _private: (),
        }
    }
}

/// One structural grid placement keyed by participant location id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralGridPlacement<'a> {
    pub location_id: &'a str,
    pub coord: StructuralCoord,
}

/// Validation failure when minting [`PlacedParticipant`] proofs from structural grid metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedParticipantValidationError {
    pub message: String,
}

impl PlacedParticipantValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for PlacedParticipantValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "placed participant validation error: {}", self.message)
    }
}

impl std::error::Error for PlacedParticipantValidationError {}

/// Validate each location id has exactly one structural placement (no proof minting).
pub fn validate_location_ids_have_structural_placements<'a>(
    participant_location_ids: &[&'a str],
    placements: &'a [StructuralGridPlacement<'a>],
) -> Result<(), PlacedParticipantValidationError> {
    let mut placed: std::collections::BTreeMap<&str, StructuralCoord> =
        std::collections::BTreeMap::new();
    for placement in placements {
        if placed
            .insert(placement.location_id, placement.coord)
            .is_some()
        {
            return Err(PlacedParticipantValidationError::new(format!(
                "duplicate structural grid placement for Location `{}`",
                placement.location_id
            )));
        }
    }
    for location_id in participant_location_ids {
        if !placed.contains_key(location_id) {
            return Err(PlacedParticipantValidationError::new(format!(
                "Location participant `{location_id}` has no structural grid placement \
                 (STEAD/Mapping: Location participation is spatially indexed through \
                 structural grid metadata, never render metadata)"
            )));
        }
    }
    Ok(())
}

/// Validate each `(SimThingId, location_id)` pair against a structural binding table and mint proofs.
///
/// Every participant must have exactly one structural placement in `placements`; duplicate location
/// keys in the table are rejected.
pub fn validate_and_mint_placed_participants_by_location_id<'a>(
    participants: &[(SimThingId, &'a str)],
    placements: &'a [StructuralGridPlacement<'a>],
) -> Result<Vec<PlacedParticipant>, PlacedParticipantValidationError> {
    validate_location_ids_have_structural_placements(
        &participants.iter().map(|(_, id)| *id).collect::<Vec<_>>(),
        placements,
    )?;
    let mut placed: std::collections::BTreeMap<&str, StructuralCoord> =
        std::collections::BTreeMap::new();
    for placement in placements {
        placed.insert(placement.location_id, placement.coord);
    }
    let mut out = Vec::with_capacity(participants.len());
    for (participant, location_id) in participants {
        let coord = placed.get(location_id).copied().expect("validated above");
        out.push(PlacedParticipant::from_validated_spatial_binding(
            *participant,
            coord,
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{
        validate_and_mint_placed_participants_by_location_id,
        validate_location_ids_have_structural_placements, PlacedParticipantValidationError,
        StructuralGridPlacement,
    };
    use crate::{SimThingId, StructuralCoord};

    #[test]
    fn mints_proofs_for_valid_binding_table() {
        let placements = [
            StructuralGridPlacement {
                location_id: "loc_a",
                coord: StructuralCoord::new(1, 2),
            },
            StructuralGridPlacement {
                location_id: "loc_b",
                coord: StructuralCoord::new(3, 4),
            },
        ];
        let participants = [
            (SimThingId::from_session_raw(10), "loc_a"),
            (SimThingId::from_session_raw(11), "loc_b"),
        ];
        let proofs =
            validate_and_mint_placed_participants_by_location_id(&participants, &placements)
                .expect("valid table");
        assert_eq!(proofs.len(), 2);
        assert_eq!(proofs[0].participant(), SimThingId::from_session_raw(10));
        assert_eq!(proofs[0].coord(), StructuralCoord::new(1, 2));
        assert_eq!(proofs[1].participant(), SimThingId::from_session_raw(11));
        assert_eq!(proofs[1].coord(), StructuralCoord::new(3, 4));
    }

}
