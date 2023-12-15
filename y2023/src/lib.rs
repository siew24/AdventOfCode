use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

#[macro_export]
macro_rules! build_execute {
    ($feature:tt, $body:expr) => {
        #[cfg(feature = $feature)]
        fn execute() { $body }
    };
}

#[macro_export]
macro_rules! build_default_execute {
    () => {
        #[cfg(not(any(feature = "PART_1", feature = "PART_2")))]
        fn execute() {
            println!("You did not enable any features.");
        }
    }
}