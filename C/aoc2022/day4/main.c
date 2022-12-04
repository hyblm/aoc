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

void part_one(FILE *input) // findnig where one of the ranges a-b x-y is contained by the other
{
    unsigned int sum = 0, a, b, x, y;
    while (fscanf(input, "%d-%d,%d-%d", &a, &b, &x, &y) == 4)
    {
        if ((a <= x && b >= y) || (x <= a && y >= b))
            sum++;
    }
    printf("\nPart 1: %d\n", sum);
}

void part_two(FILE *input) // Finding overlaps in ranges a-b x-y
{
    unsigned int sum = 0, a, b, x, y;
    while (fscanf(input, "%d-%d,%d-%d", &a, &b, &x, &y) == 4)
    {
        if ((b >= x && y > a) || (y >= a && b > x))
            sum++;
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
