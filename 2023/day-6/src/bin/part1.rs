use day_6::race;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let times_record_distances = input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .filter_map(|number| number.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_wins_product = &times_record_distances[0]
        .iter()
        .zip(&times_record_distances[1])
        .map(|(time, record_distance)| race::Race::new(*time as u64, *record_distance))
        .fold(1, |acc, race| acc * race.num_wins());

    println!("{}", num_wins_product);
}
