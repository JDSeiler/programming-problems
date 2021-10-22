params = input().split() # ["1", "22", "333"]
k = int(params[0])       # price per nanner
n = int(params[1])       # dollars in pocket
w = int(params[2])       # num nanners desired

def calculatePriceForNanners(numNammers, basePrice):
    total = 0
    for bananaNumber in range(1, numNammers+1):
        priceForThisBanana = basePrice*bananaNumber
        total = total + priceForThisBanana

    return total

nannerCost = calculatePriceForNanners(w, k)
debt = nannerCost - n

if (debt <= 0):
    print(0)
else:
    print(debt)
