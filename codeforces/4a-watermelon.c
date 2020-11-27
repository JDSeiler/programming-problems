#include <stdio.h>

/*
For a watermelon to be divisible such that both pieces of the watermelon weigh an even amount, the
entire watermelon must weigh an even amount. This can be shown in a proof by contradiction:

Let the watermelon weigh W kilos, W is odd, so W = 2K+1. W can be divided into two pieces such
that both pieces weight an even amount. Thus W = 2A + 2B or W = 2(A+B).

W = 2(A+B) means W is even, but W is odd as a premise, this is a contradicition so W cannot be odd.
*/

void main(void) {
    int weight;
    scanf("%d", &weight);

    if (weight % 2 == 0) {
        printf("YES\n");
    } else {
        printf("NO\n");
    }
}