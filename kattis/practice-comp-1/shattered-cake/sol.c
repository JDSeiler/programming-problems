#include <stdio.h>

int sum(int* nums, int len) {
    int acc = 0;
    for(int i=0; i<len; i++) {
        acc+=nums[i];
    }
    return acc;
}

int main(void) {
    int w;
    scanf("%d", &w);

    int num_bits;
    scanf("%d", &num_bits);

    int areas[num_bits];
    for(int i=0; i<num_bits; i++) {
        int c_w;
        int c_l;
        scanf("%d %d", &c_w, &c_l);
        areas[i] = c_w * c_l;
    }

    int total_area = sum(areas, num_bits);
    int ans = total_area / w;
    printf("%d\n", ans);

    return 0;
}