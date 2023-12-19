use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    category: char,
    comp_operator: char,
    target: u64,
    destination: &'a str,
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Copy, Clone)]
struct PartRange<'a> {
    x_min: u64,
    x_max: u64,
    m_min: u64,
    m_max: u64,
    a_min: u64,
    a_max: u64,
    s_min: u64,
    s_max: u64,
    location: &'a str,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day19.txt").unwrap();

    let mut split = lines.split(|line| line.is_empty());
    let (workflow_lines, part_lines) = (split.next().unwrap(), split.next().unwrap());

    let workflows =
        HashMap::<&str, (Vec<Rule>, &str)>::from_iter(workflow_lines.iter().map(|line| {
            let mut line_split = line.split(&['{', ':', ',', '}']);
            let name = line_split.next().unwrap();

            let mut rules = Vec::new();
            loop {
                let rule_str = line_split.next().unwrap();
                if rule_str.contains(['<', '>']) {
                    let mut chars = rule_str.chars();
                    let category = chars.next().unwrap();
                    let comp_operator = chars.next().unwrap();
                    let rating = rule_str[2..].parse().expect(rule_str);
                    let destination = line_split.next().unwrap();
                    rules.push(Rule {
                        category,
                        comp_operator,
                        target: rating,
                        destination,
                    });
                } else {
                    break (name, (rules, rule_str));
                }
            }
        }));

    let mut total_ratings = 0;
    for line in part_lines {
        let mut line_split = line.split(&['{', '=', ',', '}']).skip(2);
        let x = line_split.next().unwrap().parse().unwrap();
        line_split.next();
        let m = line_split.next().unwrap().parse().unwrap();
        line_split.next();
        let a = line_split.next().unwrap().parse().unwrap();
        line_split.next();
        let s = line_split.next().unwrap().parse().unwrap();
        line_split.next();
        let part = Part { x, m, a, s };

        let mut current = "in";
        'workflow_loop: loop {
            if current == "R" || current == "A" {
                break;
            }
            let workflow = workflows.get(current).unwrap();
            for rule in workflow.0.iter() {
                let rating = match rule.category {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Bad rule category!"),
                };
                let rule_applies = match rule.comp_operator {
                    '>' => rating > rule.target,
                    '<' => rating < rule.target,
                    _ => panic!("Bad rule comparison operator!"),
                };
                if rule_applies {
                    current = rule.destination;
                    continue 'workflow_loop;
                }
            }

            // use default
            current = workflow.1;
        }

        match current {
            "A" => total_ratings += part.x + part.m + part.a + part.s,
            "R" => (),
            _ => panic!("Bad final destination!"),
        }
    }

    println!("Part 1: {}", total_ratings);

    let mut accepted_combinations = 0;
    let mut active_ranges = vec![PartRange {
        x_min: 1,
        x_max: 4000,
        m_min: 1,
        m_max: 4000,
        a_min: 1,
        a_max: 4000,
        s_min: 1,
        s_max: 4000,
        location: "in",
    }];
    while !active_ranges.is_empty() {
        let mut next_ranges = Vec::new();
        'ranges_loop: for part_range in active_ranges.iter() {
            if part_range.location == "A" {
                accepted_combinations += (part_range.x_max - part_range.x_min + 1)
                    * (part_range.m_max - part_range.m_min + 1)
                    * (part_range.a_max - part_range.a_min + 1)
                    * (part_range.s_max - part_range.s_min + 1);
                continue 'ranges_loop;
            } else if part_range.location == "R" {
                continue 'ranges_loop;
            }

            let workflow = workflows.get(part_range.location).unwrap();
            for rule in workflow.0.iter() {
                let (min_rating, max_rating) = match rule.category {
                    'x' => (part_range.x_min, part_range.x_max),
                    'm' => (part_range.m_min, part_range.m_max),
                    'a' => (part_range.a_min, part_range.a_max),
                    's' => (part_range.s_min, part_range.s_max),
                    _ => panic!("Bad rule category!"),
                };
                match rule.comp_operator {
                    '>' => {
                        if rule.target < min_rating {
                            next_ranges.push(PartRange {
                                location: rule.destination,
                                ..*part_range
                            });
                            continue 'ranges_loop;
                        } else if rule.target < max_rating {
                            let original_max = max_rating;
                            let mut new_range = *part_range;
                            match rule.category {
                                'x' => new_range.x_max = rule.target,
                                'm' => new_range.m_max = rule.target,
                                'a' => new_range.a_max = rule.target,
                                's' => new_range.s_max = rule.target,
                                _ => panic!("Bad rule category!"),
                            }
                            next_ranges.push(new_range);

                            match rule.category {
                                'x' => {
                                    new_range.x_min = rule.target + 1;
                                    new_range.x_max = original_max;
                                }
                                'm' => {
                                    new_range.m_min = rule.target + 1;
                                    new_range.m_max = original_max;
                                }
                                'a' => {
                                    new_range.a_min = rule.target + 1;
                                    new_range.a_max = original_max;
                                }
                                's' => {
                                    new_range.s_min = rule.target + 1;
                                    new_range.s_max = original_max;
                                }
                                _ => panic!("Bad rule category!"),
                            }
                            new_range.location = rule.destination;
                            next_ranges.push(new_range);
                            continue 'ranges_loop;
                        }
                        // otherwise, check next rule
                    }
                    '<' => {
                        if rule.target > max_rating {
                            next_ranges.push(PartRange {
                                location: rule.destination,
                                ..*part_range
                            });
                            continue 'ranges_loop;
                        } else if rule.target > min_rating {
                            let original_min = min_rating;
                            let mut new_range = *part_range;
                            match rule.category {
                                'x' => new_range.x_min = rule.target,
                                'm' => new_range.m_min = rule.target,
                                'a' => new_range.a_min = rule.target,
                                's' => new_range.s_min = rule.target,
                                _ => panic!("Bad rule category!"),
                            }
                            next_ranges.push(new_range);

                            match rule.category {
                                'x' => {
                                    new_range.x_min = original_min;
                                    new_range.x_max = rule.target - 1;
                                }
                                'm' => {
                                    new_range.m_min = original_min;
                                    new_range.m_max = rule.target - 1;
                                }
                                'a' => {
                                    new_range.a_min = original_min;
                                    new_range.a_max = rule.target - 1;
                                }
                                's' => {
                                    new_range.s_min = original_min;
                                    new_range.s_max = rule.target - 1;
                                }
                                _ => panic!("Bad rule category!"),
                            }
                            new_range.location = rule.destination;
                            next_ranges.push(new_range);
                            continue 'ranges_loop;
                        }
                        // otherwise, check next rule
                    }
                    _ => panic!("Bad comparison operator!"),
                };
            }
            next_ranges.push(PartRange {
                location: workflow.1,
                ..*part_range
            });
        }

        active_ranges = next_ranges;
    }

    println!("Part 2: {}", accepted_combinations);
}
