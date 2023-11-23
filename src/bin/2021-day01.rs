use aoc_2023::collect_lines;

fn main() {
    let lines = collect_lines("./inputs/2021-day01.txt").unwrap();

    let mut num_increasing_single = 0;
    let mut num_increasing_triple = 0;
    let mut last = [
        lines[0].parse::<u32>().unwrap(),
        lines[1].parse::<u32>().unwrap(),
        lines[2].parse::<u32>().unwrap(),
    ];
    if last[1] > last[0] {
        num_increasing_single += 1;
    }
    if last[2] > last[1] {
        num_increasing_single += 1;
    }
    for line in lines.iter().skip(3) {
        let current = line.parse::<u32>().unwrap();
        if current > last[2] {
            num_increasing_single += 1;
        }
        if last[1] + last[2] + current > last[0] + last[1] + last[2] {
            num_increasing_triple += 1;
        }
        last[0] = last[1];
        last[1] = last[2];
        last[2] = current;
    }

    println!("Part A: {}", num_increasing_single);
    println!("Part B: {}", num_increasing_triple);
}
