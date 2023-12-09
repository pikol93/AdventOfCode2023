trait FindFirstAndLast: DoubleEndedIterator {
    #[inline]
    fn find_first_and_last(&mut self) -> Option<(Self::Item, Self::Item)>
        where
            Self: Sized,
            Self::Item: Copy,
    {
        let Some(first) = self.next() else {
            return None;
        };

        let Some(last) = self.next_back() else {
            return Some((first, first));
        };

        Some((first, last))
    }
}

impl<I: DoubleEndedIterator> FindFirstAndLast for I {}

pub fn part1(input_text: &str) {
    let sum = input_text
        .split('\n')
        .filter_map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .find_first_and_last()
        })
        .map(|(first_digit, second_digit)| {
            ascii_digit_to_u32(first_digit) * 10 + ascii_digit_to_u32(second_digit)
        })
        .sum::<u32>();

    println!("Sum: {}", sum);
}

fn ascii_digit_to_u32(value: char) -> u32 {
    value as u32 - '0' as u32
}
