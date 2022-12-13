use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Result};

#[derive(PartialEq, Debug)]
enum Element {
    Number(i32),
    List(Vec<Element>),
}

// impl FromStr for Element {
//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         for ch in s.chars() {
//             match ch {
//                 '[' => {

//                 },
//                 ']' => {

//                 },

//             }
//         }
//     }
// }

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Result<Element> {
    let mut lists = Vec::new();
    let mut current_num = String::new();

    for ch in line.chars() {
        match ch {
            '[' => {
                lists.push(Vec::new());
            }
            ']' => {}
            digit if ch.is_ascii_digit() => {
                current_num.push(digit);
            }
            ',' => {
                if !current_num.is_empty() {
                    let number: i32 = current_num.parse()?;

                    current_num.clear();

                    lists
                        .last_mut()
                        .context("no list started yet")?
                        .push(Element::Number(number));
                }
            }
            ' ' => {}
            _ => bail!("invalid character"),
        }
    }

    todo!()
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("Input file not provided");
    let mut lines = read_lines(filename)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Element::*;
    use super::*;

    #[test]
    fn test_simple() {
        let a = List(vec![Number(1), Number(2), Number(3)]);

        assert_eq!(parse_line("[1,2,3]"), a);
    }
}
