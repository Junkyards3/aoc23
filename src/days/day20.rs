use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

#[derive(Debug)]
pub struct Day20 {
    modules: HashMap<String, Module>,
}

fn handle_button_press(
    modules: &mut HashMap<String, Module>,
    nb_steps: u64,
    rx_predecessors: &mut HashMap<String, u64>,
) -> [u64; 2] {
    let mut remaining_pulses =
        VecDeque::from([("broadcast".to_string(), "button".to_string(), Pulse::Low)]);
    let mut nb_pulses = [0, 0];
    let mut changed_ids = vec![];
    while let Some((id_pulse, id_origin, pulse)) = remaining_pulses.pop_front() {
        nb_pulses[pulse as usize] += 1;
        changed_ids.push(id_pulse.clone());
        let module = modules.get_mut(&id_pulse).unwrap();
        let new_pulse = module.module_type.handle_pulse(pulse, &id_origin);
        if let Some(new_pulse) = new_pulse {
            for output_id in module.ids_outputs.iter() {
                if let Some(t) = rx_predecessors.get_mut(output_id) {
                    if *t == 0 && new_pulse == Pulse::Low {
                        *t = nb_steps;
                    }
                }
                remaining_pulses.push_back((output_id.to_string(), id_pulse.to_string(), new_pulse))
            }
        }
    }
    nb_pulses
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    id: String,
    ids_outputs: Vec<String>,
}

impl Module {
    fn make_output_module(id: &str) -> Self {
        Module {
            module_type: ModuleType::Output,
            id: id.to_string(),
            ids_outputs: vec![],
        }
    }

    fn construct_from_line_with_empty_inputs(line: &str) -> Self {
        let (id_str, output_str) = line.split_once(" -> ").unwrap();
        let module_type = match id_str.chars().next().unwrap() {
            '%' => ModuleType::FlipFlop { state: false },
            '&' => ModuleType::Conjunction {
                most_recent_pulses: HashMap::new(),
            },
            'b' => ModuleType::Broadcast,
            _ => panic!("unexpected id char"),
        };
        let id = match module_type {
            ModuleType::FlipFlop { .. } | ModuleType::Conjunction { .. } => id_str[1..].to_string(),
            ModuleType::Broadcast => "broadcast".to_string(),
            ModuleType::Output => panic!("output does not appear as input"),
        };
        let ids_outputs = output_str.split(", ").map(String::from).collect_vec();
        Module {
            module_type,
            id,
            ids_outputs,
        }
    }

    fn add_input_to_conjunction(&mut self, input: &str) {
        match &mut self.module_type {
            ModuleType::Conjunction { most_recent_pulses } => {
                most_recent_pulses.insert(input.to_string(), Pulse::Low);
            }
            ModuleType::FlipFlop { .. } | ModuleType::Broadcast | ModuleType::Output => {}
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        most_recent_pulses: HashMap<String, Pulse>,
    },
    Broadcast,
    Output,
}

impl ModuleType {
    fn handle_pulse(&mut self, pulse: Pulse, id_input: &str) -> Option<Pulse> {
        match (self, pulse) {
            (ModuleType::FlipFlop { .. }, Pulse::High) => None,
            (ModuleType::FlipFlop { state }, Pulse::Low) => {
                let new_pulse = if *state { Pulse::Low } else { Pulse::High };
                *state = !*state;
                Some(new_pulse)
            }
            (ModuleType::Conjunction { most_recent_pulses }, pulse) => {
                *most_recent_pulses.get_mut(id_input).unwrap() = pulse;
                if most_recent_pulses
                    .values()
                    .all(|pulse| *pulse == Pulse::High)
                {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            (ModuleType::Broadcast, pulse) => Some(pulse),
            (ModuleType::Output, _) => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low = 0,
    High = 1,
}

impl Day for Day20 {
    fn make_day(file: File) -> Self {
        let mut modules: HashMap<String, Module> = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                let module = Module::construct_from_line_with_empty_inputs(
                    &line.expect("doc should have lines"),
                );
                (module.id.clone(), module)
            })
            .collect();
        let temp_input_output = modules
            .values()
            .map(|module| (module.id.clone(), module.ids_outputs.clone()))
            .collect_vec();
        for (id_input, id_outputs) in temp_input_output {
            for id_output in id_outputs.iter() {
                match modules.get_mut(id_output) {
                    Some(module) => module.add_input_to_conjunction(&id_input),
                    None => {
                        modules.insert(id_output.clone(), Module::make_output_module(&id_output));
                    }
                }
            }
        }
        Day20 { modules }
    }

    fn solution1(&self) -> String {
        let (mut nb_low, mut nb_high) = (0, 0);
        let mut modules = self.modules.clone();
        let mut map = HashMap::new();
        for i in 0..1000 {
            let t = handle_button_press(&mut modules, i, &mut map);
            nb_low += t[0];
            nb_high += t[1];
        }
        (nb_low * nb_high).to_string()
    }

    fn solution2(&self) -> String {
        let rx_predecessor = self
            .modules
            .values()
            .find(|module| module.ids_outputs.iter().any(|output| output == "rx"))
            .unwrap()
            .id
            .clone();
        let mut rx_ante_predecessors = self
            .modules
            .values()
            .filter(|module| {
                module
                    .ids_outputs
                    .iter()
                    .any(|output| *output == rx_predecessor)
            })
            .map(|module| (module.id.clone(), 0))
            .collect();
        let mut modules = self.modules.clone();
        let mut nb_before_rx = 0;
        loop {
            nb_before_rx += 1;
            let _ = handle_button_press(&mut modules, nb_before_rx, &mut rx_ante_predecessors);
            if rx_ante_predecessors.values().all(|period| *period != 0) {
                break;
            }
        }
        let result = rx_ante_predecessors.values().fold(1, |x, y| lcm(x, *y));
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_sol1() {
        let input = File::open("./inputs/day20/input_test.txt").expect("File not found");
        let day = Day20::make_day(input);
        assert_eq!(day.solution1(), "11687500");
    }

    #[test]
    fn test_day20_sol2() {
        let input = File::open("./inputs/day20/input_test.txt").expect("File not found");
        let day = Day20::make_day(input);
        assert_eq!(day.solution2(), "sol2");
    }
}
