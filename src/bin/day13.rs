fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day13.txt").unwrap();

    let mut a_total = 0;
    let mut b_total = 0;
    let patterns = lines.split(|line| line.is_empty());
    'pattern_loop: for pattern in patterns {
        let original_score = pattern_scores(pattern)[0];
        a_total += original_score;

        for row_idx in 0..pattern.len() {
            for col_idx in 0..pattern[0].len() {
                let new_pattern = pattern
                    .iter()
                    .enumerate()
                    .map(|(cur_row_idx, line)| {
                        if cur_row_idx == row_idx {
                            line.chars()
                                .enumerate()
                                .map(|(cur_col_idx, character)| {
                                    if cur_col_idx == col_idx {
                                        match character {
                                            '#' => '.',
                                            '.' => '#',
                                            _ => panic!("Bad character: {}", character),
                                        }
                                    } else {
                                        character
                                    }
                                })
                                .collect::<String>()
                        } else {
                            line.clone()
                        }
                    })
                    .collect::<Vec<_>>();

                let new_scores = pattern_scores(&new_pattern);
                if !new_scores.is_empty() && new_scores[0] != original_score {
                    b_total += new_scores[0];
                    continue 'pattern_loop;
                } else if new_scores.len() >= 2 {
                    b_total += new_scores[1];
                    continue 'pattern_loop;
                }
            }
        }

        panic!("No smudged reflection for pattern\n{}", pattern.join("\n"));
    }

    println!("Part A: {}", a_total);
    println!("Part B: {}", b_total);
}

fn pattern_scores(pattern: &[String]) -> Vec<usize> {
    let width = pattern[0].len();
    let height = pattern.len();
    let mut scores = Vec::with_capacity(2);
    for vertical_pos in 1..width {
        if pattern.iter().all(|row| {
            let reversed = row.chars().rev().collect::<String>();
            row[vertical_pos..width].starts_with(&reversed[(width - vertical_pos)..])
                || row[0..vertical_pos].ends_with(&reversed[0..(width - vertical_pos)])
        }) {
            scores.push(vertical_pos);
        }
    }

    for horizontal_pos in 1..height {
        let reversed = &pattern.iter().rev().cloned().collect::<Vec<_>>();
        if pattern[horizontal_pos..height].starts_with(&reversed[(height - horizontal_pos)..])
            || pattern[0..horizontal_pos].ends_with(&reversed[0..(height - horizontal_pos)])
        {
            scores.push(100 * horizontal_pos);
        }
    }

    scores
}
