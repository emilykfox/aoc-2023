use std::collections::HashSet;

// struct TrenchCube {
//     row_idx: usize,
//     col_idx: usize,
//     color: [u8; 3],
// }

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day18.txt").unwrap();

    let mut trench_pos = HashSet::new();

    let mut current = (0, 0);
    for line in lines.iter() {
        let mut split = line.split_whitespace();
        let (direction, length) = (
            split.next().unwrap().chars().next().unwrap(),
            split.next().unwrap().parse::<isize>().unwrap(),
        );
        match direction {
            'U' => {
                for row_idx in (current.0 - length)..current.0 {
                    trench_pos.insert((row_idx, current.1));
                }
                current = (current.0 - length, current.1);
            }
            'D' => {
                for row_idx in (current.0 + 1)..=(current.0 + length) {
                    trench_pos.insert((row_idx, current.1));
                }
                current = (current.0 + length, current.1);
            }
            'L' => {
                for col_idx in (current.1 - length)..current.1 {
                    trench_pos.insert((current.0, col_idx));
                }
                current = (current.0, current.1 - length);
            }
            'R' => {
                for col_idx in (current.1 + 1)..=(current.1 + length) {
                    trench_pos.insert((current.0, col_idx));
                }
                current = (current.0, current.1 + length);
            }
            _ => panic!("Bad direction!"),
        }
    }

    let top = trench_pos.iter().map(|coors| coors.0).min().unwrap() - 1;
    let bottom = trench_pos.iter().map(|coors| coors.0).max().unwrap() + 1;
    let leftmost = trench_pos.iter().map(|coors| coors.1).min().unwrap() - 1;
    let rightmost = trench_pos.iter().map(|coors| coors.1).max().unwrap() + 1;

    let mut outside = HashSet::new();
    outside.insert((top, leftmost));
    let mut stack = vec![(top, leftmost)];
    while let Some(next) = stack.pop() {
        let targets = vec![
            ((next.0 - 1).max(top), next.1),
            ((next.0 + 1).min(bottom), next.1),
            (next.0, (next.1 - 1).max(leftmost)),
            (next.0, (next.1 + 1).min(rightmost)),
        ];
        for target in targets {
            if !trench_pos.contains(&target) && !outside.contains(&target) {
                outside.insert(target);
                stack.push(target);
            }
        }
    }

    let num_inside = (rightmost - leftmost + 1) * (bottom - top + 1) - outside.len() as isize;
    // println!("top: {}", top);
    // println!("bottom: {}", bottom);
    // println!("leftmost: {}", leftmost);
    // println!("rightmost: {}", rightmost);
    // println!("Number outside: {}", outside.len());
    println!("Part 1: {}", num_inside);

    let mut vert_endpoints = Vec::new();

    let mut num_inside = 0;
    let mut current = (0, 0);
    for line in lines.iter() {
        vert_endpoints.push(current);
        let mut split = line.split_whitespace().skip(2);
        let mut hex_chars = split.next().unwrap().chars().skip(2);
        let (length, direction) = (
            16isize.pow(4) * hex_chars.next().unwrap().to_digit(16).unwrap() as isize
                + 16isize.pow(3) * hex_chars.next().unwrap().to_digit(16).unwrap() as isize
                + 16isize.pow(2) * hex_chars.next().unwrap().to_digit(16).unwrap() as isize
                + 16isize.pow(1) * hex_chars.next().unwrap().to_digit(16).unwrap() as isize
                + 16isize.pow(0) * hex_chars.next().unwrap().to_digit(16).unwrap() as isize,
            hex_chars.next().unwrap(),
        );
        num_inside += length;
        match direction {
            '3' => {
                current = (current.0 - length, current.1);
            }
            '1' => {
                current = (current.0 + length, current.1);
            }
            '2' => {
                current = (current.0, current.1 - length);
            }
            '0' => {
                current = (current.0, current.1 + length);
            }
            _ => panic!("Bad direction!"),
        }
    }
    assert_eq!(current, (0, 0));

    vert_endpoints.sort();
    let mut cur_row = isize::MIN + 1;
    let mut cur_verts = HashSet::new();
    let mut outgoing = HashSet::new();
    let mut incoming = HashSet::new();
    for endpoint in vert_endpoints {
        if endpoint.0 > cur_row {
            // count strictly inside current row
            cur_verts.extend(incoming.iter().copied());
            let mut vert_vec = cur_verts.iter().copied().collect::<Vec<_>>();
            vert_vec.sort();
            let mut in_horizontal = false;
            let mut last_change = isize::MIN + 1;
            let mut inside = false;
            for col_idx in vert_vec.iter() {
                if in_horizontal {
                    if (outgoing.contains(&last_change) && incoming.contains(col_idx))
                        || (incoming.contains(&last_change) && outgoing.contains(col_idx))
                    {
                        inside = !inside;
                    }
                    in_horizontal = false;
                } else {
                    if inside {
                        num_inside += col_idx - last_change - 1;
                    }
                    if outgoing.contains(col_idx) || incoming.contains(col_idx) {
                        in_horizontal = true;
                    } else {
                        inside = !inside;
                    }
                }
                last_change = *col_idx;
            }

            // count strictly inside rows we skipped
            cur_verts = HashSet::from_iter(cur_verts.difference(&outgoing).copied());
            let mut vert_vec = cur_verts.iter().copied().collect::<Vec<_>>();
            vert_vec.sort();
            assert_eq!(vert_vec.len() % 2, 0);
            let mut num_per_row = 0isize;
            for idx in 0..vert_vec.len() / 2 {
                num_per_row += vert_vec[2 * idx + 1] - vert_vec[2 * idx] - 1;
                assert!(!num_per_row.is_negative());
            }
            num_inside += (endpoint.0 - cur_row - 1) * num_per_row;
            cur_row = endpoint.0;
            outgoing.clear();
            incoming.clear();
        }
        if cur_verts.contains(&endpoint.1) {
            outgoing.insert(endpoint.1);
        } else {
            incoming.insert(endpoint.1);
        }
    }

    println!("Part B: {}", num_inside);
}
