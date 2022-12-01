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

void part_one(char line[], FILE *input)
{

    int current = 0;
    int biggest = 0;
    while (fgets(line, 2237, input) != NULL)
    {
        if (strcmp(line, "\n"))
        {
            current += atoi(line);
        }
        else
        {
            for (char i = 0; i < 3; i++)
            {
                if (current > biggest)
                    biggest = current;
            }
            current = 0;
        }
    }
    printf("%d\n", biggest);
}

void part_two(char line[], FILE *input)
{
    int current = 0;
    int elves[3] = {0, 0, 0};
    while (fgets(line, 2237, input) != NULL)
    {
        if (strcmp(line, "\n"))
        {
            current += atoi(line);
        }
        else
        {
            for (char i = 0; i < 3; i++)
            {
                if (current > elves[i])
                {
                    // Make sure we're replacing the smallest number from most
                    for (int j = i; j < 3; j++)
                        if (elves[j] < elves[i])
                            i = j;

                    elves[i] = current;
                    break;
                }
            }
            current = 0;
        }
    }

    for (char i = 0; i < 3; i++)
        if (current > elves[i])
        {
            elves[i] = current;
            break;
        }

    printf("%d\n", elves[0] + elves[1] + elves[2]);
}

int main(void)
{
    char line[2237];

    FILE *input;
    input = read_file("input.txt");
    part_one(line, input);
    input = read_file("input.txt");
    part_two(line, input);
}
