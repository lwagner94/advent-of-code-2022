use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    ops::{Add, Index},
    path::Path,
};

use anyhow::{bail, Context, Error, Result};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Default, Ord, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const UP: Vec2 = Vec2 { x: 0, y: -1 };
const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };
const LEFT: Vec2 = Vec2 { x: -1, y: 0 };

#[derive(Eq, PartialEq, Debug)]
struct OpenSetEntry {
    vec: Vec2,
    f_score: i32,
}
impl OpenSetEntry {
    fn new(vec: Vec2, dest: Vec2) -> Self {
        Self {
            vec,
            f_score: heuristic(vec, dest),
        }
    }
}
impl Ord for OpenSetEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for OpenSetEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // match self.vec.partial_cmp(&other.vec) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }
        other.f_score.partial_cmp(&self.f_score)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Start,
    End,
    Terrain(i32),
}

impl Field {
    fn may_enter_from(&self, from: Self) -> bool {
        match self {
            Field::End if Field::Terrain(25) == from => true,
            Field::Terrain(upper) => {
                if *upper == 0 && from == Field::Start {
                    true
                } else if let Field::Terrain(lower) = from {
                    upper - lower <= 1
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl TryFrom<char> for Field {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'S' => Field::Start,
            'E' => Field::End,
            ch => {
                if !ch.is_ascii_lowercase() {
                    bail!("invalid terrain height {ch}");
                }

                let height = ch as u8 - b'a';

                Field::Terrain(height as i32)
            }
        })
    }
}

impl From<&Field> for char {
    fn from(f: &Field) -> Self {
        match f {
            Field::Start => 'S',
            Field::End => 'E',
            Field::Terrain(height) => {
                let a: u8 = (*height).try_into().unwrap();
                (b'a' + a) as char
            }
        }
    }
}

struct Map {
    fields: Vec<Vec<Field>>,
    start: Vec2,
    destination: Vec2,
}

impl Map {
    fn new_from_lines(lines: &mut Lines<BufReader<File>>) -> Result<Self> {
        let mut fields = Vec::new();

        let mut start = Default::default();
        let mut destination = Default::default();

        for (y, line) in lines.enumerate() {
            let line = line.context("failed to read line")?;

            let mut current_row = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                let field = ch.try_into()?;

                match &field {
                    Field::Start => {
                        start = Vec2 {
                            x: x as i32,
                            y: y as i32,
                        }
                    }
                    Field::End => {
                        destination = Vec2 {
                            x: x as i32,
                            y: y as i32,
                        }
                    }
                    _ => {}
                }
                current_row.push(field);
            }

            fields.push(current_row);
        }

        Ok(Map {
            fields,
            start,
            destination,
        })
    }

    fn print(&self) {
        for line in &self.fields {
            for field in line {
                let a: char = field.into();
                print!("{}", a);
            }

            println!();
        }
    }

    fn index(&self, index: Vec2) -> Option<Field> {
        if index.x < 0 || index.y < 0 {
            None
        } else {
            self.fields
                .get(index.y as usize)
                .and_then(|v| v.get(index.x as usize))
                .copied()
        }
    }

    fn count_steps(came_from: HashMap<Vec2, Vec2>, current: Vec2) -> i32 {
        let mut current = current;
        let mut steps = 0;

        while let Some(c) = came_from.get(&current) {
            steps += 1;
            current = *c;
        }

        steps
    }

    fn find_path(&self) -> Option<i32> {
        let mut open_set: BinaryHeap<OpenSetEntry> = BinaryHeap::new();

        open_set.push(OpenSetEntry::new(self.start, self.destination));
        let mut came_from = HashMap::new();

        let mut g_score = HashMap::new();
        g_score.insert(self.start, 0);

        let mut f_score = HashMap::new();
        f_score.insert(self.start, heuristic(self.start, self.destination));

        while let Some(current) = open_set.pop() {
            // dbg!("here");
            // println!("current: {:?}", current);

            if current.vec == self.destination {
                return Some(Self::count_steps(came_from, current.vec));
            }

            let current_field = self.index(current.vec).unwrap();

            for direction in [UP, DOWN, LEFT, RIGHT] {
                let neighbor_pos = current.vec + direction;
                let neighbor = self.index(neighbor_pos);
                if neighbor.is_none() {
                    // dbg!("here2");
                    continue;
                }

                let neighbor = neighbor.unwrap();

                if !neighbor.may_enter_from(current_field) {
                    // dbg!("here3");
                    continue;
                }

                let tentative_g_score = g_score.get(&current.vec).unwrap_or(&i32::MAX) + 1;

                let g_score_neighbor = g_score.get(&neighbor_pos).unwrap_or(&i32::MAX);

                if tentative_g_score < *g_score_neighbor {
                    came_from.insert(neighbor_pos, current.vec);

                    g_score.insert(neighbor_pos, tentative_g_score);
                    f_score.insert(
                        neighbor_pos,
                        tentative_g_score + heuristic(neighbor_pos, self.destination),
                    );

                    if !open_set.iter().any(|s| s.vec == neighbor_pos) {
                        open_set.push(OpenSetEntry::new(neighbor_pos, self.destination));
                    }
                }
            }
        }

        None
    }
}

fn heuristic(pos: Vec2, dest: Vec2) -> i32 {
    ((pos.x as i64 - dest.x as i64).abs() + (pos.y as i64 - dest.y as i64))
        .try_into()
        .unwrap()
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("Input file not provided");
    let mut lines = read_lines(filename)?;
    let map = Map::new_from_lines(&mut lines)?;

    // map.print();

    dbg!(map.find_path());

    Ok(())
}
