use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").unwrap();

    let mut left_list = Vec::<isize>::new();
    let mut right_list = Vec::<isize>::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    part1(left_list.clone(), right_list.clone());
    part2(left_list, right_list);
}

fn part1(mut left_list: Vec<isize>, mut right_list: Vec<isize>) {
    println!("Day 01 - Part 1");

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for i in 0..left_list.len() {
        sum += (left_list[i] - right_list[i]).abs();
    }

    println!("Sum: {}", sum);
}

fn part2(left_list: Vec<isize>, right_list: Vec<isize>) {
    println!("Day 01 - Part 2");

    let mut right_list_count = HashMap::<isize, isize>::new();

    for n in right_list {
        *right_list_count.entry(n).or_insert(0) += 1;
    }

    let mut score = 0;
    for n in left_list {
        score += n * right_list_count.get(&n).unwrap_or(&0);
    }

    println!("Score: {}", score);
}
