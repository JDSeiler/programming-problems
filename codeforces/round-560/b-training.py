n = int(input())
unordered_contests = list(map(int, input().split()))

contests = sorted(unordered_contests, reverse=True)

day_count = 0
for k in range(1, n+1): # 1-index
    try:
        contest_completed = False
        while not contest_completed:
            some_contest = contests.pop()
            if k <= some_contest:
                day_count += 1
                contest_completed = True
    except IndexError:
        # Our stack is empty, we're done
        break

print(day_count)
