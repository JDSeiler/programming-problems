from collections import deque

_w, h = list(map(int, input().split()))

island = []
start_coords = (-1,-1)
for row_num in range(h):
    row = []
    row_string = input()
    for col, c in enumerate(row_string):
        if c == '#':
            row.append(-1)
        elif c == '.':
            row.append(0)
        else:
            start_coords =  (row_num, col)
            row.append(0)
    
    island.append(row)

instructions = input()

def next_cells(dirr, curr):
    r, c = curr
    north = (r-1, c)
    south = (r+1, c)
    east = (r, c+1)
    west = (r, c-1)
    if dirr == 'N':
        return [
            south,
            east,
            west
        ]
    elif dirr == 'S':
        return [
            north,
            east,
            west
        ]
    elif dirr == 'E':
        return [
            north,
            south,
            west
        ]
    else:
        return [
            north,
            south,
            east
        ]

frontier = deque()
frontier.append(start_coords)

sr, sc = start_coords
island[sr][sc] = 2

seen = set()
seen.add(start_coords)

for dirr in instructions:
    to_explore = list(frontier)
    frontier.clear()

    for r, c in to_explore:
        neighbors = next_cells(dirr, (r,c))

        for nr, nc in neighbors:
            if (nr, nc) not in seen and island[nr][nc] != -1:
                frontier.append((nr, nc))
                seen.add((nr, nc))

for r, c in frontier:
    island[r][c] = 1

def print_island(island):
    for row in island:
        for cell in row:
            if cell == 1:
                print('!', end='')
            elif cell == 0:
                print('.', end='')
            elif cell == -1:
                print('#', end='')
            else:
                print('S', end='')
        print()
    
print_island(island)
