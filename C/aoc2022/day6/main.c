#include <stdio.h>
#include <stdlib.h>
#include <string.h>

FILE *read_file(char *filename)
{
    FILE *input = fopen(filename, "r");
    if (!input)
    {
        printf("Error reading input file\n");
        exit(-1);
    }
    return input;
}

char marker(char line[4])
{
    for (char i = 3; i > 0; i--)
        for (char j = i - 1; j >= 0; j--)
            if (line[i] == line[j])
                return 0;
    return 1;
}

void part_one(FILE *input)
{
    char line[10000];
    if (!fgets(line, 10000, input)) return;
    for (unsigned int i = 3; line[i] != EOF; i++)
        if (marker(line + i - 3))
        {
            printf("Part 1: %d", i + 1);
            return;
        }
}

char message(char line[14])
{
    for (char i = 13; i > 0; i--)
        for (char j = i - 1; j >= 0; j--)
            if (line[i] == line[j])
                return 0;
    return 1;
}

void part_two(FILE *input)
{
    char line[10000];
    if (!fgets(line, 10000, input)) return;
    for (unsigned int i = 13; line[i] != EOF; i++)
        if (message(line + i - 13))
        {
            printf("\nPart 2: %d", i + 1);
            return;
        }
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");
    part_one(input);

    rewind(input);
    part_two(input);
}