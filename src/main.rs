pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod input;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut problem_num : i32 = 5;
    let mut problem_letter : char = 'b';

    if args.len() >= 2 {
        problem_num = args[0].parse().unwrap();
        problem_letter = args[1].parse().unwrap();
    }
    if problem_num == 1 {
        if problem_letter == 'a' {
            println!("solving 1 a");
            problem1::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 1 b");
            problem1::solve_b();
        }
    }
    if problem_num == 2 {
        if problem_letter == 'a' {
            println!("solving 2 a");
            problem2::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 2 b");
            problem2::solve_b();
        }
    }
    if problem_num == 3 {
        if problem_letter == 'a' {
            println!("solving 3 a");
            problem3::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 3 b");
            problem3::solve_b();
        }
    }
    if problem_num == 4 {
        if problem_letter == 'a' {
            println!("solving 4 a");
            problem4::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 4 b");
            problem4::solve_b();
        }
    } 
    if problem_num == 5 {
        if problem_letter == 'a' {
            println!("solving 5 a");
            problem5::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 5 b");
            problem5::solve_b();
        }
    }
}
