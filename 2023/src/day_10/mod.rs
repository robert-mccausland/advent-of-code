use std::fs;

use crate::*;

struct Map {
    start: (usize, usize),
    tiles: Vec<Vec<char>>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Connection {
    North,
    South,
    East,
    West,
}

fn get_opposite(connection: Connection) -> Connection {
    match connection {
        Connection::North => Connection::South,
        Connection::South => Connection::North,
        Connection::West => Connection::East,
        Connection::East => Connection::West,
    }
}

fn follow_connection(start: (usize, usize), connection: Connection) -> (usize, usize) {
    match connection {
        Connection::North => (start.0 + 1, start.1),
        Connection::South => (start.0 - 1, start.1),
        Connection::West => (start.0, start.1 + 1),
        Connection::East => (start.0, start.1 - 1),
    }
}

fn find_length(data: &Vec<Vec<char>>) -> Result<u32> {
    let mut start = None;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'S' {
                start = Some((i, j));
            }
        }
    }
    let start = start.ok_or("No start found")?;
    let mut start_connections = vec![];
    let neighbors = [
        (
            Connection::North,
            get_connections(data[start.0 + 1][start.1])?,
        ),
        (
            Connection::South,
            get_connections(data[start.0 - 1][start.1])?,
        ),
        (
            Connection::West,
            get_connections(data[start.0][start.1 + 1])?,
        ),
        (
            Connection::East,
            get_connections(data[start.0][start.1 - 1])?,
        ),
    ];

    for neighbor in neighbors {
        if neighbor.1.contains(&get_opposite(neighbor.0)) {
            start_connections.push(neighbor.0)
        }
    }

    let start_connections: [Connection; 2] = start_connections
        .try_into()
        .map_err(|_| "Exactly 2 pipes must point at the start tile")?;

    let mut current_position = start;
    let mut next_connection = start_connections[0];

    let mut count = 0;
    loop {
        current_position = follow_connection(current_position, next_connection);
        next_connection = get_connections(data[current_position.0][current_position.1])?
            .into_iter()
            .find(|x| *x != get_opposite(next_connection))
            .ok_or("No valid connection")?;
        count += 1;
    }

    todo!();
}

fn get_connections(tile: char) -> Result<[Connection; 2]> {
    Ok(match tile {
        '|' => [Connection::North, Connection::South],
        '-' => [Connection::East, Connection::West],
        'L' => [Connection::North, Connection::East],
        'J' => [Connection::North, Connection::West],
        '7' => [Connection::South, Connection::West],
        'F' => [Connection::South, Connection::East],
        _ => return Err("Invalid pipe")?,
    })
}

fn parse_data(data_path: &str) -> Result<Vec<Vec<char>>> {
    return Ok(fs::read_to_string(data_path)?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(114, get_next_sum("./src/day_10/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(1901217887, get_next_sum("./src/day_10/input.txt")?);
        return Ok(());
    }
}
