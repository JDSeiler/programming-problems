#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
This was surprisingly difficult to do in C. It's an interesting experience to struggle implementing
something you know you could do in a handful of lines of Python code. Props to the kernel module
programmers out there. Anyway, lots of good patterns gleaned while going through this.
*/

void abbrev(char* word) {
    int len = strlen(word);
    if (len <= 10) {
        printf("%s\n", word);
    } else {
        // You need 2 cells, one for the char, 1 for the null terminator. An array enables strcat later.
        char first_char[2];
        // Copy INTO first_char, FROM word, and copy 1 byte.
        memcpy(first_char, word, 1);
        // Terminate! Terminate! Terminate!
        first_char[1] = '\0';

        // Word is the address of the first char, so add the length, then go back 1 to get the last char.
        // abc\0 - len 3, address of a is 10, address of c is 12
        // 10 + 3 = 13 - 1 = 12
        char* last_char_pos;
        last_char_pos = word+len-1;

        // Do the same memcpy procedure
        char last_char[2];
        memcpy(last_char, last_char_pos, 1);
        last_char[1] = '\0';

        int num = len - 2;

        // When the buffer and size are NULL and 0 respectively, this function returns the
        // number of bytes needed to format 'num', by convention I suppose.
        int number_len = snprintf(NULL, 0, "%d", num);

        char number_string[number_len+1];
        snprintf(number_string, number_len+1, "%d", num);

        // Use calloc to init the memory for us, otherwise bad things happen
        char* final_output = calloc(1, number_len+3);

        // Squish it all together with strcat
        strcat(final_output, first_char);
        strcat(final_output, number_string);
        strcat(final_output, last_char);

        printf("%s\n", final_output);
        // Remember to free your memory!
        free(final_output);
    }
}

void main(void) {
    int num_words;
    scanf("%d", &num_words);

    // Note! Arrays of strings are 2D char arrays in C.
    char words[num_words][100];
    for(int i=0; i<num_words; i++) {
        scanf("%s", words[i]);
    }

    for(int i=0; i<num_words; i++) {
        abbrev(words[i]);
    }
}
