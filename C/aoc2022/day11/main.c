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

typedef struct monkey Monkey;
struct monkey
{
    unsigned long long items[100];
    int items_len;
    unsigned long long activity;
    char op;
    int val;
    int test;
    int if_true;
    int if_false;
};

unsigned long long inspect(Monkey *M, unsigned long long worry)
{
    M->activity += 1;
    unsigned long long val = (M->val == -1) ? worry : M->val;
    unsigned long long result = (M->op == '*') ? worry * val : (worry + val);
    return result;
}

int test(Monkey *M, unsigned long long worry)
{
    return (worry % M->test == 0) ? M->if_true : M->if_false;
}

void throw_to(Monkey M[], int m, unsigned long long worry)
{
    M[m].items[M[m].items_len++] = worry;
}

void part_one(FILE *input)
{
    int m, num_monkeys = 4;
    Monkey Monkeys[num_monkeys];
    while (fscanf(input, "Monkey %d:\n", &m) == 1)
    {
        char op[64];
        Monkey *monkey = &Monkeys[m];

        int items_len = 1;
        int ret = fscanf(input, "Starting items: %lld", &Monkeys[m].items[0]);
        if (fscanf(input, "%[\n]", op) != 1)
            for (; fscanf(input, ", %lld ", &monkey->items[items_len]) == 1; items_len++)
                ;
        monkey->items_len = items_len;

        ret = fscanf(input, "\nOperation: new = old %[^\n]\n", op);
        if (sscanf(op, "%c %d", &Monkeys[m].op, &Monkeys[m].val) == 1)
            Monkeys[m].val = -1; // Which means there's 'old' as the second arg as well
        ret = fscanf(input, "\nTest: divisible by %d\n", &Monkeys[m].test);
        ret = fscanf(input, "\nIf true: throw to monkey %d\n", &Monkeys[m].if_true);
        ret = fscanf(input, "\nIf false: throw to monkey %d\n", &Monkeys[m].if_false);
        Monkeys[m].activity = 0;
    }

    for (unsigned int round = 1; round <= 20; round++)
    {
        printf("\nRound %d\n", round);
        for (m = 0; m < num_monkeys; m++)
        {
            printf("Monkey %d:", m);
            for (int i = 0; i < Monkeys[m].items_len; i++)
            {
                // int worry = inspect(&Monkey[m], Monkey[m].items[i]) / 3;
                unsigned long long worry = inspect(&Monkeys[m], Monkeys[m].items[i]);
                int monkey = test(&Monkeys[m], worry);
                throw_to(Monkeys, monkey, worry);
                printf(" (%llu/%d)-%d", worry, Monkeys[m].test, monkey);
            }
            printf("\n");
            Monkeys[m].items_len = 0;
        }
    }
    unsigned long long first = 0, second = 0;
    for (m = 0; m < num_monkeys; m++)
    {
        if (Monkeys[m].activity > second)
        {
            if (Monkeys[m].activity > first)
                second = first, first = Monkeys[m].activity;
            else
                second = Monkeys[m].activity;
        }
    }

    for (m = 0; m < num_monkeys; m++)
        printf("Monkey %d: %llu\n", m, Monkeys[m].activity);

    printf("Part 1: %llu\n", first * second);
}

void part_two(FILE *input)
{
    // printf("Part 2: %d", first * second);
}

int main(void)
{
    FILE *input;
    input = read_file("test.txt");
    part_one(input);

    rewind(input);
    part_two(input);
}
