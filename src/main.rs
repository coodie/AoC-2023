pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem9;
pub mod problem10;
pub mod problem11;
pub mod problem12;
pub mod problem13;
pub mod problem14;
pub mod problem15;
pub mod problem16;
pub mod problem17;
pub mod problem18;
pub mod problem19;
pub mod problem20;
pub mod problem21;
pub mod problem22;
pub mod problem23;
pub mod problem24;
pub mod problem25;
pub mod itertools2d;
pub mod direction;
pub mod input;
pub mod debug;

use std::{env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut problem_num : i32 = 21;
    let mut problem_letter : char = 'a';

    if args.len() >= 2 {
        problem_num = args[0].parse().unwrap();
        problem_letter = args[1].parse().unwrap();
    }
    let start = Instant::now();
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
    if problem_num == 6 {
        if problem_letter == 'a' {
            println!("solving 6 a");
            problem6::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 6 b");
            problem6::solve_b();
        }
    } 
    if problem_num == 7 {
        if problem_letter == 'a' {
            println!("solving 7 a");
            problem7::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 7 b");
            problem7::solve_b();
        }
    }
    if problem_num == 8 {
        if problem_letter == 'a' {
            println!("solving 8 a");
            problem8::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 8 b");
            problem8::solve_b();
        }
    }
    if problem_num == 10 {
        if problem_letter == 'a' {
            println!("solving 10 a");
            problem10::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 10 b");
            problem10::solve_b();
        }
    }
    if problem_num == 11 {
        if problem_letter == 'a' {
            println!("solving 11 a");
            problem11::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 11 b");
            problem11::solve_b();
        }
    } 
    if problem_num == 12 {
        if problem_letter == 'a' {
            println!("solving 12 a");
            problem12::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 12 b");
            problem12::solve_b();
        }
    }
    if problem_num == 13 {
        if problem_letter == 'a' {
            println!("solving 13 a");
            problem13::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 13 b");
            problem13::solve_b();
        }
    }
    if problem_num == 14 {
        if problem_letter == 'a' {
            println!("solving 14 a");
            problem14::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 14 b");
            problem14::solve_b();
        }
    }
    if problem_num == 15 {
        if problem_letter == 'a' {
            println!("solving 15 a");
            problem15::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 15 b");
            problem15::solve_b();
        }
    }
    if problem_num == 16 {
        if problem_letter == 'a' {
            println!("solving 16 a");
            problem16::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 16 b");
            problem16::solve_b();
        }
    }
    if problem_num == 17 {
        if problem_letter == 'a' {
            println!("solving 17 a");
            problem17::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 17 b");
            problem17::solve_b();
        }
    }
    if problem_num == 18 {
        if problem_letter == 'a' {
            println!("solving 18 a");
            problem18::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 18 b");
            problem18::solve_b();
        }
    }
    if problem_num == 19 {
        if problem_letter == 'a' {
            println!("solving 19 a");
            problem19::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 19 b");
            problem19::solve_b();
        }
    }
    if problem_num == 20 {
        if problem_letter == 'a' {
            println!("solving 20 a");
            problem20::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 20 b");
            problem20::solve_b();
        }
    }
    if problem_num == 21 {
        if problem_letter == 'a' {
            println!("solving 21 a");
            problem21::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 21 b");
            problem21::solve_b();
        }
    }
    if problem_num == 22 {
        if problem_letter == 'a' {
            println!("solving 22 a");
            problem22::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 22 b");
            problem22::solve_b();
        }
    }
    if problem_num == 23 {
        if problem_letter == 'a' {
            println!("solving 23 a");
            problem23::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 23 b");
            problem23::solve_b();
        }
    }
    if problem_num == 24 {
        if problem_letter == 'a' {
            println!("solving 24 a");
            problem24::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 24 b");
            problem24::solve_b();
        }
    }
    if problem_num == 25 {
        if problem_letter == 'a' {
            println!("solving 25 a");
            problem25::solve_a();
        }
        if problem_letter == 'b' {
            println!("solving 25 b");
            problem25::solve_b();
        }
    }

    println!("Runtime: {}ms", start.elapsed().as_millis());
}
