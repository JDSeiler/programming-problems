t = int(input())
import math

def solve_case():
    n = int(input())
    num = input()

    top = []
    tValSoFar = 0

    bot = []
    bValSoFar = 0

    for idx, digit in enumerate(num):
        place = n - idx - 1;

        if digit == '0':
            top.append('0')
            bot.append('0')
        elif digit == '1':
            choice, newVal = compute_best_for_one(tValSoFar, bValSoFar, place)
            if choice == 'top':
                top.append('1')
                bot.append('0')
                tValSoFar = newVal
            else:
                bot.append('1')
                top.append('0')
                bValSoFar = newVal
        else:
            result = compute_best_for_two(tValSoFar, bValSoFar, place)
            choice = result[0]
            rest = result[1]
            
            if choice == 'top':
                top.append('2')
                bot.append('0')
                tValSoFar = rest
            elif choice == 'bot':
                bot.append('2')
                top.append('0')
                bValSoFar = rest
            else:
                top.append('1')
                bot.append('1')

                # Uhg
                tValSoFar = tValSoFar + rest
                bValSoFar = bValSoFar + rest

    print(''.join(top))
    print(''.join(bot))

def compute_best_for_one(top, bot, placeVal):
    toAdd = 3 ** placeVal
    topScore = top + toAdd
    botScore = bot + toAdd

    if (topScore < botScore):
        return ('top', topScore)
    elif (botScore < topScore):
        return ('bot', botScore)
    else:
        return ('top', topScore)

def compute_best_for_two(top, bot, placeVal):
    # place value could be VERY LARGE
    # arbitrary precision integers are too slow
    ifSplit = int(math.pow(3, placeVal))
    ifChoose = ifSplit * 2

    chooseTop = top + ifChoose
    chooseBot = bot + ifChoose

    topIfSplit = top + ifSplit
    botIfSplit = bot + ifSplit
    split = max(topIfSplit, botIfSplit)

    choices = [
        ('top', chooseTop),
        ('bot', chooseBot),
        ('split', split)
    ]

    bestKey, bestScore = min(choices, key=lambda t: t[1])
    if bestKey == 'split':
        if (split == chooseTop):
            return ('top', chooseTop)
        elif (split == chooseBot):
            return ('bot', chooseBot)
        else:
            return ('split', ifSplit)
    else:
        return (bestKey, bestScore) 


for i in range(0, t):
    solve_case()