use serde::{Deserialize, Serialize};

/// A space for the board.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Space {
    /// Not a valid space
    Invalid,
    /// A normal space
    Normal,
    /// A goal space for a seat
    Goal(u64),
}
impl Default for Space{
    fn default() -> Self {
        Self::Normal
    }
}
