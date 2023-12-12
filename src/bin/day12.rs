use std::iter::repeat;

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day12.txt").unwrap();

    let mut count_sum = 0;
    for line in lines.iter() {
        let mut split = line.split_whitespace();
        let records_str = split.next().unwrap();
        let size_str = split.next().unwrap();

        let records = records_str.chars().collect::<Vec<_>>();
        let sizes = size_str
            .split(',')
            .filter_map(|string| string.parse::<usize>().ok())
            .collect::<Vec<_>>();

        // count[record_idx][size_idx]: number of valid arrangements of
        // records[..records_idx] with sizes[..size_idx]
        let mut count = vec![vec![0; sizes.len() + 1]; records.len() + 1];
        count[0][0] = 1;
        for records_idx in 1..(records.len() + 1) {
            count[records_idx][0] = if records[records_idx - 1] != '#' {
                count[records_idx - 1][0]
            } else {
                0
            };
            for size_idx in 1..(sizes.len() + 1) {
                count[records_idx][size_idx] = if sizes[size_idx - 1] <= records_idx
                    && records[(records_idx - sizes[size_idx - 1])..records_idx]
                        .iter()
                        .all(|&record| record != '.')
                {
                    if sizes[size_idx - 1] == records_idx {
                        count[0][size_idx - 1]
                    } else if records[records_idx - sizes[size_idx - 1] - 1] != '#' {
                        count[records_idx - sizes[size_idx - 1] - 1][size_idx - 1]
                    } else {
                        0
                    }
                } else {
                    0
                } + if records[records_idx - 1] != '#' {
                    count[records_idx - 1][size_idx]
                } else {
                    0
                }
            }
        }
        count_sum += count[records.len()][sizes.len()];
    }

    println!("Part A: {}", count_sum);

    let mut count_sum = 0;
    for line in lines.iter() {
        let mut split = line.split_whitespace();
        let records_str = split.next().unwrap();
        let size_str = split.next().unwrap();

        let records = records_str.chars().collect::<Vec<_>>();
        let records = std::iter::repeat(records.clone())
            .take(5)
            .collect::<Vec<_>>()
            .join(&'?');

        let sizes = size_str
            .split(',')
            .filter_map(|string| string.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let sizes = repeat(sizes.clone()).take(5).flatten().collect::<Vec<_>>();

        // count[record_idx][size_idx]: number of valid arrangements of
        // records[..records_idx] with sizes[..size_idx]
        let mut count = vec![vec![0u64; sizes.len() + 1]; records.len() + 1];
        count[0][0] = 1;
        for records_idx in 1..(records.len() + 1) {
            count[records_idx][0] = if records[records_idx - 1] != '#' {
                count[records_idx - 1][0]
            } else {
                0
            };
            for size_idx in 1..(sizes.len() + 1) {
                count[records_idx][size_idx] = if sizes[size_idx - 1] <= records_idx
                    && records[(records_idx - sizes[size_idx - 1])..records_idx]
                        .iter()
                        .all(|&record| record != '.')
                {
                    if sizes[size_idx - 1] == records_idx {
                        count[0][size_idx - 1]
                    } else if records[records_idx - sizes[size_idx - 1] - 1] != '#' {
                        count[records_idx - sizes[size_idx - 1] - 1][size_idx - 1]
                    } else {
                        0
                    }
                } else {
                    0
                } + if records[records_idx - 1] != '#' {
                    count[records_idx - 1][size_idx]
                } else {
                    0
                }
            }
        }
        count_sum += count[records.len()][sizes.len()];
    }

    println!("Part B: {}", count_sum);
}
