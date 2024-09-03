use quart_engine::game::Player;
use quart_engine::policies::{MCSPolicy, OneStepLookAheadPolicy, Policy, RandomPolicy};
use quart_engine::runner::Runner;
use tqdm::tqdm;

const NUM_TRIALS: u64 = 10;

// 汎用的なテスト関数を定義
fn test_policy_vs_policy<P1, P2>(player1_policy: P1, player2_policy: P2, description: &str)
where
    P1: Policy + Clone + 'static,
    P2: Policy + Clone + 'static,
{
    let mut win_count = 0;
    for _ in tqdm(0..NUM_TRIALS) {
        let mut runner = Runner::new(
            Box::new(player1_policy.clone()),
            Box::new(player2_policy.clone()),
        );
        let winner = runner.run();
        if let Some(Player::Player1) = winner {
            win_count += 1;
        }
    }
    let win_rate = win_count as f64 / NUM_TRIALS as f64;
    println!("{} Win rate: {}", description, win_rate);
}

#[test]
fn test_random_vs_random_policy() {
    test_policy_vs_policy(
        RandomPolicy::new(),
        RandomPolicy::new(),
        "Random vs Random Policy",
    );
}

#[test]
fn test_random_vs_one_step_look_ahead_policy() {
    test_policy_vs_policy(
        OneStepLookAheadPolicy::new(),
        RandomPolicy::new(),
        "One Step Look Ahead vs Random Policy",
    );
}

#[test]
fn test_one_step_look_ahead_vs_one_step_look_ahead_policy() {
    test_policy_vs_policy(
        OneStepLookAheadPolicy::new(),
        OneStepLookAheadPolicy::new(),
        "One Step Look Ahead vs One Step Look Ahead Policy",
    );
}

#[test]
fn test_mcs_policy_vs_one_step_look_ahead_policy() {
    test_policy_vs_policy(
        MCSPolicy {
            policy: OneStepLookAheadPolicy::new(),
            max_time: 0.001,
        },
        OneStepLookAheadPolicy::new(),
        "MCS vs One Step Look Ahead Policy",
    );
}
