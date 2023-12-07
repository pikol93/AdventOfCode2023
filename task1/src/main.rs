const INPUT_TEXT: &str = include_str!("../input.txt");

trait FindFirstAndLast: Iterator {
    #[inline]
    fn find_first_and_last(
        &mut self,
    ) -> Option<(<Self as Iterator>::Item, <Self as Iterator>::Item)>
        where
            Self: Sized,
            Self::Item: Copy,
    {
        let Some(first) = self.next() else {
            return None;
        };

        let mut last = first;
        loop {
            match self.next() {
                None => break,
                Some(value) => last = value,
            }
        }

        Some((first, last))
    }
}

impl<I: Iterator> FindFirstAndLast for I {}

fn main() {
    process(INPUT_TEXT);
}

fn process(input_text: &str) {
    let sum = input_text
        .split('\n')
        .filter_map(|line| line
            .chars()
            .filter(char::is_ascii_digit)
            .find_first_and_last())
        .map(|(first_digit, second_digit)| {
            ascii_digit_to_u32(first_digit) * 10 + ascii_digit_to_u32(second_digit)
        })
        .sum::<u32>();

    println!("Sum: {}", sum);
}

fn ascii_digit_to_u32(value: char) -> u32 {
    value as u32 - '0' as u32
}
