use crate::game::Game;
use crate::game::action::Action;

pub trait Policy {
    fn new() -> Self
    where
        Self: Sized;

    /// CPUが次の手を決定するためのメソッド
    fn action(&self, game: &Game) -> Action;
}
