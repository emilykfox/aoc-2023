use regex::Regex;

const EPS: f64 = 0.00001;

const MIN_COORDINATE: isize = 7;
// const MIN_COORDINATE: isize = 200000000000000;
const MAX_COORDINATE: isize = 27;
// const MAX_COORDINATE: isize = 400000000000000;

#[derive(Debug, Clone, Copy, PartialEq)]
struct HailStone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day24small.txt").unwrap();

    let re = Regex::new(r"^(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)$").unwrap();
    let stones = lines
        .iter()
        .map(|line| {
            let captures = re.captures(line).expect(line);
            HailStone {
                position: (
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                ),
                velocity: (
                    captures[4].parse().unwrap(),
                    captures[5].parse().unwrap(),
                    captures[6].parse().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>();

    let num_intersections = stones
        .iter()
        .enumerate()
        .map(|(idx_a, stone_a)| {
            stones
                .iter()
                .enumerate()
                .filter(|&(idx_b, stone_b)| {
                    if idx_a >= idx_b {
                        false
                    } else {
                        let slope_a = stone_a.velocity.1 as f64 / stone_a.velocity.0 as f64;
                        let slope_b = stone_b.velocity.1 as f64 / stone_b.velocity.0 as f64;
                        if (slope_a - slope_b).abs() < EPS {
                            false
                        } else {
                            let intersept_a =
                                stone_a.position.1 as f64 - slope_a * stone_a.position.0 as f64;
                            let intersept_b =
                                stone_b.position.1 as f64 - slope_b * stone_b.position.0 as f64;

                            let intersection_x = (intersept_b - intersept_a) / (slope_a - slope_b);
                            let intersection_y = slope_a * intersection_x + intersept_a;

                            (intersection_x - stone_a.position.0 as f64) * stone_a.velocity.0 as f64
                                > -EPS
                                && (intersection_x - stone_b.position.0 as f64)
                                    * stone_b.velocity.0 as f64
                                    > -EPS
                                && (MIN_COORDINATE as f64 - intersection_x) < EPS
                                && (intersection_x - MAX_COORDINATE as f64) < EPS
                                && (MIN_COORDINATE as f64 - intersection_y) < EPS
                                && (intersection_y - MAX_COORDINATE as f64) < EPS
                            // if intersect {
                            //     println!(
                            //         "Intersection between {:?} and {:?} at ({}, {}).",
                            //         stone_a, stone_b, intersection_x, intersection_y
                            //     );
                            // } else {
                            //     // println!(
                            //     //     "Not counting intersection between {:?} and {:?} at ({}, {}).",
                            //     //     stone_a, stone_b, intersection_x, intersection_y
                            //     // );
                            // }
                            // intersect
                        }
                    }
                })
                .count()
        })
        .sum::<usize>();
    println!("Part 1: {}", num_intersections);

    let mut solution = None;
    // Prediction: you hit one stone at time 1 and another at time 2
    'find_solution: for (idx1, stone1) in stones.iter().enumerate() {
        for (_idx2, stone2) in stones
            .iter()
            .enumerate()
            .filter(|&(idx2, _stone2)| idx2 != idx1)
        {
            let velocity = (
                stone2.position.0 + 2 * stone2.velocity.0 - (stone1.position.0 + stone1.velocity.0),
                stone2.position.1 + 2 * stone2.velocity.1 - (stone1.position.1 + stone1.velocity.1),
                stone2.position.2 + 2 * stone2.velocity.2 - (stone1.position.2 + stone1.velocity.2),
            );
            let position = (
                stone1.position.0 + stone1.velocity.0 - velocity.0,
                stone1.position.1 + stone1.velocity.1 - velocity.1,
                stone1.position.2 + stone1.velocity.2 - velocity.2,
            );

            // test if I found the right information
            let good = stones.iter().all(|stone3| {
                let time = if stone3.velocity.0 != velocity.0 {
                    (stone3.position.0 - position.0) / (stone3.velocity.0 - velocity.0)
                } else if stone3.velocity.1 != velocity.1 {
                    (stone3.position.1 - position.1) / (stone3.velocity.1 - velocity.1)
                } else if stone3.velocity.2 != velocity.2 {
                    (stone3.position.2 - position.2) / (stone3.velocity.2 - velocity.2)
                } else {
                    return false;
                };

                position.1 + time * velocity.1 == stone3.position.1 + time * stone3.velocity.1
                    && position.2 + time * velocity.2
                        == stone3.position.2 + time * stone3.velocity.2
                    && position.0 + time * velocity.0
                        == stone3.position.0 + time * stone3.velocity.0
            });

            if good {
                solution = Some(position.0 + position.1 + position.2);
                break 'find_solution;
            }
        }
    }

    println!("Part 2: {:?}", solution);
}
