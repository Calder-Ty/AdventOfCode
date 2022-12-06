use std::collections::VecDeque;
use std::str::FromStr;

/// Crane Move problem

#[derive(Debug, Default)]
struct CargoBay {
    stacks: Vec<CargoStack>
}

impl CargoBay {
    fn add_layer(&mut self, chars: Vec<char>) {
        if self.stacks.is_empty() {
            // We have not initialized our Cargo Bay yet
            for _ in &chars {
                self.stacks.push(CargoStack::default());
            }
        }
        for (i, c) in chars.into_iter().enumerate() {
            match c {
                ' ' => continue,
                _ => self.stacks[i].push(c)
            }
        }
    }

    fn execute_move_order(&mut self, mo: MoveOrder) {
        let to = mo.to - 1;
        let from = mo.from - 1;
        let mut temp = VecDeque::default();
        for _ in 0..mo.quant {
            if let Some(x) = self.stacks[from].pop() {
                temp.insert(0, x);
                // self.stacks[to].push(x);
            } else {
                // There is nothing left to move
                println!("warning, ran out of things to move");
                break;
            }
        }

        for c in temp {
            self.stacks[to].push(c);
        }

    }

    fn top_cargo(&self) -> Vec<&char> {
        let mut res =vec![];
        for stack in &self.stacks {
            res.push(stack.last());
        }
        res
    }
}


#[derive(Debug, Default)]
struct CargoStack {
    inner: Vec<char>
}

impl CargoStack {
    fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    fn pop(&mut self) -> Option<char> {
        self.inner.pop()
    }

    fn last(&self) -> &char {
        match self.inner.last()  {
            Some(c) => c,
            None => &' '
        }
    }
}

#[derive(Debug)]
struct MoveOrder {
    pub from: usize,
    pub quant: usize,
    pub to: usize,
}

impl FromStr for MoveOrder {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<usize> = s.split(' ').skip(1).step_by(2).map(|x| x.parse::<usize>().unwrap()).collect();
        let quant = vals[0];
        let from = vals[1];
        let to = vals[2];
        Ok(Self { from, quant, to })
    }

}


fn main () {
    let input = include_str!("./data/day_5.prod.txt");

    // Split Graph from move Orders
    if let Some((graph, orders)) = input.split_once("\n\n") {
        // Build out CargoBay
        let mut bay = CargoBay::default();
        for line in graph.lines().rev().skip(1) {
            bay.add_layer(line.bytes().skip(1).step_by(4).map(|x| x as char).collect::<Vec<char>>());
        }

        for line in orders.lines() {
            let order = MoveOrder::from_str(line).unwrap();
            bay.execute_move_order(order);
        }

        let mut buf = String::new();
        for c in bay.top_cargo() {
            buf.push(*c)
        }

        println!("{}", buf);

    } else {
        panic!("Invalid Input")
    }


}


