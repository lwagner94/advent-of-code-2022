use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use anyhow::Result;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug)]
enum Node {
    File(usize),
    Directory(RefCell<HashMap<String, Node>>),
}

#[derive(PartialEq, Debug)]
enum InputLine {
    ChangeDir(String),
    List,
    NodeInfo(String, Node),
}

fn parse_line(line: &str) -> InputLine {
    if line.starts_with("$ cd") {
        let dir_name = line.split(' ').nth(2).unwrap();
        InputLine::ChangeDir(dir_name.into())
    } else if line.starts_with("$ ls") {
        InputLine::List
    } else if line.starts_with("dir") {
        let dir_name = line.split(' ').nth(1).unwrap().into();
        InputLine::NodeInfo(dir_name, Node::Directory(RefCell::new(HashMap::new())))
    } else {
        let mut it = line.split(' ');
        let size = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap().into();
        InputLine::NodeInfo(name, Node::File(size))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Input file not provided");

    let root = Node::Directory(RefCell::new(HashMap::new()));

    let mut current = &root;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten().skip(1) {
            let input = parse_line(&line);

            match input {
                InputLine::ChangeDir(dir) => {
                    if let Node::Directory(children) = current {
                        let map = children.borrow();
                        let new_dir = map.get(&dir).unwrap();
                        current = new_dir;
                    }
                }
                InputLine::List => {}
                InputLine::NodeInfo(name, node) => {
                    if let Node::Directory(children) = current {
                        children.borrow_mut().insert(name, node);
                    }
                }
            }
        }
    }

    dbg!(root);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("$ cd /"), InputLine::ChangeDir("/".into()));
        assert_eq!(
            parse_line("$ cd foobar"),
            InputLine::ChangeDir("foobar".into())
        );
        assert_eq!(parse_line("$ ls"), InputLine::List);
        assert_eq!(
            parse_line("196636 dssh.rwn"),
            InputLine::NodeInfo(Node::File(196636))
        );
        assert_eq!(
            parse_line("dir dirname"),
            InputLine::NodeInfo(Node::Directory(vec![]))
        );
    }
}
