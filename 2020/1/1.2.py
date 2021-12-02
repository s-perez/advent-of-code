#!/bin/sh python3
import itertools

from functools import reduce
from typing import List

Input = List[int]
Solution = int


def read_data(filename: str) -> Input:
    with open(filename, 'r') as f:
        return [int(x) for x in list(f)]


def solve_problem(numbers: Input) -> Solution:
    combinations = itertools.combinations(numbers, 3)
    right = next(combination for combination in combinations if sum(combination) == 2020)
    return reduce(lambda x, y: x*y, right)



def main():
    data = read_data("input_1.txt")
    print(solve_problem(data))
    


if __name__ == "__main__":
    main()
