use std::fs;

use crate::*;

pub fn get_next_sum(data_path: &str) -> Result<i32> {
    let data = parse_data(data_path)?
        .into_iter()
        .map(|x| Ok(extrapolate(x)?.1))
        .collect::<Result<Vec<_>>>()?;

    return Ok(data.iter().sum::<i32>());
}

pub fn get_previous_sum(data_path: &str) -> Result<i32> {
    let data = parse_data(data_path)?
        .into_iter()
        .map(|x| Ok(extrapolate(x)?.0))
        .collect::<Result<Vec<_>>>()?;

    return Ok(data.iter().sum::<i32>());
}

fn extrapolate(sequence: Vec<i32>) -> Result<(i32, i32)> {
    let mut sequences = vec![sequence];
    while !sequences.last().unwrap().iter().all(|x| *x == 0) {
        let new = get_difference_sequence(sequences.last().unwrap());
        sequences.push(new);
    }

    let mut next = 0;
    let mut previous = 0;
    sequences.reverse();
    for sequence in sequences {
        next = *sequence.last().ok_or("Sequence must have elements")? + next;
        previous = *sequence.first().ok_or("Sequence must have elements")? - previous;
    }

    return Ok((previous, next));
}

fn get_difference_sequence(sequence: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut iter = sequence.iter();

    let mut left = iter.next();
    let mut right = iter.next();
    while right != None {
        result.push(right.unwrap() - left.unwrap());
        left = right;
        right = iter.next();
    }

    return result;
}

fn parse_data(data_path: &str) -> Result<Vec<Vec<i32>>> {
    return fs::read_to_string(data_path)?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|value| Ok(value.parse::<i32>()?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(114, get_next_sum("./src/day_09/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(1901217887, get_next_sum("./src/day_09/input.txt")?);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(2, get_previous_sum("./src/day_09/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(905, get_previous_sum("./src/day_09/input.txt")?);
        return Ok(());
    }
}
