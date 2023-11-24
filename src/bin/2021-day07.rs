fn main() {
    let input = &aoc_2023::collect_lines("./inputs/2021-day07.txt").unwrap()[0];

    let mut positions = Vec::new();
    for position in input.split(',') {
        positions.push(position.parse::<i32>().unwrap());
    }
    positions.sort();
    let median = positions[positions.len() / 2];
    let median_sum = positions
        .iter()
        .map(|position| (median - position).abs())
        .sum::<i32>();
    println!("Part A: {}", median_sum);

    let mean = positions.iter().sum::<i32>() / positions.len() as i32;

    let mean_sum = positions
        .iter()
        .map(|position| {
            let difference = (mean - position).abs();
            (difference) * (difference + 1) / 2
        })
        .sum::<i32>();
    println!("Part B: {}", mean_sum);
}
