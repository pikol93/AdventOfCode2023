use std::str::FromStr;

#[derive(Debug)]
pub struct CardData {
    pub winning_numbers: Vec<u32>,
    pub owned_numbers: Vec<u32>,
}

pub fn read_input(input_text: &str) -> Vec<CardData> {
    input_text.split('\n').filter_map(read_line).collect()
}

fn read_line(line: &str) -> Option<CardData> {
    let Some((_, right)) = line.split_once(':') else {
        return None;
    };

    let right = right.trim();

    let Some((winning_numbers_string, owned_numbers_string)) = right.split_once('|') else {
        return None;
    };

    let winning_numbers = read_numbers(winning_numbers_string);
    let owned_numbers = read_numbers(owned_numbers_string);

    Some(CardData {
        winning_numbers,
        owned_numbers,
    })
}

fn read_numbers(text: &str) -> Vec<u32> {
    text.trim()
        .split(' ')
        .filter_map(|substring| u32::from_str(substring).ok())
        .collect()
}
