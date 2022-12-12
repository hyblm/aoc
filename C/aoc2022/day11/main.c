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
struct monkey {
    unsigned long long items[100]; int n;
    unsigned long long activity;
    char op; int val;
    unsigned long long test;
    int if_true; int if_false;
};

unsigned long long inspect (Monkey *M, unsigned long long worry)
{
    M->activity += 1;
    unsigned long long val = (M->val == -1) ? worry : M->val;
    unsigned long long result = (M->op == '*') ? worry * val : (worry + val);
    return result;
}

int test (Monkey *M, unsigned long long worry)
{
    return (worry % M->test == 0) ? M->if_true : M->if_false;
}

void throw_to(Monkey M[], int m, unsigned long long worry)
{
    M[m].items[M[m].n++] = worry;
}

void part_one(FILE *input)
{
    int m, monkeys = 4;
    Monkey Monkey[monkeys];
    while (fscanf(input, "Monkey %d:\n", &m) == 1)
    {
        char op[64];
        int n = 1;
        int ret = fscanf(input, "Starting items: %lld", &Monkey[m].items[0]);
        if (fscanf(input, "%[\n]", op) != 1)
          for (; fscanf(input, ", %lld ", &Monkey[m].items[n]) == 1; n++)
            ;

        Monkey[m].n = n;
        ret = fscanf(input, "\nOperation: new = old %[^\n]\n", op);
        if (sscanf(op, "%c %d", &Monkey[m].op, &Monkey[m].val) == 1)
            Monkey[m].val = -1;
        ret = fscanf(input, "\nTest: divisible by %lld\n", &Monkey[m].test);
        ret = fscanf(input, "\nIf true: throw to monkey %d\n", &Monkey[m].if_true);
        ret = fscanf(input, "\nIf false: throw to monkey %d\n", &Monkey[m].if_false);
        Monkey[m].activity = 0;
    }

    for (unsigned int round = 1; round <= 20; round++)
    {
        printf("\nRound %d\n", round);
        for (m = 0; m < monkeys; m++)
        {
            printf("Monkey %d:", m);
            for (int i = 0; i < Monkey[m].n; i++)
            {
                // int worry = inspect(&Monkey[m], Monkey[m].items[i]) / 3;
                unsigned long long worry = inspect(&Monkey[m], Monkey[m].items[i]);
                int monkey = test(&Monkey[m], worry);
                throw_to(Monkey, monkey, worry);
                printf(" (%llu/%llu)-%d", worry, Monkey[m].test, monkey);
            }
            printf("\n");
            Monkey[m].n = 0;
        }
    }
    unsigned long long first = 0, second = 0;
    for (m = 0; m < monkeys; m++)
    {
        if (Monkey[m].activity > second)
        {
            if (Monkey[m].activity > first)
                second = first, first = Monkey[m].activity;
            else
                second = Monkey[m].activity;
        }
    }

    for (m = 0; m < monkeys; m++)
        printf("Monkey %d: %llu\n", m, Monkey[m].activity);

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