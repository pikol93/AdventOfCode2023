#[derive(Debug)]
pub struct PartNumber {
    pub line: usize,
    pub start_char_index: usize,
    pub end_char_index: usize,
    pub value: u32,
}

#[derive(Debug)]
pub struct Symbol {
    pub line: usize,
    pub char_index: usize,
}

#[derive(Debug)]
pub struct EngineSchematic {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}

struct FindPartNumbers<I: Iterator<Item=(usize, char)>> {
    iterator: I,
    line_index: usize,
}

impl<I: Iterator<Item=(usize, char)>> Iterator for FindPartNumbers<I> {
    type Item = PartNumber;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Ignore all characters until the first digit
        let (start_char_index, mut value) = loop {
            let Some((current_index, character)) = self.iterator.next() else {
                // No characters left in the iterator
                return None;
            };

            let Some(digit) = character.to_digit(10) else {
                // Not a digit character
                continue;
            };

            break (current_index, digit);
        };

        // Read every digit until a character appears or the iterator ends
        let mut end_char_index = start_char_index;
        loop {
            let Some((current_index, character)) = self.iterator.next() else {
                // No characters left in the iterator
                break;
            };

            let Some(digit) = character.to_digit(10) else {
                // Not a digit character
                break;
            };

            value = value * 10 + digit;
            end_char_index = current_index;
        }

        Some(PartNumber {
            line: self.line_index,
            start_char_index,
            end_char_index,
            value,
        })
    }
}

trait FindPartNumbersTrait: Iterator<Item=(usize, char)>
    where
        Self: Sized,
{
    fn find_part_numbers(self, line_index: usize) -> FindPartNumbers<Self> {
        FindPartNumbers {
            iterator: self,
            line_index,
        }
    }
}

impl<I> FindPartNumbersTrait for I where I: Iterator<Item=(usize, char)> {}

pub fn read_input(input: &str) -> EngineSchematic {
    EngineSchematic {
        part_numbers: read_part_numbers(input).collect(),
        symbols: read_symbols(input).collect(),
    }
}

fn read_part_numbers(input: &str) -> impl Iterator<Item=PartNumber> + '_ {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(line_index, line)| read_part_numbers_from_line(line_index, line))
}

fn read_part_numbers_from_line(
    line_index: usize,
    line: &str,
) -> impl Iterator<Item=PartNumber> + '_ {
    line.chars().enumerate().find_part_numbers(line_index)
}

fn read_symbols(line: &str) -> impl Iterator<Item=Symbol> + '_ {
    line.split('\n')
        .enumerate()
        .flat_map(|(line_index, line)| read_symbols_from_line(line_index, line))
}

fn read_symbols_from_line(line_index: usize, input: &str) -> impl Iterator<Item=Symbol> + '_ {
    input
        .chars()
        .enumerate()
        .filter(|(_, character)| *character != '.')
        .filter(|(_, character)| !character.is_ascii_digit())
        .map(move |(char_index, _)| Symbol {
            line: line_index,
            char_index,
        })
}
