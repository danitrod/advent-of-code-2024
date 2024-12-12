mod instructions;

use instructions::{Do, Dont, Mul};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("Day 03 - Part 1");

    let mut sum = 0;

    let mut instruction = Mul::new();
    for c in input.chars() {
        instruction.add_char(c);

        if instruction.is_done() {
            sum += instruction
                .evaluate()
                .expect("instruction should have two numbers");

            instruction.clear();
        }
    }

    println!("Sum: {}", sum);
}

fn part2(input: &str) {
    println!("Day 03 - Part 2");

    let mut sum = 0;
    let mut should_operate = true;
    let mut mul_instr = Mul::new();
    let mut do_instr = Do::new();
    let mut dont_instr = Dont::new();
    for c in input.chars() {
        mul_instr.add_char(c);
        do_instr.add_char(c);
        dont_instr.add_char(c);

        if mul_instr.is_done() {
            if should_operate {
                sum += mul_instr
                    .evaluate()
                    .expect("instruction should have two numbers");
            }

            mul_instr.clear();
        }

        if do_instr.is_done() {
            should_operate = true;
            do_instr.clear();
        }

        if dont_instr.is_done() {
            should_operate = false;
            dont_instr.clear();
        }
    }

    println!("Sum: {}", sum);
}
