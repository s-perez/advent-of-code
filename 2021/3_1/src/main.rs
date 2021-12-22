extern crate clap;

mod types;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

use clap::{Arg, App};

use types::{Reading, PowerConsumption};


fn read_testcase<P>(filename: P) -> io::Result<Vec<Reading>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    parse_testcase(BufReader::new(file).lines())
}

fn parse_testcase(content: Lines<BufReader<File>>) -> Result<Vec<Reading>, Error> {
    Ok(content
       .filter(Result::is_ok)
       .map(Result::unwrap)
       .map(Reading::from_str)
       .collect::<Vec<Reading>>())
}

fn solve(test_case: Vec<Reading>) -> Result<PowerConsumption, Error> {
    Ok(test_case
       .into_iter()
       .reduce(Reading::reducer)
       .unwrap()
       .get_power_consumption())
}

fn print_solution(result: Result<PowerConsumption, Error>) {
    match result {
        Ok(solution) => print!("Gamma: {}\nEpsilon: {}\nPower Consumption: {}\n", solution.gamma_rate, solution.epsilon_rate, solution.value),
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
