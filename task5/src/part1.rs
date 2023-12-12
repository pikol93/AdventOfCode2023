use crate::common::read_input;

pub fn part1(input_text: &str) {
    let almanac = read_input(input_text);
    let min = almanac
        .map_seeds_to_location()
        .min()
        .expect("Should not have empty seed list");

    println!("Task 1 min: {min}");
}
