use crate::game::Game;
use crate::policies::policy::{Action, Policy};

#[derive(Clone)]
pub struct MCTSPolicy {}

impl Policy for MCTSPolicy {
    fn new() -> Self {
        MCTSPolicy {}
    }

    fn action(&self, game: &Game) -> Action {
        Action {
            row: 0,
            col: 0,
            piece_index: Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::mcts_policy::MCTSPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_random_policy_action() {
        test_policy_action(MCTSPolicy::new());
    }

    #[test]
    fn test_random_policy_game_progression() {
        test_policy_game_progression(MCTSPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_positions() {
        test_policy_no_available_positions(MCTSPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_pieces() {
        test_policy_no_available_pieces(MCTSPolicy::new());
    }
}
