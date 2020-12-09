days = input()
model = input()
arr = model.split(" ")
arr = [int(value) for value in arr]
arr = list(enumerate(arr))
arr.sort(key=lambda tup: tup[1])
print(arr[0][0])
