pub mod enums;
pub mod utils;
use enums::{hand_shape::*, round_result::*, round_winner::*};

pub struct RockPaperScissorsRealStrategy {
    pub expected_result: RoundResult,
    pub opponents_play: HandShape,
}

impl RockPaperScissorsRealStrategy {
    pub fn build(
        opponents_play: HandShape,
        expected_result: RoundResult,
    ) -> RockPaperScissorsRealStrategy {
        RockPaperScissorsRealStrategy {
            expected_result,
            opponents_play,
        }
    }

    pub fn get_expected_hand_shape_to_satisfy_result(&self) -> HandShape {
        match self.expected_result {
            RoundResult::DRAW => self.opponents_play.clone(),
            RoundResult::WIN => self.opponents_play.get_defeater(),
            RoundResult::LOSS => self.opponents_play.get_defeated(),
        }
    }

    pub fn get_winner(&self) -> RoundWinner {
        match self.expected_result {
            RoundResult::DRAW => RoundWinner::NONE,
            RoundResult::WIN => RoundWinner::YOU,
            RoundResult::LOSS => RoundWinner::OPPONENT,
        }
    }

    pub fn get_total_points(&self) -> i32 {
        let hand_shape = self.get_expected_hand_shape_to_satisfy_result();
        let symbol_points = hand_shape.get_points();
        let round_points = self.expected_result.get_points();

        symbol_points + round_points
    }
}

#[cfg(test)]
mod rock_paper_scissors_real_strategy_tests {
    use crate::RockPaperScissorsRealStrategy;

    mod build {
        use super::*;

        #[test]
        fn should_build() {
            RockPaperScissorsRealStrategy {
                expected_result: crate::RoundResult::DRAW,
                opponents_play: crate::HandShape::ROCK,
            };
        }
    }

    mod get_expected_play_to_satisfy_result {
        use super::*;

        mod to_draw {
            use super::*;

            #[test]
            fn should_play_rock_if_opponent_played_rock() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::DRAW,
                    opponents_play: crate::HandShape::ROCK,
                };

                assert_eq!(
                    crate::HandShape::ROCK,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_paper_if_opponent_played_paper() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::DRAW,
                    opponents_play: crate::HandShape::PAPER,
                };

                assert_eq!(
                    crate::HandShape::PAPER,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_scissors_if_opponent_played_scissors() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::DRAW,
                    opponents_play: crate::HandShape::SCISSORS,
                };

                assert_eq!(
                    crate::HandShape::SCISSORS,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }
        }

        mod to_win {
            use super::*;

            #[test]
            fn should_play_paper_if_opponent_played_rock() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::WIN,
                    opponents_play: crate::HandShape::ROCK,
                };

                assert_eq!(
                    crate::HandShape::PAPER,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_scissors_if_opponent_played_paper() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::WIN,
                    opponents_play: crate::HandShape::PAPER,
                };

                assert_eq!(
                    crate::HandShape::SCISSORS,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_rock_if_opponent_played_scissors() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::WIN,
                    opponents_play: crate::HandShape::SCISSORS,
                };

                assert_eq!(
                    crate::HandShape::ROCK,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }
        }

        mod to_lose {
            use super::*;

            #[test]
            fn should_play_scissors_if_opponent_played_rock() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::LOSS,
                    opponents_play: crate::HandShape::ROCK,
                };

                assert_eq!(
                    crate::HandShape::SCISSORS,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_rock_if_opponent_played_paper() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::LOSS,
                    opponents_play: crate::HandShape::PAPER,
                };

                assert_eq!(
                    crate::HandShape::ROCK,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }

            #[test]
            fn should_play_paper_if_opponent_played_scissors() {
                let round = RockPaperScissorsRealStrategy {
                    expected_result: crate::RoundResult::LOSS,
                    opponents_play: crate::HandShape::SCISSORS,
                };

                assert_eq!(
                    crate::HandShape::PAPER,
                    round.get_expected_hand_shape_to_satisfy_result()
                );
            }
        }
    }

    mod get_winner {
        use crate::{RockPaperScissorsRealStrategy, RoundWinner};

        #[test]
        fn should_return_you_if_expected_result_is_a_win() {
            let round = RockPaperScissorsRealStrategy {
                expected_result: crate::RoundResult::WIN,
                opponents_play: crate::HandShape::PAPER,
            };

            assert_eq!(RoundWinner::YOU, round.get_winner());
        }

        #[test]
        fn should_return_opponent_if_expected_result_is_a_loss() {
            let round = RockPaperScissorsRealStrategy {
                expected_result: crate::RoundResult::LOSS,
                opponents_play: crate::HandShape::PAPER,
            };

            assert_eq!(RoundWinner::OPPONENT, round.get_winner());
        }

        #[test]
        fn should_return_none_if_expected_result_is_a_draw() {
            let round = RockPaperScissorsRealStrategy {
                expected_result: crate::RoundResult::DRAW,
                opponents_play: crate::HandShape::PAPER,
            };

            assert_eq!(RoundWinner::NONE, round.get_winner());
        }
    }

    mod get_total_points {
        use super::*;
        use crate::enums::{hand_shape::*, round_result::*};

        #[test]
        fn should_return_the_total_points_for_a_round() {
            let round = RockPaperScissorsRealStrategy {
                opponents_play: HandShape::ROCK,
                expected_result: RoundResult::WIN,
            };

            assert_eq!(8, round.get_total_points());
        }
    }
}
