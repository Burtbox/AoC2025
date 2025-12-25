import numpy as np
import urllib.request as req
import ssl
import os
from dotenv import load_dotenv

load_dotenv()

roll = '@'
empty_space = '.'

# Source - https://stackoverflow.com/a
# Posted by Nick Presta, modified by community. See post 'Timeline' for change history
# Retrieved 2025-12-25, License - CC BY-SA 4.0
ssl_context = ssl._create_unverified_context()

headers = {
    "Cookie": f"session={os.getenv('SESSION_COOKIE')}"
}
request = req.Request("https://adventofcode.com/2025/day/4/input", headers=headers)
contents = req.urlopen(request, context=ssl_context).read().decode('utf-8')
# f = open("exampleInput.txt")



def part2(puzzle_input):
    grid = np.array(
        [[char == '@' for char in line] for line in puzzle_input.splitlines()], 
        dtype=np.uint8
    )

    def is_removable(grid):
        padded = np.pad(grid, 1)
        neighbors = (
            padded[:-2, :-2] +  # up-left
            padded[:-2, 1:-1] + # up
            padded[:-2, 2:] +   # up-right
            padded[1:-1, :-2] + # left
            padded[1:-1, 2:] +  # right
            padded[2:, :-2] +   # down-left
            padded[2:, 1:-1] +  # down
            padded[2:, 2:]      # down-right
        )
        return grid & (neighbors < 4)
    
    removed = 0
    removable = is_removable(grid)
    while removable.any():
        grid -= removable
        removed += removable.sum()
        removable = is_removable(grid)
    
    return int(removed)

print(part2(contents))