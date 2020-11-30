#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
This problem was painful to implement because I had some undefined behavior that took me a
while to track down. That said I learned a lot wrestling through it!

1. strtol sets the value of the tail pointer! That's why tail doesn't have a default value.
2. ZERO OUT YOUR MEMORY.. PLEASE..

The buckets array gave me no end of trouble. Before I wrote the "create_equation" function
everything was fine, but then as soon as I added it to the code the contents of buckets
started being garbage. For whatever reason, I had gotten lucky earlier and the memory at 'buckets'
/happened/ to be 0 by sheer luck. But once I added other code the memory layout changed
and garbage started getting put in there. Once I set buckets to 0 by default
the problem went away.

Moral of the story: Zero your memory

*/

void count(char *string, int len, int buckets[]) {
    char *tail;

    for (int i=0; i<len; i++) {
        int next = strtol(string, &tail, 0);
        if (strlen(string) == 0) {
            break;
        }
        else {
            buckets[next-1] = buckets[next-1] + 1;
            string = tail;
        }
    }
}

void create_equation(int buckets[], char output[]) {
    for (int i=0; i<3; i++) {
        while(buckets[i] > 0) {
            char term[3]; 
            sprintf(term, "%d+", i+1);
            strcat(output, term);
            buckets[i]--;
        }
    }
    int len = strlen(output);
    output[len-1] = '\0';
}

int main(void) {
    char output[100] = {'\0'};
    char input_equation[100];
    int buckets[3] = {0,0,0};

    scanf("%s", input_equation);
    int len = strlen(input_equation);
    count(input_equation, len, buckets);

    create_equation(buckets, output);
    printf("%s\n", output);
    return 0;
}  
