use day_02::strategies::{
    rock_paper_scissors_guessed_strategy::*, rock_paper_scissors_real_strategy::*,
};
use day_02::utils::{letter_to_suggested_result::*, letter_to_symbol::*};
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the input file");
    let contents: Vec<&str> = contents.lines().collect();

    run_real_strategy(&contents);
    println!("-------------------");
    run_guessed_strategy(&contents);
}

fn run_guessed_strategy(contents: &Vec<&str>) {
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

fn run_real_strategy(contents: &Vec<&str>) {
    let mut total_score = 0;

    for line in contents {
        let round_info: Vec<&str> = line.split(" ").collect();
        let opponents_play = round_info
            .get(0)
            .expect("should be able to find the opponent's play");

        let suggested_result_letter = round_info
            .get(1)
            .expect("should be able to find the suggested result letter");

        let round = RockPaperScissorsRealStrategy::build(
            letter_to_symbol(opponents_play),
            letter_to_suggested_result(suggested_result_letter),
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
