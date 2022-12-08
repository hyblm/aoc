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

int check(int cols, char grid[cols][cols], int x, int y)
{
    char tree = grid[y][x];
    int visible = 4;
    for (int i = y - 1; i >= 0; i--)
        if (grid[i][x] >= tree)
        {
            visible--;
            break;
        }
    for (int i = y + 1; i < cols; i++)
        if (grid[i][x] >= tree)
        {
            visible--;
            break;
        }
    for (int i = x - 1; i >= 0; i--)
        if (grid[y][i] >= tree)
        {
            visible--;
            break;
        }
    for (int i = x + 1; i < cols; i++)
        if (grid[y][i] >= tree)
        {
            visible--;
            break;
        }

    return (visible) ? 1 : 0; // If visible from any direction (visible > 0) return 1
}

void part_one(int cols, char grid[cols][cols])
{
    unsigned int visible = 4 * cols - 4;
    for (int y = 1; y < cols - 1; y++)
        for (int x = 1; x < cols - 1; x++)
            visible += check(cols, grid, x, y);

    printf("\nPart 1: %d", visible);
}

int scenic_score(int cols, char grid[cols][cols], int x, int y)
{
    char tree = grid[y][x];
    int score = 1, i;

    for (i = y - 2; i >= 0; i--) // UP
        if (grid[i + 1][x] >= tree)
            break;
    score *= y - i - 1;

    for (i = x - 2; i >= 0; i--) // LEFT
        if (grid[y][i + 1] >= tree)
            break;
    score *= x - i - 1;

    for (i = y + 2; i < cols; i++) // DOWN
        if (grid[i - 1][x] >= tree)
            break;
    score *= i - y - 1;

    for (i = x + 2; i < cols; i++) // RIGHT
        if (grid[y][i - 1] >= tree)
            break;
    score *= i - x - 1;

    return score;
}

void part_two(int cols, char grid[cols][cols])
{
    unsigned int best_score = 0; // view distance
    for (int y = 1; y < cols - 1; y++)
    {
        for (int x = 1; x < cols - 1; x++)
        {
            int tree_score = scenic_score(cols, grid, x, y);
            if (tree_score > best_score)
                best_score = tree_score;
        }
    }

    printf("\nPart 2: %d", best_score);
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");

    int cols = 99, i = 0;
    char grid[cols][cols];
    char line[128];
    while (fgets(line, 128, input) != NULL)
    {
        strcpy(grid[i], line);
        i++;
    }

    part_one(cols, grid);
    part_two(cols, grid);
}