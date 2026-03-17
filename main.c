#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int rgb(int r, int g, int b, char *text) {
    printf("\033[48;2;%i;%i;%im%s\033[0m\n", r, g, b, text);
    return 0;
}

int main(int argc, char* argv[]) {
    char *type = (argc > 1) ? argv[1] : "normal";

    if (argc < 2) {
        printf("Usage %s:\n", argv[0]);
        printf("  program <name>\n");
        printf("  program rgb R G B TEXT\n");
        return 1;
    }

    if (strcmp(type, "rgb") == 0) {
        if (argc < 6) {
            printf("Usage: program rgb R G B TEXT\n");
            return 1;
        }
        int r=atoi(argv[2]);
        int g=atoi(argv[3]);
        int b=atoi(argv[4]);
        char text[1024] = "";
        for (int i = 5; i < argc; i++) {
            strcat(text, argv[i]);
            if (i < argc - 1) {
                strcat(text, " ");
            }
        }
        rgb(r,g,b,text);
        return 0;
    }

    // normal greeting
    printf("Hello %s\n", argv[1]);

    return 0;
}
