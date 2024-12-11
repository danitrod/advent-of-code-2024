use std::fs;

struct Map {
    map: Vec<Vec<u32>>,
}

impl Map {
    fn new(map: Vec<Vec<u32>>) -> Self {
        Self { map }
    }

    fn get_zero_indexes(&self) -> Vec<(isize, isize)> {
        let mut indexes = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    indexes.push((x as isize, y as isize));
                }
            }
        }

        indexes
    }

    fn find_near_number_indexes(&self, x: isize, y: isize, num: u32) -> Vec<(isize, isize)> {
        let mut indexes = Vec::new();

        for (i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            if x + i >= 0
                && x + i < self.map[0].len() as isize
                && y + j >= 0
                && y + j < self.map.len() as isize
                && self.map[(y + j) as usize][(x + i) as usize] == num
            {
                indexes.push((x + i, y + j));
            }
        }

        indexes
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").unwrap();

    let mut map = Vec::<Vec<u32>>::new();

    for line in input.lines() {
        map.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect())
    }

    part1(Map::new(map.clone()));
    part2(Map::new(map));
}

fn part1(map: Map) {
    println!("Day 10 - Part 1");

    let mut score = 0;
    for zero in map.get_zero_indexes() {
        let mut steps = map.find_near_number_indexes(zero.0, zero.1, 1);
        for height in 2..=9 {
            let mut next_steps = Vec::new();
            for (x, y) in steps {
                for index in map.find_near_number_indexes(x, y, height) {
                    if !next_steps.contains(&index) {
                        next_steps.push(index);
                    }
                }
            }

            steps = next_steps;
        }

        score += steps.len();
    }

    println!("Total trailhead score: {}", score);
}

fn part2(map: Map) {
    println!("Day 10 - Part 2");

    let mut trails = map.get_zero_indexes();
    for height in 1..=9 {
        let mut next_steps = Vec::new();
        for (x, y) in trails {
            next_steps.extend_from_slice(&map.find_near_number_indexes(x, y, height))
        }
        trails = next_steps;
    }

    println!("Sum of ratings of all trailheads: {}", trails.len());
}
