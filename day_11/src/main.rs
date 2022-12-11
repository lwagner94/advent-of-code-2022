use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Error, Result};

const PART2: bool = true;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

impl Operation {
    fn apply(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(nr) => old + *nr,
            Operation::Multiply(nr) => old * *nr,
            Operation::Square => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.trim_start().strip_prefix("new = ") {
            if rest == "old * old" {
                return Ok(Operation::Square);
            } else if let Some(number) = rest.strip_prefix("old + ") {
                return Ok(Operation::Add(number.parse()?));
            } else if let Some(number) = rest.strip_prefix("old * ") {
                return Ok(Operation::Multiply(number.parse()?));
            }
        }

        bail!("invalid operation")
    }
}
#[derive(Debug)]
enum Test {
    DivisibleBy(WorryLevel),
}

impl Test {
    fn test(&self, number_to_test: WorryLevel) -> bool {
        match self {
            Test::DivisibleBy(nr) => number_to_test % nr == 0,
        }
    }
}

impl FromStr for Test {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(divisor) = s.trim_start().strip_prefix("divisible by ") {
            return Ok(Test::DivisibleBy(divisor.parse()?));
        }

        bail!("invalid test")
    }
}

type WorryLevel = u64;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    action: Operation,
    test: Test,
    throw_true: usize,
    throw_false: usize,
    inspected_items: u64,
}

fn read_single_line(lines: &mut Lines<BufReader<File>>) -> Result<String> {
    Ok(lines.next().context("unexpected end of file")??)
}

impl Monkey {
    fn new_from_lines(lines: &mut Lines<BufReader<File>>) -> Result<Self> {
        let _id = read_single_line(lines)?
            .strip_prefix("Monkey ")
            .context("wrong format")?
            .strip_suffix(':')
            .context("wrong format")?
            .parse::<i32>()?;

        let items: VecDeque<WorryLevel> = read_single_line(lines)?
            .trim_start()
            .strip_prefix("Starting items: ")
            .context("wrong format")?
            .split(", ")
            .map(|nr| nr.parse().unwrap())
            .collect();

        let operation: Operation = read_single_line(lines)?
            .trim_start()
            .strip_prefix("Operation:")
            .context("wrong format")?
            .parse()?;

        let test: Test = read_single_line(lines)?
            .trim_start()
            .strip_prefix("Test:")
            .context("wrong format")?
            .parse()?;

        let pos: usize = read_single_line(lines)?
            .trim_start()
            .strip_prefix("If true: throw to monkey ")
            .and_then(|nr| nr.parse().ok())
            .context("failed to parse number")?;

        let neg: usize = read_single_line(lines)?
            .trim_start()
            .strip_prefix("If false: throw to monkey ")
            .and_then(|nr| nr.parse().ok())
            .context("failed to parse number")?;

        let _ = read_single_line(lines);

        Ok(Monkey {
            items,
            action: operation,
            test,
            throw_true: pos,
            throw_false: neg,
            inspected_items: 0,
        })
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    common_denominator: WorryLevel,
}

impl MonkeyBusiness {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let common_denominator = monkeys.iter().fold(1, |acc, m| {
            let Test::DivisibleBy(div) = m.test;
            acc * div
        });
        MonkeyBusiness {
            monkeys,
            common_denominator,
        }
    }

    fn simulate_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                self.monkeys[i].inspected_items += 1;

                let worry = self.monkeys[i].action.apply(item) / if PART2 { 1 } else { 3 };
                let worry = worry % self.common_denominator;

                let pos = if self.monkeys[i].test.test(worry) {
                    self.monkeys[i].throw_true
                } else {
                    self.monkeys[i].throw_false
                };
                self.monkeys[pos].items.push_back(worry);
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (i, monkey) in self.monkeys.iter().enumerate() {
            print!("Monkey {i}: ");

            for item in &monkey.items {
                print!("{item}, ");
            }

            println!();
        }
    }

    fn level(&self) -> u64 {
        let mut inspected_items: Vec<u64> =
            self.monkeys.iter().map(|m| m.inspected_items).collect();

        inspected_items.sort_by(|a, b| b.cmp(a));

        inspected_items[0] * inspected_items[1]
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("Input file not provided");

    let mut lines = read_lines(filename)?;

    let mut monkeys = Vec::new();

    while let Ok(monkey) = Monkey::new_from_lines(&mut lines) {
        monkeys.push(monkey);
    }

    let mut business = MonkeyBusiness::new(monkeys);

    let limit = if PART2 { 10000 } else { 20 };

    for _ in 0..limit {
        business.simulate_round();
    }

    println!("Monkey business: {}", business.level());

    Ok(())
}
