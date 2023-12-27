extern crate AdventOfCode2023;

use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

build_execute!("PART_1", {

    let mut duration: Vec<i64> = vec![];
    let mut product = 1;

    for line_result in read_lines("inputs/day6.txt").unwrap() {
        if let Ok(line) = line_result {
            match line.split(' ').filter(|item| !item.is_empty()).collect::<Vec<_>>().as_slice() {
                ["Time:", all @ ..] => duration = all.iter().map(|item| item.parse::<i64>().unwrap()).collect::<Vec<_>>(),
                ["Distance:", all @ ..] => {
                    for i in 0..all.len() {
                        let n = duration[i];
                        let c = all[i].parse::<i64>().unwrap();

                        let sqrt_value = f64::sqrt((n * n - 4 * c) as f64);
                        let mut start_value = (- (-(n as f64) + sqrt_value) / 2.0).ceil();
                        let mut end_value = (- (-(n as f64) - sqrt_value) / 2.0).floor();

                        if start_value as i64 * (n - start_value as i64) == c {
                            start_value += 1.0;
                        }

                        if end_value as i64 * (n - end_value as i64) == c {
                            end_value -= 1.0;
                        }

                        println!("Start: {}, End: {}, Middle: {}",  start_value, end_value, end_value - start_value + 1.0);
                        product *= (end_value - start_value + 1.0) as i64;
                    }
                },
                [..] => unreachable!()
            }
        }
    }

    println!("Result: {}", product);
});

build_execute!("PART_2", {

    let mut duration: i64 = 0;

    for line_result in read_lines("inputs/day6.txt").unwrap() {
        if let Ok(line) = line_result {
            match line.split(' ').filter(|item| !item.is_empty()).collect::<Vec<_>>().as_slice() {
                ["Time:", all @ ..] => duration = all.join("").parse::<i64>().unwrap(),
                ["Distance:", all @ ..] => {
                    let n = duration;
                    let c = all.join("").parse::<i64>().unwrap();

                    let sqrt_value = f64::sqrt((n * n - 4 * c) as f64);
                    let mut start_value = (- (-(n as f64) + sqrt_value) / 2.0).ceil();
                    let mut end_value = (- (-(n as f64) - sqrt_value) / 2.0).floor();

                    if start_value as i64 * (n - start_value as i64) == c {
                        start_value += 1.0;
                    }

                    if end_value as i64 * (n - end_value as i64) == c {
                        end_value -= 1.0;
                    }

                    println!("Start: {}, End: {}, Middle: {}",  start_value, end_value, end_value - start_value + 1.0);
                },
                [..] => unreachable!()
            }
        }
    }
});

build_default_execute!();

fn main() { execute(); }