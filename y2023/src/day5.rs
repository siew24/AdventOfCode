extern crate AdventOfCode2023;

use std::cmp::{max, min};
use std::ops::Range;
use AdventOfCode2023::{build_default_execute, build_execute, read_lines};

enum DifferenceResult<A, B> {
    Single(A),
    Multiple(B),
    None,
    OutOfBounds
}

trait RangeExt {
    fn add(self, other: i64) -> Self where Self: Sized;
    fn overlaps(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn difference(&self, other: &Self) -> DifferenceResult<Self, (Self, Self)> where Self: Sized;
}


impl RangeExt for Range<i64> {
    fn add(self, other: i64) -> Self where Self: Sized {
        (self.start+other)..(self.end+other)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersects(&self, other: &Self) -> bool {
        self.start < other.end && self.end > other.start
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(max(self.start, other.start)..min(self.end, other.end))
        } else {
            None
        }
    }

    fn difference(&self, other: &Self) -> DifferenceResult<Self, (Self, Self)> {
        if other.overlaps(self) {
            DifferenceResult::None
        } else if self.overlaps(other) {
            DifferenceResult::Multiple((self.start..other.start, other.end..self.end))
        } else if self.intersects(other) {
            if self.start < other.start {
                DifferenceResult::Single(self.start..other.start)
            } else {
                DifferenceResult::Single(other.end..self.end)
            }
        } else {
            DifferenceResult::OutOfBounds
        }
    }
}

build_execute!("PART_1", {
    let mut seeds: Vec<i64> = vec![];
    let mut converted: Vec<bool> = vec![];

    for line_result in read_lines("inputs/day5.txt").unwrap() {
        if let Ok(line) = line_result {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                ["seeds:", tail @ ..] =>  {
                    seeds = tail.iter().map(|item| item.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                    converted = seeds.iter().map(|_| false).collect::<Vec<bool>>();
                    println!("Seeds: {:?}", seeds);
                },
                [category, "map:"] => {
                    println!("Category: {}", category);

                    for i in 0..seeds.len() {
                        converted[i] = false;
                    }
                },
                [dest_start, src_start, dest_length] => {
                    let src_start_int = src_start.parse::<i64>().unwrap();
                    let dest_start_int = dest_start.parse::<i64>().unwrap();
                    let length_int = dest_length.parse::<i64>().unwrap();

                    let src_range = src_start_int..src_start_int+length_int;
                    let dest_range = dest_start_int..dest_start_int+length_int;

                    for i in 0..seeds.len() {
                        if !converted[i] && src_range.contains(&seeds[i]) {
                            seeds[i] += dest_start_int - src_start_int;
                            converted[i] = true;
                        }
                    }

                    println!("Triples: {:?} => {:?}, New seeds: {:?}", src_range, dest_range, seeds);
                },
                [all @ ..] => println!("Default: {:?}", all),
            }
        }
    }

    println!("Lowest location number: {}", seeds.iter().min().unwrap());
});

build_execute!("PART_2", {
    let mut seeds: Vec<Range<i64>> = vec![];
    let mut converted: Vec<bool> = vec![];

    for line_result in read_lines("inputs/day5.txt").unwrap() {
        if let Ok(line) = line_result {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                ["seeds:", tail @ ..] =>  {

                    for i in (0..tail.len()).step_by(2) {
                        let start = tail[i].parse::<i64>().unwrap();
                        let length = tail[i + 1].parse::<i64>().unwrap();

                        seeds.push(start..start+length);
                        converted.push(false);
                    }

                    println!("Seeds: {:?}", seeds);
                },
                [category, "map:"] => {
                    println!("Category: {}", category);

                    for i in 0..seeds.len() {
                        converted[i] = false;
                    }
                },
                [dest_start, src_start, dest_length] => {
                    let src_start_int = src_start.parse::<i64>().unwrap();
                    let dest_start_int = dest_start.parse::<i64>().unwrap();
                    let length_int = dest_length.parse::<i64>().unwrap();

                    let src_range = src_start_int..src_start_int+length_int;
                    let dest_range = dest_start_int..dest_start_int+length_int;

                    for i in 0..seeds.len() {
                        if !converted[i] {
                            match seeds[i].difference(&src_range) {
                                DifferenceResult::Multiple(two) => {

                                    seeds.push(two.0.clone());
                                    seeds.push(two.1.clone());
                                    converted.push(false);
                                    converted.push(false);

                                    seeds[i] = src_range.clone().add(dest_start_int - src_start_int);
                                    converted[i] = true;

                                },
                                DifferenceResult::Single(one) => {
                                    seeds.push(seeds[i].intersection(&src_range).unwrap().add(dest_start_int - src_start_int));
                                    converted.push(true);

                                    seeds[i] = one;
                                    converted[i] = false;
                                },
                                DifferenceResult::None => {
                                    seeds[i] = seeds[i].clone().add(dest_start_int - src_start_int);
                                    converted[i] = true;
                                },
                                DifferenceResult::OutOfBounds => {},
                            }
                        }
                    }

                    println!("Triples: {:?} => {:?}, Length: {}, New seeds: {:?}, converted: {:?}", src_range, dest_range, dest_start_int - src_start_int, seeds, converted);
                },
                [all @ ..] => println!("Default: {:?}", all),
            }
        }
    }

    println!("Lowest location number: {:?}", seeds.iter().min_by(|left, right| left.start.cmp(&right.start)).unwrap().start);
});

build_default_execute!();

fn main() { execute(); }