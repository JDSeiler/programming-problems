#include <stdio.h>
#include <stdlib.h>

void main(void) {
    int matrix[5][5];

    for (int i=0; i<5; i++) {
        for (int j=0; j<5; j++) {
            scanf("%d", &matrix[i][j]);
        }
    }

    int cell = 0;
    int x;
    int y;
    for (int i=0; i<5; i++) {
        for (int j=0; j<5; j++) {
            cell = matrix[i][j];
            if (cell == 1) {
                x = j;
                y = i;
                break;
            }
        }
        if (cell == 1) {
            break;
        }
    }

    // Adjust for 0 indexing
    int x_diff = abs(3-(x+1));
    int y_diff = abs(3-(y+1));

    printf("%d\n", x_diff+y_diff);
}