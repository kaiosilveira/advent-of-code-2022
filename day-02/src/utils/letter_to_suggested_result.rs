use crate::enums::round_result::*;

pub fn letter_to_suggested_result(letter: &str) -> RoundResult {
    match letter {
        "X" => RoundResult::LOSS,
        "Y" => RoundResult::DRAW,
        "Z" => RoundResult::WIN,
        _ => panic!("Invalid letter provided to be converted into a suggested round result"),
    }
}
