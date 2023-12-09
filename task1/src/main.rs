use crate::part1::part1;
use crate::part2::part2;

mod part1;
mod part2;

const INPUT_TEXT: &str = include_str!("../input.txt");

fn main() {
    part1(INPUT_TEXT);
    part2(INPUT_TEXT);
}
