n = int(input())

rows = []
cols = []
for i in range(0, n):
    raw_new_row = input().strip()
    new_row = []
    for char in raw_new_row:
        if char == "B":
            new_row.append(-1)
        else:
            new_row.append(1)
    rows.append(new_row)

for i in range(0, n):
    new_col = []
    for row in rows:
        new_col.append(row[i])
    cols.append(new_col)

def is_balanced(matrix):
    for line in matrix:
        if sum(line) != 0:
            return False
    
    return True

board_is_balanced = is_balanced(rows) and is_balanced(cols)

def no_consec(matrix):
    for line in matrix:
        for i in range(1,n-1):
            if sum(line[i-1:i+2]) == 3 or sum(line[i-1:i+2]) == -3:
                return False

    return True

board_has_no_consec = no_consec(rows) and no_consec(cols)

board_valid = board_has_no_consec and board_is_balanced
if board_valid:
    print(1)
else:
    print(0)
        