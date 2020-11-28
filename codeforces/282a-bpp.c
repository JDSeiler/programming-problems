#include <stdio.h>
#include <string.h>

int sum(int arr[], int size) {
    int acc = 0;
    for (int i=0; i<size; i++) {
        acc += arr[i];
    }
    return acc;
}

void main(void) {
    int num_instructions;
    scanf("%d", &num_instructions);

    int instructions[num_instructions];

    for (int i=0; i<num_instructions; i++) {
        char instruction[3];
        scanf("%s", instruction);
        if (strstr(instruction, "+")) {
            instructions[i] = 1;
        } else {
            instructions[i] = -1;
        }
    }

    int result = sum(instructions, num_instructions);
    printf("%d\n", result);
}