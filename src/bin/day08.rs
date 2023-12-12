use std::{collections::HashMap, mem};

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    while n != 0 {
        mem::swap(&mut m, &mut n);
        n %= m;
    }
    m
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day08.txt").unwrap();

    let instructions = &lines[0];
    let re = regex::Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut exit_map = HashMap::new();
    for line in lines.iter().skip(2) {
        let (_, [node, left, right]) = re.captures(line).unwrap().extract();
        exit_map.insert(node, Node { left, right });
    }

    let mut journeys = HashMap::new();
    for location in exit_map.keys().filter(|location| location.ends_with('A')) {
        journeys.insert(location, location);
    }
    let mut time_to_z = HashMap::<&str, u64>::new();
    let mut num_steps = 0;
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => {
                for current_location in journeys.values_mut() {
                    *current_location = &exit_map[*current_location].left;
                }
            }
            _ => {
                for current_location in journeys.values_mut() {
                    *current_location = &exit_map[*current_location].right;
                }
            }
        }
        num_steps += 1;
        for (_, &&current_location) in journeys.iter() {
            if current_location.ends_with('Z') {
                time_to_z.entry(current_location).or_insert(num_steps);
            }
        }
        if time_to_z.len() == journeys.len() {
            break;
        }
    }
    println!("Part A: {}", time_to_z["ZZZ"]);

    // wtf, the period is equal to warmup time and they all appear in neat cycles?!
    let co_np_steps = time_to_z
        .values()
        .fold(1u64, |acc, &current| acc * (current / gcd(acc, current)));
    println!("Part B: {}", co_np_steps);
}
