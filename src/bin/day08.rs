use std::collections::HashMap;

struct Node<'a> {
    left: &'a str,
    right: &'a str,
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

    let mut location = "AAA";
    let mut num_steps = 0;
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => {
                location = exit_map[location].left;
            }
            _ => {
                location = exit_map[location].right;
            }
        }
        num_steps += 1;
        if location == "ZZZ" {
            break;
        }
    }
    println!("Part A: {}", num_steps);

    let mut journeys = HashMap::new();
    for location in exit_map
        .keys()
        .filter(|location| location.ends_with('A') || location.ends_with('Z'))
    {
        journeys.insert(location, location);
    }
    let mut time_to_z = HashMap::<&str, u32>::new();
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
        for (&&starting_location, &&current_location) in journeys.iter() {
            if current_location.ends_with('Z') {
                time_to_z.entry(starting_location).or_insert(num_steps);
            }
        }
        if time_to_z.len() == journeys.len() {
            break;
        }
    }
    println!("time_to_z: {:?}", time_to_z);

    println!("Part B: {}", num_steps);
}
