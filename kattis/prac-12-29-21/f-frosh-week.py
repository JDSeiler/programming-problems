from bisect import bisect_left

nAndM = input()

tasks = sorted(list(map(int, input().split())))
intervals = sorted(list(map(int, input().split())))

completed = 0

for task in tasks:
    # Hooray for the Python docs
    # https://docs.python.org/3/library/bisect.html#searching-sorted-lists
    # bisect_left finds the leftmost (smallest, in an ASC sorted list)
    # item larger than "task" in "intervals"
    # We greedily pair each task with the smallest interval that can fit it

    interval_idx = bisect_left(intervals, task)
    if interval_idx != len(intervals):
        completed += 1
        # Hail mary to try and emulate constant time removal?
        intervals[interval_idx] = -1

print(completed)

