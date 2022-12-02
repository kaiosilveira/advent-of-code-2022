use crate::enums::hand_shape::*;

pub fn letter_to_symbol(letter: &str) -> HandShape {
    match letter {
        "A" | "X" => HandShape::ROCK,
        "B" | "Y" => HandShape::PAPER,
        "C" | "Z" => HandShape::SCISSORS,
        _ => panic!("Invalid letter to represent a RockPaperScissors play"),
    }
}

#[cfg(test)]
mod letter_to_symbol_tests {
    use super::*;

    #[test]
    fn should_convert_letter_a_to_rock() {
        assert_eq!(HandShape::ROCK, letter_to_symbol("A"));
    }

    #[test]
    fn should_convert_letter_b_to_paper() {
        assert_eq!(HandShape::PAPER, letter_to_symbol("B"));
    }

    #[test]
    fn should_convert_letter_c_to_scissors() {
        assert_eq!(HandShape::SCISSORS, letter_to_symbol("C"));
    }

    #[test]
    fn should_convert_letter_x_to_rock() {
        assert_eq!(HandShape::ROCK, letter_to_symbol("X"));
    }

    #[test]
    fn should_convert_letter_y_to_paper() {
        assert_eq!(HandShape::PAPER, letter_to_symbol("Y"));
    }

    #[test]
    fn should_convert_letter_z_to_scissors() {
        assert_eq!(HandShape::SCISSORS, letter_to_symbol("Z"));
    }
}
