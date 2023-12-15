extern crate AdventOfCode2023;

use std::{
    io::{BufRead},
};
use std::cmp::max;
use std::ops::{Range, RangeInclusive};
use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

build_execute!("PART_1", {
    let mut sum = 0;

    let mut engine_dimensions: (i32, i32) = (0, 0);
    let mut num_lines = 0;

    let mut number_data: Vec<(u32, Range<i32>, Range<i32>, bool)> = Vec::new();
    let mut symbol_data: Vec<(i32, i32)> = Vec::new();

    for lineResult in read_lines("y2023/inputs/day3.txt").unwrap() {
        if let Ok(line) = lineResult {
            // Update engine dimensions
            engine_dimensions.1 += 1;
            engine_dimensions.0 = max(line.len() as i32, engine_dimensions.0);

            // Parse this line to get list of ints
            let mut number: Vec<char> = vec![];
            let mut x_range = Range {start: 0, end: 0};

            for pair in line.char_indices() {
                if (pair.1.is_ascii_digit()) {
                    number.push(pair.1);
                    x_range.end = pair.0 as i32;
                } else if (number.is_empty()) {

                    if (pair.1 != '.') {
                        println!("Found {} at ({}, {})", pair.1, pair.0, engine_dimensions.1 - 1);
                        symbol_data.push((pair.0 as i32, engine_dimensions.1 - 1));
                    }

                    x_range.start += 1;
                    continue;
                } else {
                    let result = number.iter().fold(String::new(), |acc, item| {
                        return format!("{}{}", acc, item);
                    });

                    x_range.start = max(x_range.start - 1, 0);
                    x_range.end += 2;

                    number_data.push((result.parse::<u32>().unwrap(), x_range.clone(), Range {start: engine_dimensions.1 - 2, end: engine_dimensions.1 + 1}, false));

                    number.clear();
                    x_range.start = pair.0 as i32 + 1;
                    x_range.end -= 1;

                    if (pair.1 != '.') {
                        println!("Found {} at ({}, {})", pair.1, pair.0, engine_dimensions.1 - 1);
                        symbol_data.push((pair.0 as i32, engine_dimensions.1 - 1));
                    }
                }
            }

            if (!number.is_empty())
            {
                let result = number.iter().fold(String::new(), |acc, item| {
                    return format!("{}{}", acc, item);
                });

                number_data.push((
                    result.parse::<u32>().unwrap(),
                    Range {
                        start: engine_dimensions.0 - result.len() as i32 - 1,
                        end: engine_dimensions.0 + 1
                    },
                    Range {
                        start: engine_dimensions.1 - 2,
                        end: engine_dimensions.1 + 2
                    },
                    false
                ));

                println!("{:?}", number_data.last());

                number.clear();
            }
        }
    }
    println!("{:?}", engine_dimensions);

    for data in symbol_data {
        for mut numbers in &mut number_data {

            let a = numbers.clone();

            if numbers.1.contains(&data.0) && numbers.2.contains(&data.1) && !numbers.3 {
                println!("Adding {}", numbers.0);
                sum += numbers.0;
                numbers.3 = true;
            }
        }
    }

    println!("{}", sum);
});

build_execute!("PART_2", {
    let mut sum = 0;

    let mut engine_dimensions: (i32, i32) = (0, 0);
    let mut num_lines = 0;

    let mut number_data: Vec<(u32, Range<i32>, Range<i32>)> = Vec::new();
    let mut symbol_data: Vec<(i32, i32)> = Vec::new();

    for lineResult in read_lines("y2023/inputs/day3.txt").unwrap() {
        if let Ok(line) = lineResult {
            // Update engine dimensions
            engine_dimensions.1 += 1;
            engine_dimensions.0 = max(line.len() as i32, engine_dimensions.0);

            // Parse this line to get list of ints
            let mut number: Vec<char> = vec![];
            let mut x_range = Range {start: 0, end: 0};

            for pair in line.char_indices() {
                if (pair.1.is_ascii_digit()) {
                    number.push(pair.1);
                    x_range.end = pair.0 as i32;
                } else if (number.is_empty()) {

                    if (pair.1 == '*') {
                        println!("Found {} at ({}, {})", pair.1, pair.0, engine_dimensions.1 - 1);
                        symbol_data.push((pair.0 as i32, engine_dimensions.1 - 1));
                    }

                    x_range.start += 1;
                    continue;
                } else {
                    let result = number.iter().fold(String::new(), |acc, item| {
                        return format!("{}{}", acc, item);
                    });

                    x_range.start = max(x_range.start - 1, 0);
                    x_range.end += 2;

                    number_data.push((result.parse::<u32>().unwrap(), x_range.clone(), Range {start: engine_dimensions.1 - 2, end: engine_dimensions.1 + 1}));

                    number.clear();
                    x_range.start = pair.0 as i32 + 1;
                    x_range.end -= 1;

                    if (pair.1 == '*') {
                        println!("Found {} at ({}, {})", pair.1, pair.0, engine_dimensions.1 - 1);
                        symbol_data.push((pair.0 as i32, engine_dimensions.1 - 1));
                    }
                }
            }

            if (!number.is_empty())
            {
                let result = number.iter().fold(String::new(), |acc, item| {
                    return format!("{}{}", acc, item);
                });

                number_data.push((
                    result.parse::<u32>().unwrap(),
                    Range {
                        start: engine_dimensions.0 - result.len() as i32 - 1,
                        end: engine_dimensions.0 + 1
                    },
                    Range {
                        start: engine_dimensions.1 - 2,
                        end: engine_dimensions.1 + 2
                    }
                ));

                println!("{:?}", number_data.last());

                number.clear();
            }
        }
    }
    println!("{:?}", engine_dimensions);


    for data in symbol_data {

        let mut not_a_gear= false;
        let mut ratio_components: Vec<i32> = vec![];

        for mut numbers in &mut number_data {

            let a = numbers.clone();

            if numbers.1.contains(&data.0) && numbers.2.contains(&data.1) {
                if ratio_components.len() >= 2 {
                    not_a_gear = true;
                    break;
                }

                ratio_components.push(numbers.0 as i32);
            }
        }

        if not_a_gear { continue; }
        if ratio_components.len() != 2 { continue; }

        sum += ratio_components.iter().fold(1, |acc, item| acc * item);
    }

    println!("{}", sum);
});

build_default_execute!();

fn main() {
    execute();
}