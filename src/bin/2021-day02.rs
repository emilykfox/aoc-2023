fn main() {
    let lines = aoc_2023::collect_lines("./inputs/2021-day02.txt").unwrap();

    let re = regex::Regex::new(r"(forward|down|up) (\d)+").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut aim_depth = 0;
    for line in lines {
        let captures = re.captures(&line).unwrap();
        let amount = captures[2].parse::<u32>().unwrap();
        match &captures[1] {
            "forward" => {
                horizontal += amount;
                aim_depth += amount * aim;
            }
            "down" => {
                depth += amount;
                aim += amount;
            }
            "up" => {
                depth -= amount;
                aim -= amount;
            }
            _ => panic!("Bad instruction!"),
        }
    }

    println!("Part A: {}", horizontal * depth);
    println!("Part B: {}", horizontal * aim_depth);
}
