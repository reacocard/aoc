use std::fs;

fn main() {
    let input = fs::read_to_string("input/01.txt").unwrap();

    let mut elves = Vec::new();
    let mut current = 0u64;

    for line in input.lines() {
        match line {
            "" => {
                elves.push(current);
                current = 0;
            },
            _ => {
                current += line.parse::<u64>().unwrap();
            }
        }
    }

    elves.sort_unstable();
    let l = elves.len();
    println!("{:?}", elves[l-1] + elves[l-2] + elves[l-3]);
}
