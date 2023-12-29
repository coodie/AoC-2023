use crate::input::read_lines;
use itertools::Itertools;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem6/input.txt"));
}

struct TimesDistances {
    times: Vec<i64>,
    distances: Vec<i64>,
}

fn parse_input(lines: Vec<String>) -> TimesDistances {
    TimesDistances {
        times: lines[0]
            .split(":")
            .collect_vec()[1]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap()).collect_vec(),
        distances: lines[1]
            .split(":")
            .collect_vec()[1]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap()).collect_vec(),
    }
}

fn solve_a_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let times_distances = parse_input(lines);

    return std::iter::zip(
        times_distances.times.iter(),
        times_distances.distances.iter()).map(|(time, distance)|{
            (1..*time)
                .map(|speed| (time-speed)*speed)
                .filter(|distance_travelled| distance_travelled > distance)
                .count() as i64
        }).fold(1, |x,y| x*y);
}

//returns (time, distance)
fn parse_input_b(lines: Vec<String>) -> TimesDistances {
    TimesDistances {
        times: vec![lines[0]
            .split(":")
            .collect_vec()[1]
            .replace(" ", "")
            .parse().unwrap() ],
        distances: vec![lines[1]
            .split(":")
            .collect_vec()[1]
            .replace(" ", "")
            .parse().unwrap()],
    }
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem6/input.txt"));
}

fn solve_b_input(file : &str) -> i64 {
    let lines: Vec<String> = read_lines(file);
    let times_distances = parse_input_b(lines);
    
    return std::iter::zip(
        times_distances.times.iter(),
        times_distances.distances.iter()).map(|(time, distance)|{
            (1..*time)
                .map(|speed| (time-speed)*speed)
                .filter(|distance_travelled| distance_travelled > distance)
                .count() as i64
        }).fold(1, |x,y| x*y);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem6/a_example.txt");
        assert_eq!(ans, 288);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem6/a_example.txt");
        assert_eq!(ans, 71503);
    }
}

