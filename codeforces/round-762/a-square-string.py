t = int(input())

def solve_case():
    string = input()
    if len(string) % 2 != 0:
        return 'NO'
    
    first = string[0:len(string)//2]
    second = string[len(string)//2:]
    if first == second:
        return 'YES'
    else:
        return 'NO'

for i in range(t):
    print(solve_case())
