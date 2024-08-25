use quart_engine::game::Player;
use quart_engine::policies::Policy;
use quart_engine::policies::RandomPolicy;
use quart_engine::runner::Runner;
use tqdm::tqdm;

#[test]
fn test_random_policy() {
    let num_trials = 10000;
    // random policy vs random policyでnum_trials回ゲームを実行する
    let mut win_count = 0;
    for _ in tqdm(0..num_trials) {
        let player1 = RandomPolicy::new();
        let player2 = RandomPolicy::new();
        let mut runner = Runner::new(Box::new(player1), Box::new(player2));
        let winner = runner.run();
        if winner.is_some() {
            if matches!(winner.unwrap(), Player::Player1) {
                win_count += 1;
            }
        }
    }
    // 勝率を表示
    let win_rate = win_count as f64 / num_trials as f64;
    println!("Win rate: {}", win_rate);
}
