use std::fs;

use crate::*;

pub fn get_calibration_value(
    document_path: &str,
    line_parser: fn(&str) -> Result<u32>,
) -> Result<u32> {
    let document = fs::read_to_string(document_path)?;
    let value = document
        .split("\n")
        .filter(|line| !line.is_empty())
        .try_fold(0, |sum, line| {
            line_parser(line).and_then(|result| Ok(sum + result))
        })?;
    return Ok(value);
}

pub fn p1_parser(line: &str) -> Result<u32> {
    let first_numeric = line
        .chars()
        .find(|char| char.is_numeric())
        .ok_or("Line has no numeric characters")?;
    let last_numeric = line
        .chars()
        .rev()
        .find(|char| char.is_numeric())
        .ok_or("Line has no numeric characters")?;

    return Ok(String::from_iter(vec![first_numeric, last_numeric]).parse()?);
}

pub fn p2_parser(line: &str) -> Result<u32> {
    let first_numeric = find_digit(line, false)?;
    let last_numeric = find_digit(line, true)?;
    return Ok((first_numeric * 10) + last_numeric);
}

const VALID_STRINGS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn lookup_string(value: &str) -> Result<u32> {
    let position = VALID_STRINGS.iter().position(|&x| x == value);
    return match position {
        Some(position) => Ok((position % 9 + 1).try_into().unwrap()),
        None => Err("Invalid string value".into()),
    };
}

fn find_digit(line: &str, reversed: bool) -> Result<u32> {
    let mut index = 1;
    while index <= line.len() {
        let current = if reversed {
            &line[line.len() - index..line.len()]
        } else {
            &line[0..index]
        };
        let matched = VALID_STRINGS.iter().find(|&value| current.contains(*value));
        match matched {
            Some(value) => return lookup_string(value),
            None => {}
        }
        index += 1;
    }

    return Err("String did not contain a valid digit".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(
            super::get_calibration_value("./src/day_01/example_p1.txt", p1_parser)?,
            142
        );
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(
            super::get_calibration_value("./src/day_01/input.txt", p1_parser)?,
            54968
        );
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(
            super::get_calibration_value("./src/day_01/example_p2.txt", p2_parser)?,
            281
        );
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(
            super::get_calibration_value("./src/day_01/input.txt", p2_parser)?,
            54094
        );
        return Ok(());
    }
}
