use shared::Direction;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut map = Vec::new();
    for line in input.lines() {
        map.push(
            line.chars()
                .map(|c| Plant {
                    label: c,
                    seen: false,
                    directions_checked: Vec::new(),
                })
                .collect(),
        );
    }

    part1(&mut map.clone());
    part2(&mut map);
}

#[derive(Clone)]
struct Plant {
    label: char,
    seen: bool,
    directions_checked: Vec<Direction>,
}

#[derive(Debug)]
struct Region {
    area: isize,
    perimeter: isize,
    sides: isize,
}

fn part1(map: &mut Vec<Vec<Plant>>) {
    println!("Day 12 - Part 1");

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !map[i][j].seen {
                let region = identify_region(map, i as isize, j as isize);
                total_price += region.area * region.perimeter
            }
        }
    }

    println!("Total price: {}", total_price);
}

fn identify_region(map: &mut Vec<Vec<Plant>>, x: isize, y: isize) -> Region {
    map[x as usize][y as usize].seen = true;

    let (mut area, mut perimeter, mut sides) = (1, 0, 0);

    for dir in Direction::all() {
        let (step_x, step_y) = dir.to_2d_step_indexes();

        if dir
            .step_takes_out_of_bounds((x as usize, y as usize), (map.len(), map[x as usize].len()))
            || map[(x + step_x) as usize][(y + step_y) as usize].label
                != map[x as usize][y as usize].label
        {
            perimeter += 1;

            if !map[x as usize][y as usize]
                .directions_checked
                .contains(&dir)
            {
                sides += 1;
                map[x as usize][y as usize]
                    .directions_checked
                    .push(dir.clone());
                for step_dir in dir.perpendicular_directions() {
                    let (next_x, next_y) = step_dir.to_2d_step_indexes();
                    check_sides(
                        map,
                        x + next_x,
                        y + next_y,
                        &dir,
                        step_dir,
                        map[x as usize][y as usize].label,
                    );
                }
            }
            continue;
        }

        if !map[(x + step_x) as usize][(y + step_y) as usize].seen
            && map[(x + step_x) as usize][(y + step_y) as usize].label
                == map[x as usize][y as usize].label
        {
            let neighboor_specs = identify_region(map, x + step_x, y + step_y);
            area += neighboor_specs.area;
            perimeter += neighboor_specs.perimeter;
            sides += neighboor_specs.sides;
        }
    }

    Region {
        area,
        perimeter,
        sides,
    }
}

fn check_sides(
    map: &mut Vec<Vec<Plant>>,
    x: isize,
    y: isize,
    side_to_check: &Direction,
    step: Direction,
    plant_type: char,
) {
    if x < 0
        || x >= map.len() as isize
        || y < 0
        || y >= map[x as usize].len() as isize
        || map[x as usize][y as usize].label != plant_type
    {
        return;
    }

    let (step_x, step_y) = side_to_check.to_2d_step_indexes();

    if x + step_x >= 0
        && x + step_x < map.len() as isize
        && y + step_y >= 0
        && y + step_y < map[(x + step_x) as usize].len() as isize
        && map[(x + step_x) as usize][(y + step_y) as usize].label == plant_type
    {
        return;
    }

    map[x as usize][y as usize]
        .directions_checked
        .push(side_to_check.clone());

    let (step_x, step_y) = step.to_2d_step_indexes();
    check_sides(map, x + step_x, y + step_y, side_to_check, step, plant_type)
}

fn part2(map: &mut Vec<Vec<Plant>>) {
    println!("Day 12 - Part 2");

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !map[i][j].seen {
                let region = identify_region(map, i as isize, j as isize);
                total_price += region.area * region.sides
            }
        }
    }

    println!("Total price: {}", total_price);
}
