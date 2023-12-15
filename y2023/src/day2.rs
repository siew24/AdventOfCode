extern crate AdventOfCode2023;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

build_execute!("PART_1", {
    let mut sum = 0;

    for lineResult in read_lines("y2023/inputs/day2.txt").unwrap() {
        if let Ok(line) = lineResult {
            let mut data: Vec<&str> = line.split(":").collect();

            let tag: Vec<&str> = data[0].split(" ").collect();

            let id = tag[1].parse::<u32>().unwrap();

            data = data[1].split(";").collect();

            let mut maxSeen: (u32, u32, u32) = (0, 0, 0);

            for section in data {
                let cubes: Vec<(u32, &str)> = section
                    .split(",")
                    .map(|value| {
                        let res: Vec<&str> = value.trim().split(" ").collect();

                        (res[0].parse::<u32>().unwrap(), res[1])
                    })
                    .collect();

                for cube in cubes {
                    match cube {
                        (value, "red") => {
                            maxSeen.0 = if value > maxSeen.0 { value } else { maxSeen.0 }
                        }
                        (value, "green") => {
                            maxSeen.1 = if value > maxSeen.1 { value } else { maxSeen.1 }
                        }
                        (value, "blue") => {
                            maxSeen.2 = if value > maxSeen.2 { value } else { maxSeen.2 }
                        }
                        (_, _) => {
                            panic!()
                        }
                    }
                }
            }

            if maxSeen.0 > 12 || maxSeen.1 > 13 || maxSeen.2 > 14 {
                continue;
            }

            sum += id;
        }
    }

    println!("{}", sum);
});

build_execute!("PART_2", {
    let mut sum = 0;

    for lineResult in read_lines("y2023/inputs/day2.txt").unwrap() {
        if let Ok(line) = lineResult {
            let mut data: Vec<&str> = line.split(":").collect();

            let tag: Vec<&str> = data[0].split(" ").collect();

            let id = tag[1].parse::<u32>().unwrap();

            data = data[1].split(";").collect();

            let mut maxSeen: (u32, u32, u32) = (0, 0, 0);

            for section in data {
                let cubes: Vec<(u32, &str)> = section
                    .split(",")
                    .map(|value| {
                        let res: Vec<&str> = value.trim().split(" ").collect();

                        (res[0].parse::<u32>().unwrap(), res[1])
                    })
                    .collect();

                for cube in cubes {
                    match cube {
                        (value, "red") => {
                            maxSeen.0 = if value > maxSeen.0 { value } else { maxSeen.0 }
                        }
                        (value, "green") => {
                            maxSeen.1 = if value > maxSeen.1 { value } else { maxSeen.1 }
                        }
                        (value, "blue") => {
                            maxSeen.2 = if value > maxSeen.2 { value } else { maxSeen.2 }
                        }
                        (_, _) => {
                            panic!()
                        }
                    }
                }
            }

            sum += maxSeen.0 * maxSeen.1 * maxSeen.2;
        }
    }

    println!("{}", sum);
});

build_default_execute!();

fn main() {
    execute();
}