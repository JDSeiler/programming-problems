# Return all of the bids that occur exactly once
def strict_unique(bids):
    candidates = set()
    seen = set()
    for bid in bids:
        if bid not in candidates and bid not in seen:
            candidates.add(bid)
        elif bid in candidates and bid not in seen:
            candidates.remove(bid)
            seen.add(bid)
        else:
            pass
    return list(candidates)
        
def solve_case():
    _size = input();
    bids = list(map(int, input().split()))
    candidates = strict_unique(bids)
    if len(candidates) == 0:
        print("-1")
    else:
        print(bids.index(min(candidates)) + 1)

t = int(input())
for i in range(0, t):
    solve_case()
