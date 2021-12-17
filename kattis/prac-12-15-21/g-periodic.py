s = input()

def right_rotate(s):
    return s[-1] + s[0:-1]

def windows(s, k):
    windows = []
    start = 0
    stop = k

    times = len(s) // k
    for _i in range(0, times):
        slyce = s[start:stop]
        start = stop
        stop += k
        windows.append(slyce)

    return windows

def is_periodic(windows):
    if len(windows) == 1:
        return True
    else:
        left = windows[0]

        for win in windows[1::]:
            if right_rotate(left) == win:
                left = win
            else:
                return False

        return True
            
for k in range(1, len(s)+1):
    if len(s) % k == 0:
        wins = windows(s, k)
        if is_periodic(wins):
            print(k)
            break
    else:
        continue
