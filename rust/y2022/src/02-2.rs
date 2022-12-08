use std::fs;

#[derive(Clone, Copy, Debug)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn from_char(c: &str) -> Action {
        match c {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissors,
            _ => panic!("No matching action for '{}'!", c),
        }
    }

    fn play_against(self, other: Action) -> Outcome {
        // There should be a better way to do this.
        match self as i32 - other as i32 {
            0 => Outcome::Draw,
            1 | -2 => Outcome::Win,
            -1 | 2 => Outcome::Lose,
            x => panic!("Unexpected outcome {}", x),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(c: &str) -> Outcome {
        match c {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("No matching outcome for '{}'!", c),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn get_move(self, opp: Action) -> Action {
        if Action::Rock.play_against(opp) == self {
            Action::Rock
        } else if Action::Paper.play_against(opp) == self {
            Action::Paper
        } else if Action::Scissors.play_against(opp) == self {
            Action::Scissors
        } else {
            panic!("No matching move")
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/02.txt").unwrap();
    let mut score = 0u32;
    for line in input.lines() {
        let mut chars = line.split(" ");
        let opp = Action::from_char(chars.next().unwrap());
        let outcome = Outcome::from_char(chars.next().unwrap());
        let me = outcome.get_move(opp);
        println!("{:?} vs {:?} = {:?}", me, opp, outcome);
        score += outcome.score() + me.score();
    }
    println!("{:?}", score);
}
