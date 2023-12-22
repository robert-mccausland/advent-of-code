use std::{collections::HashMap, fs};

use crate::*;

struct PartNumber {
    value: u32,
    symbol_position: (usize, usize),
    symbol: char,
}

pub fn get_part_number_sum(data_path: &str) -> Result<u32> {
    let schematic = fs::read_to_string(data_path)?
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part_numbers = get_part_numbers(&schematic)?;

    return Ok(part_numbers.iter().map(|x| x.value).sum());
}

pub fn get_gear_ratios(data_path: &str) -> Result<u32> {
    let schematic = fs::read_to_string(data_path)?
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let possible_gears = get_part_numbers(&schematic)?
        .into_iter()
        .filter(|x| x.symbol == '*')
        .collect::<Vec<_>>();

    let mut gears: HashMap<(usize, usize), Vec<&PartNumber>> = HashMap::new();
    for possible_gear in possible_gears.iter() {
        if gears.contains_key(&possible_gear.symbol_position) {
            gears
                .get_mut(&possible_gear.symbol_position)
                .unwrap()
                .push(possible_gear);
        } else {
            gears.insert(possible_gear.symbol_position, vec![possible_gear]);
        }
    }

    let gear_ratios = gears.values().filter_map(|x| match x.len() {
        2 => Some(x[0].value * x[1].value),
        _ => None,
    });

    return Ok(gear_ratios.sum());
}

fn get_part_numbers(schematic: &Vec<Vec<char>>) -> Result<Vec<PartNumber>> {
    let numbers = schematic
        .iter()
        .enumerate()
        .flat_map(|(index, line)| {
            get_numbers_positions(line)
                .into_iter()
                .map(move |position| (index, position))
        })
        .filter_map(
            |(index, position)| match get_part_number(index, position, schematic) {
                Ok(value) => value.map(|x| Ok(x)),
                Err(err) => Some(Err(err)),
            },
        )
        .collect::<Result<Vec<_>>>()?;

    return Ok(numbers);
}

fn get_numbers_positions(line: &Vec<char>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = vec![];
    let mut was_previous_numeric = false;
    for (index, char) in line.iter().enumerate() {
        if char.is_numeric() {
            if was_previous_numeric {
                positions.last_mut().unwrap().1 += 1;
            } else {
                positions.push((index, index))
            }
            was_previous_numeric = true
        } else {
            was_previous_numeric = false
        }
    }

    return positions;
}

fn get_part_number(
    line_number: usize,
    line_position: (usize, usize),
    schematic: &Vec<Vec<char>>,
) -> Result<Option<PartNumber>> {
    let top = if line_number == 0 { 0 } else { line_number - 1 };
    let bottom = (line_number + 1).min(schematic.len() - 1);
    let left = if line_position.0 == 0 {
        0
    } else {
        line_position.0 - 1
    };
    let right = (line_position.1 + 1).min(schematic[0].len() - 1);

    for i in top..=bottom {
        for j in left..=right {
            if is_symbol(schematic[i][j]) {
                return Ok(Some(PartNumber {
                    symbol_position: (i, j),
                    symbol: schematic[i][j],
                    value: String::from_iter(
                        schematic[line_number][line_position.0..=line_position.1].iter(),
                    )
                    .parse::<u32>()?,
                }));
            }
        }
    }
    return Ok(None);
}

fn is_symbol(char: char) -> bool {
    !char.is_numeric() && char != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(4361, get_part_number_sum("./src/day_03/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(554003, get_part_number_sum("./src/day_03/input.txt")?);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(467835, get_gear_ratios("./src/day_03/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(87263515, get_gear_ratios("./src/day_03/input.txt")?);
        return Ok(());
    }
}
