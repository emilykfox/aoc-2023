use std::{
    collections::{BTreeSet, HashMap, HashSet},
    thread::Builder,
};

const MAX_SUBSET_SIZE: usize = 23;
const STACK_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DryState {
    location: (usize, usize),
    visited_branches: BTreeSet<(usize, usize)>,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day23.txt").unwrap();

    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let start = (0, 1);
    let goal = (height - 1, width - 2);

    let mut max_lengths = HashMap::<(usize, usize), Option<usize>>::new();
    max_lengths.insert(goal, Some(0));
    let max_length = hike_dfs(&grid, &mut max_lengths, start).unwrap();
    println!("Part 1: {}", max_length);

    let mut start_state = DryState {
        location: start,
        visited_branches: BTreeSet::new(),
    };
    let mut dry_lengths =
        HashMap::<DryState, Option<usize>>::with_capacity(2usize.pow(MAX_SUBSET_SIZE as u32));
    let mut location_stack = HashSet::new();
    // let max_dry_length = dry_backtracking(&grid, start, &mut location_stack).unwrap();
    let child = Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            dry_dfs(
                &grid,
                &mut dry_lengths,
                &mut start_state,
                &mut location_stack,
            )
        })
        .unwrap();
    let max_dry_length = child.join().unwrap().unwrap();
    println!("Part 2: {}", max_dry_length);
}

fn hike_dfs(
    grid: &Vec<Vec<char>>,
    max_lengths: &mut HashMap<(usize, usize), Option<usize>>,
    current: (usize, usize),
) -> Option<usize> {
    if !max_lengths.contains_key(&current) {
        // prevent infinite recursion;
        // DFS will always move 'forward' down undirected paths
        max_lengths.insert(current, None);

        let mut neighbors = Vec::new();
        match grid[current.0][current.1] {
            '^' => {
                neighbors.push((current.0 - 1, current.1));
            }
            'v' => {
                neighbors.push((current.0 + 1, current.1));
            }
            '<' => {
                neighbors.push((current.0, current.1 - 1));
            }
            '>' => {
                neighbors.push((current.0, current.1 + 1));
            }
            '.' => {
                if current.0 > 0 && grid[current.0 - 1][current.1] != 'v' {
                    neighbors.push((current.0 - 1, current.1));
                }
                if grid[current.0 + 1][current.1] != '^' {
                    neighbors.push((current.0 + 1, current.1));
                }
                if grid[current.0][current.1 - 1] != '>' {
                    neighbors.push((current.0, current.1 - 1));
                }
                if grid[current.0][current.1 + 1] != '<' {
                    neighbors.push((current.0, current.1 + 1));
                }
            }
            _ => {
                max_lengths.insert(current, None);
            }
        }

        let max_length = neighbors
            .iter()
            .filter_map(|&neighbor| hike_dfs(grid, max_lengths, neighbor))
            .max();
        max_lengths.insert(current, max_length.map(|length| length + 1));
    }

    max_lengths[&current]
}

fn _dry_backtracking(
    grid: &Vec<Vec<char>>,
    location: (usize, usize),
    location_stack: &mut HashSet<(usize, usize)>,
) -> Option<usize> {
    if grid[location.0][location.1] == '#' || location_stack.contains(&location) {
        None
    } else if location.0 == grid.len() - 1 && location.1 == grid.len() - 2 {
        Some(0)
    } else {
        let mut neighbors = vec![
            (location.0 + 1, location.1),
            (location.0, location.1 - 1),
            (location.0, location.1 + 1),
        ];
        if location.0 > 0 {
            neighbors.push((location.0 - 1, location.1));
        }
        location_stack.insert(location);

        let max_length = neighbors
            .iter()
            .filter_map(|&neighbor| _dry_backtracking(grid, neighbor, location_stack))
            .max();
        location_stack.remove(&location);
        max_length.map(|length| length + 1)
    }
}

fn dry_dfs(
    grid: &Vec<Vec<char>>,
    dry_lengths: &mut HashMap<DryState, Option<usize>>,
    state: &mut DryState,
    location_stack: &mut HashSet<(usize, usize)>,
) -> Option<usize> {
    let current_location = state.location;
    // don't walk through forest or previously used locations
    if grid[current_location.0][current_location.1] == '#'
        || location_stack.contains(&current_location)
    {
        None
    } else if state.location.0 == grid.len() - 1 && state.location.1 == grid.len() - 2 {
        Some(0)
    } else {
        let mut neighbors = vec![
            (current_location.0 + 1, current_location.1),
            (current_location.0, current_location.1 - 1),
            (current_location.0, current_location.1 + 1),
        ];
        if current_location.0 > 0 {
            neighbors.push((current_location.0 - 1, current_location.1));
        }
        let mut branching = false;
        if grid[current_location.0][current_location.1] == '.' {
            let num_adjacent_slopes = neighbors
                .iter()
                .filter(|neighbor| ['v', '<', '^', '>'].contains(&grid[neighbor.0][neighbor.1]))
                .count();
            if num_adjacent_slopes >= 2 {
                branching = true;
            }
        }

        let mut inserted = false;
        if branching && state.visited_branches.len() < MAX_SUBSET_SIZE {
            state.visited_branches.insert(state.location);
            inserted = true;
            if dry_lengths.contains_key(state) {
                let memoized = dry_lengths[state];
                state.visited_branches.remove(&current_location);
                return memoized;
            }
        }
        location_stack.insert(current_location);
        let max_total = neighbors
            .iter()
            .filter_map(|&neighbor| {
                state.location = neighbor;
                dry_dfs(grid, dry_lengths, state, location_stack)
            })
            .max()
            .map(|length| length + 1);
        state.location = current_location;
        location_stack.remove(&current_location);
        if inserted {
            dry_lengths.insert(state.clone(), max_total);
            if state.visited_branches.len() == 2usize.pow(MAX_SUBSET_SIZE as u32 - 4) {
                println!(
                    "Currently memoizing for {} subsets of branch points.",
                    dry_lengths.len()
                );
            }
            state.visited_branches.remove(&current_location);
        }
        max_total
    }
}
