use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::game::coordinate::{Coordinate, flip_coordinate, rotate_coordinate};
use crate::game::ruleset::board_type::space::Space;
use crate::game::ruleset::Ruleset;

/// Placement area definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlacementArea {
    /// Players can place on half the board.
    Half,
    /// Players can place on a mirrored set of places.
    /// Mirroring will flip.
    /// Will error if overlapping.
    MirroredFlipped(HashSet<Coordinate>),
    /// Players can place on a mirrored set of places.
    /// Mirroring will rotate.
    /// Will error if overlapping.
    MirroredRotated(HashSet<Coordinate>),
    /// Players can place on a given set of places based on seat.
    /// Must be set for all seats.
    NonMirrored(Vec<HashSet<Coordinate>>),
}
impl PlacementArea {
    pub fn verify(&self, ruleset: &Ruleset) -> PlacementAreaResult<()> {
        match self {
            Self::Half => {},
            Self::MirroredFlipped(positions) | Self::MirroredRotated(positions) => {
                let func = if let Self::MirroredFlipped(_) = self { flip_coordinate } else { rotate_coordinate };
                let mut found = positions.clone();
                for &position in positions {
                    if position.row < 0 || position.row >= ruleset.board_type.rows() as i16 || position.column < 0 || position.column >= ruleset.board_type.columns() as i16 {
                        return Err(PlacementAreaError::PositionCannotPlace(Space::Invalid, position))
                    }
                    let opposite = func(&ruleset.board_type, position);
                    if !found.insert(position) || found.insert(opposite) {
                        return Err(PlacementAreaError::PositionCollision(position));
                    }
                }
            },
            Self::NonMirrored(seat_map) => {
                let mut found = HashSet::new();
                if seat_map.len() as u64 != ruleset.seats{
                    return Err(PlacementAreaError::InvalidSeatNumber(seat_map.len()));
                }
                for coordinate_set in seat_map {
                    for &coordinate in coordinate_set {
                        if !found.insert(coordinate) {
                            return Err(PlacementAreaError::PositionCollision(coordinate));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
pub type PlacementAreaResult<T> = Result<T, PlacementAreaError>;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlacementAreaError {
    PositionCannotPlace(Space, Coordinate),
    PositionCollision(Coordinate),
    InvalidSeatNumber(usize),
}
impl Display for PlacementAreaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PlacementAreaError {}
