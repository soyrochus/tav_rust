#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

// Function to uppercase the string and print it out
void shout(char *str) {
    // if (str == NULL) {
    //     printf("NULL string\n");
    //     return;
    // }
    
    char *upper_str = strdup(str); // Duplicate the string
    // if (upper_str == NULL) {
    //     fprintf(stderr, "Memory allocation failed\n");
    //     return;
    // }

    for (int i = 0; upper_str[i]; i++) {
        upper_str[i] = toupper((unsigned char)upper_str[i]);
    }

    printf("%s\n", upper_str);
    //free(upper_str); // Free the duplicated string
    //Not freeing the duplicated string will cause a memory leak
}

int main() {
    // Allocate a string
    char *str = (char*) malloc(20 * sizeof(char));
    if (str == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    strcpy(str, "Hello, World!");

    // Call shout with the string
    shout(str);

    // Write a NULL to the string address
    // free(str);
    str = NULL;  //BUG: This is a memory leak

    // Call shout again with the NULL string
    shout(str);

    return 0;
}
