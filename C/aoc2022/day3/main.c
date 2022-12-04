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

char find_match(char *line1, char len1, char *line2, char len2)
{
    for (char i = 0; i < len1; i++)
        for (char j = 0; j < len2; j++)
            if (line1[i] == line2[j])
                return i;
}

char priority(char letter)
{
    return (letter > 90) ? letter - 96 : letter - 38;
}

void part_one(FILE *input)
{
    char line[100];
    unsigned int sum = 0;
    while (fgets(line, 100, input) != NULL)
    {
        char len = 0;
        for (; line[len] > 60; len++)
            ;
        char m = find_match(line, len / 2, line + len / 2, len / 2);
        sum += priority(line[m]);
    }
    printf("\nPart 1: %d\n", sum);
}

char in_three(char elf_1[], char elf_2[], char elf_3[], char len_1, char len_2, char len_3)
{
    for (char i = 0; i < len_1; i++)
        for (char j = 0; j < len_2; j++)
            if (elf_1[i] == elf_2[j])
                for (char k = 0; k < len_3; k++)
                    if (elf_3[k] == elf_1[i])
                        return i;
}

void part_two(FILE *input)
{
    char elf_1[100];
    char elf_2[100];
    char elf_3[100];
    unsigned int sum = 0;
    while (fgets(elf_1, 100, input) != NULL &&
           fgets(elf_2, 100, input) != NULL &&
           fgets(elf_3, 100, input) != NULL)
    {
        char len_1 = 0, len_2 = 0, len_3 = 0;
        for (; elf_1[len_1] > 60; len_1++);
        for (; elf_2[len_2] > 60; len_2++);
        for (; elf_3[len_3] > 60; len_3++);

        char m = in_three(elf_1, elf_2, elf_3, len_1, len_2, len_3);
        sum += priority(elf_1[m]);
    }
    printf("\nPart 2: %d\n", sum);
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");
    part_one(input);
    rewind(input);
    part_two(input);
}
