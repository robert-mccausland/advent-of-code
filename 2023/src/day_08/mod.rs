use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::*;

struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

enum Direction {
    Left,
    Right,
}

pub fn get_ghost_steps(data_path: &str) -> Result<u64> {
    let data = parse_data(data_path)?;
    let cycles = data
        .nodes
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|start| {
            let mut history = HashSet::new();
            let mut current = start.as_str();
            let mut steps = 0;
            let mut finishes = vec![];

            while !history.contains(&(current, steps % data.instructions.len())) {
                history.insert((current, steps % data.instructions.len()));
                if current.ends_with("Z") {
                    finishes.push((current, steps))
                }
                current = get_next_node(current, steps, &data)?;
                steps += 1;
                if steps > 1_000_000 {
                    return Err("Max number of steps reached".into());
                }
            }

            // Data always ends up being a single cycle which ends at the finish, so we use that
            // fact here to make things a bit easier.
            return Ok(finishes[0].1);
        })
        .collect::<Result<Vec<_>>>()?;

    return Ok(cycles
        .iter()
        .fold(1, |a, b| num::integer::lcm(a, *b as u64)));
}

pub fn get_steps(data_path: &str) -> Result<u32> {
    let data = parse_data(data_path)?;

    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        current = get_next_node(current, steps, &data)?;
        steps += 1;

        if steps > 1_000_000 {
            return Err("Max number of steps reached".into());
        }
    }

    return Ok(u32::try_from(steps)?);
}

fn get_next_node<'a>(current: &str, steps: usize, map: &'a Map) -> Result<&'a str> {
    let next = map
        .nodes
        .get(current)
        .ok_or(format!("Could not find node: {:}", current))?;
    return Ok(match map.instructions[steps % map.instructions.len()] {
        Direction::Left => &next.0,
        Direction::Right => &next.1,
    });
}

fn parse_data<'a>(data_path: &str) -> Result<Map> {
    fn parse_direction(direction: char) -> Result<Direction> {
        match direction {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err("Invalid direction".into()),
        }
    }

    let data = fs::read_to_string(data_path)?;
    let mut data = data.split("\n");
    let instructions = data
        .next()
        .ok_or("Data must include at least one line")?
        .chars()
        .map(parse_direction)
        .collect::<Result<Vec<_>>>()?;

    let mut nodes = HashMap::new();
    for node in data.filter(|line| !line.is_empty()) {
        let base = node[0..3].to_owned();
        let left = node[7..10].to_owned();
        let right = node[12..15].to_owned();
        nodes.insert(base, (left, right));
    }

    return Ok(Map {
        instructions,
        nodes,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() -> Result<()> {
        assert_eq!(2, get_steps("./src/day_08/example_1.txt")?);
        return Ok(());
    }

    #[test]
    fn part1_example2() -> Result<()> {
        assert_eq!(6, get_steps("./src/day_08/example_2.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(19631, get_steps("./src/day_08/input.txt")?);
        return Ok(());
    }

    #[test]
    fn part2_example3() -> Result<()> {
        assert_eq!(6, get_ghost_steps("./src/day_08/example_3.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(21003205388413, get_ghost_steps("./src/day_08/input.txt")?);
        return Ok(());
    }
}
