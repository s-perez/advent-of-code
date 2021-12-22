extern crate clap;

mod types;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

use clap::{Arg, App};

use types::{LifeSupport, Reading};


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

fn solve(test_case: Vec<Reading>) -> Result<LifeSupport, Error> {
    let mut remaining = test_case.clone();
    let mut index = 0;
    while remaining.len() > 1 {
        remaining = remaining
            .into_iter()
            .filter(|reading| Reading::oxygen_filter(remaining, index)(reading.clone()))
            .collect();
        index += 1;
    }
    let oxygen_value = remaining[0].get_value();

    while remaining.len() > 1 {
        remaining = remaining.clone()
            .into_iter()
            .filter(|reading| Reading::co2_filter(remaining, index)(reading.clone()))
            .collect();
        index += 1;
    }
    let co2_value = remaining[0].get_value();

    Ok(LifeSupport{
        oxygen_generator_rating: oxygen_value,
        co2_scrubber_rating: co2_value,
        value: oxygen_value * co2_value
    })
}

fn print_solution(result: Result<LifeSupport, Error>) {
    match result {
        Ok(solution) => print!("Oxygen: {}\nCO2: {}\nLife Support: {}\n", solution.oxygen_generator_rating, solution.co2_scrubber_rating, solution.value),
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
