use std::fs;

use crate::*;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

enum Version {
    V1,
    V2,
}

pub fn get_margin_for_error_v1(data_path: &str) -> Result<u64> {
    let races = parse_data(data_path, Version::V1)?;
    return Ok(races.iter().map(get_ways_to_win).product());
}

pub fn get_margin_for_error_v2(data_path: &str) -> Result<u64> {
    let races = parse_data(data_path, Version::V2)?;
    return Ok(races.iter().map(get_ways_to_win).product());
}

fn parse_data(data_path: &str, version: Version) -> Result<Vec<Race>> {
    let data = fs::read_to_string(data_path)?;
    let mut data = data.split("\n");
    let times = data
        .next()
        .ok_or("Data must contain 2 lines")?
        .strip_prefix("Time:")
        .ok_or("Invalid prefix")?;
    let distances = data
        .next()
        .ok_or("Data must contain 2 lines")?
        .strip_prefix("Distance:")
        .ok_or("Invalid prefix")?;

    let data = match version {
        Version::V1 => {
            let times = times
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| Ok(x.parse::<u64>()?))
                .collect::<Result<Vec<_>>>()?;
            let distances = distances
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| Ok(x.parse::<u64>()?))
                .collect::<Result<Vec<_>>>()?;
            times
                .into_iter()
                .zip(distances.into_iter())
                .collect::<Vec<_>>()
        }
        Version::V2 => {
            let time = times.replace(" ", "").parse::<u64>()?;
            let distance = distances.replace(" ", "").parse::<u64>()?;
            vec![(time, distance)]
        }
    };

    return data
        .into_iter()
        .map(|(time, distance)| Ok(Race { time, distance }))
        .collect::<Result<Vec<_>>>();
}

fn get_ways_to_win(race: &Race) -> u64 {
    // Calculate ways to match current time by solving x^2 - xt + d = 0
    // x is time held, t is total time, d is winning distance
    let determinant = ((race.time * race.time - 4 * race.distance) as f64).sqrt();

    // No solutions implies that current record is impossible
    if determinant.is_nan() {
        return 0;
    }

    let lower = (((race.time as f64 - determinant) / 2f64) + 1f64).floor();
    let upper = (((race.time as f64 + determinant) / 2f64) - 1f64).ceil();

    // lower greater than upper implies that current record can't be beaten
    if lower > upper {
        return 0;
    }

    return (upper - lower) as u64 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(288, get_margin_for_error_v1("./src/day_06/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(1195150, get_margin_for_error_v1("./src/day_06/input.txt")?);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(71503, get_margin_for_error_v2("./src/day_06/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(42550411, get_margin_for_error_v2("./src/day_06/input.txt")?);
        return Ok(());
    }
}
