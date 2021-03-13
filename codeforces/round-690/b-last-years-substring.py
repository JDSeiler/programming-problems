def test(n):
    if n == "2020":
        return "YES"
    elif n.startswith("2020"):
        return "YES"
    elif n.endswith("2020"):
        return "YES"
    elif n.startswith("2") and n.endswith("020"):
        return "YES"
    elif n.startswith("20") and n.endswith("20"):
        return "YES"
    elif n.startswith("202") and n.endswith("0"):
        return "YES"
    else:
        return "NO"

test = "~/media/pictures/wallpapers/I-like-the-moon.jpg"

cases = int(input())
answers = []
for i in range(0, cases):
    _ = input()
    test_case = input()
    answers.append(test(test_case))

for answer in answers:
    print(answer)
