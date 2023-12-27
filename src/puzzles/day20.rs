use core::panic;
use std::collections::VecDeque;

use either::Either;
use hashbrown::HashMap;
use num::Integer;

#[test]
fn test() {
    //     solve(String::from(
    //         "broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a",
    //     ));
    //     solve(String::from(
    //         "broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output",
    //     ));
    solve(String::from(
        "broadcaster -> a, b
%a -> con
%b -> con
&con -> rx",
    ));
}

enum SearchState {
    Started,
    Impossible,
    Ended(u64),
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    pending: Either<Option<Pulse>, VecDeque<Pulse>>,
    targets: Vec<String>,
}

#[derive(Debug)]
struct ModuleArray {
    modules: HashMap<String, Module>,
    least_memo: HashMap<(String, Pulse), u64>,
    presses: usize,
    lows: usize,
    highs: usize,
    module_map: HashMap<String, Vec<String>>,
}

impl Module {
    fn new(data: &str) -> Self {
        let mut parts = data.split(" -> ");
        let mut name = parts.next().unwrap().to_owned();
        let prefix = name.chars().next().unwrap();
        let module_type = match prefix {
            '%' => ModuleType::FlipFlop(false),
            '&' => ModuleType::Conjunction(HashMap::new()),
            _ => ModuleType::Broadcaster,
        };
        if module_type != ModuleType::Broadcaster {
            name = name[1..].to_owned();
        }
        let targets = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        Module {
            name,
            pending: if matches!(module_type, ModuleType::Conjunction(_)) {
                either::Right(VecDeque::new())
            } else {
                either::Left(None)
            },
            module_type,
            targets,
        }
    }

    fn least_to_send(
        &self,
        pulse: Pulse,
        modules: &ModuleArray,
        least_memo: &mut HashMap<(String, Pulse), SearchState>,
    ) -> Option<u64> {
        println!("least to send {:?} from {}", pulse, self.name);
        match least_memo.get(&(self.name.clone(), pulse)) {
            None => {}
            Some(SearchState::Started) => return None,
            Some(SearchState::Ended(n)) => return Some(*n),
            Some(SearchState::Impossible) => return None,
        }
        least_memo.insert((self.name.clone(), pulse), SearchState::Started);
        let inputs = modules
            .modules
            .iter()
            .filter(|(_, v)| v.targets.contains(&self.name));
        let ways = match (&self.module_type, pulse) {
            (ModuleType::Broadcaster, Pulse::High) => inputs
                .map(|(n, m)| m.least_to_send(Pulse::High, modules, least_memo))
                .min()
                .unwrap(),
            (ModuleType::Broadcaster, Pulse::Low) => Some(1),
            (ModuleType::FlipFlop(_), p) => {
                let ways_to_get_low = inputs
                    .map(|(n, m)| m.least_to_send(Pulse::Low, modules, least_memo))
                    .filter_map(|n| n)
                    .min();
                if p == Pulse::High {
                    ways_to_get_low
                } else {
                    if let Some(n) = ways_to_get_low {
                        Some(n + 1)
                    } else {
                        None
                    }
                }
            }
            (ModuleType::Conjunction(_), Pulse::High) => inputs
                .map(|(n, m)| {
                    min(
                        m.least_to_send(Pulse::High, modules, least_memo),
                        m.least_to_send(Pulse::Low, modules, least_memo),
                    )
                })
                .min()
                .unwrap(),

            (ModuleType::Conjunction(_), Pulse::Low) => Some(lcm(&inputs
                .map(|(n, m)| m.least_to_send(Pulse::High, modules, least_memo))
                .filter_map(|n| n)
                .collect::<Vec<_>>())),
        };

        println!("caching value {:?} for {}, {:?}", ways, self.name, ways);

        least_memo.insert(
            (self.name.clone(), pulse),
            if let Some(n) = ways {
                SearchState::Ended(n)
            } else {
                SearchState::Impossible
            },
        );

        ways
    }

    fn receive_pulse(&mut self, origin: &str, pulse: Pulse) {
        use ModuleType::*;
        match (&mut self.module_type, pulse) {
            (Broadcaster, p) => {
                // self.send_pulses(p);
                self.pending = either::Left(Some(p));
            }
            (FlipFlop(ref mut state), Pulse::Low) => {
                *state = !*state;
                self.pending = either::Left(Some(if *state { Pulse::High } else { Pulse::Low }));
            }
            (Conjunction(ref mut states), p) => {
                // println!("receiving pulse on {}", self.name);
                states.insert(origin.to_string(), p);
                let to_send = if states.iter().all(|(_, p)| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                self.pending.as_mut().right().unwrap().push_back(to_send);
                // dbg!(&self.pending);
                // self.send_pulses(if states.iter().all(|(_, p)| *p == Pulse::High) {
                //     Pulse::Low
                // } else {
                //     Pulse::High
                // });
            }
            _ => {}
        }
    }

    // fn send_pulses(&mut self, mods: &mut ModuleArray) {
    //     for (target, pulse) in self.pending.drain(..) {
    //         mods.modules
    //             .get_mut(&target)
    //             .unwrap()
    //             .receive_pulse(&self.name, pulse);
    //         if pulse == Pulse::High {
    //             mods.highs += 1;
    //         } else {
    //             mods.lows += 1;
    //         }
    //     }
    // }

    fn has_pending(&self) -> bool {
        if let either::Left(pulse) = self.pending {
            pulse.is_some()
        } else if let either::Right(ref pulse_vec) = self.pending {
            !pulse_vec.is_empty()
        } else {
            panic!("error in has_pending");
        }
    }

    fn get_pending(&mut self) -> Option<Pulse> {
        if let either::Left(pulse) = self.pending {
            self.pending = either::Left(None);
            pulse
        } else if let either::Right(ref mut pulse_vec) = self.pending {
            pulse_vec.pop_front()
        } else {
            panic!("error in get_pending");
        }
    }
}

impl ModuleArray {
    fn add(&mut self, module: Module) {
        let module = module;
        self.module_map
            .insert(module.name.clone(), module.targets.clone());
        self.modules.insert(module.name.clone(), module);
    }

    fn push_button(&mut self) {
        self.presses += 1;
        // println!("----------------------------");
        self.modules
            .get_mut("broadcaster")
            .unwrap()
            .receive_pulse("button", Pulse::Low);
        self.lows += 1;
        // println!("button -low-> broadcaster");
        while self.modules.iter().any(|(_, m)| m.has_pending()) {
            self.tick();
        }
    }

    fn tick(&mut self) {
        for (name, targets) in self.module_map.clone() {
            while let Some(pulse) = &self.modules.get_mut(&name).unwrap().get_pending() {
                for target in &targets {
                    // println!(
                    //     "{} -{}-> {}",
                    //     name,
                    //     if *pulse == Pulse::High { "high" } else { "low" },
                    //     target
                    // );
                    if let Some([sender, receiver]) = self.modules.get_many_mut([&name, &target]) {
                        receiver.receive_pulse(&sender.name, *pulse);
                    }
                    if *pulse == Pulse::High {
                        self.highs += 1;
                    } else {
                        self.lows += 1;
                    }
                    if pulse == &Pulse::Low && target == "rx" {
                        panic!("LOW PULSE SENT: {}", self.presses);
                    }
                    if self.highs % 100000 == 0 || self.lows % 100000 == 0 {
                        println!("highs: {}, lows: {}", self.highs, self.lows);
                    }
                }
            }
        }
    }

    fn resolve_inputs(&mut self) {
        for (name, module) in &mut self.modules {
            if let ModuleType::Conjunction(ref mut inputs) = module.module_type {
                self.module_map
                    .keys()
                    .filter(|n| self.module_map.get(*n).unwrap().contains(name))
                    .for_each(|n| {
                        inputs.insert(n.to_string(), Pulse::Low);
                    });
            }
        }
    }

    fn _inspect(&self) {
        for module in self.modules.values() {
            dbg!(&module);
        }
    }
}

fn min(num1: Option<u64>, num2: Option<u64>) -> Option<u64> {
    match (num1, num2) {
        (None, None) => None,
        (None, Some(_)) => num2,
        (Some(_), None) => num1,
        (Some(_), Some(_)) => {
            if num1 < num2 {
                num1
            } else {
                num2
            }
        }
    }
}

fn lcm(nums: &Vec<u64>) -> u64 {
    nums.iter()
        .copied()
        .reduce(|x: u64, y: u64| x.lcm(&y))
        .unwrap()
}

pub fn solve(data: String) {
    let mut modules = ModuleArray {
        modules: HashMap::new(),
        least_memo: HashMap::new(),
        presses: 0,
        module_map: HashMap::new(),
        highs: 0,
        lows: 0,
    };
    for line in data.lines() {
        if line == "" {
            continue;
        }
        println!("{}", line);
        let module = Module::new(line);
        modules.add(module);
    }
    modules.resolve_inputs();
    let final_module = modules
        .modules
        .iter()
        .filter(|(_, m)| m.targets.contains(&String::from("rx")))
        .next()
        .unwrap()
        .1;

    let mut least_memo = HashMap::new();

    // loop {
    //     modules.push_button();
    // }
    println!(
        "highs: {} lows: {} prod: {} rx low: {:?}",
        modules.highs,
        modules.lows,
        modules.highs * modules.lows,
        final_module.least_to_send(Pulse::Low, &modules, &mut least_memo)
    );
}
