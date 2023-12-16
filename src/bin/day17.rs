use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    row_idx: usize,
    col_idx: usize,
    orientation: Orientation,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day17.txt").unwrap();

    let heat_losses = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|loss| loss.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = heat_losses.len();
    let width = heat_losses[0].len();

    let start_states = vec![
        State {
            row_idx: 0,
            col_idx: 0,
            orientation: Orientation::Horizontal,
        },
        State {
            row_idx: 0,
            col_idx: 0,
            orientation: Orientation::Vertical,
        },
    ];
    let mut states_to_loss =
        HashMap::<State, u32>::from_iter(start_states.into_iter().map(|state| (state, 0u32)));
    let mut preds = HashMap::<State, State>::new();
    let mut states_by_loss = BTreeMap::<u32, HashSet<State>>::new();
    states_by_loss.insert(0u32, HashSet::from_iter(states_to_loss.keys().copied()));
    while let Some(mut good_entry) = states_by_loss.first_entry() {
        let loss = *good_entry.key();
        let good_states = good_entry.get_mut();
        let from = *good_states.iter().next().unwrap();
        good_states.remove(&from);
        if good_states.is_empty() {
            states_by_loss.remove(&loss);
        }

        for distance in 1..=3 {
            let next_states = match from.orientation {
                Orientation::Vertical => vec![
                    State {
                        row_idx: from.row_idx.saturating_sub(distance),
                        orientation: Orientation::Horizontal,
                        ..from
                    },
                    State {
                        row_idx: (from.row_idx + distance).min(height - 1),
                        orientation: Orientation::Horizontal,
                        ..from
                    },
                ],
                Orientation::Horizontal => vec![
                    State {
                        col_idx: from.col_idx.saturating_sub(distance),
                        orientation: Orientation::Vertical,
                        ..from
                    },
                    State {
                        col_idx: (from.col_idx + distance).min(width - 1),
                        orientation: Orientation::Vertical,
                        ..from
                    },
                ],
            };
            for next_state in next_states {
                if next_state.row_idx == from.row_idx && next_state.col_idx == from.col_idx {
                    continue;
                }
                let total_loss = loss
                    + match from.orientation {
                        Orientation::Vertical => (from.row_idx.min(next_state.row_idx)
                            ..=from.row_idx.max(next_state.row_idx))
                            .map(|row_idx| {
                                if row_idx == from.row_idx {
                                    0
                                } else {
                                    heat_losses[row_idx][next_state.col_idx]
                                }
                            })
                            .sum::<u32>(),
                        Orientation::Horizontal => (from.col_idx.min(next_state.col_idx)
                            ..=from.col_idx.max(next_state.col_idx))
                            .map(|col_idx| {
                                if col_idx == from.col_idx {
                                    0
                                } else {
                                    heat_losses[next_state.row_idx][col_idx]
                                }
                            })
                            .sum::<u32>(),
                    };
                let old_loss = *states_to_loss.get(&next_state).unwrap_or(&u32::MAX);
                if total_loss < old_loss {
                    states_to_loss.insert(next_state, total_loss);
                    preds.insert(next_state, from);
                    if let Some(misplaced_states) = states_by_loss.get_mut(&old_loss) {
                        misplaced_states.remove(&next_state);
                        if misplaced_states.is_empty() {
                            states_by_loss.remove(&old_loss);
                        }
                    }
                    states_by_loss
                        .entry(total_loss)
                        .or_default()
                        .insert(next_state);
                }
            }
        }
    }

    let min_heat_loss = states_to_loss[&State {
        row_idx: height - 1,
        col_idx: width - 1,
        orientation: Orientation::Vertical,
    }]
        .min(
            states_to_loss[&State {
                row_idx: height - 1,
                col_idx: width - 1,
                orientation: Orientation::Horizontal,
            }],
        );
    println!("Part 1: {}", min_heat_loss);

    let start_states = vec![
        State {
            row_idx: 0,
            col_idx: 0,
            orientation: Orientation::Horizontal,
        },
        State {
            row_idx: 0,
            col_idx: 0,
            orientation: Orientation::Vertical,
        },
    ];
    let mut states_to_loss =
        HashMap::<State, u32>::from_iter(start_states.into_iter().map(|state| (state, 0u32)));
    let mut preds = HashMap::<State, State>::new();
    let mut states_by_loss = BTreeMap::<u32, HashSet<State>>::new();
    states_by_loss.insert(0u32, HashSet::from_iter(states_to_loss.keys().copied()));
    while let Some(mut good_entry) = states_by_loss.first_entry() {
        let loss = *good_entry.key();
        let good_states = good_entry.get_mut();
        let from = *good_states.iter().next().unwrap();
        good_states.remove(&from);
        if good_states.is_empty() {
            states_by_loss.remove(&loss);
        }

        for distance in 4..=10 {
            let next_states = match from.orientation {
                Orientation::Vertical => vec![
                    State {
                        row_idx: from.row_idx.saturating_sub(distance),
                        orientation: Orientation::Horizontal,
                        ..from
                    },
                    State {
                        row_idx: (from.row_idx + distance).min(height - 1),
                        orientation: Orientation::Horizontal,
                        ..from
                    },
                ],
                Orientation::Horizontal => vec![
                    State {
                        col_idx: from.col_idx.saturating_sub(distance),
                        orientation: Orientation::Vertical,
                        ..from
                    },
                    State {
                        col_idx: (from.col_idx + distance).min(width - 1),
                        orientation: Orientation::Vertical,
                        ..from
                    },
                ],
            };
            for next_state in next_states {
                if next_state.row_idx.abs_diff(from.row_idx) < 4
                    && next_state.col_idx.abs_diff(from.col_idx) < 4
                {
                    continue;
                }
                let total_loss = loss
                    + match from.orientation {
                        Orientation::Vertical => (from.row_idx.min(next_state.row_idx)
                            ..=from.row_idx.max(next_state.row_idx))
                            .map(|row_idx| {
                                if row_idx == from.row_idx {
                                    0
                                } else {
                                    heat_losses[row_idx][next_state.col_idx]
                                }
                            })
                            .sum::<u32>(),
                        Orientation::Horizontal => (from.col_idx.min(next_state.col_idx)
                            ..=from.col_idx.max(next_state.col_idx))
                            .map(|col_idx| {
                                if col_idx == from.col_idx {
                                    0
                                } else {
                                    heat_losses[next_state.row_idx][col_idx]
                                }
                            })
                            .sum::<u32>(),
                    };
                let old_loss = *states_to_loss.get(&next_state).unwrap_or(&u32::MAX);
                if total_loss < old_loss {
                    states_to_loss.insert(next_state, total_loss);
                    preds.insert(next_state, from);
                    if let Some(misplaced_states) = states_by_loss.get_mut(&old_loss) {
                        misplaced_states.remove(&next_state);
                        if misplaced_states.is_empty() {
                            states_by_loss.remove(&old_loss);
                        }
                    }
                    states_by_loss
                        .entry(total_loss)
                        .or_default()
                        .insert(next_state);
                }
            }
        }
    }

    let min_heat_loss = states_to_loss[&State {
        row_idx: height - 1,
        col_idx: width - 1,
        orientation: Orientation::Vertical,
    }]
        .min(
            states_to_loss[&State {
                row_idx: height - 1,
                col_idx: width - 1,
                orientation: Orientation::Horizontal,
            }],
        );
    println!("Part 2: {}", min_heat_loss);
}
