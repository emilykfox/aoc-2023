fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day02.txt").unwrap();

    let re = regex::Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut sum = 0;
    let mut power_sum = 0;
    let mut id = 0;
    for line in lines.iter() {
        id += 1;
        let captures_iter = re.captures_iter(line);
        let mut possible = true;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for captures in captures_iter {
            let number = captures[1].parse::<u32>().unwrap();
            match &captures[2] {
                "red" => {
                    if number > 12 {
                        possible = false;
                    }
                    min_red = min_red.max(number);
                }
                "green" => {
                    if number > 13 {
                        possible = false;
                    }
                    min_green = min_green.max(number);
                }
                "blue" => {
                    if number > 14 {
                        possible = false;
                    }
                    min_blue = min_blue.max(number);
                }
                _ => panic!(),
            }
        }

        if possible {
            sum += id;
        }
        power_sum += min_red * min_green * min_blue;
    }

    println!("Part A: {}", sum);

    println!("Part B: {}", power_sum);
}
