#![feature(array_methods, array_chunks)]
extern crate arrayvec;
extern crate clap;
extern crate itertools;
use clap::{Arg, App};
use itertools::Itertools;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

use types::TestCase;

mod types;

fn read_testcase<P>(filename: P) -> io::Result<TestCase>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    parse_testcase(BufReader::new(file).lines())
}

fn parse_testcase(mut content: Lines<BufReader<File>>) -> Result<TestCase, Error> {
    let numbers: Vec<u8> = content
        .next()
        .unwrap().unwrap()
        .split(",")
        .map(|n| n.parse::<u8>())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    let boards: Vec<Vec<u8>> = content
        .map(Result::unwrap)
        .flat_map(|line| line.split(" ").map(String::from).collect::<Vec<String>>())
        .map(|n: String| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .into_iter()
        .chunks(25)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    Ok(TestCase::new(boards, numbers))
}

fn solve(mut test_case: TestCase) -> Result<u32, Error> {
    Ok(test_case.solve())
}

fn print_solution(result: Result<u32, Error>) {
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
