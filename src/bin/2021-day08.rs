fn unique_digit(digit: &str) -> bool {
    let len = digit.len();
    len == 2 || len == 4 || len == 3 || len == 7
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/2021-day08.txt").unwrap();

    let re = regex::Regex::new(
        r"(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) \| (\w+) (\w+) (\w+) (\w+)",
    )
    .unwrap();

    let mut unique_count = 0;
    for line in lines {
        let (_, captures) = re.captures(&line).unwrap().extract::<14>();
        unique_count += captures[10..14]
            .iter()
            .filter(|digit| unique_digit(digit))
            .count();
    }

    println!("Part A: {}", unique_count);
}
