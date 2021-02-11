cases = int(input());

for i in range(0, cases):
    # We don't care about the size, but have to read it anyway
    case_size = input();
    raw_case = input();
    case = []
    for char in raw_case:
        if char == '(':
            case.append(1)
        else:
            case.append(-1)

    misplaced = 0;
    max_misplaced = 0
    for num in case:
        misplaced += num
        if misplaced < max_misplaced:
            max_misplaced = misplaced

    print(-max_misplaced)
