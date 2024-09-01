use crate::game::action::Action;
use crate::game::Game;
use crate::policies::policy::Policy;

#[derive(Clone)]
pub struct MCSPolicy {}

impl Policy for MCSPolicy {
    fn new() -> Self {
        MCSPolicy {}
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
    use crate::policies::mcs_policy::MCSPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_random_policy_action() {
        test_policy_action(MCSPolicy::new());
    }

    #[test]
    fn test_random_policy_game_progression() {
        test_policy_game_progression(MCSPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_positions() {
        test_policy_no_available_positions(MCSPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_pieces() {
        test_policy_no_available_pieces(MCSPolicy::new());
    }
}
