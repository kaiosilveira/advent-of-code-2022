use day_02::utils::{letter_to_suggested_result::*, letter_to_symbol::*};
use day_02::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the input file");
    let contents: Vec<&str> = contents.lines().collect();

    let mut total_score = 0;
    for line in contents {
        let round_info: Vec<&str> = line.split(" ").collect();
        let opponents_play = round_info
            .get(0)
            .expect("should be able to find the opponent's play");

        let your_play = round_info
            .get(1)
            .expect("should be able to find the user's plays");

        let round = RockPaperScissorsRealStrategy::build(
            letter_to_symbol(opponents_play),
            letter_to_suggested_result(your_play),
        );

        println!(
            "You should: {:?} | Opponent: {:?} | You: {:?} | Winner: {:?} | Symbol points: {} | Total points: {}",
            round.expected_result,
            round.opponents_play,
            round.get_expected_hand_shape_to_satisfy_result(),
            round.get_winner(),
            round.get_expected_hand_shape_to_satisfy_result().get_points(),
            round.get_total_points()
        );

        total_score += round.get_total_points();
    }

    println!("Total score for the whole strategy is {}", total_score);
}
