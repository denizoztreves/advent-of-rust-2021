use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::read;
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::str::{FromStr, Lines};

use Direction::DOWN;
use Direction::FORWARDS;
use Direction::UP;

const FILENAME: &str = "resources/day2_input.txt";

pub fn part_1() -> i32{
    let (horizontal, depth) = read_commands()
        .into_iter()
        .fold((0, 0), |result, command| {
            let direction = command.0;
            let amount = command.1;
            match direction {
                FORWARDS => (result.0 + amount, result.1),
                UP => (result.0, result.1 - amount),
                DOWN => (result.0, result.1 + amount),
            }
        });

    return horizontal * depth;
}

pub fn part_2() -> i32 {
    let (horizontal, depth, _) = read_commands()
        .into_iter()
        .fold((0, 0, 0), |result, command| {
            let direction = command.0;
            let amount = command.1;
            match direction {
                FORWARDS => (result.0 + amount, result.1 + (result.2 * amount), result.2),
                UP => (result.0, result.1, result.2 - amount),
                DOWN => (result.0, result.1, result.2 + amount),
            }
        });
    return horizontal * depth;
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    FORWARDS,
    UP,
    DOWN
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "forward" => Ok(FORWARDS),
            "up" => Ok(UP),
            "down" => Ok(DOWN),
            _ => Err(())
        };
        return result
    }
}

fn read_commands() -> Vec<(Direction, i32)> {
    return fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|command| {
            let mut split = command.split_whitespace();
            let direction = Direction::from_str(split.next().unwrap())
                .expect("Something went wrong parsing direction");
            let amount = split.next().unwrap().trim().parse().unwrap();
            (direction, amount)
        }).collect()
}

#[cfg(test)]
mod test {

}
