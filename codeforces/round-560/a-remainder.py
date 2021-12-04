len_n, x, y = list(map(int, input().split()))
n = input()

def left_pad_to_size(size, arr):
    amount_to_add = max(size - len(arr), 0)
    # Add to the end by reversing the start sequence, adding the padding
    # then reversing the result.
    rev = list(reversed(arr))
    return list(reversed(rev + ['0']*amount_to_add))

def placewise_diff(n, m):
    count = 0
    for (x, y) in zip(n, m):
        if x != y:
            count += 1
    return count  


# 11 5 2
# 11010100101
# x means take the first x digits from the right
rem = n[-x:] # 00101

# Create the target remainder
target = ['1'] + ['0']*y

# Make sure both strings are the same length
largest = max(x, y)
n_rem, n_target = (left_pad_to_size(largest, rem), left_pad_to_size(largest, target))

# Basically, do a bitwise xor / count the differences between the two strings.
differences = placewise_diff(n_rem, n_target)
print(differences)
