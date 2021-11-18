use crate::ToyRobotError;
use core::fmt::{Display, Error as FmtError, Formatter};
use core::str::FromStr;

#[derive(Clone, Copy)]
pub enum Direction {
    East,
    West,
    South,
    North,
}

impl Display for Direction {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Direction::East => write!(fmt, "EAST"),
            Direction::South => write!(fmt, "SOUTH"),
            Direction::West => write!(fmt, "WEST"),
            Direction::North => write!(fmt, "NORTH"),
        }
    }
}

impl FromStr for Direction {
    type Err = ToyRobotError;

    fn from_str(parsing_str: &str) -> Result<Self, ToyRobotError> {
        match parsing_str.to_uppercase().as_str() {
            "EAST" => Ok(Direction::East),
            "SOUTH" => Ok(Direction::South),
            "WEST" => Ok(Direction::West),
            "NORTH" => Ok(Direction::North),
            _ => Err(ToyRobotError::InvalidDirection(parsing_str.to_owned())),
        }
    }
}

impl Direction {
    pub const fn pattern_str() -> &'static str {
        "EAST|SOUTH|WEST|NORTH"
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}
