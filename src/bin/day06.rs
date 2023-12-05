fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day06.txt").unwrap();

    let times = lines[0]
        .split_whitespace()
        .filter_map(|string| string.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let goals = lines[1]
        .split_whitespace()
        .filter_map(|string| string.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut good_product = 1;
    for race_idx in 0..4 {
        let mut num_good = 0;
        for hold_time in 0..(times[race_idx] + 1) {
            let distance = (times[race_idx] - hold_time) * hold_time;
            if distance > goals[race_idx] {
                num_good += 1;
            }
        }
        good_product *= num_good;
    }
    println!("Part A: {}", good_product);

    let real_time = 51926890 as u64;
    let real_goal = 222203111261225 as u64;
    let mut num_good = 0;
    for hold_time in 0..(real_time + 1) {
        let distance = (real_time - hold_time) * hold_time;
        if distance > real_goal {
            num_good += 1;
        }
    }

    println!("Part B: {}", num_good);
}
