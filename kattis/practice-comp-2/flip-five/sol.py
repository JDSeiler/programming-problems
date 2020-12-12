
def bfs(target):
    moves = [
        416,
        464,
        200,
        308,
        186,
        89,
        38,
        23,
        11
    ]

    start_grid = int('111111111', 2)
    frontier = set()
    frontier.add(start_grid)
    depth = 0
    while target not in frontier:
        new_states = set()
        for state in frontier:
            for move in moves:
                new_state = state ^ move
                new_states.add(new_state)

        frontier.clear()
        frontier.update(new_states)
        depth += 1
    
    return depth


num_puzzles = int(input())
results = []
for i in range(0, num_puzzles):
    target_grid = []
    for i in range(0, 3):
        row = input().strip()
        target_grid.extend(['1' if c == '.' else '0' for c in row])

    target_grid_as_int = int(''.join(target_grid), 2)
    results.append(bfs(target_grid_as_int))

for result in results:
    print(result)