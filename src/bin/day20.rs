use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse<'a> {
    pulse_type: PulseType,
    from: &'a str,
    to: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlipFlopState {
    Off,
    On,
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    FlipFlop(FlipFlopState),
    Conjunction(HashMap<&'a str, PulseType>),
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day20.txt").unwrap();

    let mut modules = HashMap::new();
    for line in lines.iter() {
        let mut split = line.split_whitespace();
        let type_and_name = split.next().unwrap();
        let (module_type, name) = match type_and_name.chars().next().unwrap() {
            '%' => (
                ModuleType::FlipFlop(FlipFlopState::Off),
                &type_and_name[1..],
            ),
            '&' => (ModuleType::Conjunction(HashMap::new()), &type_and_name[1..]),
            _ if type_and_name == "broadcaster" => (ModuleType::Broadcast, type_and_name),
            _ => panic!("Unknown module type!"),
        };

        split.next(); // skip the ->
        let destinations = split
            .map(|destinationc| destinationc.strip_suffix(',').unwrap_or(destinationc)) // drop the comma
            .collect::<Vec<_>>();

        modules.insert(
            name,
            Module {
                module_type,
                destinations,
            },
        );
    }

    // learn inputs for conjunction modules
    let connections = modules
        .iter()
        .flat_map(|(&name, module)| {
            std::iter::repeat(name).zip(module.destinations.iter().copied())
        })
        .collect::<Vec<_>>();
    for (from, to) in connections {
        if let Some(to_module) = modules.get_mut(to) {
            if let ModuleType::Conjunction(recent_pulses) = &mut to_module.module_type {
                recent_pulses.insert(from, PulseType::Low);
            }
        }
    }

    let mut num_presses = 0;
    let mut num_low = 0;
    let mut num_high = 0;
    let mut pulse_queue = VecDeque::new();
    let mut dt_high_firsts = HashMap::new();
    'presses: loop {
        pulse_queue.push_back(Pulse {
            pulse_type: PulseType::Low,
            from: "button",
            to: "broadcaster",
        });
        num_presses += 1;
        while let Some(pulse) = pulse_queue.pop_front() {
            if pulse.to == "rx" && pulse.pulse_type == PulseType::Low {
                break 'presses;
            }

            if pulse.to == "dt"
                && pulse.pulse_type == PulseType::High
                && !dt_high_firsts.contains_key(pulse.from)
            {
                dt_high_firsts.insert(pulse.from, num_presses);
                if dt_high_firsts.len() == 4 {
                    break 'presses;
                }
            }

            match pulse.pulse_type {
                PulseType::Low => num_low += 1,
                PulseType::High => num_high += 1,
            }
            let module = modules.get_mut(pulse.to);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let to_send = match module.module_type {
                ModuleType::FlipFlop(ref mut state) => match pulse.pulse_type {
                    PulseType::Low => Some(match *state {
                        FlipFlopState::Off => {
                            *state = FlipFlopState::On;
                            PulseType::High
                        }
                        FlipFlopState::On => {
                            *state = FlipFlopState::Off;
                            PulseType::Low
                        }
                    }),
                    PulseType::High => None,
                },
                ModuleType::Conjunction(ref mut recent_pulses) => {
                    *recent_pulses.get_mut(pulse.from).unwrap() = pulse.pulse_type;
                    Some(
                        if recent_pulses
                            .values()
                            .all(|&recent_pulse| recent_pulse == PulseType::High)
                        {
                            PulseType::Low
                        } else {
                            PulseType::High
                        },
                    )
                }
                ModuleType::Broadcast => Some(pulse.pulse_type),
            };

            if let Some(to_send) = to_send {
                for &destination in module.destinations.iter() {
                    pulse_queue.push_back(Pulse {
                        pulse_type: to_send,
                        from: pulse.to,
                        to: destination,
                    });
                }
            }
        }
        if num_presses == 1000 {
            println!("Part 1: {}", num_low * num_high);
        }
    }

    // It's the dumb lcm thing again for the modules broadcasting directly to
    // dt, but also they're prime.
    let product = dt_high_firsts.values().product::<u64>();
    println!("Part 2: {}", product);
}
