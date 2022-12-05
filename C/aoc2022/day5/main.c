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

struct stack
{
    char vals[100];
    char height;
};
typedef struct stack Stack;

void push(Stack *stack, char val)
{
    if (val < 'A' || val > 'Z')
        return;
    stack->vals[stack->height] = val;
    stack->height++;
}

void transfer(Stack *from, Stack *to)
{
    from->height--;
    push(to, from->vals[from->height]);
}

void insert(Stack *stack, char val, char index)
{
    if (val < 'A' || val > 'Z')
        return;
    stack->vals[index] = val;
    if (stack->height <= index)
        stack->height = index + 1;
}

void print_stack(Stack *s)
{
    for (char i = 0; i < s->height; i++)
        printf("[%c]\n", s->vals[i]);
    printf("\n");
}

void read_stacks(FILE *input, Stack stacks[], char stack_count, char level)
{
    for (char i = 0; i < stack_count; i++)
        stacks[i].height = 0;

    char line[100];
    line[0] = 0;
    while (fgets(line, 100, input) != NULL && line[0] != 10)
        if (line[1] != '1')
        {
            for (char i = 0; i < stack_count; i++)
                insert(&stacks[i], line[i * 4 + 1], level);
            level--;
        }
}

void part_one(FILE *input, Stack stacks[], char stack_count)
{
    int count, from, to;
    while (fscanf(input, "move %d from %d to %d\n", &count, &from, &to) == 3)
        for (char i = 0; i < count; i++)
            transfer(&stacks[from - 1], &stacks[to - 1]);

    printf("Part 1: ");
    for (char i = 0; i < stack_count; i++)
        printf("%c", stacks[i].vals[stacks[i].height - 1]);
}

void transfer_together(Stack *from, Stack *to, char count)
{
    for (char i = 0; i < count; i++)
        to->vals[to->height + i] = from->vals[from->height - count + i];
    to->height += count;
    from->height -= count;
}

void part_two(FILE *input, Stack stacks[], char stack_count)
{
    int count, from, to;
    while (fscanf(input, "move %d from %d to %d\n", &count, &from, &to) == 3)
        transfer_together(&stacks[from - 1], &stacks[to - 1], count);

    printf("\n\nPart 2: ");
    for (char i = 0; i < stack_count; i++)
        printf("%c", stacks[i].vals[stacks[i].height - 1]);
    printf("\n");
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");

    char stack_count = 9, level = 7;
    struct stack stacks[stack_count];

    read_stacks(input, stacks, stack_count, level);
    part_one(input, stacks, stack_count);

    rewind(input);

    read_stacks(input, stacks, stack_count, level);
    part_two(input, stacks, stack_count);
}
