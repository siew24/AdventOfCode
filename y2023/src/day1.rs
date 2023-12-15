extern crate AdventOfCode2023;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

build_execute!("PART_1", {
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("y2023/inputs/day1.txt") {
        for line in lines {
            if let Ok(result) = line {
                println!("{}", result);

                let chars: Vec<char> = result.chars().collect();
                let chars_rev: Vec<char> = result.chars().rev().collect();

                let mut value: u32 = 0;

                for char in chars {
                    if char.is_ascii_digit() {
                        value += char.to_digit(10).unwrap() * 10;
                        break;
                    }
                }

                for char in chars_rev {
                    if char.is_ascii_digit() {
                        value += char.to_digit(10).unwrap();
                        break;
                    }
                }

                println!("{}", value);
                sum += value;
            }
        }
    }

    println!("result: {}", sum);
});

build_execute!("PART_2", {
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("y2023/inputs/day1.txt") {
        for line in lines {
            if let Ok(result) = line {
                println!("{}", result);

                let s: Vec<&str> = vec![
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1",
                    "2", "3", "4", "5", "6", "7", "8", "9",
                ];

                let v = vec![1..9];

                let mut merged: Vec<_> = s
                    .iter()
                    .flat_map(|value| result.match_indices(value))
                    .collect();

                merged.sort();

                let mut value = 10
                    * match merged.first() {
                        Some((_, "one")) => 1,
                        Some((_, "two")) => 2,
                        Some((_, "three")) => 3,
                        Some((_, "four")) => 4,
                        Some((_, "five")) => 5,
                        Some((_, "six")) => 6,
                        Some((_, "seven")) => 7,
                        Some((_, "eight")) => 8,
                        Some((_, "nine")) => 9,
                        Some((_, "1")) => 1,
                        Some((_, "2")) => 2,
                        Some((_, "3")) => 3,
                        Some((_, "4")) => 4,
                        Some((_, "5")) => 5,
                        Some((_, "6")) => 6,
                        Some((_, "7")) => 7,
                        Some((_, "8")) => 8,
                        Some((_, "9")) => 9,
                        Some((_, _)) => 0,
                        None => 0,
                    };

                value += match merged.last() {
                    Some((_, "one")) => 1,
                    Some((_, "two")) => 2,
                    Some((_, "three")) => 3,
                    Some((_, "four")) => 4,
                    Some((_, "five")) => 5,
                    Some((_, "six")) => 6,
                    Some((_, "seven")) => 7,
                    Some((_, "eight")) => 8,
                    Some((_, "nine")) => 9,
                    Some((_, "1")) => 1,
                    Some((_, "2")) => 2,
                    Some((_, "3")) => 3,
                    Some((_, "4")) => 4,
                    Some((_, "5")) => 5,
                    Some((_, "6")) => 6,
                    Some((_, "7")) => 7,
                    Some((_, "8")) => 8,
                    Some((_, "9")) => 9,
                    Some((_, _)) => 0,
                    Some((_, _)) => 0,
                    None => 0,
                };

                println!("{}", value);

                sum += value;
            }
        }
    }

    println!("result: {}", sum);
});

build_default_execute!();

fn main() {
    execute();
}