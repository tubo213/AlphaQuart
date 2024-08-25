use alpha_quart::game::Game;
use alpha_quart::game::Player;
use alpha_quart::policies::Policy;
use alpha_quart::policies::RandomPolicy;
use tqdm::tqdm;

#[test]
fn test_random_game() {
    // random policy vs random policyで10000回ゲームを実行する
    for i in tqdm(0..10000) {
        println!("Executing game run: {}", i + 1);
        let mut game = Game::new();
        let player1 = RandomPolicy::new();
        let player2 = RandomPolicy::new();

        while !game.is_game_over() {
            let action;
            if matches!(game.current_player, Player::Player1) {
                action = player1.action(&game);
            } else {
                action = player2.action(&game);
            }
            game.play_turn(action.row, action.col, action.piece_index)
                .unwrap();
            game.switch_player();
        }
    }
}
