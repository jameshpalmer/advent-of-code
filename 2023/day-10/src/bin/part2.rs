use day_10::grid::Grid;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let grid = Grid::new(input);
    println!("{}", grid.count_interior_points());
}
