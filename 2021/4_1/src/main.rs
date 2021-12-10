extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

mod types;


fn read_testcase<P>(filename: P) -> io::Result<Vec<i32>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    parse_testcase(BufReader::new(file).lines())
}

fn parse_testcase(content: Lines<BufReader<File>>) -> Result<Vec<i32>, Error> {
    Ok(content
       .filter(Result::is_ok)
       .map(Result::unwrap)
       .map(|line| line.parse::<i32>())
       .map(Result::unwrap)
       .collect())
}

fn solve(test_case: Vec<i32>) -> Result<usize, Error> {
    Ok(test_case
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count())
}

fn print_solution(result: Result<usize, Error>) {
    match result {
        Ok(solution) => print!("{}\n", solution),
        Err(_) => print!("Error solving for provided test case")
    };
}

fn main() {
    let matches = App::new("Advent of Code")
        .version("1.0")
        .author("Salvador P. <sisekeom@protonmail.com>")
        .about("Solution for advent of code 2021 1.1")
        .arg(Arg::with_name("filepath")
                .help("path to the test case file")
                .required(true)
                .index(1))
        .get_matches();

    let filepath = matches.value_of("filepath").unwrap();

    read_testcase(filepath).map(solve).map(print_solution).unwrap();
}
