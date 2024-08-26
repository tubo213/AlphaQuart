use crate::game::Game;
use crate::game::Player;
use crate::policies::policy::Policy;

pub struct Runner {
    pub game: Game,
    pub player1: Box<dyn Policy>,
    pub player2: Box<dyn Policy>,
}

impl Runner {
    pub fn new(player1: Box<dyn Policy>, player2: Box<dyn Policy>) -> Self {
        Runner {
            game: Game::new(),
            player1,
            player2,
        }
    }

    pub fn run(&mut self) -> Option<Player> {
        while !self.game.is_game_over() {
            let action;
            if matches!(self.game.current_player, Player::Player1) {
                action = self.player1.action(&self.game);
            } else {
                action = self.player2.action(&self.game);
            }
            self.game
                .play_turn(action.row, action.col, Some(action.piece_index))
                .unwrap();
        }
        
        self.game.judge_winner()
    }
}
