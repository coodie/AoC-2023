use itertools::Itertools;

use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem9/input.txt"));
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<i64>> {
    lines.iter()
        .map(|line|{
            line.split(" ")
                .map(|el| el.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn value_of_history(numbers: &Vec<i64>) -> i64 {
    let mut triangle: Vec<Vec<i64>> = Vec::new();
    triangle.push(numbers.clone());

    while !triangle
            .last()
            .unwrap()
            .iter()
            .all(|x| *x == 0)  {
        triangle.push(
            triangle.last().unwrap().iter()
                .tuple_windows()
                .map(|(a,b)| *b - *a)
                .collect_vec()
        );
    }

    triangle.iter()
        .map(|x| *x.last().unwrap())
        .sum()
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let res = parse_input(lines).iter()
        .map(value_of_history)
        .sum::<i64>();
    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem9/input.txt"));
}

fn value_of_history_back(numbers: &Vec<i64>) -> i64 {
    let mut triangle: Vec<Vec<i64>> = Vec::new();
    triangle.push(numbers.clone());

    while !triangle
            .last()
            .unwrap()
            .iter()
            .all(|x| *x == 0)  {
        triangle.push(
            triangle.last().unwrap().iter()
                .tuple_windows()
                .map(|(a,b)| *b - *a)
                .collect_vec()
        );
    }

    triangle.iter().enumerate()
        .map(|(i, vec)| vec[0] * (-1i64).pow(i as u32 % 2))
        .sum()
}
fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let res = parse_input(lines).iter()
        .map(value_of_history_back)
        .sum::<i64>();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem9/a_example.txt");
        assert_eq!(ans, 114);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem9/a_example.txt");
        assert_eq!(ans, 2);
    }
}

