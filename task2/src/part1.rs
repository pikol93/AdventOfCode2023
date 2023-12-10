use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            &_ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Value {
    game_id: u32,
    cube_color: CubeColor,
    count: u32,
}

pub fn part1(input_text: &str) {
    let sum = input_text
        .split('\n')
        .filter_map(read_line)
        .flatten()
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

    println!("Sum: {}", sum);
}

fn read_line(line: &str) -> Option<impl Iterator<Item=Value> + '_> {
    let Some((game_id_str, game_str)) = line.split_once(':') else {
        return None;
    };

    let Some(game_id) = read_game_id(game_id_str) else {
        return None;
    };

    let game_str = game_str.trim();
    Some(read_game(game_id, game_str))
}

fn read_game_id(game_id_str: &str) -> Option<u32> {
    let Some((_, right)) = game_id_str.split_once(' ') else {
        return None;
    };

    u32::from_str(right).ok()
}

fn read_game(game_id: u32, game_str: &str) -> impl Iterator<Item=Value> + '_ {
    game_str
        .split(';')
        .map(|round_str| round_str.trim_start())
        .flat_map(move |round_str| read_round(game_id, round_str))
}

fn read_round(game_id: u32, round_str: &str) -> impl Iterator<Item=Value> + '_ {
    round_str
        .split(',')
        .map(|value_str| value_str.trim_start())
        .filter_map(|value_str| value_str.split_once(' '))
        .filter_map(move |(count_str, cube_color_str)| {
            let Ok(count) = u32::from_str(count_str) else {
                return None;
            };

            let Ok(cube_color) = CubeColor::from_str(cube_color_str) else {
                return None;
            };

            Some(Value {
                game_id,
                cube_color,
                count,
            })
        })
}

#[inline]
fn get_limit_for_cube_color(cube_color: CubeColor) -> u32 {
    match cube_color {
        CubeColor::Red => 12,
        CubeColor::Green => 13,
        CubeColor::Blue => 14,
    }
}
