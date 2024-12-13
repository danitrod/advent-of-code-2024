use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day04.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut puzzle = Vec::new();
    for line in input.lines() {
        puzzle.push(line.chars().collect());
    }

    part1(&puzzle);
    part2(&puzzle);
}

fn part1(input: &[Vec<char>]) {
    println!("Day 04 - Part 1");

    let mut xmas_count = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            for step_x in -1..=1 {
                for step_y in -1..=1 {
                    if input[x][y] == 'X' && has_xmas(input, x, y, step_x, step_y) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    println!("{}", xmas_count);
}

fn has_xmas(puzzle: &[Vec<char>], x: usize, y: usize, step_x: isize, step_y: isize) -> bool {
    if puzzle[x][y] == 'S' {
        return true;
    }

    let next_letter = match puzzle[x][y] {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => 'X',
    };

    if x as isize + step_x < 0
        || x as isize + step_x >= puzzle.len() as isize
        || y as isize + step_y < 0
        || y as isize + step_y >= puzzle[(x as isize + step_x) as usize].len() as isize
        || puzzle[(x as isize + step_x) as usize][(y as isize + step_y) as usize] != next_letter
    {
        return false;
    }

    has_xmas(
        puzzle,
        (x as isize + step_x) as usize,
        (y as isize + step_y) as usize,
        step_x,
        step_y,
    )
}

fn part2(puzzle: &[Vec<char>]) {
    println!("Day 04 - Part 2");

    let mut xmas_count = 0;
    for x in 0..puzzle.len() {
        for y in 0..puzzle[x].len() {
            if puzzle[x][y] == 'A' && has_cross_mas(puzzle, x, y) {
                xmas_count += 1;
            }
        }
    }

    println!("{}", xmas_count);
}

fn has_cross_mas(puzzle: &[Vec<char>], x: usize, y: usize) -> bool {
    if x as isize - 1 < 0
        || x + 1 >= puzzle.len()
        || y as isize - 1 < 0
        || y + 1 >= puzzle[x + 1].len()
    {
        return false;
    }

    ((puzzle[x + 1][y + 1] == 'M' && puzzle[x - 1][y - 1] == 'S')
        || (puzzle[x + 1][y + 1] == 'S' && puzzle[x - 1][y - 1] == 'M'))
        && ((puzzle[x + 1][y - 1] == 'M' && puzzle[x - 1][y + 1] == 'S')
            || (puzzle[x + 1][y - 1] == 'S' && puzzle[x - 1][y + 1] == 'M'))
}
