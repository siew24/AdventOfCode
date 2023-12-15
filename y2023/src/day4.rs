extern crate AdventOfCode2023;

use std::collections::VecDeque;
use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

build_execute!("PART_1", {
    let mut sum = 0;

    for line_result in read_lines("y2023/inputs/day4.txt").unwrap() {
        if let Ok(line) = line_result {

            match line.split(&[':', '|'][..]).map(|data| data.trim()).collect::<Vec<_>>().as_slice() {
                [tag, winning_str, actual_str] => {
                    let winnings_seq: Vec<i8> = winning_str.split(' ').fold(vec![], |mut acc, item| {
                        if item.is_empty() { return acc; }

                        acc.push(item.parse::<i8>().unwrap());

                        return acc;
                    });

                    let actuals_seq: Vec<_> = actual_str.split(' ').fold(vec![], |mut acc, item| {
                        if item.is_empty() { return acc; }

                        acc.push(item.parse::<i8>().unwrap());

                        return acc;
                    });

                    let mut score = 0;

                    for number in &winnings_seq {
                        if actuals_seq.contains(&number) {
                            score = if score == 0 { 1 } else { score << 1 };
                        }
                    }

                    println!("Tag: {}, Winnings: {:?}, Actual: {:?}, Score: {}", tag, winnings_seq, actuals_seq, score);

                    sum += score;
                },
                _ => panic!()
            }
        }
    }

    println!("Total: {}", sum);
});

build_execute!("PART_2", {
    let mut sum = 0;

    let mut counts = VecDeque::<u32>::from([1]);

    for line_result in read_lines("y2023/inputs/day4.txt").unwrap() {
        if let Ok(line) = line_result {

            println!("{:?}", counts);

            let acc = match counts.pop_front() {
                Some(result) => result,
                _ => 1
            };

            sum += acc;

            match line.split(&[':', '|'][..]).map(|data| data.trim()).collect::<Vec<_>>().as_slice() {
                [tag, winning_str, actual_str] => {
                    let winnings_seq: Vec<i8> = winning_str.split(' ').fold(vec![], |mut acc, item| {
                        if item.is_empty() { return acc; }

                        acc.push(item.parse::<i8>().unwrap());

                        return acc;
                    });

                    let actuals_seq: Vec<_> = actual_str.split(' ').fold(vec![], |mut acc, item| {
                        if item.is_empty() { return acc; }

                        acc.push(item.parse::<i8>().unwrap());

                        return acc;
                    });

                    let mut index = 0;

                    for number in &winnings_seq {
                        if actuals_seq.contains(&number) {
                            if index >= counts.len() {
                                counts.push_back(1);
                            }

                            counts[index] += acc;
                            index += 1;
                        }
                    }

                    println!("Tag: {}, Winnings: {:?}, Actual: {:?}", tag, winnings_seq, actuals_seq);
                },
                _ => panic!()
            }
        }
    }

    println!("Total: {}", sum);
});

build_default_execute!();

fn main() { execute(); }