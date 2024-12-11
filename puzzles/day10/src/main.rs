use clap::Parser;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.input_file).unwrap();
    let mut reader = io::BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let (part1, part2) = resolve(&input);
    println!(
        "#### Solutions ####\n Part 1: {:?}\n Part 2: {:?}",
        part1, part2
    );
}

type Matrix = Vec<Vec<u8>>;

struct Node {
    val: u8,
    next: Option<Box<Node>>,
}

fn resolve(lines: &str) -> (i32, i32) {
    let _matrix = parse_input(lines);

    unimplemented!()
}

fn build_lists(matrix: &Matrix) -> Vec<Node> {
    matrix.iter().for_each(|val| todo!());
}

fn parse_input(input: &str) -> Matrix {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn trailhead_score(matrix: &Matrix) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailhead_score() {
        let input = "
            ...0...\n
            ...1...\n
            ...2...\n
            6543456\n
            7.....7\n
            8.....8\n
            9.....9";

        let matrix = parse_input(input);
        assert_eq!(trailhead_score(&matrix), 2);
    }
}
