use crate::game::Game;
use crate::game::Piece;
use crate::policies::policy::{Action, Policy};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomPolicy {}

impl Policy for RandomPolicy {
    fn new() -> Self {
        RandomPolicy {}
    }

    fn action(&self, game: &Game) -> Action {
        let mut rng = thread_rng();

        // 利用可能な位置を取得する
        let available_positions: Vec<(usize, usize)> = (0..4)
            .flat_map(|row| (0..4).map(move |col| (row, col)))
            .filter(|&(row, col)| game.board.grid[row][col].is_none())
            .collect();

        // 利用可能な位置がない場合のエラーチェック
        if available_positions.is_empty() {
            panic!("No available moves left.");
        }

        // 利用可能なピースとそのインデックスをベクタに収集する
        let available_pieces_with_index: Vec<(usize, &Piece)> =
            game.available_pieces.iter().enumerate().collect();

        let piece_index: Option<usize>;
        if game.available_pieces.is_empty() {
            piece_index = None;
        } else {
            piece_index = Some(available_pieces_with_index.choose(&mut rng).unwrap().0);
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
