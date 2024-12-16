use std::{
    fmt::{self, Display},
    fs,
};

use shared::Direction;

fn main() {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();

    let mut parts = input.split("\n\n");

    let warehouse = Warehouse::from(parts.next().unwrap());
    let instructions = parts.next().unwrap().trim().replace("\n", "");

    part1(&mut warehouse.clone(), &instructions);
}

#[derive(Debug, Clone, PartialEq)]
enum Location {
    Free,
    Box,
    Wall,
    Robot,
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Free,
            'O' => Self::Box,
            '#' => Self::Wall,
            '@' => Self::Robot,
            _ => panic!("Unknown location {}", c),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
            Location::Free => ".".to_owned(),
            Location::Box => "O".to_owned(),
            Location::Wall => "#".to_owned(),
            Location::Robot => "@".to_owned(),
        };

        write!(f, "{}", content)
    }
}

#[derive(Clone)]
struct Warehouse {
    map: Vec<Vec<Location>>,
    robot_position: (usize, usize),
}

impl From<&str> for Warehouse {
    fn from(s: &str) -> Self {
        let mut map = Vec::new();
        let mut robot_position = (0, 0);

        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let loc = Location::from(c);
                if loc == Location::Robot {
                    robot_position = (i, j);
                }
                row.push(loc);
            }

            map.push(row);
        }

        Self {
            map,
            robot_position,
        }
    }
}

impl fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.map.iter() {
            for loc in row {
                write!(f, "{}", loc)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Warehouse {
    fn step(&mut self, movement: Direction) {
        if movement
            .step_takes_out_of_bounds(self.robot_position, (self.map.len(), self.map[0].len()))
        {
            return;
        }

        let current_potision = self.robot_position;
        let next_position = movement.step_2d(self.robot_position);
        match self.map[next_position.0][next_position.1] {
            Location::Wall => (),
            Location::Box => {
                if self.move_boxes(next_position, &movement).is_ok() {
                    self.move_robot(current_potision, &movement);
                }
            }
            Location::Free => {
                self.move_robot(current_potision, &movement);
            }
            _ => panic!("Invalid location next to robot"),
        }
    }

    fn move_robot(&mut self, from_position: (usize, usize), direction: &Direction) {
        let next_position = direction.step_2d(from_position);
        self.map[from_position.0][from_position.1] = Location::Free;
        self.map[next_position.0][next_position.1] = Location::Robot;
        self.robot_position = next_position;
    }

    fn move_boxes(
        &mut self,
        from_position: (usize, usize),
        direction: &Direction,
    ) -> Result<(), ()> {
        let mut step = direction.step_2d(from_position);
        while self.map[step.0][step.1] == Location::Box {
            step = direction.step_2d(step)
        }

        match &self.map[step.0][step.1] {
            Location::Wall => Err(()),
            Location::Free => {
                while self.map[step.0][step.1] != Location::Robot {
                    self.map[step.0][step.1] = Location::Box;
                    step = direction.opposite().step_2d(step);
                }

                self.map[step.0][step.1] = Location::Free;
                step = direction.step_2d(step);
                self.map[step.0][step.1] = Location::Robot;
                self.robot_position = step;

                Ok(())
            }
            l => panic!("Trying to move box to invalid location: {}", l),
        }
    }

    fn get_sum_of_box_coords(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] == Location::Box {
                    sum += 100 * i + j;
                }
            }
        }

        sum
    }
}

fn part1(warehouse: &mut Warehouse, instructions: &str) {
    println!("Day 15 - Part 1");

    for c in instructions.chars() {
        warehouse.step(Direction::from(c));
    }

    println!("Final Warehouse:\n{:?}", warehouse);
    println!(
        "Sum of box coordinates: {}",
        warehouse.get_sum_of_box_coords()
    );
}
