use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Error, Result};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::Noop)
        } else if s.starts_with("addx") {
            let operand = s
                .split(' ')
                .nth(1)
                .context("malformed instruction")?
                .parse()?;
            Ok(Self::AddX(operand))
        } else {
            bail!("Invalid operation")
        }
    }
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    cycle: i32,
    sum_of_signal_strengths: i32,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            cycle: 1,
            sum_of_signal_strengths: 0,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        self.during_cycle();

        match instruction {
            Instruction::Noop => {}
            Instruction::AddX(i) => {
                self.cycle += 1;
                self.during_cycle();

                self.x += i;
            }
        }

        self.cycle += 1;
    }

    fn during_cycle(&mut self) {
        if [20, 60, 100, 140, 180, 220].contains(&self.cycle) {
            self.sum_of_signal_strengths += self.x * self.cycle;
        }
        let col = (self.cycle - 1) % 40;
        if col == 0 {
            println!();
        }

        if (self.x - col).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Input file not provided");

    let mut cpu = Cpu::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let instr = line.parse().unwrap();

            cpu.execute_instruction(instr);
        }
    }

    println!();

    println!("sum: {}", cpu.sum_of_signal_strengths);
}
