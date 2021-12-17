s, x, y = list(map(int, input().split()))

# We want a, b such that s = ax + by. Where (a+b) is minimized

# Rearrange to get b = (s-ax) / y
# Plug in possible values for a, store all valid solutions, then pull out the
# 'right' one later.

solutions = []

for a in range(1, s):
    b = (s - (a*x)) / y
    
    # b has to be a non-negative integer for a,b to be a valid solution.
    if int(b) == b and b >= 0:
        solutions.append((a, int(b), a + int(b)))
    else:
        continue

if len(solutions) == 0:
    print("Impossible")
else:
    a, b, _ = sorted(solutions, key=lambda sol: sol[2])[0]
    print(f"{a} {b}")
