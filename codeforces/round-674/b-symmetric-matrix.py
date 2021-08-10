t = int(input())

'''
type a:
x A
A y

type b:
B B
B B

junk tile:
w x
y z
'''
def classify_tile(tile):
    # Gotta check this one first or you might get a 'false positive' type a or b tile
    if tile[0][0] == tile[0][1] and tile[0][0] == tile[1][0] and tile[0][0] == tile[1][1]:
        return "b"
    elif tile[0][1] == tile[1][0]:
        return "a"
    else:
        return "junk"

def read_tiles(num_tiles):
    tile_types = {
        "a": 0,
        "b": 0,
        "junk": 0
    }

    for _ in range(0, num_tiles):
        row_one = list(map(int, input().split()))
        row_two = list(map(int, input().split()))
        tile = [row_one, row_two]

        tile_type = classify_tile(tile)
        tile_types[tile_type] += 1

    return tile_types

def solve_case():
    n, m = list(map(int, input().split()))
    if m % 2 != 0:
        # Gotta read the tiles anyway so the rest of the cases aren't messed up
        tile_types = read_tiles(n)
        print("NO")
    else:
        tile_types = read_tiles(n)
        if tile_types["b"] >= 1 or tile_types["a"] >= 1:
            print("YES")
        else:
            print("NO")

for i in range(0, t):
    solve_case()
