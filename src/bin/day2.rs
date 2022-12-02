use std::fs;
use std::iter::zip;

fn main() {
    // let path = "src/bin/data/day2_example.txt";
    let path = "src/bin/data/day2.txt";
    let data = fs::read_to_string(path).unwrap();

    let mut opponent: Vec<&str> = vec![];
    let mut me: Vec<&str> = vec![];

    for line in data.lines() {
        let mut split = line.split(' ');
        opponent.push(split.next().unwrap());
        me.push(split.next().unwrap());
    }
    let mut scores: Vec<usize> = vec![];
    for (o, m) in zip(opponent, me) {
        scores.push((win_loss_accounting(o, m)).into())
    }
    println!("Final Score: {}", scores.iter().sum::<usize>())
}

/// Takes in an opponents strat and my strat and determines my payoff
fn win_loss_accounting(o: &str, m: &str) -> u8 {
    // X => Loose
    // Y => Draw
    // Z => Win
    match m {
        "X" => strat_score(loosing_strat(o)),
        "Y" => 3 + strat_score(draw_strat(o)),
        "Z" => 6 + strat_score(winning_strat(o)),
        _ => panic!("Invalid Input!"),
    }
}

fn loosing_strat(o: &str) -> &str{
    // A => Rock
    // B => Paper
    // C => Scissors
    match o  {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => panic!("Invalid Input!")

    }
}

fn winning_strat(o: &str) -> &str{
    // A => Rock
    // B => Paper
    // C => Scissors
    match o  {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => panic!("Invalid Input!")

    }
}

fn draw_strat(o: &str) -> &str{
    o
}

fn strat_score(m: &str) -> u8 {
    match m {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid Input!"),
    }
}
