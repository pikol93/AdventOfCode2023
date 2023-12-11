use crate::common::read_input;

pub fn part2(input_text: &str) {
    let cards_data = read_input(input_text);
    let mut card_count = vec![1; cards_data.len()];

    for index in 0..cards_data.len() {
        let count = *card_count.get(index).unwrap();
        let card_data = cards_data.get(index).unwrap();
        for i in 0..card_data.win_count {
            let a = card_count.get_mut(index + i + 1).unwrap();
            *a += count;
        }
    }

    let sum = card_count.iter().sum::<u32>();
    println!("Part 2 sum: {}", sum);
}
