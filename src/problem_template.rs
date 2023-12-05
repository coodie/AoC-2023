use crate::input::read_lines;

pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem_template/input.txt"));
}

fn solve_a_input(file : &str) -> u32 {
    let lines: Vec<String> = read_lines(file);
    return 0;
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem_template/input.txt"));
}

fn solve_b_input(file : &str) -> u32 {
    let lines: Vec<String> = read_lines(file);
    return 0;
}

mod tests {
    use super::solve_a_input;
    use super::solve_b_input;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem_template/a_example.txt");
        assert_eq!(ans, 142);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem_template/b_example.txt");
        assert_eq!(ans, 281);
    }
}
