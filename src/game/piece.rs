use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Round,
    Square,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Height {
    Tall,
    Short,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Surface {
    Hollow,
    Solid,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub color: Color,
    pub shape: Shape,
    pub height: Height,
    pub surface: Surface,
}

impl Piece {
    pub fn new(color: Color, shape: Shape, height: Height, surface: Surface) -> Self {
        Piece {
            color,
            shape,
            height,
            surface,
        }
    }
}
