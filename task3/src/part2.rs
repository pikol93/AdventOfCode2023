use crate::common::read_input;

const SYMBOL_NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn part2(input_text: &str) {
    let engine_schematic = read_input(input_text);

    let sum = engine_schematic
        .symbols
        .iter()
        .filter(|symbol| symbol.character == '*')
        .filter_map(|symbol| {
            let mut symbol_neighbours = SYMBOL_NEIGHBOUR_OFFSETS.clone();
            symbol_neighbours
                .iter_mut()
                .for_each(|(offset_x, offset_y)| {
                    *offset_x += symbol.char_index as i32;
                    *offset_y += symbol.line as i32;
                });

            let mut part_numbers = engine_schematic.part_numbers.iter().filter(|part_number| {
                symbol_neighbours
                    .iter()
                    .filter(|(_, point_y)| part_number.line == *point_y as usize)
                    .filter(|(point_x, _)| part_number.start_char_index <= *point_x as usize)
                    .any(|(point_x, _)| part_number.end_char_index >= *point_x as usize)
            });

            let Some(first_part_number) = part_numbers.next() else {
                return None;
            };

            let Some(second_part_number) = part_numbers.next() else {
                return None;
            };

            // Need exactly two neighbouring part numbers
            if part_numbers.next().is_some() {
                return None;
            }

            Some(first_part_number.value * second_part_number.value)
        })
        .sum::<u32>();

    println!("Part 2 sum: {}", sum);
}
