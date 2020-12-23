use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Shape{
    Square,
    Circle,
    /// bar girth is size * ratio
    Plus{ ratio: f64 },
    /// width is height * ratio
    VerticalBar{ ratio: f64 },
    /// height is width * ratio
    HorizontalBar{ ratio: f64 },
}
