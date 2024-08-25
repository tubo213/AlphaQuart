use super::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub grid: [[Option<Piece>; 4]; 4],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; 4]; 4],
        }
    }

    pub fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), String> {
        if self.grid[row][col].is_some() {
            Err("The cell is already occupied.".to_string())
        } else {
            self.grid[row][col] = Some(piece);
            Ok(())
        }
    }

    pub fn check_win(&self) -> bool {
        for i in 0..4 {
            if self.check_row(i) || self.check_col(i) {
                return true;
            }
        }
        self.check_diagonals()
    }

    fn check_row(&self, row: usize) -> bool {
        self.check_line(
            self.grid[row][0],
            self.grid[row][1],
            self.grid[row][2],
            self.grid[row][3],
        )
    }

    fn check_col(&self, col: usize) -> bool {
        self.check_line(
            self.grid[0][col],
            self.grid[1][col],
            self.grid[2][col],
            self.grid[3][col],
        )
    }

    fn check_diagonals(&self) -> bool {
        self.check_line(
            self.grid[0][0],
            self.grid[1][1],
            self.grid[2][2],
            self.grid[3][3],
        ) || self.check_line(
            self.grid[0][3],
            self.grid[1][2],
            self.grid[2][1],
            self.grid[3][0],
        )
    }

    fn check_line(
        &self,
        a: Option<Piece>,
        b: Option<Piece>,
        c: Option<Piece>,
        d: Option<Piece>,
    ) -> bool {
        match (a, b, c, d) {
            (Some(p1), Some(p2), Some(p3), Some(p4)) => {
                let all_same_color = p1.color == p2.color && p2.color == p3.color && p3.color == p4.color;
                let all_same_shape = p1.shape == p2.shape && p2.shape == p3.shape && p3.shape == p4.shape;
                let all_same_height = p1.height == p2.height && p2.height == p3.height && p3.height == p4.height;
                let all_same_surface = p1.surface == p2.surface && p2.surface == p3.surface && p3.surface == p4.surface;

                all_same_color || all_same_shape || all_same_height || all_same_surface
            }
            _ => false,
        }
    }
}
