use std::collections::BTreeSet;
use std::fs;

fn main() {
    // let path = "src/bin/data/day_3.test.txt";
    let path = "src/bin/data/day_3.txt";
    let data = fs::read_to_string(path).unwrap();

    let mut dups = vec![];
    for line in data.lines() {
        let n = line.chars().count() / 2;
        // dbg!(n);
        // dbg!(&line[..n]);
        // dbg!(&line[n..]);
        let dup = find_duplicates(&line[..n], &line[n..]);
        // dbg!(&dup);
        dups.push(calc_score(dup));
    }
    println!("{}", dups.iter().sum::<usize>());

    let mut badges = vec![];
    let mut lines = data.lines();
    loop {
        let left = lines.next();
        let middle = lines.next();
        let right = lines.next();
        if let (Some(l), Some(m), Some(r)) = (left, right, middle) {
            badges.push(calc_score(find_badges(l, m ,r)))
        }
        else {
            break
        }
    }

    println!("Badge Values: {}", badges.iter().sum::<usize>());
}

fn find_badges(first: &str, second:&str, third: &str) -> char {
    let left: BTreeSet<char> = first.chars().collect();
    let middle: BTreeSet<char> = second.chars().collect();
    let right: BTreeSet<char> = third.chars().collect();
    left.intersection(&right.intersection(&middle).cloned().collect()).cloned().collect::<Vec<char>>()[0]
}


fn find_duplicates(items1: &str, items2: &str) -> char {
    let left: BTreeSet<char> = items1.chars().collect();
    let right: BTreeSet<char> = items2.chars().collect();
    // dbg!(&left);
    // dbg!(&right);
    // dbg!(left.intersection(&right).cloned().collect::<Vec<char>>());
    left.intersection(&right).cloned().collect::<Vec<char>>()[0]
}

fn calc_score(c: char) -> usize {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid code for Item Type")
    }
}
