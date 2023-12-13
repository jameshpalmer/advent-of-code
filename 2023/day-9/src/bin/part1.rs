use day_9::sequence::Sequence;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let next_term_sum: i32 = input
        .lines()
        .map(|line| {
            Sequence::new(
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .map(|seq| seq.get_next_term())
        .sum();
    println!("{}", next_term_sum);
}
