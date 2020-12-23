use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game::ruleset::starting_positions::piece_limit::PieceLimit;

/// The alteration for placement
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum AlternationType {
    /// Players alternate placing per_turn_count pieces.
    TurnsCount {
        /// Must be > 0
        per_turn_count: usize
    },
    /// Players alternate placing per_turn_points points.
    /// Requires piece_limits to contain a point limit.
    TurnsPoints {
        per_turn_points: usize,
        hard_limit: bool,
    },
    /// The player with the lowest total points places, first seat places on ties.
    /// Must have points limit set
    Points,
    /// Players place their whole side on their turn.
    WholePlacement,
    /// Players place their pieces hidden to each other.
    Hidden,
}
impl AlternationType {
    pub fn verify(&self, piece_limits: &HashSet<PieceLimit>) -> AlterationTypeResult<()> {
        match self {
            AlternationType::TurnsCount { per_turn_count } => if *per_turn_count == 0 {
                return Err(AlterationTypeError::CountIs0);
            },
            AlternationType::TurnsPoints { .. } | AlternationType::Points => {
                if let AlternationType::TurnsPoints { per_turn_points, hard_limit: _ } = self {
                    if *per_turn_points == 0 {
                        return Err(AlterationTypeError::PerTurnPointsIs0);
                    }
                }
                if !piece_limits.contains(&PieceLimit::PointLimit { point_values: Default::default(), point_limit: Default::default() }) {
                    return Err(AlterationTypeError::NoPointLimitForTurnsPoints);
                }
            },
            AlternationType::WholePlacement | AlternationType::Hidden => {},
        }
        Ok(())
    }
}
pub type AlterationTypeResult<T> = Result<T, AlterationTypeError>;
#[derive(Copy, Clone, Debug)]
pub enum AlterationTypeError {
    CountIs0,
    PerTurnPointsIs0,
    NoPointLimitForTurnsPoints,
}
impl Display for AlterationTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for AlterationTypeError {}
