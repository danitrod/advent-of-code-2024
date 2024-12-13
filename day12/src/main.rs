use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut map = Vec::new();
    for line in input.lines() {
        map.push(line.chars().map(|c| (c, false)).collect());
    }

    part1(&mut map.clone());
    part2(&map);
}

fn part1(map: &mut Vec<Vec<(char, bool)>>) {
    println!("Day 12 - Part 1");

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !map[i][j].1 {
                let (area, perimeter) = get_region_specs(map, i as isize, j as isize);
                total_price += area * perimeter
            }
        }
    }

    println!("Total price: {}", total_price);
}

fn get_region_specs(map: &mut Vec<Vec<(char, bool)>>, x: isize, y: isize) -> (isize, isize) {
    map[x as usize][y as usize].1 = true;

    let (mut area, mut perimeter) = (1, 0);

    for (i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
        if x + i < 0
            || x + i >= map.len() as isize
            || y + j < 0
            || y + j >= map[(x + i) as usize].len() as isize
        {
            perimeter += 1;
            continue;
        }

        if map[(x + i) as usize][(y + j) as usize].0 != map[x as usize][y as usize].0 {
            perimeter += 1;
        }
    }

    for (i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
        if x + i >= 0
            && x + i < map.len() as isize
            && y + j >= 0
            && y + j < map[(x + i) as usize].len() as isize
            && !map[(x + i) as usize][(y + j) as usize].1
            && map[(x + i) as usize][(y + j) as usize].0 == map[x as usize][y as usize].0
        {
            let neighboor_specs = get_region_specs(map, x + i, y + j);
            area += neighboor_specs.0;
            perimeter += neighboor_specs.1;
        }
    }

    (area, perimeter)
}

fn part2(_map: &Vec<Vec<(char, bool)>>) {
    println!("Day 12 - Part 2");
}
