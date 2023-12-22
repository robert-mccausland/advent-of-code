use std::{cmp::Ordering, collections::HashMap, fs};

use crate::*;

struct Hand {
    value: String,
    bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub fn get_winnings(data_path: &str, use_jokers: bool) -> Result<u32> {
    let mut hands = parse_data(data_path)?;
    hands.sort_by(|left, right| compare_hands(&left.value, &right.value, use_jokers));

    let mut rank = 1;
    let mut total = 0;
    for hand in hands {
        total += hand.bid * rank;
        rank += 1;
    }

    return Ok(total);
}

fn parse_data(data_path: &str) -> Result<Vec<Hand>> {
    fn parse_hand(line: &str) -> Result<Hand> {
        let [value, bid] = line.split(" ").collect::<Vec<_>>()[..] else {
          return Err("Line must the value and bid separated by a space".into());
        };

        return Ok(Hand {
            value: value.to_owned(),
            bid: bid.parse::<u32>()?,
        });
    }

    let data = fs::read_to_string(data_path)?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(parse_hand)
        .collect::<Result<Vec<_>>>()?;

    return Ok(data);
}

fn get_hand_type(hand: &str, use_jokers: bool) -> HandType {
    let mut counts = HashMap::new();
    let mut jokers = 0;
    for card in hand.chars() {
        if get_card_type(card, use_jokers) == CardType::Joker {
            jokers += 1;
            continue;
        }

        if let Some(count) = counts.get_mut(&card) {
            *count += 1;
        } else {
            counts.insert(card, 1);
        }
    }

    let mut counts = counts.values().map(|x| x.clone()).collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    // We can make the best hand by adding making the jokers equal to whatever we have the most
    // duplicates of already.

    // Handle edge case if we have 5 jokers.
    if counts.is_empty() {
        counts.push(jokers);
    } else {
        counts[0] += jokers;
    }

    return match (counts[0], counts.get(1).unwrap_or(&0)) {
        (5, 0) => HandType::FiveOfAKind,
        (4, 1) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, 1) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, 1) => HandType::OnePair,
        (1, 1) => HandType::HighCard,
        _ => panic!("Invalid hand: {:}", hand),
    };
}

fn get_card_type(card: char, use_jokers: bool) -> CardType {
    match card {
        'A' => CardType::Ace,
        'K' => CardType::King,
        'Q' => CardType::Queen,
        'J' => {
            if use_jokers {
                CardType::Joker
            } else {
                CardType::Jack
            }
        }
        'T' => CardType::Ten,
        '9' => CardType::Nine,
        '8' => CardType::Eight,
        '7' => CardType::Seven,
        '6' => CardType::Six,
        '5' => CardType::Five,
        '4' => CardType::Four,
        '3' => CardType::Three,
        '2' => CardType::Two,
        _ => panic!("Invalid card: {:}", card),
    }
}

fn compare_hands(left: &str, right: &str, use_jokers: bool) -> Ordering {
    let left_type = get_hand_type(left, use_jokers);
    let right_type = get_hand_type(right, use_jokers);

    if left_type != right_type {
        return left_type.cmp(&right_type);
    }

    for (left_char, right_char) in left.chars().zip(right.chars()) {
        if left_char != right_char {
            return get_card_type(left_char, use_jokers)
                .cmp(&get_card_type(right_char, use_jokers));
        }
    }

    return Ordering::Equal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(6440, get_winnings("./src/day_07/example.txt", false)?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(248113761, get_winnings("./src/day_07/input.txt", false)?);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(5905, get_winnings("./src/day_07/example.txt", true)?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(246285222, get_winnings("./src/day_07/input.txt", true)?);
        return Ok(());
    }
}
