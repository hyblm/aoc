#include <stdio.h>
#include <stdlib.h>
#include <string.h>

FILE* read_file(char* filename) {
  FILE* input = fopen(filename, "r");
  if (!input) {
    printf("Error reading input file\n");
    exit(-1);
  }
  return input;
}

void part_one(FILE* input) {
  char one[100], two[100];
  while (fscanf(input, "%[^\n]\n %[^\n]\n", one, two) == 2) {
    printf("%s\n%s\n", one, two);
  }

  // printf("Part 1: %", );
}

void part_two(FILE* input) {
  // printf("Part 2: %", );
}

int main(void) {
  FILE* input;
  input = read_file("test.txt");
  part_one(input);

  rewind(input);
  part_two(input);
}
