{ pkgs ? import <nixpkgs> { } }:
let
  GetTodaysInput = pkgs.writeScriptBin "today" ''
   #! /usr/bin/env nu
    let year = (pwd | sed -E 's|^.*aoc(....)/.+$|\1|')
    let day = (pwd | sed -E 's|.+/day(.+)|\1|g')
    curl -H $"Cookie: session=($env.session)" $"https://adventofcode.com/($year)/day/($day)/input" | save src/input.txt
    # cp ../../template.c ./main.c
  '';
in
pkgs.mkShell {
  name = "Advent of Code";
  buildInputs = with pkgs; [
    GetTodaysInput
  ];
}
