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

typedef struct dir Dir;
struct dir
{
    char name[64];
    int size;
    int parrent;
    int subcount;
    int subs[512];
};

Dir mkdir(Dir dirs[], int *lastdir, int pwd, char name[64])
{
    // Insert newly made dir into subs of PWD
    dirs[pwd].subs[dirs[pwd].subcount] = *lastdir;
    dirs[pwd].subcount++;

    Dir dir = {
        .size = 0,
        .parrent = pwd,
    };
    strcpy(dir.name, name);

    *lastdir += 1;
    return dir;
}

int cd(Dir dirs[], int *pwd, char name[64])
{
    if (!strcmp(name, "/"))
    {
        *pwd = 0;
        return 1;
    }

    if (!strcmp(name, ".."))
    {
        *pwd = dirs[*pwd].parrent;
        return 1;
    }

    for (int i = 0; i < dirs[*pwd].subcount; i++)
        if (dirs[dirs[*pwd].subs[i]].name == name)
        {
            *pwd = dirs[*pwd].subs[i];
            return 1;
        }

    return 0;
}

unsigned int du(Dir dirs[], Dir dir)
{
    if (dir.subcount == 0)
        return dir.size;
    
    unsigned int size = dir.size;
    for (int i = 0; i < dir.subcount; i++)
        size += du(dirs, dirs[dir.subs[i]]);

    return size;
}

void part_one(FILE *input, Dir dirs[], int lastdir)
{
    unsigned int sum = 0;
    for (int i = 0; i < lastdir; i++)
    {
        unsigned int size = du(dirs, dirs[i]);
        if (size <= 100000)
            sum += size;
    }
    printf("Part 1: %d\n", sum);
}

void part_two(FILE *input, Dir dirs[], int lastdir)
{
    unsigned int size = 70000000, space_needed = 30000000 - (size - du(dirs, dirs[0]));
    printf("Space Needed: %d\n", space_needed);
    
    for( int i = 0; i < lastdir; i++)
    {
        int dirsize = du(dirs, dirs[i]);
        if (dirsize > space_needed && dirsize < size)
            size = dirsize;
    }
    
    printf("Part 2: %d", size);
}

int main(void)
{
    FILE *input;
    input = read_file("input.txt");

    Dir dirs[512];
    int lastdir = 0, pwd = 0;
    dirs[lastdir] = mkdir(dirs, &lastdir, pwd, "root");

    char line[64];
    while (fgets(line, 64, input) != NULL)
    {
        char dirname[64];
        if (sscanf(line, "$ cd %s", dirname) == 1)
            if (!cd(dirs, &pwd, dirname))
            {
                dirs[lastdir] = mkdir(dirs, &lastdir, pwd, dirname);
                pwd = lastdir - 1;
            }
        
        int size;
        if (sscanf(line, "%d", &size) == 1)
            dirs[pwd].size += size;
    }

    part_one(input, dirs, lastdir);
    part_two(input, dirs, lastdir);
}
