use serde::ser::SerializeStruct;
use serde::Serializer;
use serde::{Deserialize, Serialize};

// 16ビットで表現される Piece
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Piece(u16);

impl Piece {
    pub fn new(color: u8, shape: u8, height: u8, surface: u8) -> Self {
        let mut piece = 0u16;
        piece |= (color as u16) << 0; // 色は下位ビットから1ビット目
        piece |= (shape as u16) << 1; // 形は下位ビットから2ビット目
        piece |= (height as u16) << 2; // 高さは下位ビットから3ビット目
        piece |= (surface as u16) << 3; // 表面は下位ビットから4ビット目
        Piece(piece)
    }

    // 各属性のゲッター
    pub fn color(&self) -> u8 {
        (self.0 & 0b0001) as u8
    }

    pub fn shape(&self) -> u8 {
        ((self.0 >> 1) & 0b0001) as u8
    }

    pub fn height(&self) -> u8 {
        ((self.0 >> 2) & 0b0001) as u8
    }

    pub fn surface(&self) -> u8 {
        ((self.0 >> 3) & 0b0001) as u8
    }
}

impl Serialize for Piece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Piece", 4)?;
        state.serialize_field("color", &self.color())?;
        state.serialize_field("shape", &self.shape())?;
        state.serialize_field("height", &self.height())?;
        state.serialize_field("surface", &self.surface())?;
        state.end()
    }
}
