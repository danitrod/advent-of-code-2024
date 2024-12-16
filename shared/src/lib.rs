#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::{Down, Left, Right, Up};

impl Direction {
    pub fn to_2d_step_indexes(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    pub fn step_2d(&self, position: (usize, usize)) -> (usize, usize) {
        let step = self.to_2d_step_indexes();

        (
            (position.0 as isize + step.0) as usize,
            (position.1 as isize + step.1) as usize,
        )
    }

    pub fn step_takes_out_of_bounds(
        &self,
        position: (usize, usize),
        boundary: (usize, usize),
    ) -> bool {
        let step = self.to_2d_step_indexes();

        position.0 as isize + step.0 < 0
            || position.0 as isize + step.0 >= boundary.0 as isize
            || position.1 as isize + step.1 < 0
            || position.1 as isize + step.1 >= boundary.1 as isize
    }

    pub fn perpendicular_directions(&self) -> Vec<Direction> {
        match self {
            Up | Down => vec![Left, Right],
            Right | Left => vec![Up, Down],
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![Up, Right, Down, Left]
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => panic!("Parsing invalid direction character: {}", c),
        }
    }
}
