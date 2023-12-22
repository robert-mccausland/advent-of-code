use std::fs;

use crate::*;

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<(u64, u64)>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    destination: (u64, u64),
    source: (u64, u64),
}

pub fn get_lowest_seed_location_v1(data_path: &str) -> Result<u64> {
    get_lowest_seed_location(data_path, Version::V1)
}

pub fn get_lowest_seed_location_v2(data_path: &str) -> Result<u64> {
    get_lowest_seed_location(data_path, Version::V2)
}

enum Version {
    V1,
    V2,
}

fn get_lowest_seed_location(data_path: &str, version: Version) -> Result<u64> {
    let almanac = parse_data(data_path)?;
    let seed_ranges: Vec<(u64, u64)> = match version {
        Version::V1 => almanac
            .seed_ranges
            .iter()
            .flat_map(|range| [(range.0, 1), (range.1, 1)])
            .collect::<Vec<_>>(),
        Version::V2 => almanac
            .seed_ranges
            .iter()
            .map(|range| (range.0, range.1))
            .collect::<Vec<_>>(),
    };

    let seed_locations = seed_ranges
        .iter()
        .map(|seed_range| Ok(map_ranges(*seed_range, "seed", "location", &almanac)?))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    return Ok(seed_locations
        .iter()
        .map(|range| range.0)
        .min()
        .ok_or("No seeds found")?);
}

fn map_ranges(
    initial_range: (u64, u64),
    initial_type: &str,
    target_type: &str,
    almanac: &Almanac,
) -> Result<Vec<(u64, u64)>> {
    let mut current_type = initial_type;
    let mut current_range = vec![initial_range];

    while current_type != target_type {
        let map = almanac
            .maps
            .iter()
            .find(|map| map.source == current_type)
            .ok_or("Could not find map for type")?;

        current_range = current_range
            .into_iter()
            .flat_map(|range| map_item_range(range, map))
            .collect::<Vec<_>>();
        current_type = &map.destination;
    }

    return Ok(current_range);
}

fn parse_data(data_path: &str) -> Result<Almanac> {
    let data = fs::read_to_string(data_path)?;
    let mut lines = data.split("\n");
    let mut seeds = lines.next().ok_or("data must contain at least 1 line")?[7..]
        .split(" ")
        .map(|x| x.parse::<u64>());
    let mut seed_ranges = vec![];
    while let Some(seed) = seeds.next() {
        seed_ranges.push((
            seed?,
            seeds.next().ok_or("must have an even number of seeds")??,
        ))
    }

    let mut maps: Vec<Map> = vec![];
    for line in lines.filter(|line| !line.is_empty()) {
        let Some(map_header) = line.strip_suffix(" map:") else {
          let map = maps.last_mut().ok_or("First line must be a header")?;
          let [destination_start, source_start, length] = line.split(" ").map(|x| Ok(x.parse::<u64>()?)).collect::<Result<Vec<_>>>()?[..] else {
            return Err("map line must contain 3 numbers".into());
          };
          map.ranges.push(MapRange { destination: (destination_start, length), source:(source_start, length), });
          continue;
        };
        let [source, destination] = map_header
            .split("-to-")
            .collect::<Vec<_>>()[..] else {
              return Err("map header must look like <source>-to-<destination> map:".into())
            };
        maps.push(Map {
            source: source.to_owned(),
            destination: destination.to_owned(),
            ranges: vec![],
        })
    }

    return Ok(Almanac { seed_ranges, maps });
}

fn map_item_range(item_range: (u64, u64), map: &Map) -> Vec<(u64, u64)> {
    let mut maps = vec![];

    // Create a map for each range in the item map
    for range in map.ranges.iter() {
        let start = item_range.0.max(range.source.0);
        let end = (item_range.0 + item_range.1).min(range.source.0 + range.source.1);

        // Its possible for start to be greater than end, this means that the ranges to not overlap.
        if start >= end {
            continue;
        }
        let length = end - start;
        let offset = start - range.source.0;

        maps.push((
            (range.source.0 + offset, length),
            (range.destination.0 + offset, length),
        ));
    }
    maps.sort_by_key(|map| map.0);

    // Create maps for the gaps between ranges
    let mut iter = maps.iter().map(|x| x.0);
    let mut left: Option<(u64, u64)> = None;
    let mut right = iter.next();
    let mut gaps: Vec<(u64, u64)> = vec![];

    loop {
        let left_position = left.map(|x| x.0 + x.1).unwrap_or(item_range.0);
        let right_position = right.map(|x| x.0).unwrap_or(item_range.0 + item_range.1);
        if left_position < right_position {
            gaps.push((left_position, right_position - left_position));
        }

        left = right;
        right = iter.next();

        if left == None {
            break;
        }
    }

    let mut result = maps.iter().map(|map| map.1).collect::<Vec<_>>();
    result.append(&mut gaps);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(35, get_lowest_seed_location_v1("./src/day_05/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(
            178159714,
            get_lowest_seed_location_v1("./src/day_05/input.txt")?
        );
        return Ok(());
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(46, get_lowest_seed_location_v2("./src/day_05/example.txt")?);
        return Ok(());
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(
            100165128,
            get_lowest_seed_location_v2("./src/day_05/input.txt")?
        );
        return Ok(());
    }
}
