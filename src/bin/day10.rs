use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    South,
    West,
    East,
    North,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    tile_type: TileType,
    in_loop: bool,
}

#[derive(Clone, Debug)]
struct TileMap {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

impl TileMap {
    fn new(height: usize, width: usize) -> TileMap {
        TileMap {
            height,
            width,
            tiles: vec![
                Tile {
                    tile_type: TileType::Unknown,
                    in_loop: false,
                };
                height * width
            ],
        }
    }
}

impl Index<(usize, usize)> for TileMap {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tiles[index.0 * self.width + index.1]
    }
}

impl IndexMut<(usize, usize)> for TileMap {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[index.0 * self.width + index.1]
    }
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day10.txt").unwrap();

    let mut tile_map = TileMap::new(lines[0].len(), lines.len());
    let mut start = (0, 0);
    for (row, line) in lines.iter().enumerate() {
        for (col, character) in line.chars().enumerate() {
            let tile_type = match character {
                '|' => TileType::Vertical,
                '-' => TileType::Horizontal,
                'L' => TileType::NEBend,
                'J' => TileType::NWBend,
                '7' => TileType::SWBend,
                'F' => TileType::SEBend,
                '.' => TileType::Ground,
                'S' => {
                    start = (row, col);
                    TileType::Start
                }
                _ => panic!("bad tile symbol"),
            };
            let in_loop = if tile_type == TileType::Start {
                start = (row, col);
                true
            } else {
                false
            };
            tile_map[(row, col)] = Tile { tile_type, in_loop };
        }
    }

    let mut length = 0;
    // my input has S as a south-west bend
    let mut direction = Direction::South;
    let mut current = (start.0 + 1, start.1);
    while current != start {
        length += 1;
        tile_map[current].in_loop = true;
        (current, direction) = match (tile_map[current].tile_type, direction) {
            (TileType::Vertical, Direction::South) => ((current.0 + 1, current.1), direction),
            (TileType::Vertical, Direction::North) => ((current.0 - 1, current.1), direction),
            (TileType::Horizontal, Direction::East) => ((current.0, current.1 + 1), direction),
            (TileType::Horizontal, Direction::West) => ((current.0, current.1 - 1), direction),
            (TileType::NEBend, Direction::South) => ((current.0, current.1 + 1), Direction::East),
            (TileType::NEBend, Direction::West) => ((current.0 - 1, current.1), Direction::North),
            (TileType::NWBend, Direction::South) => ((current.0, current.1 - 1), Direction::West),
            (TileType::NWBend, Direction::East) => ((current.0 - 1, current.1), Direction::North),
            (TileType::SWBend, Direction::North) => ((current.0, current.1 - 1), Direction::West),
            (TileType::SWBend, Direction::East) => ((current.0 + 1, current.1), Direction::South),
            (TileType::SEBend, Direction::North) => ((current.0, current.1 + 1), Direction::East),
            (TileType::SEBend, Direction::West) => ((current.0 + 1, current.1), Direction::South),
            _ => panic!("ran into a wall!"),
        };
    }

    let halfway = (length + 1) / 2;
    println!("Part A: {}", halfway);

    // based on the parity argument for the Jordan curve theorem
    // we'll walk just above the mid-line of each row as we maintain an `inside` variable
    let mut num_inside = 0;
    for row in 0..tile_map.height {
        let mut inside = false;
        for col in 0..tile_map.width {
            let tile = tile_map[(row, col)];
            if tile.in_loop {
                match tile.tile_type {
                    TileType::Vertical | TileType::NEBend | TileType::NWBend => {
                        inside = !inside;
                    }
                    _ => {}
                }
            } else if inside {
                num_inside += 1;
            }
        }
    }
    println!("Part B: {}", num_inside);
}
