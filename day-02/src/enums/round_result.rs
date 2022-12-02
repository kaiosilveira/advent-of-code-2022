#[derive(Debug, PartialEq)]
pub enum RoundResult {
    WIN,
    LOSS,
    DRAW,
}

impl RoundResult {
    pub fn get_points(&self) -> i32 {
        match self {
            RoundResult::LOSS => 0,
            RoundResult::DRAW => 3,
            RoundResult::WIN => 6,
        }
    }
}

#[cfg(test)]
mod round_result_tests {
    use super::*;

    #[test]
    fn should_return_zero_points_for_a_loss() {
        assert_eq!(0, RoundResult::LOSS.get_points());
    }

    #[test]
    fn should_return_three_points_for_a_draw() {
        assert_eq!(3, RoundResult::DRAW.get_points());
    }

    #[test]
    fn should_return_six_points_for_a_victory() {
        assert_eq!(6, RoundResult::WIN.get_points());
    }
}
