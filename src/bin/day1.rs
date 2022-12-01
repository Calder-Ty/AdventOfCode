use std::fs;

fn main() {
    let path = "src/bin/data/day_1.txt";
    let data = fs::read_to_string(path).unwrap();
    let inventories: Vec<&str> = data.split("\n\n").collect();

    let inventories = sum_calories(inventories);
    println!(
        "{:?}",
        top_n_calories(inventories, 3)
    );
}

fn top_n_calories(mut inventories: Vec<usize>, n: usize) -> usize {
    inventories.sort_unstable();
    inventories.iter().skip(inventories.len() - n).sum()
}

fn sum_calories(inventories: Vec<&str>) -> Vec<usize> {
    inventories
        .iter()
        .map(|inventory| {
            inventory
                .split('\n')
                .map(|x| {
                    if x.is_empty() {
                        0
                    } else {
                        x.parse::<usize>().unwrap()
                    }
                })
                .sum()
        })
        .collect()
}
