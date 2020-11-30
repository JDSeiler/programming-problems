#include <stdio.h>

// Today I Learned: Getting the size of an array is... difficult.. in C
int sum(int arr[], int size) {
    int acc = 0;
    for (int i=0; i<size; i++) {
        acc += arr[i];
    }
    return acc;
}

int main(void) {
    int num_problems;
    scanf("%d", &num_problems);

    int problems[num_problems][3];

    for (int i=0; i<num_problems; i++) {
        scanf("%d %d %d", &(problems[i][0]), &(problems[i][1]), &(problems[i][2]));
    }

    int will_solve = 0;
    for (int i=0; i<num_problems; i++) {
        if (sum(problems[i], 3) >= 2) {
            will_solve++;
        }
    }
    printf("%d\n", will_solve);
    return 0;
}
