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

typedef struct vec Vec;
struct vec { int x; int y; };

int can_step(char from, char to)
{
    if (to == 'E') to = 'z';

    if (to < from || from + 1 == to)
        return 1;

    return 0;
}

void step(Vec *S, int dir)
{}

void part_one(int height, int width, char map[height][width], Vec S, Vec E)
{
    int steps[height][width];
    steps[S.y][S.x] = 0;

    // Consider options
    if (can_step(map[S.y][S.x], map[S.y][S.x + 1])) // Right
    if (can_step(map[S.y][S.x], map[S.y][S.x - 1])) // Left
    if (can_step(map[S.y][S.x], map[S.y + 1][S.x])) // Down
    if (can_step(map[S.y][S.x], map[S.y - 1][S.x])) // Up
            ;
    // Recursively select all the options
    // Mark spots by the amount of steps it took to get to them
    // printf("Part 1: %", );
}

void part_two(int height, int width, char map[height][width])
{
    // printf("Part 2: %", );
}

int main(void)
{
    FILE *input;
    input = read_file("test.txt");

    Vec S, E;
    int width = 8, height = 5;
    char map[height][width + 2];

    for (int i = 0; i < height && fgets(map[i], width + 2, input) != NULL; i++)
        for (int j = 0; j < width; j++)
        {
            if (map[i][j] == 'S')
                S.x = j, S.y = i;
            if (map[i][j] == 'E')
                E.x = j, E.y = i;
        }

    part_one(height, width, map, S, E);

    rewind(input);
    part_two(height, width, map);
}
