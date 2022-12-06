use std::{
    fs::{read, File},
    io::{self, BufRead},
    path::Path,
};

use anyhow::Result;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum State {
    ParsingStacks,
    ParsingCommands,
}

const EXTENDED: bool = true;

fn main() -> Result<()> {
    let mut stacks = Vec::new();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut state = State::ParsingStacks;

    for line in read_lines("input")? {
        let line = line?;

        match state {
            State::ParsingStacks => {
                for (index, ch) in line.chars().enumerate() {
                    let slot = index / 4;
                    if index % 4 == 1 {
                        if slot >= stacks.len() {
                            stacks.push(Vec::new());
                        }

                        if ch.is_alphabetic() {
                            stacks[slot].push(ch);
                        }
                    }
                }
            }
            State::ParsingCommands => {
                let cap = re.captures(&line);

                if let Some(cap) = cap {
                    let number = cap.get(1).unwrap().as_str().parse()?;
                    let src: usize = cap.get(2).unwrap().as_str().parse()?;
                    let dest: usize = cap.get(3).unwrap().as_str().parse()?;

                    let mut temp = Vec::new();
                    for _ in 0..number {
                        let a = stacks[src - 1].pop().unwrap();

                        temp.push(a);
                    }

                    if EXTENDED {
                        temp.reverse();
                    }

                    for ch in temp {
                        stacks[dest - 1].push(ch);
                    }
                }
            }
        }

        if line.is_empty() {
            state = State::ParsingCommands;

            for stack in &mut stacks {
                stack.reverse();
            }
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }

    Ok(())
}
