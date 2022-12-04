use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
struct CleanZone {
    pub start: u32,
    pub end: u32,
}

impl CleanZone {
    pub fn encloses(&self, other: &CleanZone) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn intersect(&self, other: &CleanZone) -> bool {
        // Self:  S---E     -> Other.start > Self.start && other.start < self.end
        // Other:   S---E
        // Self:    S-----E   -> Other.end > other.start && other.end < other.start
        // Other: S---E
        // Self:  S-----E
        // Other:   s-E
        // dbg!(dbg!(self.start) >= dbg!(other.end) || dbg!(self.end) <= dbg!(other.start));
        (other.start >= self.start && other.start <= self.end) ||
        (other.end >= self.start && other.end <= self.end)
    }
}

impl FromStr for CleanZone {
    type Err = Day4Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<&str> = s.split("-").collect();
        Ok(Self {
            start: x[0].parse::<u32>().unwrap(),
            end: x[1].parse::<u32>().unwrap(),
        })
    }
}

#[derive(Error, Debug)]
enum Day4Errors {
    #[error("Could Not Parse into CleanZone")]
    CleanZoneParseError,
}

fn main() -> Result<(), Day4Errors> {
    // let path = "src/bin/data/day_3.test.txt";
    // let path = "./data/day_4.txt";
    // let data = include_str!("./data/day_4.test.txt");
    let data = include_str!("./data/day_4.prod.txt");
    let mut count_contained = 0;
    let mut count_overlaping = 0;
    for line in data.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let first = CleanZone::from_str(a)?;
        let second = CleanZone::from_str(b)?;
        if first.encloses(&second) || second.encloses(&first) {
            count_contained += 1
        }
        if first.intersect(&second) || second.encloses(&first) || first.encloses(&second) {
            dbg!((first, second));
            count_overlaping += 1
        }
    }
    println!("ContainedZones: {}", count_contained);
    println!("Overlapping Zones: {}", count_overlaping);

    Ok(())
}
