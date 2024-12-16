mod location;
mod warehouse;

use std::fs;

use shared::Direction;
use warehouse::Warehouse;

fn main() {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();

    let mut parts = input.split("\n\n");

    let warehouse = Warehouse::from(parts.next().unwrap());
    let instructions = parts.next().unwrap().trim().replace("\n", "");

    part1(&mut warehouse.clone(), &instructions);
    part2(&mut warehouse.clone(), &instructions);
}

fn part1(warehouse: &mut Warehouse, instructions: &str) {
    println!("Day 15 - Part 1");

    /* println!("Initial warehouse:\n{:?}", warehouse);
    sleep(Duration::from_secs(5)); */

    for c in instructions.chars() {
        warehouse.step(Direction::from(c));
        /* println!("Step: {}\nWarehouse:\n{:?}", c, warehouse);
        sleep(Duration::from_secs(2)); */
    }

    println!("Final Warehouse:\n{:?}", warehouse);
    println!(
        "Sum of box coordinates: {}",
        warehouse.get_sum_of_box_coords()
    );
}

fn part2(warehouse: &mut Warehouse, instructions: &str) {
    println!("Day 15 - Part 2");

    warehouse.transform_to_part_2();

    /* println!("Initial warehouse:\n{:?}", warehouse);
    sleep(Duration::from_secs(5)); */

    for c in instructions.chars() {
        warehouse.step(Direction::from(c));
        /* println!("Step: {}\nWarehouse:\n{:?}", c, warehouse);
        sleep(Duration::from_secs(2)); */
    }

    println!("Final Warehouse:\n{:?}", warehouse);
    println!(
        "Sum of box coordinates: {}",
        warehouse.get_sum_of_box_coords_v2()
    );
}
