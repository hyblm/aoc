{ pkgs ? import <nixpkgs> { } }:
let
  GetTodaysInput = pkgs.writeScriptBin "today" ''
   #! /usr/bin/env nu
    let year = (pwd | sed -E 's|^.*\/(....)-.+$|\1|')
    let day = (pwd | sed -E 's|^.*\/....-0*(.+)$|\1|')
    let path = $"($year)/day/($day)/input"
    echo $path
    curl -H $"Cookie: session=($env.session)" $"https://adventofcode.com/($path)" | save input.txt
    # cp ../../template.c ./main.c
  '';
in
pkgs.mkShell {
  name = "Advent of Code";
  buildInputs = with pkgs; [
    GetTodaysInput
  ];
}
