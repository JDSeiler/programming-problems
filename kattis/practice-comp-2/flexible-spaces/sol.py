width, num_p = [int(num) for num in input().split(' ')]
partitions = [int(num) for num in input().split(' ')]

def solve(width, inner_partitions):
    partitions = [0]
    partitions.extend(inner_partitions)
    partitions.append(width)

    results = set()

    for i in range(0, len(partitions)-1):
        for j in range(i+1, len(partitions)):
            new_val = partitions[j] - partitions[i]
            results.add(new_val)

    results = sorted(list(results))
    print(' '.join([str(val) for val in results]).strip())

solve(width, partitions)