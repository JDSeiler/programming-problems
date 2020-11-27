#include <stdio.h>
#include <string.h>

/*
I've written very little C code so there is probably a better way to do this.
Would be fun to come back to this after I learn more C and see if I can solve
this better. This solution will be over documented to help me solidify concepts.
*/

void main(void) {
    char hand[15];
    char ranks[6];
    // Bracket looks like a regex, but isn't. It's a shorthand format specifier for a set of characters.
    // This set means "something that isn't \n", very similar to a PCRE.
    scanf("%[^\n]", hand);

    // The suit of the card is useless information, so I pull out every third character (the ranks)
    // into another char array (recall char arrays and strings are the same thing)
    int j = 0;
    for (int i=0; i<15; i+=3) {
        ranks[j] = hand[i];
        j+=1;
    }
    // Add the null terminator to stop the string from running into other memory!!!
    // Surely there is a better way to do this.
    ranks[5] = '\0';

    // The expected characters in the ranks cover a small range of ASCII characters,
    // so basically just perform the first half of a bucket/bin sort. There is a single 
    // bucket for every ascii character between 2 (ascii d50) and Q (ascii d81), the bucket
    // index is derived by casting the character to an int and moving it inside the bounds
    // of the array.
    int buckets[32] = {0};
    for (int i=0; i<5; i++) {
        int index = ((int)ranks[i]) - 50;
        buckets[index]++;
    }
    // Find the biggest value in the array. The {0} bit on the buckets array decl. is important 
    // because C does NOT init memory for you so it will contain random garbage otherwise.
    int best_score = 0;
    for (int i=0; i<32; i++) {  
        if (buckets[i] > best_score) {
            best_score = buckets[i];
        }
    }

    printf("%d\n", best_score);
}