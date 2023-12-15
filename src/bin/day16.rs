use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct LightUnit {
    row_idx: isize,
    col_idx: isize,
    direction: Direction,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day16.txt").unwrap();

    let height = lines.len();
    let width = lines[0].len();
    let map = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starting_units = vec![
        (0..width)
            .map(|col_idx| LightUnit {
                row_idx: 0,
                col_idx: col_idx as isize,
                direction: Direction::Down,
            })
            .collect::<Vec<_>>(),
        (0..height)
            .map(|row_idx| LightUnit {
                row_idx: row_idx as isize,
                col_idx: 0,
                direction: Direction::Right,
            })
            .collect::<Vec<_>>(),
        (0..width)
            .map(|col_idx| LightUnit {
                row_idx: height as isize - 1,
                col_idx: col_idx as isize,
                direction: Direction::Up,
            })
            .collect::<Vec<_>>(),
        (0..height)
            .map(|row_idx| LightUnit {
                row_idx: row_idx as isize,
                col_idx: width as isize - 1,
                direction: Direction::Left,
            })
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let mut max_energized = 0;
    for start in starting_units {
        let mut energized = vec![vec![false; width]; height];
        let mut reachable = HashSet::new();
        let mut light_stack = vec![start];
        while let Some(light_unit) = light_stack.pop() {
            energized[light_unit.row_idx as usize][light_unit.col_idx as usize] = true;
            let new_units = match (
                map[light_unit.row_idx as usize][light_unit.col_idx as usize],
                light_unit.direction,
            ) {
                ('.', Direction::Right) => vec![LightUnit {
                    col_idx: light_unit.col_idx + 1,
                    ..light_unit
                }],
                ('.', Direction::Down) => vec![LightUnit {
                    row_idx: light_unit.row_idx + 1,
                    ..light_unit
                }],
                ('.', Direction::Left) => vec![LightUnit {
                    col_idx: light_unit.col_idx - 1,
                    ..light_unit
                }],
                ('.', Direction::Up) => vec![LightUnit {
                    row_idx: light_unit.row_idx - 1,
                    ..light_unit
                }],
                ('/', Direction::Right) => vec![LightUnit {
                    row_idx: light_unit.row_idx - 1,
                    direction: Direction::Up,
                    ..light_unit
                }],
                ('/', Direction::Down) => vec![LightUnit {
                    col_idx: light_unit.col_idx - 1,
                    direction: Direction::Left,
                    ..light_unit
                }],
                ('/', Direction::Left) => vec![LightUnit {
                    row_idx: light_unit.row_idx + 1,
                    direction: Direction::Down,
                    ..light_unit
                }],
                ('/', Direction::Up) => vec![LightUnit {
                    col_idx: light_unit.col_idx + 1,
                    direction: Direction::Right,
                    ..light_unit
                }],
                ('\\', Direction::Right) => vec![LightUnit {
                    row_idx: light_unit.row_idx + 1,
                    direction: Direction::Down,
                    ..light_unit
                }],
                ('\\', Direction::Down) => vec![LightUnit {
                    col_idx: light_unit.col_idx + 1,
                    direction: Direction::Right,
                    ..light_unit
                }],
                ('\\', Direction::Left) => vec![LightUnit {
                    row_idx: light_unit.row_idx - 1,
                    direction: Direction::Up,
                    ..light_unit
                }],
                ('\\', Direction::Up) => vec![LightUnit {
                    col_idx: light_unit.col_idx - 1,
                    direction: Direction::Left,
                    ..light_unit
                }],
                ('|', Direction::Right | Direction::Left) => vec![
                    LightUnit {
                        row_idx: light_unit.row_idx - 1,
                        direction: Direction::Up,
                        ..light_unit
                    },
                    LightUnit {
                        row_idx: light_unit.row_idx + 1,
                        direction: Direction::Down,
                        ..light_unit
                    },
                ],
                ('|', Direction::Down) => vec![LightUnit {
                    row_idx: light_unit.row_idx + 1,
                    ..light_unit
                }],
                ('|', Direction::Up) => vec![LightUnit {
                    row_idx: light_unit.row_idx - 1,
                    ..light_unit
                }],
                ('-', Direction::Down | Direction::Up) => vec![
                    LightUnit {
                        col_idx: light_unit.col_idx - 1,
                        direction: Direction::Left,
                        ..light_unit
                    },
                    LightUnit {
                        col_idx: light_unit.col_idx + 1,
                        direction: Direction::Right,
                        ..light_unit
                    },
                ],
                ('-', Direction::Right) => vec![LightUnit {
                    col_idx: light_unit.col_idx + 1,
                    ..light_unit
                }],
                ('-', Direction::Left) => vec![LightUnit {
                    col_idx: light_unit.col_idx - 1,
                    ..light_unit
                }],
                _ => panic!("Unknown token!"),
            };

            for new_unit in new_units {
                if new_unit.row_idx >= 0
                    && new_unit.row_idx < height as isize
                    && new_unit.col_idx >= 0
                    && new_unit.col_idx < width as isize
                    && reachable.insert(new_unit)
                {
                    light_stack.push(new_unit);
                }
            }
        }

        let num_energized = energized
            .iter()
            .map(|row| row.iter().filter(|&&is_energized| is_energized).count())
            .sum::<usize>();
        max_energized = max_energized.max(num_energized);
        if start
            == (LightUnit {
                row_idx: 0,
                col_idx: 0,
                direction: Direction::Right,
            })
        {
            println!("Part A: {}", num_energized);
        }
    }

    println!("Part B: {}", max_energized);
}
