use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, )]
pub struct Color{
    /// represented as `rgba(r, g, b, a)`
    color: String,
}
impl Color{
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self{
        Self{
            color: "rgba(".to_string() + &r.to_string() + ", " + &g.to_string() + ", " + &b.to_string() + ", " + &a.to_string() + ")",
        }
    }
}
impl From<[u8; 4]> for Color{
    fn from(from: [u8; 4]) -> Self {
        Self::new(from[0], from[1], from[2], from[3])
    }
}
impl TryFrom<Color> for [u8; 4]{
    type Error = ParseIntError;

    fn try_from(value: Color) -> Result<Self, Self::Error> {
        let mut out = [0; 4];
        for (index, num) in value.color.chars()
            .filter(|char| char.is_numeric() || *char == ',')
            .collect::<String>()
            .split(',')
            .enumerate(){
            out[index] = u8::from_str(num)?;
        }
        Ok(out)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ColorBuiltIn{
    Black,
    White,
    SeeThrough,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
}
impl From<ColorBuiltIn> for Color{
    fn from(from: ColorBuiltIn) -> Self {
        match from{
            ColorBuiltIn::Black => Color::new(0, 0, 0, 255),
            ColorBuiltIn::White => Color::new(255, 255, 255, 255),
            ColorBuiltIn::SeeThrough => Color::new(0, 0, 0, 0),
            ColorBuiltIn::Red => Color::new(255, 0, 0 , 255),
            ColorBuiltIn::Green => Color::new(0, 255, 0, 255),
            ColorBuiltIn::Blue => Color::new(0, 0, 255, 255),
            ColorBuiltIn::Yellow => Color::new(255, 255, 0, 255),
            ColorBuiltIn::Magenta => Color::new(255, 0, 255, 255),
        }
    }
}
