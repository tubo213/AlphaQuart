pub mod board;
pub mod piece;
pub mod player;
pub use board::Board;
pub use piece::Piece;
pub use player::Player;

use rand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub board: Board,
    pub available_pieces: Vec<Piece>,
    pub selected_piece: Piece,
    pub current_player: Player,
}

impl Game {
    pub fn new() -> Self {
        let mut available_pieces = Game::create_pieces();
        // 最初のターンはどのpieceを選んでも同じなので、ランダムに最初のpieceを選ぶ
        let selected_piece =
            available_pieces.remove(rand::random::<usize>() % available_pieces.len());
        Game {
            board: Board::new(),
            available_pieces,
            selected_piece,
            current_player: Player::Player1,
        }
    }

    pub fn create_pieces() -> Vec<Piece> {
        let mut pieces = Vec::new();
        for color in 0..2 {
            // Loop over colors 0 (Black), 1 (White)
            for shape in 0..2 {
                // Loop over shapes 0 (Round), 1 (Square)
                for height in 0..2 {
                    // Loop over heights 0 (Tall), 1 (Short)
                    for surface in 0..2 {
                        // Loop over surfaces 0 (Hollow), 1 (Solid)
                        pieces.push(Piece::new(
                            color as u8,
                            shape as u8,
                            height as u8,
                            surface as u8,
                        ));
                    }
                }
            }
        }
        pieces
    }

    pub fn play_turn(
        &mut self,
        row: usize,
        col: usize,
        piece_index: Option<usize>,
    ) -> Result<(), String> {
        // selected_pieceを置く
        self.board.place_piece(row, col, self.selected_piece)?;

        // 選ばれたpieceをavailable_piecesから取り除く
        if piece_index.is_some() {
            let piece_index = piece_index.unwrap();
            self.selected_piece = self.available_pieces.remove(piece_index);
        }

        // ターンが終了したら、current_playerを切り替える
        self.switch_player();

        Ok(())
    }

    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    pub fn is_game_over(&self) -> bool {
        self.board.check_win() || self.board.is_full()
    }

    // 勝者がいる場合はSome(Player)を返し、引き分けの場合はNoneを返す
    // is_game_over()でゲームが終了しているかを確認してから呼び出すこと
    pub fn judge_winner(&self) -> Option<Player> {
        if self.board.check_win() {
            match self.current_player {
                // 直前のプレイヤーが勝者
                Player::Player1 => Some(Player::Player2),
                Player::Player2 => Some(Player::Player1),
            }
        } else {
            None
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
