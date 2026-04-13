#![allow(unused)]
use std::{convert::identity, fmt::Display};

static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 2: {}", part2(&input));
}
fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (lights, rest) = line.split_once(' ').unwrap();
            let mut iter = rest.split_whitespace().rev();
            let joltage = iter.next().unwrap();
            let joltage = parse_joltage(joltage);
            let lights = parse_lights(lights);
            let buttons = iter.rev().map(parse_button).collect();
            Machine {
                lights,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn parse_joltage(joltage: &str) -> Vec<u16> {
    let joltage = joltage.strip_prefix('{').unwrap();
    let joltage = joltage.strip_suffix('}').unwrap();
    joltage
        .split(',')
        .map(|num| {
            num.parse()
                .unwrap_or_else(|_| panic!("{num} in {joltage}",))
        })
        .collect()
}

fn parse_button(button_str: &str) -> Vec<u16> {
    let button_str = button_str.strip_prefix('(').unwrap();
    let button_str = button_str.strip_suffix(')').unwrap();
    button_str
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_lights(lights: &str) -> Vec<bool> {
    let mut iter = lights.bytes();
    let first = iter.next();
    assert_eq!(first, Some(b'['));
    iter.take_while(|c| *c != b']').map(|c| c == b'#').collect()
}

const MAX_DEPTH: usize = 100;
// same button combination that turns on the correct lights will also turn off exactly those
// lights. So we can work backwards.
fn part1(machines: &[Machine]) -> usize {
    let mut sim_states1 = vec![];
    let mut sim_states2 = vec![];
    let mut sim_states = &mut sim_states1;
    let mut sim_states_next = &mut sim_states2;
    let mut sum = 0;
    'main: for Machine {
        lights: reference,
        buttons,
        joltage: _,
    } in machines
    {
        sim_states.clear();
        sim_states_next.clear();
        sim_states.push(reference.clone());
        for iteration in 1..MAX_DEPTH {
            for lights in sim_states.drain(..) {
                for button in buttons {
                    if button.iter().any(|wire| lights[*wire as usize]) {
                        let mut sim_lights = lights.clone();
                        for wire in button {
                            let Some(light) = sim_lights.get_mut(*wire as usize) else {
                                panic!("{sim_lights:?} {button:?}");
                            };
                            *light = !*light;
                        }
                        if sim_lights.iter().all(|is_on| !*is_on) {
                            sum += iteration;
                            // FIXME(matyas): @readability
                            // refactor continue into a function that returns
                            continue 'main;
                        } else {
                            sim_states_next.push(sim_lights);
                        }
                    }
                }
            }
            std::mem::swap(&mut sim_states, &mut sim_states_next);
        }
    }

    sum
}
fn part2(machines: &[Machine]) -> usize {
    0
}

// fn part2_old(machines: &[Machine]) -> usize {
//     let mut sim_states1 = vec![];
//     let mut sim_states2 = vec![];
//     let mut sim_states = &mut sim_states1;
//     let mut sim_states_next = &mut sim_states2;
//     let mut sum = 0;
//     'main: for machine in machines.iter().take(1) {
//         let Machine {
//             lights: _,
//             buttons,
//             joltage: reference,
//         } = machine;
//         sim_states.clear();
//         sim_states_next.clear();
//         sim_states.push(reference.clone());
//         for iteration in 1..4 {
//             // println!("{sim_states:?}");
//             println!("{sim_states:?}");
//             for joltages in sim_states.drain(..) {
//                 for button in buttons
//                     .iter()
//                     .filter(|b| b.iter().all(|wire| joltages[*wire as usize] > 0))
//                 {
//                     let mut sim_joltages = joltages.clone();
//                     for wire in button {
//                         let Some(joltage) = sim_joltages.get_mut(*wire as usize) else {
//                             panic!("{sim_joltages:?} {button:?}");
//                         };
//                         *joltage -= 1;
//                     }
//                     if sim_joltages.iter().all(|left| *left == 0) {
//                         sum += iteration;
//                         println!("{sum}: {machine}");
//                         continue 'main;
//                     } else {
//                         sim_states_next.push(sim_joltages);
//                     }
//                 }
//             }
//             std::mem::swap(&mut sim_states, &mut sim_states_next);
//         }
//     }
//     sum
// }

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u16>>,
    joltage: Vec<u16>,
}
impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            lights,
            buttons,
            joltage,
        } = self;
        write!(f, "[")?;
        for &is_on in lights {
            if is_on {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "]")?;

        for button in buttons.iter() {
            write!(f, " (")?;
            for (i, wire) in button.iter().enumerate() {
                if i < button.len() - 1 {
                    write!(f, "{wire},")?;
                } else {
                    write!(f, "{wire}")?;
                }
            }
            write!(f, ")")?;
        }

        write!(f, " {{")?;
        for (i, jolts) in joltage.iter().enumerate() {
            if i < joltage.len() - 1 {
                write!(f, "{jolts},")?;
            } else {
                write!(f, "{jolts}")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
