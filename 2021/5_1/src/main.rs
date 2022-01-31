extern crate clap;
#[macro_use] extern crate lazy_static;

mod types;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Error};
use std::path::Path;

use clap::{Arg, App};

use types::{Matrix, Segment};


fn read_testcase<P>(filename: P) -> io::Result<Vec<Segment>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    parse_testcase(BufReader::new(file).lines())
}

fn parse_testcase(content: Lines<BufReader<File>>) -> Result<Vec<Segment>, Error> {
    Ok(content
       .filter(Result::is_ok)
       .map(Result::unwrap)
       .map(Segment::from_line)
       .filter(Result::is_ok)
       .map(Result::unwrap)
       .collect())
}

fn solve(test_case: Vec<Segment>) -> Result<u16, Error> {
    let mut matrix = Matrix::new();
    test_case
        .into_iter()
        .for_each(|segment| matrix.mark_segment(segment));

    Ok(matrix.count_crowded())
}

fn print_solution(result: Result<u16, Error>) {
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
