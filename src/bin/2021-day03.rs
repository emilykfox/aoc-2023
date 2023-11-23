#[derive(Clone, Copy, Debug)]
struct BitCount {
    zero: u32,
    one: u32,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/2021-day03.txt").unwrap();

    let num_bits = lines[0].len();
    let mut bit_counts = vec![BitCount { zero: 0, one: 0 }; num_bits];

    for line in lines.iter() {
        for (bit_place, char) in line.chars().enumerate() {
            match char {
                '0' => bit_counts[bit_place].zero += 1,
                '1' => bit_counts[bit_place].one += 1,
                _ => panic!("Bad character"),
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (bit_place, bit_count) in bit_counts.iter().rev().enumerate() {
        if bit_count.zero > bit_count.one {
            epsilon_rate += 1 << bit_place;
        } else {
            gamma_rate += 1 << bit_place;
        }
    }

    println!("Part A: {}", gamma_rate * epsilon_rate);

    let mut oxygen_remaining = lines.clone();
    let mut co2_remaining = lines;
    // count and filter oxygen
    for bit_place in 0..num_bits {
        let mut num_zeros = 0;
        let mut num_ones = 0;

        for line in &oxygen_remaining {
            if line.chars().nth(bit_place).unwrap() == '0' {
                num_zeros += 1;
            } else {
                num_ones += 1;
            }
        }
        if num_ones >= num_zeros {
            oxygen_remaining.retain(|line| line.chars().nth(bit_place).unwrap() == '1');
        } else {
            oxygen_remaining.retain(|line| line.chars().nth(bit_place).unwrap() == '0');
        }

        if oxygen_remaining.len() == 1 {
            break;
        }
    }
    let oxygen_chars = oxygen_remaining[0].chars().collect::<Vec<_>>();

    for bit_place in 0..num_bits {
        // count and filter co2
        let mut num_zeros = 0;
        let mut num_ones = 0;
        for line in &co2_remaining {
            if line.chars().nth(bit_place).unwrap() == '0' {
                num_zeros += 1;
            } else {
                num_ones += 1;
            }
        }
        if num_zeros <= num_ones {
            co2_remaining.retain(|line| line.chars().nth(bit_place).unwrap() == '0');
        } else {
            co2_remaining.retain(|line| line.chars().nth(bit_place).unwrap() == '1');
        }

        if co2_remaining.len() == 1 {
            break;
        }
    }

    let co2_chars = co2_remaining[0].chars().collect::<Vec<_>>();

    let mut oxygen_rating = 0;
    let mut co2_rating = 0;
    for bit_place in (0..num_bits).rev() {
        if oxygen_chars[bit_place] == '1' {
            oxygen_rating += 1 << (num_bits - bit_place - 1);
        }
        if co2_chars[bit_place] == '1' {
            co2_rating += 1 << (num_bits - bit_place - 1);
        }
    }

    println!("Part B: {}", oxygen_rating * co2_rating);
}
