use std::collections::HashMap;

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day14.txt").unwrap();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut holds_north_rock = vec![vec![false; num_rows]; num_cols];
    let mut free_north_spaces = vec![0; num_cols];

    // slide rocks north only
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, (token, free_north_spaces)) in
            line.chars().zip(free_north_spaces.iter_mut()).enumerate()
        {
            match token {
                'O' => {
                    holds_north_rock[row_idx - *free_north_spaces][col_idx] = true;
                }
                '#' => {
                    *free_north_spaces = 0;
                }
                _ => {
                    *free_north_spaces += 1;
                }
            }
        }
    }

    let mut total_load = 0;
    for (row_idx, row) in holds_north_rock.iter().enumerate() {
        for _ in row.iter().filter(|&&has_rock| has_rock) {
            total_load += num_rows - row_idx;
        }
    }

    println!("Part A: {}", total_load);

    // spin cycle!
    const GOAL: u64 = 1000000000;
    let mut state = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut seen = HashMap::new();
    let mut spin_count = 0;
    let mut jumped = false;
    seen.insert(state.clone(), spin_count);
    while spin_count < GOAL {
        // slide north
        let mut free_north_spaces = vec![0; num_cols];
        for row_idx in 0..num_rows {
            for col_idx in 0..num_cols {
                match state[row_idx][col_idx] {
                    'O' => {
                        state[row_idx - free_north_spaces[col_idx]][col_idx] = 'O';
                        state[row_idx][col_idx] = match free_north_spaces[col_idx] {
                            0 => 'O',
                            _ => '.',
                        };
                    }
                    '#' => {
                        free_north_spaces[col_idx] = 0;
                        state[row_idx][col_idx] = '#'
                    }
                    _ => {
                        free_north_spaces[col_idx] += 1;
                        state[row_idx][col_idx] = '.'
                    }
                }
            }
        }
        // slide west
        let mut free_west_spaces = vec![0; num_rows];
        for col_idx in 0..num_cols {
            for row_idx in 0..num_rows {
                match state[row_idx][col_idx] {
                    'O' => {
                        state[row_idx][col_idx - free_west_spaces[row_idx]] = 'O';
                        state[row_idx][col_idx] = match free_west_spaces[row_idx] {
                            0 => 'O',
                            _ => '.',
                        };
                    }
                    '#' => {
                        free_west_spaces[row_idx] = 0;
                        state[row_idx][col_idx] = '#'
                    }
                    _ => {
                        free_west_spaces[row_idx] += 1;
                        state[row_idx][col_idx] = '.'
                    }
                }
            }
        }
        // slide south
        let mut free_south_spaces = vec![0; num_cols];
        for row_idx in (0..num_rows).rev() {
            for col_idx in 0..num_cols {
                match state[row_idx][col_idx] {
                    'O' => {
                        state[row_idx + free_south_spaces[col_idx]][col_idx] = 'O';
                        state[row_idx][col_idx] = match free_south_spaces[col_idx] {
                            0 => 'O',
                            _ => '.',
                        };
                    }
                    '#' => {
                        free_south_spaces[col_idx] = 0;
                        state[row_idx][col_idx] = '#'
                    }
                    _ => {
                        free_south_spaces[col_idx] += 1;
                        state[row_idx][col_idx] = '.'
                    }
                }
            }
        }
        // slide east
        let mut free_east_spaces = vec![0; num_rows];
        for col_idx in (0..num_cols).rev() {
            for row_idx in 0..num_rows {
                match state[row_idx][col_idx] {
                    'O' => {
                        state[row_idx][col_idx + free_east_spaces[row_idx]] = 'O';
                        state[row_idx][col_idx] = match free_east_spaces[row_idx] {
                            0 => 'O',
                            _ => '.',
                        };
                    }
                    '#' => {
                        free_east_spaces[row_idx] = 0;
                        state[row_idx][col_idx] = '#'
                    }
                    _ => {
                        free_east_spaces[row_idx] += 1;
                        state[row_idx][col_idx] = '.'
                    }
                }
            }
        }

        spin_count += 1;
        if !jumped {
            let prev_count = seen.get(&state);
            if let Some(prev_count) = prev_count {
                let cycle_length = spin_count - *prev_count;
                let old_spin_count = spin_count;
                spin_count += ((GOAL - spin_count) / cycle_length) * cycle_length;
                jumped = true;
                println!(
                    "Jumped to {} after {} spins using a cycle length of {}.",
                    spin_count, old_spin_count, cycle_length
                );
            } else {
                seen.insert(state.clone(), spin_count);
            }
        }
    }

    let mut total_load = 0;
    for (row_idx, row) in state.iter().enumerate() {
        for _ in row.iter().filter(|&&token| token == 'O') {
            total_load += num_rows - row_idx;
        }
    }

    println!("Part B: {}", total_load);
}
