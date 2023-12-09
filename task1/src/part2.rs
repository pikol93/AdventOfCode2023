const DIGIT_MAPPING: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

pub fn part2(input_text: &str) {
    let sum = input_text
        .split('\n')
        .map(|line| {
            let Some((first_digit, second_digit)) = find_first_and_last_digit(line) else {
                return 0;
            };

            first_digit * 10 + second_digit
        })
        .sum::<u32>();

    println!("Sum: {}", sum);
}

fn find_first_and_last_digit(characters: &str) -> Option<(u32, u32)> {
    let mut result_first = None;
    let mut result_last = None;

    for (pattern, digit) in DIGIT_MAPPING {
        let Some(found_index) = characters.find(pattern) else {
            continue;
        };

        if let Some((index, _)) = result_first {
            if found_index > index {
                continue;
            }
        }

        result_first = Some((found_index, digit));
    }
    for (pattern, digit) in DIGIT_MAPPING {
        let Some(found_index) = characters.rfind(pattern) else {
            continue;
        };

        if let Some((index, _)) = result_last {
            if found_index < index {
                continue;
            }
        }

        result_last = Some((found_index, digit));
    }

    if let Some((_, first_digit)) = result_first {
        if let Some((_, last_digit)) = result_last {
            return Some((first_digit, last_digit));
        }
    }

    None
}
