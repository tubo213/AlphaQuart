use crate::game::Game;
use crate::policies::policy::{Action, Policy};
use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
pub struct GreedyRandomPolicy {}

impl Policy for GreedyRandomPolicy {
    fn new() -> Self {
        GreedyRandomPolicy {}
    }

    fn action(&self, game: &Game) -> Action {
        let mut rng = thread_rng();
        let available_positions: Vec<(usize, usize)> = game.board.available_positions();
        // 利用可能な位置がない場合のエラーチェック
        if available_positions.is_empty() {
            panic!("No available moves left.");
        }

        let winning_cell = game.board.find_winning_cell(game.selected_piece);

        if winning_cell.is_some() {
            // 勝利する手がある場合は、その手を返す
            let position = winning_cell.unwrap();
            // 渡すpieceはランダム
            let piece_index = Some(rng.gen_range(0..game.available_pieces.len()));
            return Action {
                row: position.0,
                col: position.1,
                piece_index: piece_index,
            };
        } else {
            // 勝利する手がない場合は、置いて、渡したときに負けない手を返す
            for cand_position in available_positions.iter() {
                for piece in game.available_pieces.iter() {
                    // 一手進めたゲームを作成
                    let mut game_copy = game.clone();
                    let piece_index = game_copy.available_pieces.iter().position(|&x| x == *piece);
                    game_copy
                        .play_turn(cand_position.0, cand_position.1, piece_index)
                        .unwrap();
                    let winning_cell = game_copy.board.find_winning_cell(*piece);
                    // 負けない手がある場合は、その手を返す
                    if winning_cell.is_none() {
                        return Action {
                            row: cand_position.0,
                            col: cand_position.1,
                            piece_index: piece_index,
                        };
                    }
                }
            }
        }

        // どの手も負ける場合は、ランダムな手を返す
        let piece_index: Option<usize>;
        if game.available_pieces.is_empty() {
            piece_index = None;
        } else {
            piece_index = Some(rng.gen_range(0..game.available_pieces.len()));
        }
        Action {
            row: available_positions[0].0,
            col: available_positions[0].1,
            piece_index: piece_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::greedy_random_policy::GreedyRandomPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_greedy_random_policy_action() {
        test_policy_action(GreedyRandomPolicy::new());
    }

    #[test]
    fn test_greedy_random_policy_game_progression() {
        test_policy_game_progression(GreedyRandomPolicy::new());
    }

    #[test]
    fn test_greedy_random_policy_no_available_positions() {
        test_policy_no_available_positions(GreedyRandomPolicy::new());
    }

    #[test]
    fn test_greedy_random_policy_no_available_pieces() {
        test_policy_no_available_pieces(GreedyRandomPolicy::new());
    }
}
