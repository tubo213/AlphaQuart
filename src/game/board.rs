use super::piece::Piece;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

// 勝利判定に使用するマスクを定数として定義
const WINNING_MASKS: [u16; 10] = [
    0x000F, 0x00F0, 0x0F00, 0xF000, // Rows
    0x1111, 0x2222, 0x4444, 0x8888, // Columns
    0x1248, 0x8421, // Diagonals
];

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Board {
    color_board: u16,
    shape_board: u16,
    height_board: u16,
    surface_board: u16,
    empty_cells: u16, // 空のセルを管理するためのビットボード
}

impl Board {
    pub fn new() -> Self {
        Board {
            color_board: 0,
            shape_board: 0,
            height_board: 0,
            surface_board: 0,
            empty_cells: 0xFFFF, // 全てのマスが空であることを示す（16ビットすべてが1）
        }
    }

    pub fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), String> {
        let position = 1 << (row * 4 + col); // 位置をビットシフトで表現

        // すでにピースが置かれているセルには置けない
        if self.empty_cells & position == 0 {
            return Err("Cell is already occupied".to_string());
        }

        // 空セルから除外
        self.empty_cells &= !position;

        // 各属性に対応するビットボードを更新
        if piece.color() == 0 {
            self.color_board |= position;
        }
        if piece.shape() == 0 {
            self.shape_board |= position;
        }
        if piece.height() == 0 {
            self.height_board |= position;
        }
        if piece.surface() == 0 {
            self.surface_board |= position;
        }

        Ok(())
    }

    pub fn check_win(&self) -> bool {
        for &mask in WINNING_MASKS.iter() {
            if self.check_line(mask) {
                return true;
            }
        }
        false
    }

    // ピースが置かれていないセルの位置を高速に取得
    pub fn available_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        let mut remaining = self.empty_cells;

        while remaining != 0 {
            let pos = remaining.trailing_zeros() as usize; // 最下位の1ビットの位置を取得
            positions.push((pos / 4, pos % 4)); // 行と列に変換
            remaining &= remaining - 1; // 最下位の1ビットをクリア
        }

        positions
    }

    // 勝利できるセルを探す
    pub fn find_winning_cell(&self, piece: Piece) -> Option<(usize, usize)> {
        // 空いているセルを取得
        let available_positions = self.available_positions();

        // 各属性のビットボードにおけるピースの属性
        let attributes = [
            (&self.color_board, piece.color()),
            (&self.shape_board, piece.shape()),
            (&self.height_board, piece.height()),
            (&self.surface_board, piece.surface()),
        ];

        for (row, col) in available_positions {
            let position = 1 << (row * 4 + col);

            for &(board, value) in &attributes {
                let temp_board = if value == 0 {
                    *board | position
                } else {
                    *board
                };

                // 勝利判定をビット演算で確認
                if WINNING_MASKS.iter().any(|&mask| temp_board & mask == mask) {
                    return Some((row, col)); // 勝利できる位置を返す
                }
            }
        }

        None // 勝利できる位置が存在しない場合
    }

    pub fn is_full(&self) -> bool {
        self.empty_cells == 0
    }

    pub fn grid(&self) -> [[Option<Piece>; 4]; 4] {
        let mut grid = [[None; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                let position = 1 << (row * 4 + col);

                if self.empty_cells & position == 0 {
                    // ピースが配置されている場合
                    let color = if self.color_board & position != 0 {
                        0 // color_boardにビットが立っている場合は0（黒）
                    } else {
                        1 // 立っていない場合は1（白）
                    };
                    let shape = if self.shape_board & position != 0 {
                        0 // shape_boardにビットが立っている場合は0（丸）
                    } else {
                        1 // 立っていない場合は1（四角）
                    };
                    let height = if self.height_board & position != 0 {
                        0 // height_boardにビットが立っている場合は0（高い）
                    } else {
                        1 // 立っていない場合は1（低い）
                    };
                    let surface = if self.surface_board & position != 0 {
                        0 // surface_boardにビットが立っている場合は0（穴あり）
                    } else {
                        1 // 立っていない場合は1（穴なし）
                    };
                    grid[row][col] = Some(Piece::new(color, shape, height, surface));
                }
            }
        }

        grid
    }

    fn check_line(&self, mask: u16) -> bool {
        // maskで指定されたラインが埋まっているかを判定する
        if self.empty_cells & mask != 0 {
            return false;
        }

        // 各属性のビットボードがすべて0またはすべて1かを確認
        let color_match = self.color_board & mask;
        let shape_match = self.shape_board & mask;
        let height_match = self.height_board & mask;
        let surface_match = self.surface_board & mask;

        // どれか一つの属性が揃っている場合に勝利
        color_match == 0
            || color_match == mask
            || shape_match == 0
            || shape_match == mask
            || height_match == 0
            || height_match == mask
            || surface_match == 0
            || surface_match == mask
    }
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Board", 1)?;
        state.serialize_field("grid", &self.grid())?;
        state.end()
    }
}
