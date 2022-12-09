use std::{
    fs::File,
    io::{self, BufRead},
    ops::{Add, Sub},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Result};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Movement {
    direction: Direction,
    number_of_steps: u32,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(' ');

        let direction = match splitted.next().context("failed to parse direction")? {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("invalid direction"),
        };

        let steps = splitted
            .next()
            .context("failed to parse movement")?
            .parse()?;

        Ok(Self {
            direction,
            number_of_steps: steps,
        })
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
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
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2 {
    fn cap(&self) -> Self {
        Self {
            x: cap(self.x),
            y: cap(self.y),
        }
    }

    fn distance(&self, other: Self) -> f32 {
        ((self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32).sqrt()
    }
}

fn cap(number: i32) -> i32 {
    match number.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

struct Grid {
    visited: Vec<Vec<bool>>,
    knots: Vec<Vec2>,
    start: Vec2,
}

impl Grid {
    fn new(knots: usize) -> Self {
        Self {
            visited: vec![vec![true]],
            knots: vec![Default::default(); knots],
            start: Default::default(),
        }
    }

    fn print(&self) {
        for y in (0i32..self.rows() as i32).rev() {
            for x in 0i32..self.columns() as i32 {
                let pos = Vec2 { x, y };

                let mut mark = if pos == self.start {
                    "s"
                } else if self.visited[y as usize][x as usize] {
                    "#"
                } else {
                    "."
                }
                .to_owned();

                for (i, knot) in self.knots.iter().enumerate() {
                    if knot == &pos {
                        mark = if i == 0 { "H".into() } else { format!("{i}") };
                        break;
                    }
                }

                print!("{mark}");
            }
            println!();
        }
    }

    fn columns(&self) -> usize {
        self.visited.get(0).map(|v| v.len()).unwrap_or_default()
    }

    fn rows(&self) -> usize {
        self.visited.len()
    }

    fn append_row(&mut self) {
        self.visited.push(vec![false; self.columns()]);
    }

    fn append_column(&mut self) {
        for y in 0..self.rows() {
            self.visited[y].push(false);
        }
    }

    fn prepend_row(&mut self) {
        self.visited.insert(0, vec![false; self.columns()]);
        let offset = Vec2 { x: 0, y: 1 };
        self.shift_grid(offset);
    }

    fn prepend_column(&mut self) {
        for y in 0..self.rows() {
            self.visited[y].insert(0, false);
        }
        let offset = Vec2 { x: 1, y: 0 };
        self.shift_grid(offset);
    }

    fn move_head(&mut self, direction: Direction) {
        let movement = match direction {
            Direction::Up => Vec2 { x: 0, y: 1 },
            Direction::Down => Vec2 { x: 0, y: -1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
        };
        self.knots[0] = self.knots[0] + movement;

        self.expand_grid();
    }

    fn expand_grid(&mut self) {
        if self.knots[0].x == self.columns() as i32 {
            self.append_column();
        }
        if self.knots[0].y == self.rows() as i32 {
            self.append_row();
        }
        if self.knots[0].x == -1 {
            self.prepend_column();
        }
        if self.knots[0].y == -1 {
            self.prepend_row();
        }
    }

    fn move_tail(&mut self, tail: usize) {
        let head = self.knots[tail - 1];

        if tail == self.knots.len() {
            self.visited[head.y as usize][head.x as usize] = true;
            return;
        }

        let next = &mut self.knots[tail];

        if head.distance(*next) >= 2f32 {
            let offset = (head - *next).cap();

            *next = *next + offset;
        }

        self.move_tail(tail + 1)
    }

    fn apply_movement(&mut self, movement: Movement) {
        for _ in 0..movement.number_of_steps {
            self.move_head(movement.direction);
            self.move_tail(1);
        }
    }

    fn count_visited(&self) -> usize {
        self.visited.iter().fold(0, |acc, v| {
            acc + v
                .iter()
                .fold(0, |acc, visited| acc + if *visited { 1 } else { 0 })
        })
    }

    fn shift_grid(&mut self, offset: Vec2) {
        for knot in &mut self.knots {
            *knot = *knot + offset;
        }

        self.start = self.start + offset;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Input file not provided");

    let number_of_knots = args.get(2).and_then(|val| val.parse().ok()).unwrap_or(2);

    let mut grid = Grid::new(number_of_knots);

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let movement = line.parse().unwrap();
            grid.apply_movement(movement);
        }
    }

    grid.print();

    println!("Visited: {}", grid.count_visited());
}
