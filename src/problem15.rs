use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem15/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<String> {
    lines[0]
        .split(",")
        .map(|x| x.to_string())
        .collect_vec()
}

fn run_hash(s: &String) -> i64 {
    s.as_bytes().iter()
        .fold(0, |cur, a | ((cur + *a as i64)* 17) % 256  )
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let init_seq = parse_input(lines);
    return init_seq.iter().map(run_hash).sum();
}


enum Op {
    Dash,
    Eq(usize),
}

fn parse_input_b(lines: Vec<String>) -> Vec<(String, Op)>{
    lines[0]
        .split(",")
        .map(|s|{
            if *s.as_bytes().last().unwrap()  == b'-' {
                (s[0..s.len()-1].to_string(), Op::Dash)
            } else {
                let seq = s.split("=").collect_vec();
                (seq[0].to_string(), Op::Eq(seq[1].parse().unwrap()))
            }
        })
        .collect_vec()
    
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem15/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let init_seq = parse_input_b(lines);

    const VEC : Vec::<(&str, usize)> = Vec::new();
    let mut boxes = [VEC; 256];

    for (label, op) in &init_seq {
        let box_id = run_hash(&label) as usize;
        match op {
            Op::Dash => {
                boxes[box_id].retain(|(boxed_lens_label, _)|
                    boxed_lens_label.ne(label)
                );
            }
            Op::Eq(focal_length) => {
                if let Some(old_lens_pos) = boxes[box_id]
                    .iter()
                    .position(|(boxed_lens_label, _)| {
                        boxed_lens_label.eq(label)
                    }) {
                        boxes[box_id][old_lens_pos] = (label.as_str(), *focal_length);
                } else {
                    boxes[box_id].push((label.as_str(), *focal_length));
                }
            }
        }
    }

    let focusing_power = boxes.iter()
        .enumerate()
        .map(|(box_id, bx)|{
            bx.iter()
                .enumerate()
                .fold(0, |acc, (slot_id, (_, focal_strength))|{
                    return acc + (box_id+1) as i64 * (slot_id+1) as i64 * (*focal_strength) as i64;
                })

        }).sum::<i64>();

    return focusing_power;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem15/a_example.txt");
        assert_eq!(ans, 1320);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem15/a_example.txt");
        assert_eq!(ans, 145);
    }
}

