use itertools::Itertools;

use crate::common::{parse_read_input, CubeColor};

pub fn part1(input_text: &str) {
    let sum = parse_read_input(input_text)
        .group_by(|a| a.game_id)
        .into_iter()
        .map(move |(game_id, rounds)| {
            // Find any value not meeting the requirements
            let cube_not_meeting_requirements = rounds.into_iter().find(|value| {
                let limit = get_limit_for_cube_color(value.cube_color);
                value.count > limit
            });

            (game_id, cube_not_meeting_requirements.is_none())
        })
        .filter(|(_, meets_requirements)| *meets_requirements)
        .map(|(game_id, _)| game_id)
        .sum::<u32>();

    println!("Part 1 sum: {}", sum);
}

#[inline]
fn get_limit_for_cube_color(cube_color: CubeColor) -> u32 {
    match cube_color {
        CubeColor::Red => 12,
        CubeColor::Green => 13,
        CubeColor::Blue => 14,
    }
}
