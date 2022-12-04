# Rock Paper Scissors

Challenge URL: https://adventofcode.com/2022/day/2

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

## Part one: the strategy

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

```
A Y
B X
C Z
```

This strategy guide predicts and recommends the following:

- In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
- In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
- The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
  In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

**What would your total score be if everything goes exactly according to your strategy guide?**

<details>
<summary><strong>See solution</strong></summary>
We can start solving this problem by modeling the structure of the Rock Paper Scissors game. A central piece of it is the hand shape each player can choose. The options are well-known and we can model them all as an enum:

```rust
enum HandShape {
    ROCK,
    PAPER,
    SCISSORS,
}
```

Then, we can start adding behavior to this enum (yes, Rust allows us to do that). We can first add information about which hand shape defeats the other one. This is as simple as:

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
impl HandShape {
    // -- snip --
    pub fn get_defeated(&self) -> HandShape {
        match self {
            HandShape::ROCK => HandShape::SCISSORS,
            HandShape::PAPER => HandShape::ROCK,
            HandShape::SCISSORS => HandShape::PAPER,
        }
    }
}

// -- snip --
#[cfg(test)]
mod hand_shape_tests {
    // -- snip --
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
```

By the same token, we can add a method to get information about which hand shape defeats the current shape:

```rust
impl HandShape {
    // -- snip --
    pub fn get_defeater(&self) -> HandShape {
        match self {
            HandShape::ROCK => HandShape::PAPER,
            HandShape::PAPER => HandShape::SCISSORS,
            HandShape::SCISSORS => HandShape::ROCK,
        }
    }
}

// -- snip --

#[cfg(test)]
mod hand_shape_tests {
    // -- snip --
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
```

As the challenge describes, we also need to store information about how many points a player can get by choosing a specific shape. This logic is implemented below:

```rust
impl HandShape {
  // -- snip --
    pub fn get_points(&self) -> i32 {
        match self {
            HandShape::ROCK => 1,
            HandShape::PAPER => 2,
            HandShape::SCISSORS => 3,
        }
    }
}

// -- snip --

#[cfg(test)]
mod hand_shape_tests {
// -- snip --
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
```

With the `HandShape` logic in place, we can start worrying about battles. We can introduce the concept of rounds, where each round is a hand shape against the other. Following this logic, we can implement a `RoundResult` enum, which (as the name strongly suggests) holds information about the result of a battle round:

```rust
pub enum RoundResult {
    WIN,
    LOSS,
    DRAW,
}
```

With this, we can now implement a method in `HandShape` to process a battle against another shape:

```rust
impl HandShape {
  // -- snip --
    pub fn against(&self, other: HandShape) -> RoundResult {
        if self.get_defeated() == other {
            RoundResult::WIN
        } else if self.get_defeater() == other {
            RoundResult::LOSS
        } else {
            RoundResult::DRAW
        }
    }
}
```

_Note: There are many tests for this method because of all possible permutations, so they'll be excluded from this text. You can find them all in the source code._

That's all for the core functionality of the game. Now that it is fully modeled, we can start implementing the logic of processing a battle round, collecting information about the winner and computing the total number of points of a round.

Let's start by implementing an enum to hold information about the winner. It is as simple as:

```rust
pub enum RoundWinner {
    USER,
    OPPONENT,
    NONE,
}
```

Next, let's create a struct that will hold the information about the hand shapes that both the user and their opponent played:

```rust
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
}
```

Now, by using the `HandShape` and `RoundWinner` enums, we can easily compute the winner of a round:

```rust
impl RockPaperScissorsGuessedStrategy {
    // -- snip --
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
}

#[cfg(test)]
mod guessed_strategy_tests {
    // -- snip
    #[test]
    fn should_return_the_correct_winner() {
        let opponent_hand_shape = HandShape::ROCK;
        let user_hand_shape = HandShape::PAPER;
        let round =
            RockPaperScissorsGuessedStrategy::build(opponent_hand_shape, user_hand_shape);

        assert_eq!(RoundWinner::USER, round.get_winner());
    }
}
```

We also need to compute the total points of a round:

```rust
impl RockPaperScissorsGuessedStrategy {
    // -- snip --
    pub fn get_total_points(&self) -> i32 {
        let symbol_points = self.user_hand_shape.get_points();
        let round_points = self.get_round_result().get_points();

        symbol_points + round_points
    }
}

#[cfg(test)]
mod guessed_strategy_tests {
    // -- snip --
    #[test]
    fn should_return_the_correct_number_of_points_for_a_win_with_paper() {
        let opponent_hand_shape = HandShape::ROCK;
        let user_hand_shape = HandShape::PAPER;
        let round =
            RockPaperScissorsGuessedStrategy::build(opponent_hand_shape, user_hand_shape);

        assert_eq!(8, round.get_total_points());
    }
}
```

And that's it! Now we just need to add boilerplate code to load the input file, parse the arguments and aggregate the results for all of the rounds considered in the strategy.

```rust
fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the input file");
    let contents: Vec<&str> = contents.lines().collect();

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

        total_score += round.get_total_points();
    }

    println!("Total score for the whole strategy is {}", total_score);
}
```

That's all!

</details>

---

## Part two: oops, the guessed strategy was wrong!

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

- In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
- In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
- In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
  Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, **what would your total score be if everything goes exactly according to your strategy guide?**

<details>
<summary><strong>See solution</strong></summary>

As the meaning of the input data changed a little bit after the latest clarifications from the elf, we now need to update the code. Thankfully, all the game logic implemented for `HandShape` still applies. With this in mind, the only thing we need to do is implement a new strategy, one that takes into consideration the expected hand shape to satisfy a battle result, but also computes the winner and the total points for a round, with minor modifications to comply with the new definitions.

Let's start by defining the struct:

```rust
pub struct RockPaperScissorsRealStrategy {
    pub expected_result: RoundResult,
    pub opponents_play: HandShape,
}
```

Then, we can add a method to return the expected hand shape to satisfy an expected battle result:

```rust
pub struct RockPaperScissorsRealStrategy {
    // -- snip --
    pub fn get_expected_hand_shape_to_satisfy_result(&self) -> HandShape {
        match self.expected_result {
            RoundResult::DRAW => self.opponents_play.clone(),
            RoundResult::WIN => self.opponents_play.get_defeater(),
            RoundResult::LOSS => self.opponents_play.get_defeated(),
        }
    }
}
```

The logic to find the winner is straightforward, considering that we are going to follow the suggested strategy:

```rust
pub struct RockPaperScissorsRealStrategy {
    // -- snip --
    pub fn get_winner(&self) -> RoundWinner {
        match self.expected_result {
            RoundResult::DRAW => RoundWinner::NONE,
            RoundResult::WIN => RoundWinner::USER,
            RoundResult::LOSS => RoundWinner::OPPONENT,
        }
    }
}
```

Calculating the total points for a battle round is also straightforward, we just need to sum the points of using the symbol calculated from `get_expected_hand_shape_to_satisfy_result` to the points from the battle round itself:

```rust
pub struct RockPaperScissorsRealStrategy {
    // -- snip --
    pub fn get_total_points(&self) -> i32 {
        let hand_shape = self.get_expected_hand_shape_to_satisfy_result();
        let symbol_points = hand_shape.get_points();
        let round_points = self.expected_result.get_points();

        symbol_points + round_points
    }
}
```

_Note: there are a lot of tests for the methods presented above. Make sure to check the source code to see them if you're curious._

And that's it! Now we just need to take care of the boilerplate:

```rust
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

        total_score += round.get_total_points();
    }

    println!("Total score for the whole strategy is {}", total_score);
}
```

And that's all!

</details>

---
