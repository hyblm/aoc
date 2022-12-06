{ pkgs ? import <nixpkgs> { } }:
let
  GetTodaysInput = pkgs.writeScriptBin "today" ''
    year=$(pwd | sed -E 's|^.*aoc(....)/.+$|\1|')
     day=$(pwd | sed -E 's|.+/day(.+)|\1|g')
    curl -H "Cookie: session=$session" "https://adventofcode.com/$year/day/$day/input" > input.txt
    cp ../../template.c ./main.c
  '';
in
pkgs.mkShell {
  name = "Advent of Code";
  buildInputs = with pkgs; [
    GetTodaysInput
  ];
}
