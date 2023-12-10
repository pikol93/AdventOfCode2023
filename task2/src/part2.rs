use itertools::Itertools;
use strum::EnumCount;

use crate::common::{parse_read_input, CubeColor};

pub fn part2(input_text: &str) {
    let sum = parse_read_input(input_text)
        .group_by(|a| a.game_id)
        .into_iter()
        .map(move |(_, rounds)| {
            let mut required_cubes = [0; CubeColor::COUNT];
            for value in rounds {
                let index = value.cube_color as usize;
                if required_cubes[index] < value.count {
                    required_cubes[index] = value.count;
                }
            }

            required_cubes.iter().product::<u32>()
        })
        .sum::<u32>();

    println!("Part 2 sum: {}", sum);
}
