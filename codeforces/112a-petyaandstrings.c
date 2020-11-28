#include <stdio.h>
#include <strings.h>

void main(void) {
    char string1[100];
    char string2[100];

    scanf("%s", string1);
    scanf("%s", string2);

    int result = strcasecmp(string1, string2);
    if (result > 0) {
        result = 1;
    } else if (result < 0) {
        result = -1;
    } else {
        // Only option left is result == 0, which is fine as is.
    }
    printf("%d\n", result);
}