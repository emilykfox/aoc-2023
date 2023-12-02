fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day03.txt").unwrap();

    let mut map = [[0; 140]; 140];
    let mut gear_count = [[0; 140]; 140];
    let mut gear_ratio = [[1; 140]; 140];

    for (row, line) in lines.iter().enumerate() {
        map[row].clone_from_slice(&line.as_bytes()[0..140]);
    }

    let re = regex::Regex::new(r"(\d)+").unwrap();
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let matches = re.find_iter(line);
        for matche in matches {
            let range = matche.range();
            let part_num = std::str::from_utf8(&map[row][range.clone()])
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut is_part_num = false;
            if row != 0 {
                if range.start >= 1 {
                    is_part_num |= map[row - 1][range.start - 1] != '.' as u8
                        && !map[row - 1][range.start - 1].is_ascii_digit();
                    if map[row - 1][range.start - 1] == '*' as u8 {
                        gear_count[row - 1][range.start - 1] += 1;
                        gear_ratio[row - 1][range.start - 1] *= part_num;
                    }
                }
                if range.end < 140 {
                    is_part_num |= map[row - 1][range.end] != '.' as u8
                        && !map[row - 1][range.end].is_ascii_digit();
                    if map[row - 1][range.end] == '*' as u8 {
                        gear_count[row - 1][range.end] += 1;
                        gear_ratio[row - 1][range.end] *= part_num;
                    }
                }
                for col in range.clone() {
                    is_part_num |=
                        map[row - 1][col] != '.' as u8 && !map[row - 1][col].is_ascii_digit();
                    if map[row - 1][col] == '*' as u8 {
                        gear_count[row - 1][col] += 1;
                        gear_ratio[row - 1][col] *= part_num;
                    }
                }
            }
            if range.start >= 1 {
                is_part_num |= map[row][range.start - 1] != '.' as u8
                    && !map[row][range.start - 1].is_ascii_digit();
                if map[row][range.start - 1] == '*' as u8 {
                    gear_count[row][range.start - 1] += 1;
                    gear_ratio[row][range.start - 1] *= part_num;
                }
            }
            if range.end < 140 {
                is_part_num |=
                    map[row][range.end] != '.' as u8 && !map[row][range.end].is_ascii_digit();
                if map[row][range.end] == '*' as u8 {
                    gear_count[row][range.end] += 1;
                    gear_ratio[row][range.end] *= part_num;
                }
            }
            for col in range.clone() {
                is_part_num |= map[row][col] != '.' as u8 && !map[row][col].is_ascii_digit();
                if map[row][col] == '*' as u8 {
                    gear_count[row][col] += 1;
                    gear_ratio[row][col] *= part_num;
                }
            }
            if row != 139 {
                if range.start >= 1 {
                    is_part_num |= map[row + 1][range.start - 1] != '.' as u8
                        && !map[row + 1][range.start - 1].is_ascii_digit();
                    if map[row + 1][range.start - 1] == '*' as u8 {
                        gear_count[row + 1][range.start - 1] += 1;
                        gear_ratio[row + 1][range.start - 1] *= part_num;
                    }
                }
                if range.end < 140 {
                    is_part_num |= map[row + 1][range.end] != '.' as u8
                        && !map[row + 1][range.end].is_ascii_digit();
                    if map[row + 1][range.end] == '*' as u8 {
                        gear_count[row + 1][range.end] += 1;
                        gear_ratio[row + 1][range.end] *= part_num;
                    }
                }
                for col in range.clone() {
                    is_part_num |=
                        map[row + 1][col] != '.' as u8 && !map[row + 1][col].is_ascii_digit();
                    if map[row + 1][col] == '*' as u8 {
                        gear_count[row + 1][col] += 1;
                        gear_ratio[row + 1][col] *= part_num;
                    }
                }
            }

            if is_part_num {
                sum += part_num;
            }
        }
    }
    println!("Part A: {}", sum);

    let mut gear_sum = 0;
    for row in 0..140 {
        for col in 0..140 {
            if gear_count[row][col] == 2 {
                gear_sum += gear_ratio[row][col];
            }
        }
    }
    println!("Part B: {}", gear_sum);
}
