#!/bin/sh python3
import itertools
import re
from typing import List, Dict

Input = Dict
Solution = int


def parse(line:str) -> Input:
    parsed = re.match("(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)", line)
    return {
        "pos1": int(parsed.group("min")),
        "pos2": int(parsed.group("max")),
        "char": parsed.group("char"),
        "password": parsed.group("password")
    }


def read_data(filename: str) -> List[Input]:
    with open(filename, 'r') as f:
        return [parse(x) for x in list(f)]


def check_password(case: Input) -> bool:
    char1 = case['password'][case['pos1'] - 1]
    char2 = case['password'][case['pos2'] - 1]
    target = case['char']
    return (char1 == target) != (char2 == target)


def solve_problem(numbers: List[Input]) -> Solution:
    return sum(1 for case in numbers if check_password(case))
    

def main():
    data = read_data("input_2.txt")
    print(solve_problem(data))


if __name__ == "__main__":
    main()
