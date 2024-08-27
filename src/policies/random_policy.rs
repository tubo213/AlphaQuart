use crate::game::Game;
use crate::policies::policy::{Action, Policy};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
pub struct RandomPolicy {}

impl Policy for RandomPolicy {
    fn new() -> Self {
        RandomPolicy {}
    }

    fn action(&self, game: &Game) -> Action {
        let mut rng = thread_rng();

        // 利用可能な位置を取得する
        let available_positions: Vec<(usize, usize)> = game.board.available_positions();

        // 利用可能な位置がない場合のエラーチェック
        if available_positions.is_empty() {
            panic!("No available moves left.");
        }

        let piece_index: Option<usize>;
        if game.available_pieces.is_empty() {
            piece_index = None;
        } else {
            // ランダムなピースを選ぶ
            piece_index = Some(rng.gen_range(0..game.available_pieces.len()));
        }

        // ランダムな位置を選ぶ
        let position = available_positions
            .choose(&mut rng)
            .expect("No available positions found.");

        Action {
            row: position.0,
            col: position.1,
            piece_index: piece_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::random_policy::RandomPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_random_policy_action() {
        test_policy_action(RandomPolicy::new());
    }

    #[test]
    fn test_random_policy_game_progression() {
        test_policy_game_progression(RandomPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_positions() {
        test_policy_no_available_positions(RandomPolicy::new());
    }

    #[test]
    fn test_random_policy_no_available_pieces() {
        test_policy_no_available_pieces(RandomPolicy::new());
    }
}
