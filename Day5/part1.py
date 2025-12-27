import ssl
import os
from dotenv import load_dotenv

load_dotenv()

# Source - https://stackoverflow.com/a
# Posted by Nick Presta, modified by community. See post 'Timeline' for change history
# Retrieved 2025-12-25, License - CC BY-SA 4.0
ssl_context = ssl._create_unverified_context()

headers = {
    "Cookie": f"session={os.getenv('SESSION_COOKIE')}"
}
# request = req.Request("https://adventofcode.com/2025/day/5/input", headers=headers)
# contents = req.urlopen(request, context=ssl_context).read().decode('utf-8')
f = open("input.txt")

fresh_ingredients: int = 0
check_ranges: list[list[int]] = []
check_numbers: list[int] = []
output_buffer: str = ""

def part1(puzzle_input):
    # print(puzzle_input)

    split_input = puzzle_input.split("\n\n")

    id_ranges = split_input[0] 

    print(id_ranges)

    product_ids = split_input[1]

    print(product_ids)

    for line in id_ranges.split("\n"):
        if line.strip():  # Skip blank lines

            split_range: list[int] = [int(x) for x in line.split("-")]
            check_ranges.append(split_range)

    for line in product_ids.split("\n"):
        if line.strip():  # Skip blank lines
            check_numbers.append(int(line.rstrip()))

    for check_number in check_numbers:
        check_in_range(check_number, check_ranges)

    print(f"{fresh_ingredients} fresh ingredients out of {len(check_numbers)}")
    return fresh_ingredients


def check_in_range(check_number: int, check_ranges: list[list[int]]):
    global fresh_ingredients
    global output_buffer
    for check_range in check_ranges:
        if check_number < check_range[0]:
            output_buffer += f"{check_number:<18}Bad \n"

        elif check_number <= check_range[1]:
            fresh_ingredients += 1
            output_buffer += f"{check_number:<18}Fresh {fresh_ingredients} \n"
            break


print(part1(f.read()))

# print(part1(contents))

f.close()