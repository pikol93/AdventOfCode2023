use crate::common::read_input;

pub fn part1(input_text: &str) {
    let card_data = read_input(input_text);
    let sum = card_data
        .iter()
        .map(|card_data| {
            let win_count = card_data
                .owned_numbers
                .iter()
                .filter(|owned_number| card_data.winning_numbers.contains(owned_number))
                .count();

            match win_count {
                0 => 0,
                _ => 2u32.pow(win_count as u32 - 1),
            }
        })
        .sum::<u32>();

    println!("Part 1 sum: {}", sum);
}
