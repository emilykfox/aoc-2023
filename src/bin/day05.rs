struct Map {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

#[derive(Copy, Clone)]
struct MappedRange {
    start: i64,
    length: i64,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day05.txt").unwrap();

    let starting_seeds = lines[0]
        .split(' ')
        .filter_map(|string| string.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut groups = lines.split(|line| line.is_empty());
    groups.next();

    let maps = groups
        .map(|group| {
            let mut map = group
                .iter()
                .skip(1)
                .map(|line| {
                    let mut parts = line.split(' ');
                    let destination_start = parts.next().unwrap().parse().unwrap();
                    let source_start = parts.next().unwrap().parse().unwrap();
                    let length = parts.next().unwrap().parse().unwrap();
                    Map {
                        destination_start,
                        source_start,
                        length,
                    }
                })
                .collect::<Vec<Map>>();
            map.sort_by_key(|map| map.source_start);
            map
        })
        .collect::<Vec<_>>();

    let min_location = starting_seeds
        .iter()
        .map(|seed| {
            let mut mapped = *seed;
            for map_set in maps.iter() {
                // try to map mapped
                for map in map_set.iter() {
                    if map.source_start <= mapped && mapped < map.source_start + map.length {
                        mapped = map.destination_start + mapped - map.source_start;
                        break;
                    }
                }
                // or we use the same number
            }
            mapped
        })
        .min()
        .unwrap();

    println!("Part A: {}", min_location);

    let mut mapped_ranges = (0..10)
        .map(|seed_group| MappedRange {
            start: starting_seeds[2 * seed_group],
            length: starting_seeds[2 * seed_group + 1],
        })
        .collect::<Vec<_>>();
    for map_set in maps.iter() {
        let mut new_ranges = Vec::new();
        for mapped_range in mapped_ranges.iter() {
            let mut range_left = *mapped_range;
            for map in map_set.iter() {
                let start = range_left.start.min(map.source_start);
                let end = (range_left.start + range_left.length - 1).min(map.source_start - 1);
                if start <= end {
                    let new_length = end - start + 1;
                    new_ranges.push(MappedRange {
                        start,
                        length: new_length,
                    });
                    range_left = MappedRange {
                        start: start + 1,
                        length: range_left.length - new_length,
                    };
                }
                let start = mapped_range.start.max(map.source_start);
                let end = (mapped_range.start + mapped_range.length - 1)
                    .min(map.source_start + map.length - 1);
                if start <= end {
                    new_ranges.push(MappedRange {
                        start: map.destination_start + start - map.source_start,
                        length: end - start + 1,
                    });
                    range_left = MappedRange {
                        start: end + 1,
                        length: range_left.length - (end - start + 1),
                    };
                }
            }
            new_ranges.push(MappedRange {
                start: range_left.start,
                length: range_left.length,
            });
        }
        mapped_ranges = new_ranges;
    }
    let min_location_again = mapped_ranges
        .iter()
        .map(|mapped_range| mapped_range.start)
        .min()
        .unwrap();

    println!("Part B: {}", min_location_again);
}
