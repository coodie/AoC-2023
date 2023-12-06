use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem1/input.txt"));
}

fn solve_a_input(file : &str) -> u32 {
    let lines: Vec<String> = read_lines(file);
    let mut res = 0;
    for line in lines {
        let first_digit = line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|x| x.is_ascii_digit()).unwrap();
        
        res += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }
    return res;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem1/input.txt"));
}


fn solve_b_input(file : &str) -> u32 {
    let lines: Vec<String> = read_lines(file);

    let mut res = 0;
    for l in lines {
        let line = l.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

        let first_digit = line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|x| x.is_ascii_digit()).unwrap();
        
        res += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem1/a_example.txt");
        assert_eq!(ans, 142);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem1/b_example.txt");
        assert_eq!(ans, 281);
    }
}