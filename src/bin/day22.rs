use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SpanningCoordinate {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    low_coords: (usize, usize, usize),
    high_coords: (usize, usize, usize),
    spanning_coordinate: SpanningCoordinate,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day22.txt").unwrap();

    let mut bricks = lines
        .iter()
        .map(|line| {
            let (low_coords, high_coords) = line.split_once('~').unwrap();

            let mut low_split = low_coords.split(',');
            let low_x = low_split.next().unwrap().parse::<usize>().unwrap();
            let low_y = low_split.next().unwrap().parse::<usize>().unwrap();
            let low_z = low_split.next().unwrap().parse::<usize>().unwrap();

            let mut high_split = high_coords.split(',');
            let high_x = high_split.next().unwrap().parse::<usize>().unwrap();
            let high_y = high_split.next().unwrap().parse::<usize>().unwrap();
            let high_z = high_split.next().unwrap().parse::<usize>().unwrap();

            let spanning_coordinate = if low_x < high_x {
                SpanningCoordinate::X
            } else if low_y < high_y {
                SpanningCoordinate::Y
            } else {
                SpanningCoordinate::Z
            };

            Brick {
                low_coords: (low_x, low_y, low_z),
                high_coords: (high_x, high_y, high_z),
                spanning_coordinate,
            }
        })
        .collect::<Vec<_>>();
    bricks.sort_by_key(|brick| brick.low_coords.2);
    let bricks = bricks;

    let mut solo_supporters = HashSet::<&Brick>::new();
    let mut supported_by = HashMap::<&Brick, HashSet<Option<&Brick>>>::new();
    let mut current_floor = vec![vec![(0, None); 10]; 10];
    for brick in bricks.iter() {
        match brick.spanning_coordinate {
            SpanningCoordinate::X => {
                let support_level = (brick.low_coords.0..=brick.high_coords.0)
                    .map(|x| current_floor[x][brick.low_coords.1].0)
                    .max()
                    .unwrap();
                let current_supports = HashSet::<Option<&Brick>>::from_iter(
                    (brick.low_coords.0..=brick.high_coords.0)
                        .filter(|&x| current_floor[x][brick.low_coords.1].0 == support_level)
                        .map(|x| current_floor[x][brick.low_coords.1].1),
                );
                if current_supports.len() == 1 && !current_supports.contains(&None) {
                    solo_supporters.insert(current_supports.iter().next().unwrap().unwrap());
                }
                supported_by.insert(brick, HashSet::from_iter(current_supports.into_iter()));

                let new_floor_level = support_level + brick.high_coords.2 - brick.low_coords.2 + 1;
                for x in brick.low_coords.0..=brick.high_coords.0 {
                    current_floor[x][brick.low_coords.1] = (new_floor_level, Some(brick));
                }
            }
            _ => {
                let support_level = (brick.low_coords.1..=brick.high_coords.1)
                    .map(|y| current_floor[brick.low_coords.0][y].0)
                    .max()
                    .unwrap();
                let current_supports = HashSet::<Option<&Brick>>::from_iter(
                    (brick.low_coords.1..=brick.high_coords.1)
                        .filter(|&y| current_floor[brick.low_coords.0][y].0 == support_level)
                        .map(|y| current_floor[brick.low_coords.0][y].1),
                );
                if current_supports.len() == 1 && !current_supports.contains(&None) {
                    solo_supporters.insert(current_supports.iter().next().unwrap().unwrap());
                }
                supported_by.insert(brick, HashSet::from_iter(current_supports.into_iter()));

                let new_floor_level = support_level + brick.high_coords.2 - brick.low_coords.2 + 1;
                for y in brick.low_coords.1..=brick.high_coords.1 {
                    current_floor[brick.low_coords.0][y] = (new_floor_level, Some(brick));
                }
            }
        }
    }

    let num_to_disintegrate = bricks.len() - solo_supporters.len();
    println!("Part 1: {}", num_to_disintegrate);

    let mut total_from_chains = 0;
    for (start_idx, start) in bricks.iter().enumerate() {
        let mut destroyed = HashSet::<Option<&Brick>>::new();
        destroyed.insert(Some(start));
        for brick in bricks[start_idx + 1..].iter() {
            if supported_by[brick].is_subset(&destroyed) {
                destroyed.insert(Some(brick));
            }
        }
        total_from_chains += destroyed.len() - 1;
    }
    println!("Part 2: {}", total_from_chains);
}
