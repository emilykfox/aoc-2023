fn parse_digit(digit: &str) -> Option<u32> {
    Some(match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.parse::<u32>().ok()?,
    })
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day01.txt").unwrap();

    let mut sum = 0;
    for line in lines.iter() {
        let first = line.chars().find_map(|char| char.to_digit(10)).unwrap();
        let last = line
            .chars()
            .rev()
            .find_map(|char| char.to_digit(10))
            .unwrap();
        sum += 10 * first + last;
    }

    println!("Part A: {}", sum);

    let mut bigger_sum = 0;
    let first_re = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let last_re =
        regex::Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    for line in lines.iter() {
        let first_captures = first_re.captures(line).expect(line);
        let first = parse_digit(&first_captures[1]).unwrap();
        let last_captures = last_re.captures(line).expect(line);
        let last = parse_digit(&last_captures[1]).unwrap();
        bigger_sum += 10 * first + last;
    }

    println!("Part B: {}", bigger_sum);
}
