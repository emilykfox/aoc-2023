fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day09.txt").unwrap();

    // learn sequences
    let mut derivatives = Vec::new();
    for line in lines.iter() {
        let firsts = line
            .split_whitespace()
            .map(|string| string.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        derivatives.push(vec![firsts]);
    }

    // learn derivatives
    for orders in derivatives.iter_mut() {
        while !orders.last().unwrap().iter().all(|&value| value == 0) {
            let order = &orders.last().unwrap();
            let mut next_order = Vec::new();
            let mut prev = order.first().unwrap();
            for value in order.iter().skip(1) {
                next_order.push(value - prev);
                prev = value;
            }
            orders.push(next_order);
        }
    }

    // extrapolate next values
    let mut sum = 0;
    for orders in derivatives.iter() {
        let mut prev_last = 0;
        for order in orders.iter().rev().skip(1) {
            prev_last += order.last().unwrap();
        }
        sum += prev_last;
    }

    println!("Part A: {}", sum);

    // extrapolate previous values
    let mut sum = 0;
    for orders in derivatives.iter() {
        let mut prev_first = 0;
        for order in orders.iter().rev().skip(1) {
            prev_first = order.first().unwrap() - prev_first;
        }
        sum += prev_first;
    }

    println!("Part B: {}", sum);
}
