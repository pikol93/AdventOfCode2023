use crate::part1::part1;

mod common;
mod part1;

const INPUT_TEXT: &str = include_str!("../input.txt");

fn main() {
    part1(INPUT_TEXT);
}

#[cfg(test)]
mod tests {
    use crate::part1::part1;

    const EXAMPLE_DATA: &str = include_str!("../example.txt");

    #[test]
    fn should_pass_part_1_with_example_data() {
        part1(EXAMPLE_DATA);
    }
}
