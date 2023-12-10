use std::str::FromStr;

use strum_macros::EnumCount;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, EnumCount)]
pub enum CubeColor {
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
pub struct Value {
    pub game_id: u32,
    pub cube_color: CubeColor,
    pub count: u32,
}

pub fn parse_read_input(input_text: &str) -> impl Iterator<Item = Value> + '_ {
    input_text.split('\n').filter_map(read_line).flatten()
}

fn read_line(line: &str) -> Option<impl Iterator<Item = Value> + '_> {
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

fn read_game(game_id: u32, game_str: &str) -> impl Iterator<Item = Value> + '_ {
    game_str
        .split(';')
        .map(|round_str| round_str.trim_start())
        .flat_map(move |round_str| read_round(game_id, round_str))
}

fn read_round(game_id: u32, round_str: &str) -> impl Iterator<Item = Value> + '_ {
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
