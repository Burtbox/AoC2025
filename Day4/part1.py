roll = '@'
empty_space = '.'

f = open("exampleInput.txt")

line = f.readline()

count_of_rolls = line.count(roll)

print("r", count_of_rolls)

count_of_spaces = line.count(empty_space)

print("s", count_of_spaces)

f.close()