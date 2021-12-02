#!/bin/sh python3
import itertools
from typing import List

Input = List[int]
Solution = int


def read_data(filename: str) -> Input:
    with open(filename, 'r') as f:
        return [int(x) for x in list(f)]


def solve_problem(numbers: Input) -> Solution:
    combinations = itertools.combinations(numbers, 2)
    right = next(pair for pair in combinations if pair[0] + pair[1] == 2020)
    return right[0] * right[1]



def main():
    data = read_data("input_1.txt")
    print(solve_problem(data))
    


if __name__ == "__main__":
    main()
