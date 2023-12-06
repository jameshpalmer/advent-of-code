use day_6::race;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let time_record_distance = input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .filter(|number_str| number_str.parse::<i64>().is_ok())
                .map(|number| number.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let race = race::Race::new(time_record_distance[0] as u64, time_record_distance[1]);
    let num_wins = race.num_wins();

    println!("{}", num_wins);
}
