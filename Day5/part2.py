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

check_ranges: list[tuple[int, int]] = []

def part2(puzzle_input):
    split_input = puzzle_input.split("\n\n")
    id_ranges = split_input[0] 

    for line in id_ranges.split("\n"):
        if line.strip():
            start, end = [int(x) for x in line.split("-")]
            check_ranges.append((start, end))

    # Merge overlapping ranges and count unique integers
    check_ranges.sort()
    merged = []
    for start, end in check_ranges:
        if merged and start <= merged[-1][1] + 1:
            #  print(merged[-1][1])
            merged[-1] = (merged[-1][0], max(merged[-1][1], end))
        else:
            merged.append((start, end))
    
    total = sum(end - start + 1 for start, end in merged)
    print(f"{total}")
    return total


print(part2(f.read()))

# print(part1(contents))

f.close()