use itertools::Itertools;

use crate::input::read_lines;
use queues::*;


pub fn solve_a() {
    println!("{}", solve_a_input("src/test_files/problem10/input.txt"));
}

fn pipe_to_adj(pipe: u8) -> Vec<(i32, i32)> {
    match pipe {
        b'|' => vec![(-1,0), (1, 0)],
        b'-' => vec![(0,-1), (0, 1)],
        b'L' => vec![(-1,0), (0, 1)],
        b'J' => vec![(-1,0), (0,-1)],
        b'7' => vec![(1, 0), (0,-1)],
        b'F' => vec![(1, 0), (0, 1)],
        _ => Vec::new(),
    }
}

fn get_dist_array(map:&Vec<Vec<u8>>) -> Vec<Vec<i32>> {
    let mut start = (0, 0);
    for (row_i, row) in map.iter().enumerate() {
         if let Some(col_i) = row.iter().position(|x| *x == b'S') {
            start = (row_i as i32, col_i as i32);
            break;
         }
    }
    
    dbg!(start);

    let mut dist_array = vec![vec![-1i32 ; map[0].len()]; map.len()];
    dist_array[start.0 as usize][start.1 as usize] = 0;
    let adj = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut q = Queue::<(i32,i32)>::new();
    for s in adj {
        let idx = s.0 + start.0;
        let idy = s.1 + start.1;
        if idx < 0 || idy < 0 || idx >= map.len() as i32 || idy >= map[0].len() as i32 {
            continue;
        }
        let pipe = map[idx as usize][idy as usize];
        let num_adj = pipe_to_adj(pipe).iter()
            .map(|s_pipe|{
                (idx+s_pipe.0, idy+s_pipe.1)
            })
            .filter(|x| *x == start)
            .count();
        if num_adj > 0 {
            dist_array[idx as usize][idy as usize] = 1;
            let _ = q.add((idx, idy));
        }
    }

    dbg!(&q);
    while q.size() > 0 {
        let cur = q.remove().unwrap();
        let pipe = map[cur.0 as usize][cur.1 as usize];
        for s_pipe in pipe_to_adj(pipe) {
            let idx = s_pipe.0 + cur.0;
            let idy = s_pipe.1 + cur.1;

            if dist_array[idx as usize][idy as usize] >= 0 {
                continue;
            }

            dist_array[idx as usize][idy as usize] =
                 dist_array[cur.0 as usize][cur.1 as usize]+1;
                 
            let _ = q.add((idx, idy));
        }
    }
    return dist_array;
}

fn solve_a_input(file : &str) -> i32 {
    let lines: Vec<String> = read_lines(file);
    let map = lines.iter()
        .map(|x| Vec::from(x.as_bytes()))
        .collect_vec();

    get_dist_array(&map)
        .iter()
        .map(|x| *x.iter().max().unwrap())
        .max().unwrap()
}

pub fn solve_b() {
    println!("{}", solve_b_input("src/test_files/problem10/input.txt"));
}

fn is_starting(pipe: u8) -> bool {
    match pipe {
        b'L' => true,
        b'F' => true,
        _ => false,
    }
}

fn is_ending(pipe: u8) -> bool {
    match pipe {
        b'J' => true,
        b'7' => true,
        _ => false,
    }
}

fn is_on_line(starting_pipe: u8, ending_pipe: u8) -> bool {
    assert!(is_starting(starting_pipe));
    assert!(is_ending(ending_pipe));
    match starting_pipe {
        b'L' => 
            match ending_pipe {
                b'J' => true,
                b'7' => false,
                _ => false,
            },
        b'F' => 
            match ending_pipe {
                b'J' => false,
                b'7' => true,
                _ => false,
            },
        _ => false,
    }
}

fn solve_b_input(file : &str) -> i32 {
    let lines: Vec<String> = read_lines(file);
    let map = lines.iter()
        .map(|x| Vec::from(x.as_bytes()))
        .collect_vec();

    let mut res = 0;
    let dist_array = get_dist_array(&map);
    let mut inside_outside_arr = dist_array.iter()
        .map(|row|
         row.iter().map(|_| 'O').collect_vec())
         .collect_vec();

    for (idx, arr_x) in dist_array.iter().enumerate(){
        let mut inside_loop = false;
        let mut starting_pipe = b'x';
        for (idy,  dist) in arr_x.iter().enumerate() {
            let mut pipe = map[idx][idy];
            if pipe == b'S' {
                pipe = b'7';
            }
            if *dist >= 0 {
                inside_outside_arr[idx][idy] = 'L';
                if pipe == b'|' {
                    inside_loop = !inside_loop;
                } else if is_starting(pipe) {
                    starting_pipe = pipe;
                } else if is_ending(pipe) {
                    if !is_on_line(starting_pipe, pipe) {
                        inside_loop = !inside_loop;
                    }
                }
            } else if inside_loop {
                inside_outside_arr[idx][idy] = 'I';
                //dbg!((idx, idy));
                res += 1;
            } else if inside_loop {
                inside_outside_arr[idx][idy] = 'I';
            }
        }
    }

    //for a in &inside_outside_arr {
    //    println!("{:?}", &a);
    //}

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a_example() {
        let ans = solve_a_input("src/test_files/problem10/a_example.txt");
        assert_eq!(ans, 8);
    }

    #[test]
    fn test_solve_b_example() {
        let ans = solve_b_input("src/test_files/problem10/b_example.txt");
        assert_eq!(ans, 10);
    }
}

