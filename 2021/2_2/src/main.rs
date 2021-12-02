extern crate clap;
use clap::{Arg, App};

mod types;
use types::{Movement, Position, get_direction_by_name};

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

fn read_testcase<P>(filename: P) -> io::Result<Vec<Movement>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    parse_testcase(BufReader::new(file).lines())
}

fn parse_testcase(content: Lines<BufReader<File>>) -> Result<Vec<Movement>, Error> {
    Ok(content
       .filter(Result::is_ok)
       .map(Result::unwrap)
       .map(split_line)
       .map(|[direction, amount]| Movement{amount:amount.parse::<i32>().unwrap(), direction:get_direction_by_name(direction).unwrap()})
       .collect())
}

fn split_line(line: String) -> [String; 2] {
    match line.split_once(" ") {
        Some(x) => [x.0.to_string(), x.1.to_string()],
        None => ["up".to_string(), "0".to_string()]
    }
}

fn solve(test_case: Vec<Movement>) -> Result<i32, Error> {
    Ok(test_case
        .into_iter()
        .fold(Position{x: 0, y: 0, aim: 0}, |position, step| position.apply_step(step))
        .product())
}

fn print_solution(result: Result<i32, Error>) {
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
