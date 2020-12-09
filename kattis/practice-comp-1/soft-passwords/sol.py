s = input()
p = input()

def digit_mod(s, p):
    if s[0].isnumeric() and s[1::] == p:
        return True
    if s[-1].isnumeric() and s[0:-1] == p:
        return True

    return False

is_accepted = s == p or digit_mod(s,p) or (s == p.swapcase())

if is_accepted:
    print("Yes")
else:
    print("No")
