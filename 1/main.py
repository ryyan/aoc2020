## AOC 2020
## Day 1: Report Repair
## Find the two entries that sum to 2020 and then multiply those two numbers together
##
## How to run:
## python main.py

# Get number entries and convert to list of ints
with open("input") as file:
    numbers = file.read().splitlines()
    numbers = [int(num) for num in numbers]

# Find the two numbers that sum to 2020
for num in numbers:
    remainder = 2020 - num

    if remainder in numbers:
        total = num * remainder
        print(f"{num} * {remainder} = {total}")
        continue

# Find the three numbers that sum to 2020
for num1 in numbers:
    remainder = 2020 - num1

    for num2 in numbers:
        remainder2 = remainder - num2

        if remainder2 in numbers:
            total = num1 * num2 * remainder2
            print(f"{num1} * {num2} * {remainder2} = {total}")
