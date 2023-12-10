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

pub fn part1(input_text: &str) {
    let engine_schematic = read_input(input_text);

    let points_adjacent_to_symbols = engine_schematic
        .symbols
        .iter()
        .flat_map(|symbol| {
            SYMBOL_NEIGHBOUR_OFFSETS
                .iter()
                .map(|(offset_x, offset_y)| {
                    (
                        offset_x + symbol.char_index as i32,
                        offset_y + symbol.line as i32,
                    )
                })
                .map(|(point_x, point_y)| (point_x as usize, point_y as usize))
        })
        .collect::<Vec<_>>();

    let sum = engine_schematic
        .part_numbers
        .iter()
        .filter(|part_number| {
            points_adjacent_to_symbols
                .iter()
                .filter(|(_, point_y)| part_number.line == *point_y)
                .filter(|(point_x, _)| part_number.start_char_index <= *point_x)
                .any(|(point_x, _)| part_number.end_char_index >= *point_x)
        })
        .map(|part_number| part_number.value)
        .sum::<u32>();

    println!("Part 1 sum: {}", sum);
}
