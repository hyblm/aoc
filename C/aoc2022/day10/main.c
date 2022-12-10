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

void next_cycle(unsigned int* cycle, unsigned int* sum, int REGX)
{
    *cycle += 1;
    if ((*cycle - 20) % 40 == 0)
        *sum += *cycle * REGX;

}

void draw(unsigned int* cycle, unsigned int* sum, int REGX)
{
    // printf("Start cycle %d\n", *cycle + 1);
    if ( *cycle && !(*cycle % 40))
        printf("\n");

    *cycle += 1;

    // printf("Sprite position: ");
    // for (int i = 0; i < 40; i++)
    // {
    //     if (i - REGX < 2 && i - REGX > -2)
    //         printf("#");
    //     else
    //         printf(".");
    // }
    // printf("\n");

    int sprite = *cycle % 40 - 1 - REGX;
    (sprite < 2 && sprite > -2) ? printf("#") : printf(".");
}

void part_one(FILE *input)
{
    char line[16];
    unsigned int cycle = 0, REGX = 1, val, sum = 0;
    while(fgets(line, 16, input) != NULL)
    {
        // printf("%s", line);
        if (sscanf(line, "addx %d", &val) == 1)
        {
            next_cycle(&cycle, &sum, REGX);
            next_cycle(&cycle, &sum, REGX);
            REGX += val;
        }
        else
            next_cycle(&cycle, &sum, REGX);
    }

    printf("Part 1: %d\n\n", sum);
}

void part_two(FILE *input)
{
    char line[16];
    unsigned int cycle = 0, REGX = 1, val, sum = 0;
    while(fgets(line, 16, input) != NULL)
    {
        if (sscanf(line, "addx %d", &val) == 1)
        {
            draw(&cycle, &sum, REGX);
            draw(&cycle, &sum, REGX);
            REGX += val;
        }
        else
            draw(&cycle, &sum, REGX);
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