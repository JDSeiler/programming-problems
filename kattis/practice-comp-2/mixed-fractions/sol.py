while True:
    case = input()
    if case == "0 0":
        break
    num, dem = [int(num) for num in case.split(" ")]
    if num < dem:
        print("0 {} / {}".format(num, dem))
    else:
        whole_part = num // dem
        rem = num % dem
        print("{} {} / {}".format(whole_part, rem, dem))
