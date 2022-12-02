use crate::enums::{hand_shape::*, round_result::*, round_winner::*};

pub struct RockPaperScissorsGuessedStrategy {
    pub user_play: HandShape,
    pub opponents_play: HandShape,
}

impl RockPaperScissorsGuessedStrategy {
    pub fn build(
        opponents_play: HandShape,
        user_play: HandShape,
    ) -> RockPaperScissorsGuessedStrategy {
        RockPaperScissorsGuessedStrategy {
            opponents_play,
            user_play,
        }
    }

    fn get_round_result(&self) -> RoundResult {
        self.user_play.against(self.opponents_play)
    }

    pub fn get_winner(&self) -> RoundWinner {
        match self.get_round_result() {
            RoundResult::DRAW => RoundWinner::NONE,
            RoundResult::WIN => RoundWinner::YOU,
            RoundResult::LOSS => RoundWinner::OPPONENT,
        }
    }

    pub fn get_total_points(&self) -> i32 {
        let symbol_points = self.user_play.get_points();
        let round_points = self.get_round_result().get_points();

        symbol_points + round_points
    }
}

#[cfg(test)]
mod guessed_strategy_tests {}
