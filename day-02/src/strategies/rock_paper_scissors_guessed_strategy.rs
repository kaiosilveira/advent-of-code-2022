use crate::{
    enums::{hand_shape::*, round_result::*, round_winner::*},
    utils::letter_to_symbol::*,
};

pub struct RockPaperScissorsGuessedStrategy {
    pub user_hand_shape: HandShape,
    pub opponent_hand_shape: HandShape,
}

impl RockPaperScissorsGuessedStrategy {
    pub fn build(
        opponent_hand_shape: HandShape,
        user_hand_shape: HandShape,
    ) -> RockPaperScissorsGuessedStrategy {
        RockPaperScissorsGuessedStrategy {
            opponent_hand_shape,
            user_hand_shape,
        }
    }

    fn get_round_result(&self) -> RoundResult {
        self.user_hand_shape.against(self.opponent_hand_shape)
    }

    pub fn get_winner(&self) -> RoundWinner {
        match self.get_round_result() {
            RoundResult::DRAW => RoundWinner::NONE,
            RoundResult::WIN => RoundWinner::USER,
            RoundResult::LOSS => RoundWinner::OPPONENT,
        }
    }

    pub fn get_total_points(&self) -> i32 {
        let symbol_points = self.user_hand_shape.get_points();
        let round_points = self.get_round_result().get_points();

        symbol_points + round_points
    }
}

pub fn run_guessed_strategy(contents: &Vec<&str>) {
    let mut total_score = 0;

    for line in contents {
        let round_info: Vec<&str> = line.split(" ").collect();
        let opponent_hand_shape = round_info
            .get(0)
            .expect("should be able to find the opponent's play");

        let user_hand_shape = round_info
            .get(1)
            .expect("should be able to find the user play");

        let round = RockPaperScissorsGuessedStrategy::build(
            letter_to_symbol(opponent_hand_shape),
            letter_to_symbol(user_hand_shape),
        );

        println!(
            "Opponent: {:?} | You: {:?} | Winner: {:?} | Symbol points: {} | Total points: {}",
            round.opponent_hand_shape,
            round.user_hand_shape,
            round.get_winner(),
            round.user_hand_shape.get_points(),
            round.get_total_points()
        );

        total_score += round.get_total_points();
    }

    println!("Total score for the whole strategy is {}", total_score);
}

#[cfg(test)]
mod guessed_strategy_tests {
    use super::*;

    mod get_winner {
        use super::*;

        #[test]
        fn should_return_the_correct_winner() {
            let opponent_hand_shape = HandShape::ROCK;
            let user_hand_shape = HandShape::PAPER;
            let round =
                RockPaperScissorsGuessedStrategy::build(opponent_hand_shape, user_hand_shape);

            assert_eq!(RoundWinner::USER, round.get_winner());
        }
    }

    mod get_total_points {
        use super::*;

        #[test]
        fn should_return_the_correct_number_of_points_for_a_win_with_paper() {
            let opponent_hand_shape = HandShape::ROCK;
            let user_hand_shape = HandShape::PAPER;
            let round =
                RockPaperScissorsGuessedStrategy::build(opponent_hand_shape, user_hand_shape);

            assert_eq!(8, round.get_total_points());
        }
    }
}
