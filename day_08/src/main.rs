use std::{
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

fn count_visible_trees(forest: &Vec<Vec<u32>>) -> usize {
    let width = forest.len();
    let height = forest[0].len();

    width * 2 + height * 2 - 4
        + (1..width - 1).fold(0, |acc, x| {
            acc + (1..height - 1).fold(0, |acc, y| {
                let tree_height = forest[y][x];

                if (0..x).all(|i| forest[y][i] < tree_height)
                    || (x + 1..width).all(|i| forest[y][i] < tree_height)
                    || (0..y).all(|i| forest[i][x] < tree_height)
                    || (y + 1..height).all(|i| forest[i][x] < tree_height)
                {
                    acc + 1
                } else {
                    acc
                }
            })
        })
}

fn calculate_max_scenic_score(forest: &Vec<Vec<u32>>) -> usize {
    let width = forest.len();
    let height = forest[0].len();

    (1..width - 1)
        .map(|x| {
            (1..height - 1)
                .map(|y| {
                    (0..x)
                        .rev()
                        .enumerate()
                        .find(|(_, i)| forest[y][*i] >= forest[y][x])
                        .map(|(distance, _)| distance + 1)
                        .unwrap_or(x)
                        * (x + 1..width)
                            .enumerate()
                            .find(|(_, i)| forest[y][*i] >= forest[y][x])
                            .map(|(distance, _)| distance + 1)
                            .unwrap_or(width - x - 1)
                        * (0..y)
                            .rev()
                            .enumerate()
                            .find(|(_, i)| forest[*i][x] >= forest[y][x])
                            .map(|(distance, _)| distance + 1)
                            .unwrap_or(y)
                        * (y + 1..height)
                            .enumerate()
                            .find(|(_, i)| forest[*i][x] >= forest[y][x])
                            .map(|(distance, _)| distance + 1)
                            .unwrap_or(height - y - 1)
                })
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Input file not provided");

    let mut forest = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let a: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            forest.push(a);
        }
    }

    let visible = count_visible_trees(&forest);

    println!("Visible trees: {visible}");

    let max_score = calculate_max_scenic_score(&forest);
    println!("max score: {max_score}");
}
