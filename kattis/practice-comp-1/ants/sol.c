#include <stdio.h>
#include <math.h>

int min(int cm, int* distances, int ants) {
    int min = 0;
    for(int i=0; i<ants; i++) {
        int curr_dist = distances[i];
        int curr_min = fminl(cm-curr_dist, curr_dist);
        if (curr_min > min) {
            min = curr_min;
        }
    }
    return min;
}

int max(int cm, int* distances, int ants) {
    int max = 0;
    for(int i=0; i<ants; i++) {
        int curr_dist = distances[i];
        int curr_max = fmaxl(cm-curr_dist, curr_dist);
        if (curr_max > max) {
            max = curr_max;
        }
    }
    return max;
}

int main(void) {
    int cases;
    scanf("%d", &cases);
    for (int i=0; i<cases; i++) {
        int cm;
        int ants;
        scanf("%d %d", &cm, &ants);
        int distances[ants];
        for (int j=0; j<ants; j++) {
            scanf("%d", &distances[j]);
        }
        int min_result = min(cm, distances, ants);
        int max_result = max(cm, distances, ants);

        printf("%d %d\n", min_result, max_result);
    }

    return 0;
}