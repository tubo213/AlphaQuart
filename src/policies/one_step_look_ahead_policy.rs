use crate::game::Game;
use crate::policies::policy::{Action, Policy};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
pub struct OneStepLookAheadPolicy {}

impl Policy for OneStepLookAheadPolicy {
    fn new() -> Self {
        OneStepLookAheadPolicy {}
    }

    fn action(&self, game: &Game) -> Action {
        let mut rng = thread_rng();
        let winning_cell = game.board.find_winning_cell(game.selected_piece);
        let mut available_positions: Vec<(usize, usize)> = game.board.available_positions();
        // 利用可能な位置がない場合のエラーチェック
        if available_positions.is_empty() {
            panic!("No available moves left.");
        }
        available_positions.shuffle(&mut rng);

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
        } else if game.available_pieces.is_empty() {
            // 勝利できる手がなくて，渡すpieceがない場合は，ランダムな場所に置く
            let position = available_positions[0];
            return Action {
                row: position.0,
                col: position.1,
                piece_index: None,
            };
        } else {
            // 勝利する手がない場合は、置いて、渡したときに負けない手を返す
            // available_piecesをシャッフル
            let mut available_pieces = game.available_pieces.clone();
            available_pieces.shuffle(&mut rng);

            // positionとpieceの組合せで、負けない手を全探索
            for cand_position in available_positions.iter() {
                for piece in available_pieces.iter() {
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
        let position = available_positions[0];
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
    use crate::game::Piece;
    use crate::policies::one_step_look_ahead_policy::OneStepLookAheadPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_one_step_look_ahead_policy_wins_when_possible() {
        // 勝てる盤面を作成
        let mut game = Game::new();
        let piece1 = Piece::new(0, 1, 0, 0);
        let piece2 = Piece::new(0, 1, 0, 1);
        let piece3 = Piece::new(0, 1, 1, 0);
        let piece4 = Piece::new(1, 1, 1, 1);
        game.board.place_piece(0, 0, piece1).unwrap();
        game.board.place_piece(0, 1, piece2).unwrap();
        game.board.place_piece(0, 2, piece3).unwrap();
        game.selected_piece = piece4;

        // 勝利する手を選ぶかチェック
        let policy = OneStepLookAheadPolicy::new();
        let action = policy.action(&game);
        // actionはrow=0, col=3の手を選ぶはず
        assert_eq!(action.row, 0, "行が正しい");
        assert_eq!(action.col, 3, "列が正しい");
    }

    #[test]
    fn test_one_step_look_ahead_policy_action() {
        test_policy_action(OneStepLookAheadPolicy::new());
    }

    #[test]
    fn test_one_step_look_ahead_policy_game_progression() {
        test_policy_game_progression(OneStepLookAheadPolicy::new());
    }

    #[test]
    fn test_one_step_look_ahead_policy_no_available_positions() {
        test_policy_no_available_positions(OneStepLookAheadPolicy::new());
    }

    #[test]
    fn test_one_step_look_ahead_policy_no_available_pieces() {
        test_policy_no_available_pieces(OneStepLookAheadPolicy::new());
    }
}
