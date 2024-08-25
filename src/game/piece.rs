use serde::{Deserialize, Serialize};

// Define traits as simple integers
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub color: u8,  // 0 for Black, 1 for White
    pub shape: u8,  // 0 for Round, 1 for Square
    pub height: u8, // 0 for Tall, 1 for Short
    pub surface: u8, // 0 for Hollow, 1 for Solid
}

impl Piece {
    pub fn new(color: u8, shape: u8, height: u8, surface: u8) -> Self {
        Piece {
            color,
            shape,
            height,
            surface,
        }
    }
}
