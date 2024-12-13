use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .trim()
        .to_owned();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("Day 11 - Part 1");

    let mut stones: Vec<String> = input.split_whitespace().map(|c| c.to_owned()).collect();

    for _ in 0..25 {
        let mut new_stones: Vec<String> = Vec::new();
        println!("Num of stones: {}", stones.len());

        for stone in stones.iter() {
            if *stone == "0" {
                new_stones.push("1".to_owned());
            } else if stone.len() % 2 == 0 {
                let left = stone[0..stone.len() / 2].parse::<isize>().unwrap();
                let right = stone[stone.len() / 2..].parse::<isize>().unwrap();
                new_stones.push(left.to_string());
                new_stones.push(right.to_string());
            } else {
                new_stones.push((stone.parse::<isize>().unwrap() * 2024).to_string());
            }
        }

        stones = new_stones;
    }

    println!("Total num of stones: {}", stones.len());
}

fn part2(input: &str) {
    println!("Day 11 - Part 2");

    let stones: Vec<isize> = input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();
    let mut count = 0;
    for stone in stones {
        count += blinked_stone_count(&mut cache, stone, 75);
    }
    println!("Total num of stones: {}", count);
}

fn blinked_stone_count(
    cache: &mut HashMap<(isize, isize), isize>,
    stone: isize,
    blinks_left: isize,
) -> isize {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, blinks_left)) {
        return *cached;
    }

    let result = if stone == 0 {
        blinked_stone_count(cache, 1, blinks_left - 1)
    } else if stone.to_string().len() % 2 == 0 {
        let left = blinked_stone_count(
            cache,
            stone.to_string()[0..stone.to_string().len() / 2]
                .parse::<isize>()
                .unwrap(),
            blinks_left - 1,
        );
        let right = blinked_stone_count(
            cache,
            stone.to_string()[stone.to_string().len() / 2..]
                .parse::<isize>()
                .unwrap(),
            blinks_left - 1,
        );

        left + right
    } else {
        blinked_stone_count(cache, stone * 2024, blinks_left - 1)
    };

    cache.insert((stone, blinks_left), result);
    result
}
