use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Errors {
    #[error("Cannot Parse this error")]
    ParseDirError,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(Self::Err::ParseDirError),
        }
    }
}

#[derive(Debug, Clone)]
struct Move {
    pub dir: Direction,
    pub distance: u32,
}

impl FromStr for Move {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Move, Self::Err> {
        let (dir, dist) = s.split_once(' ').unwrap();

        Ok(Self {
            dir: Direction::from_str(dir)?,
            distance: dist.parse()?,
        })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coord(i32, i32);

impl Coord {
    fn shift(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Left => self.0 -= 1,
        }
    }

    ///
    /// H.T -> HT
    /// T.H -> TH
    /// T
    /// . -> T
    /// H    H
    fn pull(&self, other: &mut Coord) {
        let diff_x = (self.0 - other.0).abs();
        let diff_y = (self.1 - other.1).abs();
        // dbg!(self);
        // dbg!(&other);
        if (diff_x > 1 && self.1 != other.1) || (diff_y > 1 && self.0 != other.0) {
            // Diagonal
            if (self.0 - other.0) < 0 {
                other.shift(&Direction::Left);
            } else {
                other.shift(&Direction::Right);
            }
            if (self.1 - other.1) < 0 {
                other.shift(&Direction::Down);
            } else {
                other.shift(&Direction::Up);
            }
        } else if diff_x > 1 && self.1 == other.1 {
            // Horizontal
            if (self.0 - other.0) < 0 {
                other.shift(&Direction::Left);
            } else {
                other.shift(&Direction::Right);
            }
        } else if diff_y > 1 && self.0 == other.0 {
            // Vertical
            if (self.1 - other.1) < 0 {
                other.shift(&Direction::Down);
            } else {
                other.shift(&Direction::Up);
            }
        }
        // dbg!(&other);
    }
}

fn run_move(mov: Move, head: &mut Coord, tail: &mut Coord) -> HashSet<Coord> {
    let mut set = HashSet::default();
    for _ in 0..mov.distance {
        head.shift(&mov.dir);
        head.pull(tail);
        set.insert(tail.clone());
    }
    set
}

fn simulate_rope(moves: Vec<Move>) -> HashSet<Coord> {
    let mut rope = vec![
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
    ];

    let mut traveled: HashSet<Coord> = HashSet::default();

    traveled.insert(Coord(0, 0));
    for mov in moves {
        for _ in 0..mov.distance {
            let (head, tail) = rope.split_at_mut(1);
            head[0].shift(&mov.dir);
            head[0].pull(&mut tail[0]);
            for split in 2..=9 {
                let (head, tail) = rope.split_at_mut(split);
                head.last().unwrap().pull(&mut tail[0])
            }
            dbg!(&rope);
            traveled.insert(rope.last().unwrap().clone());
        }
    }
    traveled
}

fn main() {
    let moves = include_str!("./data/day_9.prod.txt");
    let moves: Vec<Move> = moves.lines().flat_map(Move::from_str).collect();
    let traveled = simulate_rope(moves);
    dbg!(traveled.len());
}
