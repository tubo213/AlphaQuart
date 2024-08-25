use crate::game::Game;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub row: usize,
    pub col: usize,
    pub piece_index: usize,
}

pub trait Policy {
    fn new() -> Self
    where
        Self: Sized;

    /// CPUが次の手を決定するためのメソッド
    fn action(&self, game: &Game) -> Action;
}
