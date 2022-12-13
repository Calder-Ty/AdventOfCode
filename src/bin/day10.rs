use std::default;
use std::fmt::{Display, write};
use std::str::FromStr;

#[derive(Debug)]
struct Register(i64);

impl Default for Register {
    fn default() -> Self {
        Self(1)
    }
}

impl Register {
    pub fn add(&mut self, v: i64) {
        self.0 += v;
    }
}

#[derive(Debug)]
enum Instructions {
    Addx { val: i64, tick_state: u8 },
    Noop,
}

#[derive(Debug, Default)]
struct CRT {
    buffer: String,
}

impl CRT {

    pub fn draw_pixel(&mut self, tick: u32, register_val: i64) {
        if ((register_val - 1)..=(register_val + 1)).contains(&(tick as i64)){
            self.buffer.push('#');
        } else {
            self.buffer.push(' ');
        }
        if self.register.len() %40
    }

}

impl Display for CRT {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
    }
}

impl FromStr for Instructions {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            addx => {
                let (_, val) = addx.split_once(' ').unwrap();
                Ok(Self::Addx {
                    val: val.parse().unwrap(),
                    tick_state: 0,
                })
            }
        }
    }
}

#[derive(Debug, Default)]
struct Clock {
    state: u32,
}

impl Clock {
    pub fn tick(&mut self) {
        self.state += 1;
    }

    pub fn state(&self) -> u32 {
        self.state
    }
}

fn main() {
    let mut sum = 0;
    let mut program = include_str!("./data/day_10.test.txt")
        .lines()
        .map(|x| Instructions::from_str(x).unwrap());

    let mut register = Register::default();
    let mut clock = Clock::default();

    let mut instruction = program.next();
    let mut crt = CRT::default();
    while let Some(ins) = &instruction {
        crt.draw_pixel(clock.state(), register.0);
        if [19, 59, 99, 139, 179, 219].contains(&clock.state()) {
            sum += (clock.state() + 1) as i64 * register.0;
            // dbg!(((clock.state() + 1), register.0));
            dbg!(sum);
        }
        match ins {
            Instructions::Noop => instruction = program.next(),
            Instructions::Addx { val, tick_state } => {
                if tick_state > &0 {
                    register.add(*val);
                    instruction = program.next()
                } else {
                    instruction = Some(Instructions::Addx {
                        val: *val,
                        tick_state: tick_state + 1_u8,
                    });
                }
            }
        }
        clock.tick();
    }
    dbg!(sum);
}
