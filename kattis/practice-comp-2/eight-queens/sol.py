def generate_covered_squares(r,c):
    squares = []
    row = [(i,x) for x in range(0,8)]
    col = [(x,j) for x in range(0,8)]
    
    dr_d = []
    start_row = -1
    start_col = -1
    if r-c >= 0:
        start_row = r-c
        start_col = 0
    else:
        start_row = 0
        start_col = abs(r-c)

    dr_d.append((start_row, start_col))
    while start_row < 8 and start_col < 8:
        start_row+=1
        start_col+=1
        dr_d.append((start_row, start_col))

    ur_d = []
    start_row = r
    start_col = c
    while start_row > 0 and start_col < 8:
        ur_d.append((r,c))
        start_row -= 1
        start_col += 1

    squares.extend(row)
    squares.extend(col)
    squares.extend(dr_d)
    squares.extend(ur_d)
    return squares

board = []
queen_count = 0
for i in range(0, 8):
    row = input()
    converted_row = []
    for c in row:
        if c == "*":
            converted_row.append(1)
            queen_count+=1
        else:
            converted_row.append(0)
    board.append(converted_row)

valid = queen_count == 8

covered = set()
if valid:
    for i, row in enumerate(board):
        for j, square in enumerate(row):
            if square == 1:
                if (i,j) in covered:
                    valid = False
                    break
                else:
                    new_covered = generate_covered_squares(i,j)
                    covered.update(new_covered)

if valid:
    print("valid")
else:
    print("invalid")
