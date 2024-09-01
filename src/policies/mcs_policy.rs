use crate::game::action::Action;
use crate::game::Game;
use crate::game::Player;
use crate::policies::one_step_look_ahead_policy::OneStepLookAheadPolicy;
use crate::policies::policy::Policy;

pub fn play_out(game: &Game, policy: &OneStepLookAheadPolicy) -> Option<Player> {
    let mut game_copy = game.clone();
    while !game_copy.is_game_over() {
        let action = policy.action(&game_copy);
        game_copy
            .play_turn(action.row, action.col, action.piece_index)
            .unwrap();
    }

    game_copy.judge_winner()
}

#[derive(Clone)]
pub struct MCSPolicy {
    num_playouts_per_action: usize,
    policy: OneStepLookAheadPolicy,
}

impl Policy for MCSPolicy {
    fn new() -> Self {
        MCSPolicy {
            num_playouts_per_action: 10,
            policy: OneStepLookAheadPolicy::new(),
        }
    }

    fn action(&self, game: &Game) -> Action {
        // 渡せるpieceがない場合は、OneStepLookAheadPolicyを使う
        if game.available_pieces.is_empty() {
            return self.policy.action(game);
        } else {
            // すべてのactionに対して、num_playouts_per_action回プレイアウトを行い、最も勝率が高いactionを選択
            let mut best_action: Option<Action> = None;
            let mut best_score = std::f64::MIN;

            for action in game.available_actions().iter() {
                let mut next_state = game.clone();
                let mut score = 0.0;
                next_state
                    .play_turn(action.row, action.col, action.piece_index)
                    .unwrap();
                for _ in 0..self.num_playouts_per_action {
                    // play out the game
                    let winner = play_out(&next_state, &self.policy);
                    if winner.is_some() {
                        if winner.unwrap() == game.current_player {
                            score += 1.0;
                        } else {
                            score -= 1.0;
                        }
                    }
                }

                score /= self.num_playouts_per_action as f64;
                if score > best_score {
                    best_score = score;
                    best_action = Some(action.clone());
                }
            }

            best_action.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::mcs_policy::MCSPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_mcs_policy_action() {
        test_policy_action(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_game_progression() {
        test_policy_game_progression(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_no_available_positions() {
        test_policy_no_available_positions(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_no_available_pieces() {
        test_policy_no_available_pieces(MCSPolicy::new());
    }
}
