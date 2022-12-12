/// Day 8: Height Map Visibility
use std::str::FromStr;
use std::vec;

type TreeHeight = i8;

#[derive(Debug)]
struct Coord(usize, usize);

#[derive(Debug)]
struct HeightMap {
    /// Column then Row
    map: Vec<Vec<TreeHeight>>,
}

impl FromStr for HeightMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<TreeHeight>> = vec![];
        for line in s.lines() {
            // organizes the data in Column, Row fashion
            map.push(
                line.split("")
                    .flat_map(|x| x.parse::<TreeHeight>())
                    .collect(),
            )
        }
        Ok(Self { map })
    }
}

impl HeightMap {
    /// give the count of trees that are visible from the edges along rows or
    /// columns
    pub fn count_visible(&self) -> u32 {
        let mut counter = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.is_visible(Coord(x, y)) {
                    counter += 1
                }
            }
        }
        counter
    }

    pub fn get_best_senic_score(&self) -> u64 {
        let mut score = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                let s = self.senic_score(Coord(x, y));
                if s > score {
                    score = s
                }
            }
        }
        score
    }

    /// Is the coord visible from outside
    pub fn is_visible(&self, loc: Coord) -> bool {
        let column: Vec<TreeHeight> = self.map.iter().map(|x| x[loc.0]).collect();
        let row = &self.map[loc.1];
        let height = row[loc.0];
        {
            column[..loc.1].iter().max().unwrap_or(&-1) < &height
                || column[loc.1 + 1..].iter().max().unwrap_or(&-1) < &height
                || row[..loc.0].iter().max().unwrap_or(&-1) < &height
                || row[loc.0 + 1..].iter().max().unwrap_or(&-1) < &height
        }
    }

    pub fn senic_score(&self, loc: Coord) -> u64 {
        let column: Vec<TreeHeight> = self.map.iter().map(|x| x[loc.0]).collect();
        let row = &self.map[loc.1];
        let height = row[loc.0];
        let up = self.view_dist(height, column[..loc.1].iter().rev());
        let down = self.view_dist(height, column[loc.1+1..].iter());
        let left = self.view_dist(height, row[..loc.0].iter().rev());
        let right = self.view_dist(height, row[loc.0+1..].iter());

        up * down * left * right
    }

    fn view_dist<'a>(&self, height: TreeHeight, line: impl Iterator<Item = &'a TreeHeight>) -> u64 {
        let mut acc = 0;
        for t in line {
            acc += 1;
            if t >= &height {
                break;
            }
        }
        acc
    }

}

fn main() {
    let input_map = include_str!("./data/day_8.prod.txt");

    let hm = HeightMap::from_str(input_map).unwrap();

    dbg!(hm.count_visible());
    dbg!(hm.get_best_senic_score());
}
