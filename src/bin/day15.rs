#[derive(Copy, Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day15.txt").unwrap();

    let total = lines[0]
        .split(',')
        .map(|string| hash(string) as u32)
        .sum::<u32>();

    println!("Part A: {}", total);

    let mut boxes = vec![vec![]; 256];

    for step in lines[0].split(',') {
        let operation_idx = step.find(['-', '=']).unwrap();
        let label = &step[0..operation_idx];
        let boxx: &mut Vec<Lens> = &mut boxes[hash(label) as usize];
        let lens_idx = boxx
            .iter()
            .enumerate()
            .find(|&(_idx, lens)| lens.label == label)
            .map(|(idx, _lens)| idx);
        match step.chars().nth(operation_idx).unwrap() {
            '-' => {
                if let Some(lens_idx) = lens_idx {
                    boxx.remove(lens_idx);
                }
            }
            '=' => {
                let focal_length = step[(operation_idx + 1)..].parse().unwrap();
                if let Some(lens_idx) = lens_idx {
                    boxx[lens_idx].focal_length = focal_length;
                } else {
                    boxx.push(Lens {
                        label,
                        focal_length,
                    });
                }
            }
            _ => panic!("Bad operation!"),
        }
    }

    let total_focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(lens_idx, lens)| {
                    (1 + box_idx as u32) * (1 + lens_idx as u32) * lens.focal_length
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Part B: {}", total_focusing_power);
}

fn hash(label: &str) -> u8 {
    label
        .as_bytes()
        .iter()
        .fold(0u8, |acc, byte| (acc.wrapping_add(*byte)).wrapping_mul(17))
}
