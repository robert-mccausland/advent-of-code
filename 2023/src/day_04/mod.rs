use std::{collections::HashMap, fs};

use crate::*;

#[derive(Debug)]
struct Scratchcard {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

pub fn get_total_points(data_path: &str) -> Result<u32> {
    let scratchcards = parse_data(data_path)?;
    return Ok(scratchcards.iter().map(get_card_value).sum());
}

pub fn get_total_scratchcards(data_path: &str) -> Result<u32> {
    let scratchcards = parse_data(data_path)?;
    let mut copy_counts: HashMap<u32, u32> = HashMap::new();

    for card in scratchcards.iter() {
        let matches: u32 = get_card_matches(card).try_into()?;
        for id in card.id + 1..card.id + 1 + matches {
            let total_count = 1 + copy_counts.get(&card.id).unwrap_or(&0);
            match copy_counts.get_mut(&id) {
                Some(count) => *count += total_count,
                None => {
                    copy_counts.insert(id, total_count);
                }
            };
        }
    }

    return Ok(copy_counts.values().sum::<u32>() + u32::try_from(scratchcards.len())?);
}

fn get_card_matches(card: &Scratchcard) -> usize {
    card.numbers
        .iter()
        .filter(|number| card.winning_numbers.contains(number))
        .count()
}

fn get_card_value(card: &Scratchcard) -> u32 {
    let matches = get_card_matches(card);
    return if matches == 0 { 0 } else { 1 << (matches - 1) };
}

fn parse_data(data_path: &str) -> Result<Vec<Scratchcard>> {
    return Ok(fs::read_to_string(data_path)?
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_scratchcard)
        .collect::<Result<Vec<_>>>()?);
}

fn parse_scratchcard(line: &str) -> Result<Scratchcard> {
    let [part1, part2] = line.split(":").collect::<Vec<_>>()[..] else {
        return Err("Line was not 2 parts separated by a colon".into());
    };

    let id = part1[4..].trim().parse::<u32>()?;
    let mut part2 = part2.split("|").map(|x| parse_numbers(x));
    let winning_numbers = part2.next().ok_or("Invalid line format")??;
    let numbers = part2.next().ok_or("Invalid line format")??;

    return Ok(Scratchcard {
        id,
        winning_numbers,
        numbers,
    });
}

fn parse_numbers(numbers: &str) -> Result<Vec<u32>> {
    return Ok(numbers
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| Ok(x.trim().parse::<u32>()?))
        .collect::<Result<Vec<_>>>()?);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(13, get_total_points("./src/day_04/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(22897, get_total_points("./src/day_04/input.txt")?);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(30, get_total_scratchcards("./src/day_04/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(5095824, get_total_scratchcards("./src/day_04/input.txt")?);
        return Ok(());
    }
}
