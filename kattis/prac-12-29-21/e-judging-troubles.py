n = int(input())

def count_submission_types(number_submissions):
    result_dict = {}

    for _ in range(number_submissions):
        submission = input()
        if submission in result_dict.keys():
            result_dict[submission] += 1
        else:
            result_dict[submission] = 1

    return result_dict

domjudge = count_submission_types(n)
kattis = count_submission_types(n)

d_keys = set(domjudge.keys())
k_keys = set(kattis.keys()) 

shared = d_keys.intersection(k_keys)

consistent = 0
for s in shared:
    consistent += min(domjudge[s], kattis[s])

print(consistent)
