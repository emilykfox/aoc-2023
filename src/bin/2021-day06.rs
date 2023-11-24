fn main() {
    let input = &aoc_2023::collect_lines("./inputs/2021-day06.txt").unwrap()[0];

    let mut timers: Vec<u8> = Vec::new();
    for timer in input.split(',') {
        timers.push(timer.parse().unwrap());
    }

    for _ in 0..80 {
        let mut num_new_fish = 0;
        for timer in timers.iter_mut() {
            if *timer == 0 {
                num_new_fish += 1;
                *timer = 6;
            } else {
                *timer -= 1;
            }
        }

        let mut new_timers: Vec<u8> = vec![8; num_new_fish];
        timers.append(&mut new_timers);
    }

    println!("Part A: {}", timers.len());

    let mut num_per_timer: [u64; 9] = [0; 9];
    for timer in input.split(',') {
        let timer = timer.parse::<usize>().unwrap();
        num_per_timer[timer] += 1;
    }
    for _ in 0..256 {
        let mut new_nums = [0; 9];
        new_nums[8] = num_per_timer[0];
        new_nums[7] = num_per_timer[8];
        new_nums[6] = num_per_timer[7] + num_per_timer[0];
        new_nums[0..6].clone_from_slice(&num_per_timer[1..7]);
        num_per_timer = new_nums;
    }

    let sum = num_per_timer.iter().sum::<u64>();

    println!("Part B: {}", sum);
}
