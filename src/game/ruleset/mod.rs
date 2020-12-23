use core::fmt;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::game::ruleset::board_type::{BoardType, BoardTypeVerifyError};
use crate::game::ruleset::color::Color;
use crate::game::ruleset::piece_definition::{PieceDefinition, PieceDefinitionError};
use crate::game::ruleset::RulesetError::{DuplicateColor, DuplicateSeatInAllies, SeatsCountInvalid};
use crate::game::ruleset::starting_positions::{StartingPositions, StartingPositionsError};
use crate::game::ruleset::victory_condition::{VictoryCondition, VictoryConditionError};

pub mod starting_positions;
pub mod board_type;

pub mod color;
pub mod piece_definition;
pub mod shape;
pub mod standard;
pub mod victory_condition;

/// The ruleset for a game of Kapto
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ruleset {
    /// The name of the ruleset
    pub name: String,
    /// Number of seats in the game
    pub seats: u64,
    /// Allied seats, a single seat may not appear more than once
    pub allies: Vec<HashSet<u64>>,
    /// Colors for each seat, must be unique
    pub seat_colors: Vec<Color>,
    /// All possible pieces
    pub pieces: Vec<PieceDefinition>,
    /// The type of board to use
    pub board_type: BoardType,
    /// Starting position type to use
    pub starting_positions: StartingPositions,
    /// How to win the game
    /// At least one must be set
    pub victory_conditions: HashSet<VictoryCondition>,
}
impl Ruleset {
    pub fn verify(&self) -> RulesetResult<()> {
        // Verify seats
        if self.seats < 2 {
            return Err(SeatsCountInvalid(self.seats));
        }

        // Verify allies
        let mut allies_found = HashSet::new();
        for set in &self.allies {
            for &seat in set {
                if !allies_found.insert(seat) {
                    return Err(DuplicateSeatInAllies(seat));
                }
            }
        }

        // Verify seat_colors
        if self.seats != self.seat_colors.len() as u64 {
            return Err(RulesetError::NotEnoughColorsSet(self.seat_colors.len()));
        }
        let mut colors = HashSet::with_capacity(self.seats as usize);
        for color in &self.seat_colors {
            if !colors.insert(color) {
                return Err(DuplicateColor(color.clone()));
            }
        }

        // Verify pieces
        let mut pieces_set = HashSet::with_capacity(self.pieces.len());
        for piece in self.pieces.iter() {
            piece.verify()?;
            if !pieces_set.insert(piece) {
                return Err(RulesetError::PieceDuplicated(piece.clone()));
            }
        }

        // Verify board_type
        self.board_type.verify(self)?;

        // Verify starting_positions
        self.starting_positions.verify(self)?;

        // Verify victory_conditions
        for victory_condition in self.victory_conditions.iter() {
            victory_condition.verify(self)?;
        }
        Ok(())
    }

    pub fn get_piece(&self, index: usize) -> Option<&PieceDefinition> {
        self.pieces.get(index)
    }
}
pub type RulesetResult<T> = Result<T, RulesetError>;
#[derive(Clone, Debug)]
pub enum RulesetError {
    SeatsCountInvalid(u64),
    DuplicateSeatInAllies(u64),
    NotEnoughColorsSet(usize),
    DuplicateColor(Color),
    PieceDuplicated(PieceDefinition),
    PieceDefinitionError(PieceDefinitionError),
    BoardTypeVerifyError(BoardTypeVerifyError),
    StartingPositionsError(StartingPositionsError),
    VictoryConditionError(VictoryConditionError),
}
impl Display for RulesetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for RulesetError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            Self::SeatsCountInvalid(_) => None,
            Self::DuplicateSeatInAllies(_) => None,
            Self::NotEnoughColorsSet(_) => None,
            Self::DuplicateColor(_) => None,
            Self::PieceDuplicated(_) => None,
            Self::PieceDefinitionError(error) => Some(error),
            Self::BoardTypeVerifyError(error) => Some(error),
            Self::StartingPositionsError(error) => Some(error),
            Self::VictoryConditionError(error) => Some(error),
        }
    }
}
impl From<PieceDefinitionError> for RulesetError {
    fn from(from: PieceDefinitionError) -> Self {
        Self::PieceDefinitionError(from)
    }
}
impl From<BoardTypeVerifyError> for RulesetError {
    fn from(from: BoardTypeVerifyError) -> Self {
        Self::BoardTypeVerifyError(from)
    }
}
impl From<StartingPositionsError> for RulesetError {
    fn from(from: StartingPositionsError) -> Self {
        Self::StartingPositionsError(from)
    }
}
impl From<VictoryConditionError> for RulesetError {
    fn from(from: VictoryConditionError) -> Self {
        Self::VictoryConditionError(from)
    }
}
