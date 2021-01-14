
def generate(x):
    stack = [1,2,3,4,5,6,7,8,9]
    answer = []
    next_x = x
    if x > 45:
        return "-1"
    else:
        while sum(answer) != x:
            if next_x in stack:
                answer.append(next_x)
                stack.remove(next_x)
            else:
                digit = stack.pop()
                answer.append(digit)
                next_x = next_x - digit

        return "".join(sorted([str(c) for c in answer]))

cases = int(input())
answers = []
for i in range(0, cases):
    test_number = int(input())
    answers.append(generate(test_number))

for answer in answers:
    print(answer)
    