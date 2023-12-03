use std::collections::HashSet;

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day04.txt").unwrap();

    let number_re = regex::Regex::new(r"(\d+)").unwrap();
    let mut total = 0;
    let mut num_copies = [1; 198];
    let mut total_copies = 0;
    for (card_num, line) in lines.iter().enumerate() {
        let mut winners: HashSet<u32> = HashSet::new();
        let mut capture_iter = number_re.captures_iter(line);
        for capture in capture_iter.by_ref().skip(1).take(10) {
            winners.insert(capture[1].parse().unwrap());
        }

        let mut won = false;
        let mut points = 0;
        let mut current_maybe_copy = card_num;
        for capture in capture_iter {
            if winners.contains(&capture[1].parse().unwrap()) {
                current_maybe_copy += 1;
                num_copies[current_maybe_copy] += num_copies[card_num];
                if won {
                    points *= 2;
                } else {
                    won = true;
                    points = 1;
                }
            }
        }
        total += points;
        total_copies += num_copies[card_num];
    }

    println!("Part A: {}", total);

    println!("Part B: {}", total_copies);
}
