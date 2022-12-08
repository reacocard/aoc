use std::fs;

fn main() {
    let input = fs::read_to_string("input/01.txt").unwrap();

    let mut best = 0u64;
    let mut current = 0u64;

    for line in input.lines() {
        match line {
            "" => {
                if current > best {
                    best = current
                }
                current = 0
            }
            _ => current += line.parse::<u64>().unwrap(),
        }
    }

    println!("{}", best)
}
