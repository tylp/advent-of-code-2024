use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    input_file: String,
}

fn main() {
    resolve(Args::parse().input_file);
}

fn resolve(input_file: String) -> u32 {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();

    let file = File::open(&input_file).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().for_each(|line| {
        let content = line.unwrap();
        let splitted: Vec<&str> = content.split_ascii_whitespace().collect();
        list_1.push(splitted[0].parse().unwrap());
        list_2.push(splitted[1].parse().unwrap());
    });

    list_1.sort();
    list_2.sort();

    let sum = list_1
        .iter()
        .zip(list_2.iter())
        .fold(0, |acc, (l1, l2)| acc + l1.abs_diff(*l2));

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve("input".to_string()), 3574690);
    }
}