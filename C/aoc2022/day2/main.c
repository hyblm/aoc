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
    else
    {
        return input;
    }
}

void part_one(FILE *input)
{
    char line[5];
    int score = 0;
    while (fgets(line, 5, input) != NULL)
    {
        line[0] = 68 - line[0];            // opponents play (R-2, P-1, S-0)
        line[2] -= 87;                     // my play (R-1, P-2, S-3)
        line[1] = (line[0] + line[2]) % 3; // Outcome (L-0, D-1, W-2)
        score += line[1] * 3 + line[2];
    }
    printf("\nPart 1: %d\n", score);
}

void part_two(FILE *input)
{
    char line[5];
    int score = 0;
    while (fgets(line, 5, input) != NULL)
    {
        line[0] -= 65;                             // opponents play (0, 1, 2)
        line[2] -= 88;                             // Result (0, 1, 2)
        line[1] = (line[0] + line[2] + 2) % 3 + 1; // shape I should play (1, 2, 3)
        score += 3 * line[2] + line[1];
    }
    printf("\nPart 2: %d\n", score);
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");
    part_one(input);
    rewind(input);
    part_two(input);
}
