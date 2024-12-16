use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Location {
    Free,
    Box,
    Wall,
    Robot,
    WideBox(usize),
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
            Location::WideBox(n) => {
                if *n == 0 {
                    "[".to_owned()
                } else {
                    "]".to_owned()
                }
            }
        };

        write!(f, "{}", content)
    }
}
