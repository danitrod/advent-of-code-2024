use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use shared::Direction;

use crate::location::*;

#[derive(Clone)]
pub struct Warehouse {
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
    pub fn step(&mut self, direction: Direction) {
        if direction
            .step_takes_out_of_bounds(self.robot_position, (self.map.len(), self.map[0].len()))
        {
            return;
        }

        let current_potision = self.robot_position;
        let next_position = direction.step_2d(self.robot_position);
        match self.map[next_position.0][next_position.1] {
            Location::Wall => (),
            Location::Box => {
                self.move_boxes(next_position, &direction);
            }
            Location::Free => {
                self.move_robot(current_potision, &direction);
            }
            Location::WideBox(_) => {
                self.move_wide_boxes(next_position, &direction);
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

    fn move_boxes(&mut self, from_position: (usize, usize), direction: &Direction) {
        let mut step = direction.step_2d(from_position);
        while self.map[step.0][step.1] == Location::Box {
            step = direction.step_2d(step);
        }

        match &self.map[step.0][step.1] {
            Location::Wall => (),
            Location::Free => {
                while self.map[step.0][step.1] != Location::Robot {
                    self.map[step.0][step.1] = Location::Box;
                    step = direction.opposite().step_2d(step);
                }

                self.map[step.0][step.1] = Location::Free;
                step = direction.step_2d(step);
                self.map[step.0][step.1] = Location::Robot;
                self.robot_position = step;
            }
            l => panic!("Trying to move box to invalid location: {}", l),
        }
    }

    fn can_wide_box_be_moved(&self, from_position: (usize, usize), direction: &Direction) -> bool {
        let mut boxes_to_check = VecDeque::new();
        boxes_to_check.push_back(from_position);
        let mut visited = HashSet::new();

        while let Some(box_pos) = boxes_to_check.pop_front() {
            let closing_box_pos = match self.map[box_pos.0][box_pos.1] {
                Location::WideBox(n) => match n {
                    0 => Direction::Right.step_2d(box_pos),
                    1 => Direction::Left.step_2d(box_pos),
                    _ => panic!("Unknown box type: {}", n),
                },
                _ => panic!(
                    "Trying to move non-box object {} at position {:?}",
                    self.map[box_pos.0][box_pos.1], box_pos
                ),
            };

            if visited.insert(closing_box_pos) {
                boxes_to_check.push_back(closing_box_pos);
            }

            let step_to_dir = direction.step_2d(box_pos);
            match self.map[step_to_dir.0][step_to_dir.1] {
                Location::Wall => return false,
                Location::WideBox(_) => {
                    if visited.insert(step_to_dir) {
                        boxes_to_check.push_back(step_to_dir);
                    }
                }
                _ => (),
            }
        }

        true
    }

    fn move_all_wide_boxes(&mut self, from_position: (usize, usize), direction: &Direction) {
        let mut boxes_to_move = VecDeque::new();
        boxes_to_move.push_back((from_position, false));
        let mut visited = HashSet::new();

        let mut new_map = self.map.clone();

        while let Some((box_pos, is_closing)) = boxes_to_move.pop_front() {
            let closing_box_pos = match self.map[box_pos.0][box_pos.1] {
                Location::WideBox(n) => match n {
                    0 => Direction::Right.step_2d(box_pos),
                    1 => Direction::Left.step_2d(box_pos),
                    _ => panic!("Unknown box type: {}", n),
                },
                _ => panic!(
                    "Trying to move non-box object {} at position {:?}",
                    self.map[box_pos.0][box_pos.1], box_pos
                ),
            };

            if visited.insert(closing_box_pos) {
                boxes_to_move.push_back((closing_box_pos, true));
            }

            let step_to_dir = direction.step_2d(box_pos);
            match &self.map[step_to_dir.0][step_to_dir.1] {
                Location::WideBox(_) => {
                    if visited.insert(step_to_dir) {
                        boxes_to_move.push_back((step_to_dir, false));
                    }
                }
                Location::Free => {}
                other => panic!("Trying to move wide box to invalid location: {}", other),
            }

            new_map[step_to_dir.0][step_to_dir.1] = self.map[box_pos.0][box_pos.1].clone();
            if (*direction == Direction::Up || *direction == Direction::Down) && is_closing {
                new_map[box_pos.0][box_pos.1] = Location::Free;
            } else {
                let opposing_step = direction.opposite().step_2d(box_pos);
                new_map[box_pos.0][box_pos.1] = self.map[opposing_step.0][opposing_step.1].clone();
            }
        }

        self.map = new_map;
    }

    fn move_wide_boxes(&mut self, from_position: (usize, usize), direction: &Direction) {
        if !self.can_wide_box_be_moved(from_position, direction) {
            return;
        }

        self.move_all_wide_boxes(from_position, direction);
        let step_back = direction.opposite().step_2d(from_position);
        self.map[step_back.0][step_back.1] = Location::Free;
        self.robot_position = from_position;
    }

    pub fn get_sum_of_box_coords(&self) -> usize {
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

    pub fn get_sum_of_box_coords_v2(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] == Location::WideBox(0) {
                    sum += 100 * i + j;
                }
            }
        }

        sum
    }

    pub fn transform_to_part_2(&mut self) {
        let mut new_map = Vec::new();
        for i in 0..self.map.len() {
            let mut new_row = Vec::new();
            for j in 0..self.map[i].len() {
                match self.map[i][j] {
                    Location::Wall => {
                        new_row.push(Location::Wall);
                        new_row.push(Location::Wall);
                    }
                    Location::Box => {
                        new_row.push(Location::WideBox(0));
                        new_row.push(Location::WideBox(1));
                    }
                    Location::Free => {
                        new_row.push(Location::Free);
                        new_row.push(Location::Free);
                    }
                    Location::Robot => {
                        new_row.push(Location::Robot);
                        new_row.push(Location::Free);
                        self.robot_position = (i, j * 2);
                    }
                    _ => (),
                }
            }

            new_map.push(new_row);
        }

        self.map = new_map;
    }
}
