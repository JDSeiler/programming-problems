#include <stdio.h>

int main(void) {
    int kth_place;
    int num_contestants;

    scanf("%d %d", &num_contestants, &kth_place);

    int scores[num_contestants];

    // Learned in this problem that a line of input does NOT correlate with a single
    // call to scanf. Here I provide one long line and iteratively consume it to form
    // the score array. Neat!
    for (int i = 0; i < num_contestants; i++) {
        scanf("%d", &(scores[i]));
    }
    
    int score_to_beat = scores[kth_place-1];

    int num_advancing = 0;
    for (int i = 0; i < num_contestants; i++) {
        if (scores[i] >= score_to_beat && scores[i] >= 1) {
            num_advancing++;
        }

        // Don't do extra work for no reason, sequence is monotonically decreasing so once
        // a single value is less than score_to_beat we can stop iterating.
        if (scores[i] < score_to_beat) {
            break;
        }
    }

    printf("%d\n", num_advancing);
    return 0;
}