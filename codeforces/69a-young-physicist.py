def solve(num_vecs):
    xs = []
    ys = []
    zs = []
    for _i in range(0, num_vecs):
        x, y, z = list(map(int, input().split()))
        xs.append(x)
        ys.append(y)
        zs.append(z)
    
    final_vec = (
        sum(xs),
        sum(ys),
        sum(zs)
    )

    if final_vec == (0,0,0):
        print('YES')
    else:
        print('NO')

# Main program
n = int(input())
vecs = solve(n)
