use super::round_result::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HandShape {
    ROCK,
    PAPER,
    SCISSORS,
}

impl HandShape {
    pub fn get_points(&self) -> i32 {
        match self {
            HandShape::ROCK => 1,
            HandShape::PAPER => 2,
            HandShape::SCISSORS => 3,
        }
    }

    pub fn against(&self, other: HandShape) -> RoundResult {
        if self.get_defeated() == other {
            RoundResult::WIN
        } else if self.get_defeater() == other {
            RoundResult::LOSS
        } else {
            RoundResult::DRAW
        }
    }

    pub fn get_defeater(&self) -> HandShape {
        match self {
            HandShape::ROCK => HandShape::PAPER,
            HandShape::PAPER => HandShape::SCISSORS,
            HandShape::SCISSORS => HandShape::ROCK,
        }
    }

    pub fn get_defeated(&self) -> HandShape {
        match self {
            HandShape::ROCK => HandShape::SCISSORS,
            HandShape::PAPER => HandShape::ROCK,
            HandShape::SCISSORS => HandShape::PAPER,
        }
    }
}

#[cfg(test)]
mod hand_shape_tests {
    use super::*;

    mod get_defeater {
        use super::*;

        #[test]
        fn should_return_the_shape_which_defeats_it() {
            let rock = HandShape::ROCK;
            let paper = HandShape::PAPER;
            let scissors = HandShape::SCISSORS;

            assert_eq!(HandShape::PAPER, rock.get_defeater());
            assert_eq!(HandShape::SCISSORS, paper.get_defeater());
            assert_eq!(HandShape::ROCK, scissors.get_defeater());
        }
    }

    mod get_defeated {
        use super::*;

        #[test]
        fn should_return_the_shape_which_it_defeats() {
            let rock = HandShape::ROCK;
            let paper = HandShape::PAPER;
            let scissors = HandShape::SCISSORS;

            assert_eq!(HandShape::SCISSORS, rock.get_defeated());
            assert_eq!(HandShape::ROCK, paper.get_defeated());
            assert_eq!(HandShape::PAPER, scissors.get_defeated());
        }
    }

    mod get_points {
        use super::*;

        #[test]
        fn should_return_1_if_it_is_a_rock_shape() {
            assert_eq!(1, HandShape::ROCK.get_points());
        }

        #[test]
        fn should_return_2_if_it_is_a_paper_shape() {
            assert_eq!(2, HandShape::PAPER.get_points());
        }

        #[test]
        fn should_return_2_if_it_is_a_scissors_shape() {
            assert_eq!(3, HandShape::SCISSORS.get_points());
        }
    }

    mod against {
        use super::*;

        #[test]
        fn should_draw_if_same_shape() {
            let rock = HandShape::ROCK;
            assert_eq!(RoundResult::DRAW, rock.against(HandShape::ROCK));

            let paper = HandShape::PAPER;
            assert_eq!(RoundResult::DRAW, paper.against(HandShape::PAPER));

            let scissors = HandShape::SCISSORS;
            assert_eq!(RoundResult::DRAW, scissors.against(HandShape::SCISSORS));
        }

        #[test]
        fn rock_beats_scissors() {
            let rock = HandShape::ROCK;
            assert_eq!(RoundResult::WIN, rock.against(HandShape::SCISSORS));
        }

        #[test]
        fn rock_loses_for_paper() {
            let rock = HandShape::ROCK;
            assert_eq!(RoundResult::LOSS, rock.against(HandShape::PAPER));
        }

        #[test]
        fn paper_beats_rock() {
            let paper = HandShape::PAPER;
            assert_eq!(RoundResult::WIN, paper.against(HandShape::ROCK));
        }

        #[test]
        fn paper_loses_for_scissors() {
            let paper = HandShape::PAPER;
            assert_eq!(RoundResult::LOSS, paper.against(HandShape::SCISSORS));
        }

        #[test]
        fn scissors_beats_paper() {
            let scissors = HandShape::SCISSORS;
            assert_eq!(RoundResult::WIN, scissors.against(HandShape::PAPER));
        }

        #[test]
        fn scissors_loses_for_rock() {
            let scissors = HandShape::SCISSORS;
            assert_eq!(RoundResult::LOSS, scissors.against(HandShape::ROCK));
        }
    }
}
