use std::collections::HashMap;

use itertools::Itertools;
use queues::{Queue, IsQueue};

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem20/input.txt"));
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Module {
    Broadcaster(Vec<String>),
    Conjunction(Vec<String>),
    FlipFlop(Vec<String>),
}

use crate::problem20::Module::*;

fn parse_input(lines: &Vec<String>) -> HashMap<String, Module>{
    let mut res = HashMap::new();
    for line in lines {
        let parts = line.split(" -> ").collect_vec();
        let neighbours = parts[1].split(", ").map(|s| s.to_string()).collect_vec();
        if parts[0] == "broadcaster" {
            res.insert(parts[0].to_string(), Broadcaster(neighbours));
        } else if parts[0].starts_with("%") {
            res.insert(parts[0][1..].to_string(), FlipFlop(neighbours));
        } else if parts[0].starts_with("&") {
            res.insert(parts[0][1..].to_string(), Conjunction(neighbours));
        }
    }


    return res;
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Pulse {
    High, Low
}

type FlipFlopState<'a> = HashMap<&'a String, bool>;
type ConjunctionState<'a> = HashMap<&'a String, HashMap<&'a String, Pulse>>;
type PulseQueue<'a> = Queue<(Pulse, &'a String, &'a String)>;

fn press_button(modules : &HashMap<String, Module>,
    flip_flop_state : &mut FlipFlopState,
    conjunction_state : &mut ConjunctionState) -> (i64, i64) {

    let mut q : PulseQueue = Queue::new();
    let mut low_cnt : i64 = 0;
    let mut high_cnt : i64 = 0;
    
    let binding1 = &String::from("broadcaster");
    let binding2 = &String::from("button");
    let _ = q.add((
        Pulse::Low,
        binding1,
        binding2,
    ));

    while let Ok((pulse, node, parent)) = q.remove() {
        if pulse == Pulse::Low {
            low_cnt += 1;
        } else {
            high_cnt += 1;
        }
        let module = modules.get(node);
        if module.is_none() {
            continue;
        }
        match module.unwrap() {
            Broadcaster(neighbours) => {
                for neighbour in neighbours {
                    let _ = q.add((pulse.clone(), neighbour, node));
                }
            }
            Conjunction(neighbours) => {
                for neighbour in neighbours {
                    let conjunction_memory = conjunction_state.get_mut(node).unwrap();
                    *conjunction_memory.get_mut(parent).unwrap() = pulse.clone();

                    if conjunction_memory.values().all(|x| *x == Pulse::High) {
                        let _ = q.add((Pulse::Low, neighbour, node));
                    } else {
                        let _ = q.add((Pulse::High, neighbour, node));
                    }
                }
            },
            FlipFlop(neighbours) => {
                if pulse == Pulse::Low {
                    let state = flip_flop_state.get_mut(node).unwrap();
                    let new_pulse = match *state {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };
                    *state = !*state;
                    for neighbour in neighbours {
                        let _ = q.add((new_pulse.clone(), neighbour, node));
                    }
                }
            }, 
        }
    }
    return (low_cnt, high_cnt);
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let modules = parse_input(&lines);

    let mut flip_flop_state : FlipFlopState = FlipFlopState::new();
    let mut conjunction_state : ConjunctionState = ConjunctionState::new();

    for (node, module) in &modules {
        if let Conjunction(_) = module {
            let hash_map = HashMap::new();
            conjunction_state.insert(node, hash_map);
        }
        if let FlipFlop(_) = module {
            flip_flop_state.insert(node, false);
        }
    }

    for (node, module) in &modules {
        let neighbours;
        match module {
            Conjunction(n) => { neighbours = n; }
            Broadcaster(n) => { neighbours = n; }
            FlipFlop(n) => { neighbours = n; }
        }
        for v in neighbours {
            if let Some(mem) = conjunction_state.get_mut(v) {
                mem.insert(node, Pulse::Low);
            }
        }
    }

    let mut low_cnt_res = 0;
    let mut high_cnt_res = 0;
    for _ in 0..1000{
        let (low_cnt, high_cnt) = press_button(&modules, &mut flip_flop_state, &mut conjunction_state);
        low_cnt_res += low_cnt;
        high_cnt_res += high_cnt;
    }
    return low_cnt_res*high_cnt_res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem20/input.txt"));
}

fn press_button_b(modules : &HashMap<String, Module>,
    flip_flop_state : &mut FlipFlopState,
    conjunction_state : &mut ConjunctionState,
    seen : &mut HashMap<&String, bool>) -> () {

    let mut q : PulseQueue = Queue::new();
    
    let binding1 = &String::from("broadcaster");
    let binding2 = &String::from("button");
    let _ = q.add((
        Pulse::Low,
        binding1,
        binding2,
    ));

    while let Ok((pulse, node, parent)) = q.remove() {
        let module = modules.get(node);
        if module.is_none() {
            continue;
        } 
        if let Some(val) = seen.get_mut(node)  {
            if pulse == Pulse::Low {
                *val = true;
            }
        }
        match module.unwrap() {
            Broadcaster(neighbours) => {
                for neighbour in neighbours {
                    let _ = q.add((pulse.clone(), neighbour, node));
                }
            }
            Conjunction(neighbours) => {
                for neighbour in neighbours {
                    let conjunction_memory = conjunction_state.get_mut(node).unwrap();
                    *conjunction_memory.get_mut(parent).unwrap() = pulse.clone();

                    if conjunction_memory.values().all(|x| *x == Pulse::High) {
                        let _ = q.add((Pulse::Low, neighbour, node));
                    } else {
                        let _ = q.add((Pulse::High, neighbour, node));
                    }
                }
            },
            FlipFlop(neighbours) => {
                if pulse == Pulse::Low {
                    let state = flip_flop_state.get_mut(node).unwrap();
                    let new_pulse = match *state {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };
                    *state = !*state;
                    for neighbour in neighbours {
                        let _ = q.add((new_pulse.clone(), neighbour, node));
                    }
                }
            }, 
        }
    }
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let modules = parse_input(&lines);

    let mut flip_flop_state : FlipFlopState = FlipFlopState::new();
    let mut conjunction_state : ConjunctionState = ConjunctionState::new();

    for (node, module) in &modules {
        if let Conjunction(_) = module {
            let hash_map = HashMap::new();
            conjunction_state.insert(node, hash_map);
        }
        if let FlipFlop(_) = module {
            flip_flop_state.insert(node, false);
        }
    }

    for (node, module) in &modules {
        let neighbours;
        match module {
            Conjunction(n) => { neighbours = n; }
            Broadcaster(n) => { neighbours = n; }
            FlipFlop(n) => { neighbours = n; }
        }
        for v in neighbours {
            if let Some(mem) = conjunction_state.get_mut(v) {
                mem.insert(node, Pulse::Low);
            }
        }
    }
    let main_conj = "dd".to_string(); // this can be found as the only edge going into 'rx', but i'm too lazy
    let feeding_nodes = conjunction_state.get(&main_conj).unwrap().keys().map(|x|*x).collect_vec();
    let mut cycles : HashMap<&String, i64> = HashMap::from_iter(feeding_nodes.iter().map(|x| (*x, 0)));
    for button_press in 1..1000000 {
        let mut seen = HashMap::from_iter(feeding_nodes.iter().map(|x| (*x, false)));
        press_button_b(&modules, &mut flip_flop_state, &mut conjunction_state, &mut seen);
        for (k, v) in &seen {
            if *v == true && *cycles.get(k).unwrap() == 0 {
                cycles.insert(k, button_press);
                dbg!(&seen);
            }
        }
    }
    dbg!(&cycles);
    return cycles.values().fold(1, |x,y| x*y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem20/a_example.txt");
        assert_eq!(ans, 32000000);

        let ans = solve_a_input("src/test_files/problem20/a_example2.txt");
        assert_eq!(ans, 11687500);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem20/b_example.txt");
        assert_eq!(ans, 281);
    }
}

