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

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/02.txt").unwrap();
    let mut score = 0u32;
    for line in input.lines() {
        let (opp, me) = if let [opp, me, ..] = line
            .split(" ")
            .map(Action::from_char)
            .collect::<Vec<Action>>()[..]
        {
            (opp, me)
        } else {
            panic!("Invalid input file")
        };
        score += me.play_against(opp).score() + me.score();
    }
    println!("{:?}", score);
}
