use crate::game::piece::Piece;
use crate::game::Game;
use crate::policies::policy::Policy;
use std::panic::AssertUnwindSafe;

pub fn test_policy_action<P: Policy>(policy: P) {
    let game = Game::new();
    let action = policy.action(&game);

    assert!(action.row < 4, "行が有効な範囲内であること");
    assert!(action.col < 4, "列が有効な範囲内であること");

    if let Some(piece_index) = action.piece_index {
        assert!(
            piece_index < game.available_pieces.len(),
            "ピースインデックスが有効な範囲内であること"
        );
    }
}

pub fn test_policy_game_progression<P: Policy>(policy: P) {
    let mut game = Game::new();

    while !game.is_game_over() {
        let action = policy.action(&game);
        game.play_turn(action.row, action.col, action.piece_index)
            .unwrap();
    }

    assert!(game.is_game_over(), "ゲームが終了しているはず");
    if let Some(winner) = game.judge_winner() {
        println!("勝者: {:?}", winner);
    } else {
        println!("引き分けです。");
    }
}

pub fn test_policy_no_available_positions<P: Policy>(policy: P) {
    let mut game = Game::new();

    let piece = Piece::new(0, 0, 0, 0);
    for row in 0..4 {
        for col in 0..4 {
            game.board.place_piece(row, col, piece.clone()).unwrap();
        }
    }

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        policy.action(&game);
    }));
    assert!(
        result.is_err(),
        "利用可能なポジションがない場合にパニックが発生するはず"
    );
}

pub fn test_policy_no_available_pieces<P: Policy>(policy: P) {
    let mut game = Game::new();

    game.available_pieces.clear();

    let action = policy.action(&game);
    assert_eq!(
        action.piece_index, None,
        "利用可能なピースがない場合はNoneが返るはず"
    );
}
