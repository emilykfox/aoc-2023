fn main() {
    let lines = aoc_2023::collect_lines("./inputs/2021-day05.txt").unwrap();
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let mut num_covering_a = [[0; 1000]; 1000];
    let mut num_covering_b = [[0; 1000]; 1000];
    for line in lines.iter() {
        let captures = re.captures(line).unwrap();
        let (x1, y1, x2, y2) = (
            captures[1].parse::<usize>().unwrap(),
            captures[2].parse::<usize>().unwrap(),
            captures[3].parse::<usize>().unwrap(),
            captures[4].parse::<usize>().unwrap(),
        );

        let x_reversed;
        let x_range = if x1 <= x2 {
            x_reversed = false;
            x1..=x2
        } else {
            x_reversed = true;
            x2..=x1
        };
        let y_reversed;
        let y_range = if y1 <= y2 {
            y_reversed = false;
            y1..=y2
        } else {
            y_reversed = true;
            y2..=y1
        };

        if x1 == x2 {
            for y in y_range {
                num_covering_a[y][x1] += 1;
                num_covering_b[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in x_range {
                num_covering_a[y1][x] += 1;
                num_covering_b[y1][x] += 1;
            }
        } else {
            // diagonals only count in part b
            if x_reversed == y_reversed {
                for (x, y) in x_range.zip(y_range) {
                    num_covering_b[y][x] += 1;
                }
            } else {
                for (x, y) in x_range.zip(y_range.rev()) {
                    num_covering_b[y][x] += 1;
                }
            }
        }
    }
    let num_overlap_a = num_covering_a
        .iter()
        .flatten()
        .filter(|num| **num >= 2)
        .count();

    println!("Part A: {}", num_overlap_a);

    let num_overlap_b = num_covering_b
        .iter()
        .flatten()
        .filter(|num| **num >= 2)
        .count();

    println!("Part B: {}", num_overlap_b);
}
