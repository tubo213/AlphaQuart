use crate::game::action::Action;
use crate::game::Game;
use crate::game::Player;
use crate::policies::one_step_look_ahead_policy::OneStepLookAheadPolicy;
use crate::policies::policy::Policy;
use crate::utils::TimeKeeper;

#[derive(Clone)]
pub struct MCSPolicy {
    pub policy: OneStepLookAheadPolicy,
    pub max_time: f64,
}

impl Policy for MCSPolicy {
    fn new() -> Self {
        MCSPolicy {
            policy: OneStepLookAheadPolicy::new(),
            max_time: 0.01,
        }
    }

    fn action(&self, game: &Game) -> Action {
        if game.available_pieces.is_empty() {
            // 渡せる駒が無い場合は，置いて勝てる手があるならそれを選択すれば良い
            // これはOneStepLookAheadPolicyのactionメソッドと同じ
            return self.policy.action(game);
        } else {
            // すべてのactionに対して、num_playouts_per_action回プレイアウトを行い、最も勝率が高いactionを選択
            // 実行時間内でできるだけ多くのプレイアウトを行い，最も勝率が高い手を選択する

            // アクションを列挙して次の状態を計算
            let available_actions = game.available_actions();
            let n_available_actions = available_actions.len();
            if n_available_actions == 0 {
                panic!("利用可能なアクションがありません");
            }
            let next_states: Vec<Game> = available_actions
                .iter()
                .map(|action| {
                    let mut next_state = game.clone();
                    next_state
                        .play_turn(action.row, action.col, action.piece_index)
                        .unwrap();
                    next_state
                })
                .collect();

            // 時間いっぱいプレイアウトを行う
            let mut time_keeper = TimeKeeper::new(self.max_time);
            let mut scores = vec![0; n_available_actions];
            let mut counts = vec![0; n_available_actions];
            while !time_keeper.is_time_over() {
                for (i, next_state) in next_states.iter().enumerate() {
                    let winner = play_out(&next_state, &self.policy);
                    if winner.is_some() {
                        if winner.unwrap() == game.current_player {
                            scores[i] += 1;
                        } else {
                            scores[i] -= 1;
                        }
                    }
                    counts[i] += 1;
                }
            }

            // // 平均scoreが最も高いactionを選択
            let mut best_action: Option<Action> = None;
            let mut best_score = std::f64::MIN;
            for (i, action) in available_actions.iter().enumerate() {
                let score = scores[i] as f64 / counts[i] as f64;
                if score > best_score {
                    best_score = score;
                    best_action = Some(action.clone());
                }
            }

            best_action.unwrap()
        }
    }
}

fn play_out(game: &Game, policy: &OneStepLookAheadPolicy) -> Option<Player> {
    let mut game_copy = game.clone();
    while !game_copy.is_game_over() {
        let action = policy.action(&game_copy);
        game_copy
            .play_turn(action.row, action.col, action.piece_index)
            .unwrap();
    }

    game_copy.judge_winner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::mcs_policy::MCSPolicy;
    use crate::policies::test_utils::*;

    #[test]
    fn test_mcs_policy_action() {
        test_policy_action(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_game_progression() {
        test_policy_game_progression(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_no_available_positions() {
        test_policy_no_available_positions(MCSPolicy::new());
    }

    #[test]
    fn test_mcs_policy_no_available_pieces() {
        test_policy_no_available_pieces(MCSPolicy::new());
    }
}