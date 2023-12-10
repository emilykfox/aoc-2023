fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day11.txt").unwrap();

    let mut galaxy_coords = Vec::new();
    let mut col_is_empty = vec![true; lines[0].len()];
    for (row, line) in lines.iter().enumerate() {
        for (col, character) in line.chars().enumerate() {
            if character == '#' {
                galaxy_coords.push((row, col));
                col_is_empty[col] = false;
            }
        }
    }

    let mut num_empty_rows_prefixes = vec![0; lines.len()];
    let mut current_empty = 0;
    for (row, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            current_empty += 1;
        }
        num_empty_rows_prefixes[row] = current_empty;
    }

    let mut num_empty_cols_prefixes = vec![0; lines[0].len()];
    let mut current_empty = 0;
    for (col, is_empty) in col_is_empty.into_iter().enumerate() {
        if is_empty {
            current_empty += 1;
        }
        num_empty_cols_prefixes[col] = current_empty;
    }

    let mut sum_distances = 0;
    for (first_idx, (first_row, first_col)) in galaxy_coords.iter().enumerate() {
        for (second_row, second_col) in galaxy_coords[(first_idx + 1)..].iter() {
            let distance = (*second_row as i32 - *first_row as i32
                + num_empty_rows_prefixes[*second_row] as i32
                - num_empty_rows_prefixes[*first_row] as i32)
                .abs()
                + (*second_col as i32 - *first_col as i32
                    + num_empty_cols_prefixes[*second_col] as i32
                    - num_empty_cols_prefixes[*first_col] as i32)
                    .abs();
            sum_distances += distance;
        }
    }

    println!("Part A: {}", sum_distances);

    let mut sum_distances = 0;
    for (first_idx, (first_row, first_col)) in galaxy_coords.iter().enumerate() {
        for (second_row, second_col) in galaxy_coords[(first_idx + 1)..].iter() {
            let distance = (*second_row as i64 - *first_row as i64
                + 999999
                    * (num_empty_rows_prefixes[*second_row] as i64
                        - num_empty_rows_prefixes[*first_row] as i64))
                .abs()
                + (*second_col as i64 - *first_col as i64
                    + 999999
                        * (num_empty_cols_prefixes[*second_col] as i64
                            - num_empty_cols_prefixes[*first_col] as i64))
                    .abs();
            sum_distances += distance;
        }
    }

    println!("Part B: {}", sum_distances);
}
