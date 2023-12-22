use std::fs;

use crate::*;

#[derive(Debug)]
struct Data {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_reveals: Vec<CubeCounts>,
}

#[derive(Debug)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn get_possible_games_id_sum(data_path: &str) -> Result<u32> {
    let total_cubes = CubeCounts {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_games = get_possible_games(data_path, &total_cubes)?;
    return Ok(possible_games.iter().map(|game| game.id).sum());
}

pub fn get_power_sum_of_minimum_cubes(data_path: &str) -> Result<u32> {
    let data = parse_data(data_path)?;
    return Ok(data
        .games
        .iter()
        .map(|game| get_power_of_cubes(&get_minimum_cubes(game)))
        .sum());
}

fn get_power_of_cubes(cubes: &CubeCounts) -> u32 {
    return cubes.blue * cubes.red * cubes.green;
}

fn get_minimum_cubes(game: &Game) -> CubeCounts {
    let mut minimum_cubes = CubeCounts {
        blue: 0,
        green: 0,
        red: 0,
    };

    game.cube_reveals.iter().for_each(|reveal| {
        minimum_cubes.blue = reveal.blue.max(minimum_cubes.blue);
        minimum_cubes.red = reveal.red.max(minimum_cubes.red);
        minimum_cubes.green = reveal.green.max(minimum_cubes.green);
    });

    return minimum_cubes;
}

fn get_possible_games(data_path: &str, total_cubes: &CubeCounts) -> Result<Vec<Game>> {
    let data = parse_data(data_path)?;
    return Ok(data
        .games
        .into_iter()
        .filter(|game| {
            game.cube_reveals.iter().all(|reveal| {
                reveal.blue <= total_cubes.blue
                    && reveal.red <= total_cubes.red
                    && reveal.green <= total_cubes.green
            })
        })
        .collect::<Vec<_>>());
}

fn parse_data(path: &str) -> Result<Data> {
    let games = fs::read_to_string(path)?
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let [game_part, reveals_part] = line.split(":").collect::<Vec<_>>()[..] else {
              return Err("Line did not contain 2 parts separated by a colon".into());
            };
            let id = game_part[5..].parse::<u32>()?;
            let reveals = reveals_part
                .split(";")
                .map(|reveal| parse_reveal(reveal))
                .collect::<Result<Vec<_>>>()?;

            return Ok(Game {
                id,
                cube_reveals: reveals,
            });
        })
        .collect::<Result<Vec<_>>>()?;
    return Ok(Data { games });
}

fn parse_reveal(data: &str) -> Result<CubeCounts> {
    let counts = data
        .split(",")
        .map(|count| {
            let [count, color] = count.trim().split(" ").collect::<Vec<_>>()[..] else {
              return Err("Count did not contain 2 parts separated by whitespace".into())
            };
            return Ok((color, count));
        })
        .collect::<Result<Vec<_>>>()?;

    let mut reveal = CubeCounts {
        blue: 0,
        green: 0,
        red: 0,
    };

    counts
        .iter()
        .map(|&count| {
            match count.0 {
                "blue" => reveal.blue += count.1.parse::<u32>()?,
                "green" => reveal.green += count.1.parse::<u32>()?,
                "red" => reveal.red += count.1.parse::<u32>()?,
                _ => (),
            }
            return Ok(());
        })
        .collect::<Result<Vec<_>>>()?;

    return Ok(reveal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        let id_sum = get_possible_games_id_sum("./src/day_02/example.txt")?;
        assert_eq!(id_sum, 8);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        let id_sum = get_possible_games_id_sum("./src/day_02/input.txt")?;
        assert_eq!(id_sum, 2795);
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        let power_sum = get_power_sum_of_minimum_cubes("./src/day_02/example.txt")?;
        assert_eq!(power_sum, 2286);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        let power_sum = get_power_sum_of_minimum_cubes("./src/day_02/input.txt")?;
        assert_eq!(power_sum, 75561);
        return Ok(());
    }
}
