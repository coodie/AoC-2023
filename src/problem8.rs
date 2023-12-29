use std::collections::HashMap;

use crate::input::read_lines;
use itertools::Itertools;
use sscanf::sscanf;
use num::integer::lcm;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem8/input.txt"));
}

struct ProblemInput{
    instructions: String,
    graph: HashMap<String, (String, String)>,
}

fn parse_input(lines: Vec<String>) -> ProblemInput {
    ProblemInput { 
        instructions: lines[0].clone(),
        graph: lines.iter()
            .skip(2)
            .map(|x| {
                let tuple = sscanf!(x, "{} = ({}, {})", String, String, String).unwrap();
                return (tuple.0,(tuple.1, tuple.2))
            }).collect()
    }
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    let mut cur = String::from("AAA");
    let mut cnt = 0;
    let ln = input.instructions.as_bytes().len();

    while !cur.eq("ZZZ") {
        cur = match input.instructions.as_bytes()[cnt % ln] {
            b'L' => input.graph.get(&cur).unwrap().0.clone(),
            b'R' => input.graph.get(&cur).unwrap().1.clone(),
            _ => cur
        };
        cnt += 1;
    }

    return cnt as i64;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem8/input.txt"));
}

fn cycle_length(start : &String, input: &ProblemInput) -> i64 {
    let mut cur_start = start.clone();
    let mut cnt = 0;

    while !cur_start.ends_with("Z") {
        cur_start = input.instructions
            .as_bytes()
            .iter()
            .fold(cur_start.clone(), 
                |cur, instr|{
                    match *instr {
                        b'L' => input.graph.get(&cur).unwrap().0.clone(),
                        b'R' => input.graph.get(&cur).unwrap().1.clone(),
                        _ => cur
                    }
                });
                cnt += 1;
    }
    return cnt;
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let input = parse_input(lines);
    let starting_nodes: Vec<String> = input.graph.keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.clone())
        .collect();
    dbg!(&starting_nodes);
    let cycles = starting_nodes
        .iter()
        .map(|starting_node| cycle_length(starting_node, &input))
        .collect_vec();
    
    let lcm = cycles
        .iter()
        .fold(cycles[0], |cur_lcm, cur_cycle|
             lcm(cur_lcm, *cur_cycle));

    return lcm*input.instructions.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem8/a_example.txt");
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem8/b_example.txt");
        assert_eq!(ans, 6);
    }
}


