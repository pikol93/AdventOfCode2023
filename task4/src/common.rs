use std::str::FromStr;

#[derive(Debug)]
pub struct CardData {
    pub card_number: u32,
    pub winning_numbers: Vec<u32>,
    pub owned_numbers: Vec<u32>,
    pub win_count: usize,
}

pub fn read_input(input_text: &str) -> Vec<CardData> {
    input_text.split('\n').filter_map(read_line).collect()
}

fn read_line(line: &str) -> Option<CardData> {
    let Some((left, right)) = line.split_once(':') else {
        return None;
    };

    let card_number = read_card_number(left).unwrap();

    let right = right.trim();

    let Some((winning_numbers_string, owned_numbers_string)) = right.split_once('|') else {
        return None;
    };

    let winning_numbers = read_numbers(winning_numbers_string);
    let owned_numbers = read_numbers(owned_numbers_string);

    let win_count = owned_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .count();

    Some(CardData {
        card_number,
        winning_numbers,
        owned_numbers,
        win_count,
    })
}

fn read_card_number(text: &str) -> Option<u32> {
    let Some((_, right)) = text.rsplit_once(' ') else {
        return None;
    };

    let Ok(number) = u32::from_str(right) else {
        return None;
    };

    Some(number)
}

fn read_numbers(text: &str) -> Vec<u32> {
    text.trim()
        .split(' ')
        .filter_map(|substring| u32::from_str(substring).ok())
        .collect()
}
