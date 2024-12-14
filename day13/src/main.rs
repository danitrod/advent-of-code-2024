mod claw_machine;

use claw_machine::ClawMachine;
use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day13.txt").unwrap();

    let mut machines = Vec::new();

    for spec in input.split("\n\n") {
        machines.push(ClawMachine::from(spec));
    }

    part1(&machines);
}

fn part1(machines: &Vec<ClawMachine>) {
    println!("Day 13 - Part 1");

    println!(
        "Minimum tokens needed: {}",
        machines
            .par_iter()
            .map(|m| m.find_minimum_price_to_prize().unwrap_or(0))
            .sum::<isize>()
    );
}
