use core::cmp::{Eq, PartialEq};
use core::fmt::{Debug, Display, Formatter};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::mem::discriminant;
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::collections::HashSet;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::game::ruleset::piece_definition::PieceDefinition;
use crate::game::ruleset::Ruleset;

/// Limits for piece placement.
///
/// Hash, Eq, PartialEq are defined for the discriminant.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PieceLimit {
    /// Limit to the total count of pieces.
    TotalLimit {
        limit: usize,
    },
    /// Limit to each type of piece.
    TypeCountLimit {
        /// Maps from pieces index to limit
        limits: Vec<usize>,
    },
    /// Limit by point count and total points available.
    PointLimit {
        /// Must be set for all pieces.
        /// Maps from pieces index to points value
        point_values: Vec<usize>,
        /// The total limit for each side.
        point_limit: usize,
    },
}
impl PieceLimit {
    pub fn verify(self_set: &HashSet<Self>, ruleset: &Ruleset) -> PieceLimitResult<()> {
        for piece_limit in self_set {
            match piece_limit {
                PieceLimit::TotalLimit { limit } => if *limit == 0 {
                    return Err(PieceLimitError::LimitIs0);
                },
                PieceLimit::TypeCountLimit { limits } => {
                    if limits.len() != ruleset.pieces.len(){
                        return Err(PieceLimitError::PieceIndexMismatch(limits.len()));
                    }
                    for (index, &limit) in limits.iter().enumerate() {
                        if limit == 0 {
                            return Err(PieceLimitError::LimitIs0ForPiece(ruleset.pieces[index].clone()));
                        }
                    }
                },
                PieceLimit::PointLimit { point_values, point_limit } => {
                    if *point_limit == 0{
                        return Err(PieceLimitError::PointLimitIs0);
                    }
                    if point_values.len() != ruleset.pieces.len(){
                        return Err(PieceLimitError::PieceIndexMismatch(point_values.len()));
                    }
                    if !point_values.is_empty(){
                        let min_value = *point_values.iter().min().unwrap();
                        if min_value == 0{
                            return Err(PieceLimitError::PointsIs0ForPiece);
                        }
                        if min_value > *point_limit{
                            return Err(PieceLimitError::NoPieceFitsInPointLimit);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
impl Hash for PieceLimit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
impl PartialEq for PieceLimit {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self).eq(&discriminant(other))
    }
}
impl Eq for PieceLimit {}

pub type PieceLimitResult<T> = Result<T, PieceLimitError>;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceLimitError {
    LimitIs0,
    PieceIndexMismatch(usize),
    LimitIs0ForPiece(PieceDefinition),
    PointsIs0ForPiece,
    PointLimitIs0,
    NoPieceFitsInPointLimit,
}
impl Display for PieceLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PieceLimitError {}
