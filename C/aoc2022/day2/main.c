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

void part_one(FILE *input)
{
    char op, me, outcome;
    int score = 0;
    while (fscanf(input, "%c %c\n", &op, &me) == 2)
    {
        op = 'D' - op;           // opponents play (R-2, P-1, S-0)
        me -= 'W';               //        my play (R-1, P-2, S-3)
        outcome = (op + me) % 3; //        Outcome (L-0, D-1, W-2)
        score += outcome * 3 + me;
    }
    printf("\nPart 1: %d\n", score);
}

void part_two(FILE *input)
{
    char op, me, result;
    int score = 0;
    while (fscanf(input, "%c %c\n", &op, &result) == 2)
    {
        op -= 'A';                      // opponents play (0, 1, 2) A is start of abc
        result -= 'X';                  // Result         (0, 1, 2) X is start of xyz
        me = (op + result + 2) % 3 + 1; // shape I should play (1, 2, 3)
        score += 3 * result + me;
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
