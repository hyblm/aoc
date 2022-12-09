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

typedef struct pos Pos;
struct pos
{
    int x;
    int y;
};

void move(Pos *knot, Pos dir)
{
    knot->x += dir.x;
    knot->y += dir.y;
}

void update_next(Pos H, Pos *T)
{
    Pos offset = {
        .x = H.x - T->x,
        .y = H.y - T->y
    };

    // Diagonal Movement
    int xlen = abs(offset.x), ylen = abs(offset.y);
    if (xlen + ylen > 2)
    {
        if (xlen == 1) offset.x *= 2;
        if (ylen == 1) offset.y *= 2;
    }

    if (offset.x < 0) offset.x++;
    if (offset.x > 0) offset.x--;
    if (offset.y < 0) offset.y++;
    if (offset.y > 0) offset.y--;
    
    move(T, offset);
}

void check_visited(Pos T, Pos locs[], unsigned int *visited)
{
    for (int i = 0; i < *visited; i++)
        if (T.x == locs[i].x && T.y == locs[i].y)
            return;
    
    locs[*visited] = T;
    *visited += 1;
}

unsigned int move_rope(FILE* input, int len)
{
    Pos locations[10000], rope[len];
    unsigned int visited = 0;
    for (int i = 0; i < len; i++)
        rope[i].x = 0, rope[i].y = 0;
    
    char direction, distance, line[16];
    while (fscanf(input, "%c %hhd\n", &direction, &distance) == 2)
    {
        Pos dir = {0,0};
        switch (direction)
        {
        case 'U': dir.y -= 1; break; case 'L': dir.x -= 1; break;
        case 'R': dir.x += 1; break; case 'D': dir.y += 1; break;
        }

        while (distance > 0)
        {
            move(&rope[0], dir);
            for (int i = 1; i < len; i++)
                update_next(rope[i - 1], &rope[i]);
            check_visited(rope[len-1], locations, &visited);
            distance--;
        }
    }
    
    return visited;
}

void part_one(FILE *input)
{
    unsigned int visited = move_rope(input, 2);
    printf("Part 1: %d\n", visited);
}

void part_two(FILE *input)
{
    unsigned int visited = move_rope(input, 10);
    printf("Part 2: %d\n", visited);
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");

    part_one(input);

    rewind(input);
    part_two(input);
}