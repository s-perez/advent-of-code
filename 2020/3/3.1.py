#!/bin/sh python3
import itertools
from typing import List

Input = List[str]
Solution = int


def read_data(filename: str) -> Input:
    with open(filename, 'r') as f:
        return list(f)


def solve_problem(numbers: Input) -> Solution:
    trees = 0
    x = 0
    y = 0
    while y < len(numbers):
        trees += 1 if numbers[y][x] == "#" else 0
        x = (x + 3) % len(numbers[0])
        y += 1 
    return trees


def main():
    data = read_data("input_3.txt")
    print(solve_problem(data))
    


if __name__ == "__main__":
    main()
