#[cfg(test)]
mod tests {
    pub use quart_engine::game::board::Board;
    pub use quart_engine::game::piece::Piece;

    #[test]
    fn test_initial_board_state() {
        let board = Board::new();
        assert!(!board.check_win(), "新しいボードでは勝者がいないはず");
        assert_eq!(
            board.available_positions().len(),
            16,
            "初期状態では全てのポジションが利用可能であるべき"
        );
    }

    #[test]
    fn test_place_piece() {
        let mut board = Board::new();
        let piece = Piece::new(0, 1, 0, 1); // 色: 黒, 形: 四角, 高さ: 高い, 表面: 穴なし
        assert!(
            board.place_piece(0, 0, piece).is_ok(),
            "ピースの配置に成功するべき"
        );
    }

    #[test]
    fn test_place_piece_on_occupied_cell() {
        let mut board = Board::new();
        let piece = Piece::new(0, 1, 0, 1);
        board.place_piece(0, 0, piece).unwrap();
        let result = board.place_piece(0, 0, piece);
        assert!(
            result.is_err(),
            "既に埋まっているセルにピースを置くことはできない"
        );
    }

    #[test]
    fn test_check_win_row() {
        let mut board = Board::new();
        let piece1 = Piece::new(0, 1, 0, 0);
        let piece2 = Piece::new(0, 1, 0, 1);
        let piece3 = Piece::new(1, 1, 1, 0);
        let piece4 = Piece::new(0, 1, 0, 0);

        board.place_piece(0, 0, piece1).unwrap();
        board.place_piece(0, 1, piece2).unwrap();
        board.place_piece(0, 2, piece3).unwrap();
        board.place_piece(0, 3, piece4).unwrap();

        assert!(
            board.check_win(),
            "同じ行にすべての属性が同じピースが並ぶと勝利になる"
        );
    }

    #[test]
    fn test_check_win_column() {
        let mut board = Board::new();
        let piece1 = Piece::new(0, 0, 0, 1);
        let piece2 = Piece::new(1, 0, 0, 1);
        let piece3 = Piece::new(0, 1, 0, 1);
        let piece4 = Piece::new(0, 0, 1, 1);

        board.place_piece(0, 0, piece1).unwrap();
        board.place_piece(1, 0, piece2).unwrap();
        board.place_piece(2, 0, piece3).unwrap();
        board.place_piece(3, 0, piece4).unwrap();

        assert!(
            board.check_win(),
            "同じ列にすべての属性が同じピースが並ぶと勝利になる"
        );
    }

    #[test]
    fn test_check_win_diagonal() {
        let mut board = Board::new();
        let piece1 = Piece::new(0, 0, 0, 0);
        let piece2 = Piece::new(0, 1, 1, 1);
        let piece3 = Piece::new(0, 0, 1, 1);
        let piece4 = Piece::new(0, 1, 0, 1);

        board.place_piece(0, 0, piece1).unwrap();
        board.place_piece(1, 1, piece2).unwrap();
        board.place_piece(2, 2, piece3).unwrap();
        board.place_piece(3, 3, piece4).unwrap();

        assert!(
            board.check_win(),
            "対角線にすべての属性が同じピースが並ぶと勝利になる"
        );
    }

    #[test]
    fn test_no_win() {
        let mut board = Board::new();
        let piece1 = Piece::new(0, 0, 0, 0);
        let piece2 = Piece::new(1, 1, 1, 1);

        board.place_piece(0, 0, piece1).unwrap();
        board.place_piece(0, 1, piece2).unwrap();
        board.place_piece(0, 2, piece1).unwrap();
        board.place_piece(0, 3, piece2).unwrap();

        assert!(
            !board.check_win(),
            "すべての属性が一致しない場合は勝利にならない"
        );
    }

    #[test]
    fn test_available_positions() {
        let mut board = Board::new();
        let piece = Piece::new(0, 0, 0, 0);

        board.place_piece(0, 0, piece).unwrap();
        let positions = board.available_positions();
        assert_eq!(
            positions.len(),
            15,
            "1つピースを置いた後は残り15のポジションが利用可能なはず"
        );
        assert!(
            !positions.contains(&(0, 0)),
            "置いた位置は利用可能なポジションに含まれない"
        );
    }

    #[test]
    fn test_find_winning_cell() {
        let mut board = Board::new();
        let piece1 = Piece::new(0, 0, 0, 0);

        board.place_piece(0, 0, piece1).unwrap();
        board.place_piece(0, 1, piece1).unwrap();
        board.place_piece(0, 2, piece1).unwrap();

        let winning_move = board.find_winning_cell(piece1);
        assert_eq!(
            winning_move,
            Some((0, 3)),
            "勝利をもたらすセルが正しく検出されるべき"
        );
    }

    #[test]
    fn test_is_full() {
        let mut board = Board::new();
        let piece = Piece::new(0, 0, 0, 0);

        // 全てのマスにピースを置く
        for row in 0..4 {
            for col in 0..4 {
                board.place_piece(row, col, piece).unwrap();
            }
        }

        assert!(
            board.is_full(),
            "すべてのセルが埋まっているとき、ボードは満杯であるべき"
        );
    }

    #[test]
    fn test_recover_piece() {
        let piece = Piece::new(0, 0, 1, 0);
        let mut board = Board::new();

        board.place_piece(0, 0, piece).unwrap();

        let grid = board.grid();
        let recovered_piece_json = serde_json::to_string(&grid[0][0]).unwrap();
        let piece_json = serde_json::to_string(&Some(piece)).unwrap();

        assert_eq!(
            recovered_piece_json, piece_json,
            "ピースが正しく復元されるべき"
        );
    }
}
