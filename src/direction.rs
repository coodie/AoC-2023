
#[derive(Clone, PartialEq, Hash, Eq, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDir,
}

use crate::direction::Direction::*;

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            NoDir => NoDir,
        }
    }
}

pub const ALL_DIRS  : [Direction; 4] =  [Up, Down, Left, Right];

pub type Point = (usize, usize);


pub fn get_shift<T: num::Signed>(dir: Direction) -> (T, T) {
    match dir {
        Direction::Down => (T::one(), T::zero()),
        Direction::Up => (-T::one(), T::zero()),
        Direction::Left => (T::zero(), -T::one()),
        Direction::Right => (T::zero(), T::one()),
        _ => unimplemented!("error"),
    }
}


pub fn go_n(dir: Direction, pt: Point, n: usize, mx: Point) -> Option<Point> {
    let shift_1 = get_shift::<i32>(dir);
    let shift = (shift_1.0 * n as i32, shift_1.1 * n as i32);
    let pt_i32 = (pt.0 as i32, pt.1 as i32);
    let new_pt = (pt_i32.0 + shift.0, pt_i32.1 + shift.1);

    if new_pt.0 < 0 || new_pt.0 >= mx.0 as i32 || new_pt.1 < 0 || new_pt.1 >= mx.1 as i32 {
        return None;
    }

    return Some((new_pt.0 as usize, new_pt.1 as usize));
}