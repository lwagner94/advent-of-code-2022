use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{self, BufRead},
    path::Path,
    rc::Rc,
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
enum NodeType {
    Directory(BTreeMap<String, Rc<RefCell<Node>>>),
    File(usize),
}

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: "/".into(),
            node_type: NodeType::Directory(Default::default()),
            parent: None,
        }))
    }

    fn add_child(&mut self, name: String, child: Rc<RefCell<Node>>) {
        match self.node_type {
            NodeType::Directory(ref mut map) => {
                let _ = map.insert(name, child);
            }
            NodeType::File(_) => panic!("Cannot insert child in file nodes"),
        }
    }

    fn print(&self) {
        self.print_recursive(0)
    }

    fn print_recursive(&self, level: u32) {
        for _ in 0..level * 4 {
            print!(" ");
        }
        match &self.node_type {
            NodeType::Directory(map) => {
                println!("- {} (dir)", self.name);

                for a in map.values() {
                    let a = a.borrow();

                    a.print_recursive(level + 1);
                }
            }
            NodeType::File(size) => {
                println!("- {} (file, size={})", self.name, size);
            }
        }
    }

    fn calculate_size(&self) -> usize {
        match &self.node_type {
            NodeType::Directory(map) => {
                let size = map.values().map(|a| a.borrow().calculate_size()).sum();

                size
            }
            NodeType::File(size) => *size,
        }
    }

    fn sum_up_small_dirs(&self, sum: &mut usize) -> usize {
        match &self.node_type {
            NodeType::Directory(map) => {
                // let mut size = 0;
                let own = map
                    .values()
                    .map(|a| a.borrow().sum_up_small_dirs(sum))
                    .sum();

                if own <= 100000 {
                    *sum += own;
                }

                own
            }
            NodeType::File(size) => *size,
        }
    }

    fn find_dir_to_delete(&self, total_size: usize, smallest: &mut usize) -> usize {
        match &self.node_type {
            NodeType::Directory(map) => {
                // let mut size = 0;
                let own = map
                    .values()
                    .map(|a| a.borrow().find_dir_to_delete(total_size, smallest))
                    .sum();

                let remaining = 70_000_000 - total_size;

                if (remaining + own) >= 30_000_000 && own < *smallest {
                    *smallest = own;
                }

                own
            }
            NodeType::File(size) => *size,
        }
    }
}

#[derive(PartialEq, Debug)]
enum InputLine {
    ChangeDir(String),
    List,
    FileInfo(String, usize),
    DirInfo(String),
}

fn parse_line(line: &str) -> InputLine {
    if line.starts_with("$ cd") {
        let dir_name = line.split(' ').nth(2).unwrap();
        InputLine::ChangeDir(dir_name.into())
    } else if line.starts_with("$ ls") {
        InputLine::List
    } else if line.starts_with("dir") {
        let dir_name = line.split(' ').nth(1).unwrap().into();
        InputLine::DirInfo(dir_name)
    } else {
        let mut it = line.split(' ');
        let size = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap().into();
        InputLine::FileInfo(name, size)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Input file not provided");

    let root = Node::root();

    let mut current = Rc::clone(&root);

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten().skip(1) {
            let input = parse_line(&line);

            match input {
                InputLine::ChangeDir(dir_name) => {
                    current = if dir_name == ".." {
                        Rc::clone(
                            current
                                .borrow()
                                .parent
                                .as_ref()
                                .expect("node has no parent"),
                        )
                    } else {
                        match &current.borrow().node_type {
                            NodeType::Directory(children) => {
                                let node = children
                                    .get(&dir_name)
                                    .expect(&format!("could not cd into {dir_name}"));

                                Rc::clone(node)
                            }
                            NodeType::File(_) => panic!("cannot enter file"),
                        }
                    }
                }
                InputLine::List => {}
                InputLine::FileInfo(name, size) => current.borrow_mut().add_child(
                    name.clone(),
                    Rc::new(RefCell::new(Node {
                        name,
                        node_type: NodeType::File(size),
                        parent: Some(Rc::clone(&current)),
                    })),
                ),
                InputLine::DirInfo(name) => current.borrow_mut().add_child(
                    name.clone(),
                    Rc::new(RefCell::new(Node {
                        name,
                        node_type: NodeType::Directory(Default::default()),
                        parent: Some(Rc::clone(&current)),
                    })),
                ),
            }
        }
    }

    root.borrow().print();

    let total = root.borrow().calculate_size();

    let mut sum = 0;
    root.borrow().sum_up_small_dirs(&mut sum);

    dbg!(sum);

    let mut smallest = total;

    root.borrow().find_dir_to_delete(total, &mut smallest);

    dbg!(&total);
    dbg!(&smallest);
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
            InputLine::FileInfo("dssh.rwn".into(), 196636)
        );
        assert_eq!(
            parse_line("dir dirname"),
            InputLine::DirInfo("dirname".into())
        );
    }
}
