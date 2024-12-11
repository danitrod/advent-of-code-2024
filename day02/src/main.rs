use std::{cmp::Ordering::Equal, fs};

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").unwrap();

    let mut puzzle_data = Vec::<Vec<isize>>::new();

    for line in input.lines() {
        puzzle_data.push(
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    part1(puzzle_data.clone());
    part2(puzzle_data);
}

fn part1(puzzle_data: Vec<Vec<isize>>) {
    println!("Day 02 - Part 1");

    let mut safe_reports = 0;
    let mut analyzed_reports = 0;
    'report_loop: for report in puzzle_data {
        analyzed_reports += 1;

        let mut levels = report.iter();
        let first_level = levels.next().unwrap();
        let second_level = levels.next().unwrap();

        let ordering = second_level.cmp(first_level);
        if ordering == Equal || (second_level - first_level).abs() > 3 {
            continue;
        }

        let mut last_level = second_level;
        for level in levels {
            if level.cmp(last_level) != ordering || (level - last_level).abs() > 3 {
                continue 'report_loop;
            }

            last_level = level;
        }

        safe_reports += 1;
    }

    println!("Safe reports: {}", safe_reports);
    println!("Total analyzed reports: {}", analyzed_reports);
}

fn part2(puzzle_data: Vec<Vec<isize>>) {
    println!("Day 02 - Part 2");

    let mut safe_reports = 0;
    let mut analyzed_reports = 0;
    for report in puzzle_data {
        analyzed_reports += 1;

        let mut levels = report.iter();
        let mut previous_level = levels.next().unwrap();
        let mut next_level = levels.next().unwrap();

        let mut bad_levels = 0;
        let mut ordering = next_level.cmp(previous_level);

        while ordering == Equal {
            bad_levels += 1;
            previous_level = next_level;
            let lev = levels.next();
            if lev.is_none() {
                break;
            }

            next_level = lev.unwrap();
            ordering = next_level.cmp(previous_level);
        }

        if (next_level - previous_level).abs() > 3 {
            bad_levels += 1;
        }

        previous_level = next_level;
        for level in levels {
            if level.cmp(previous_level) != ordering || (level - previous_level).abs() > 3 {
                bad_levels += 1;
            }

            previous_level = level;
        }

        if bad_levels <= 1 {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
    println!("Total analyzed reports: {}", analyzed_reports);
}
