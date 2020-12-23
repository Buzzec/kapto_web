use core::fmt::{Debug, Display, Formatter};
use core::fmt;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};
use core::result::Result;
use std::collections::HashSet;
use std::error::Error;

use serde::{Deserialize, Serialize};

use placement_area::PlacementArea;

use crate::game::coordinate::{Coordinate, flip_coordinate, rotate_coordinate};
use crate::game::ruleset::board_type::space::Space;
use crate::game::ruleset::piece_definition::PieceDefinition;
use crate::game::ruleset::Ruleset;
use crate::game::ruleset::starting_positions::alteration_type::{AlterationTypeError, AlternationType};
use crate::game::ruleset::starting_positions::piece_limit::{PieceLimit, PieceLimitError};
use crate::game::ruleset::starting_positions::placement_area::PlacementAreaError;

pub mod alteration_type;
pub mod piece_limit;
pub mod placement_area;

/// Defines the starting positions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StartingPositions {
    /// Mirrored start positions, only defines a single side.
    /// Mirror will flip about horizontal center.
    /// Will error if overlapping.
    MirroredFlipped(Vec<Vec<Coordinate>>),
    /// Mirrored start positions, only defines a single side.
    /// Mirror will rotate.
    /// Will error if overlapping.
    MirroredRotated(Vec<Vec<Coordinate>>),
    /// Start positions for all seats.
    /// Will error if overlapping.
    /// All seats must be set.
    /// indexed by seat, piece, generic list of positions
    NotMirrored(Vec<Vec<Vec<Coordinate>>>),
    /// Players will alternate placing pieces.
    Placement {
        /// The seat to go first.
        first_seat: u64,
        /// The way turns are alternated.
        alternation_type: AlternationType,
        /// The valid placement area.
        placement_area: PlacementArea,
        /// The limitations on piece placement.
        piece_limits: HashSet<PieceLimit>,
    },
}
impl StartingPositions {
    fn verify_mirrored_flipped(piece_positions: &[Vec<Coordinate>], ruleset: &Ruleset) -> StartingPositionsResult<()> {
        // Tracks already used positions
        let mut found = HashSet::new();
        for (piece_index, positions) in piece_positions.iter().enumerate() {
            let piece = match ruleset.get_piece(piece_index) {
                None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                Some(piece) => piece,
            };
            for &position in positions {
                // Check already used positions and add to list
                if !found.insert(position) {
                    return Err(StartingPositionsError::DuplicatePosition {
                        piece: piece.clone(),
                        position,
                    });
                }

                match ruleset.board_type.get_space(position) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
                match ruleset.board_type.get_space(flip_coordinate(&ruleset.board_type, position)) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_mirrored_rotated(piece_positions: &[Vec<Coordinate>], ruleset: &Ruleset) -> StartingPositionsResult<()> {
        // Tracks already used positions
        let mut found = HashSet::new();
        for (piece_index, positions) in piece_positions.iter().enumerate() {
            let piece = match ruleset.get_piece(piece_index) {
                None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                Some(piece) => piece,
            };
            for &position in positions {
                // Check already used positions and add to list
                if !found.insert(position) {
                    return Err(StartingPositionsError::DuplicatePosition {
                        piece: piece.clone(),
                        position,
                    });
                }

                match ruleset.board_type.get_space(position) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
                match ruleset.board_type.get_space(rotate_coordinate(&ruleset.board_type, position)) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_not_mirrored(seat_piece_positions: &[Vec<Vec<Coordinate>>], ruleset: &Ruleset) -> StartingPositionsResult<()> {
        if seat_piece_positions.len() as u64 != ruleset.seats{
            return Err(StartingPositionsError::SeatNumberDoesNotMatch(seat_piece_positions.len()))
        }

        // Tracks already used positions
        let mut found = HashSet::new();
        for piece_positions in seat_piece_positions {
            for (piece_index, positions) in piece_positions.iter().enumerate() {
                let piece = match ruleset.get_piece(piece_index) {
                    None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                    Some(piece) => piece,
                };
                for &position in positions {
                    // Check already used positions and add to list
                    if !found.insert(position) {
                        return Err(StartingPositionsError::DuplicatePosition {
                            piece: piece.clone(),
                            position,
                        });
                    }

                    match ruleset.board_type.get_space(position) {
                        Space::Normal => {}
                        space => {
                            return Err(StartingPositionsError::InvalidPositionForBoard {
                                space,
                                piece: piece.clone(),
                                position,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_placement(_first_seat: u64, alternation_type: AlternationType, placement_area: &PlacementArea, piece_limits: &HashSet<PieceLimit>, ruleset: &Ruleset) -> Result<(), StartingPositionsError> {
        alternation_type.verify(piece_limits)?;
        placement_area.verify(ruleset)?;
        PieceLimit::verify(piece_limits, ruleset)?;
        Ok(())
    }

    pub fn verify(&self, ruleset: &Ruleset) -> StartingPositionsResult<()> {
        match self {
            StartingPositions::MirroredFlipped(self_data) => {
                Self::verify_mirrored_flipped(self_data, ruleset)
            }
            StartingPositions::MirroredRotated(self_data) => {
                Self::verify_mirrored_rotated(self_data, ruleset)
            }
            StartingPositions::NotMirrored(positions) => {
                Self::verify_not_mirrored(positions, ruleset)
            }
            StartingPositions::Placement {
                first_seat,
                alternation_type,
                placement_area,
                piece_limits,
            } => Self::verify_placement(
                *first_seat,
                *alternation_type,
                placement_area,
                piece_limits,
                ruleset,
            ),
        }
    }
}

pub type StartingPositionsResult<T> = Result<T, StartingPositionsError>;
#[derive(Clone, Debug)]
pub enum StartingPositionsError {
    /// Wrong number of seats
    SeatNumberDoesNotMatch(usize),
    /// Piece index was not found
    PieceIndexNotFound(usize),
    /// Position duplicate found
    DuplicatePosition {
        piece: PieceDefinition,
        position: Coordinate,
    },
    /// Position invalid
    InvalidPositionForBoard {
        space: Space,
        piece: PieceDefinition,
        position: Coordinate,
    },
    AlterationTypeError(AlterationTypeError),
    PlacementAreaError(PlacementAreaError),
    PieceLimitError(PieceLimitError),
}
impl Display for StartingPositionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for StartingPositionsError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            StartingPositionsError::SeatNumberDoesNotMatch(_) => None,
            StartingPositionsError::PieceIndexNotFound(_) => None,
            StartingPositionsError::DuplicatePosition { .. } => None,
            StartingPositionsError::InvalidPositionForBoard { .. } => None,
            StartingPositionsError::AlterationTypeError(error) => Some(error),
            StartingPositionsError::PlacementAreaError(error) => Some(error),
            StartingPositionsError::PieceLimitError(error) => Some(error),
        }
    }
}
impl From<AlterationTypeError> for StartingPositionsError {
    fn from(from: AlterationTypeError) -> Self {
        Self::AlterationTypeError(from)
    }
}
impl From<PlacementAreaError> for StartingPositionsError {
    fn from(from: PlacementAreaError) -> Self {
        Self::PlacementAreaError(from)
    }
}
impl From<PieceLimitError> for StartingPositionsError {
    fn from(from: PieceLimitError) -> Self {
        Self::PieceLimitError(from)
    }
}
