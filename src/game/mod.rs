pub mod board;
pub mod piece;
pub mod player;
pub use board::Board;
pub use piece::{Color, Height, Piece, Shape, Surface};
pub use player::Player;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub available_pieces: Vec<Piece>,
    pub current_player: Player,
}

impl Game {
    pub fn new() -> Self {
        let available_pieces = Game::create_pieces();
        Game {
            board: Board::new(),
            available_pieces,
            current_player: Player::Player1,
        }
    }

    pub fn create_pieces() -> Vec<Piece> {
        let mut pieces = Vec::new();
        for &color in &[Color::Black, Color::White] {
            for &shape in &[Shape::Round, Shape::Square] {
                for &height in &[Height::Tall, Height::Short] {
                    for &surface in &[Surface::Hollow, Surface::Solid] {
                        pieces.push(Piece::new(color, shape, height, surface));
                    }
                }
            }
        }
        pieces
    }

    pub fn play_turn(&mut self, row: usize, col: usize, piece_index: usize) -> Result<(), String> {
        let piece = self.available_pieces.remove(piece_index);
        self.board.place_piece(row, col, piece)?;

        if self.board.check_win() {
            println!("{:?} wins!", self.current_player);
            return Ok(());
        }

        Ok(())
    }

    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    pub fn is_game_over(&self) -> bool {
        self.board.check_win() || self.available_pieces.is_empty()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
