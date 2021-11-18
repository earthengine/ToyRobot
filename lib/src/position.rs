use crate::toy_robot_error::ToyRobotError;
use core::fmt::{Display, Error as FmtError, Formatter};
use core::str::FromStr;
use regex::Regex;

#[derive(Clone, Copy)]
pub struct Position(i8, i8);

impl Display for Position {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "{},{}", self.0, self.1)
    }
}

impl FromStr for Position {
    type Err = ToyRobotError;
    fn from_str(to_parse: &str) -> Result<Self, ToyRobotError> {
        let invalid_position = || ToyRobotError::InvalidPosition(to_parse.to_owned());
        let reg = Regex::new(Self::pattern_str()).unwrap();
        let captures = reg.captures(to_parse).ok_or_else(invalid_position)?;
        let x = i8::from_str(captures.name("x").unwrap().as_str()).unwrap();
        let y = i8::from_str(captures.name("y").unwrap().as_str()).unwrap();

        Ok(Position(x, y))
    }
}

impl Position {
    pub const fn pattern_str() -> &'static str {
        r#"(?P<x>\d)\s*,\s*(?P<y>\d)"#
    }

    pub(crate) fn new(x: i8, y: i8) -> Result<Self, ToyRobotError> {
        if x < 0 || y < 0 || x > 5 || y > 5 {
            Err(ToyRobotError::PlacingOutOfBoundry(x, y))
        } else {
            Ok(Self(x, y))
        }
    }
    pub(crate) fn move_east(&mut self) -> Result<(), ToyRobotError> {
        if self.0 >= 5 {
            Err(ToyRobotError::MovingOutOfBoundry(self.0, self.1))
        } else {
            self.0 = self.0 + 1;
            Ok(())
        }
    }
    pub(crate) fn move_south(&mut self) -> Result<(), ToyRobotError> {
        if self.1 <= 0 {
            Err(ToyRobotError::MovingOutOfBoundry(self.0, self.1))
        } else {
            self.1 = self.1 - 1;
            Ok(())
        }
    }
    pub(crate) fn move_west(&mut self) -> Result<(), ToyRobotError> {
        if self.0 <= 0 {
            Err(ToyRobotError::MovingOutOfBoundry(self.0, self.1))
        } else {
            self.0 = self.0 - 1;
            Ok(())
        }
    }
    pub(crate) fn move_north(&mut self) -> Result<(), ToyRobotError> {
        if self.1 >= 5 {
            Err(ToyRobotError::MovingOutOfBoundry(self.0, self.1))
        } else {
            self.1 = self.1 + 1;
            Ok(())
        }
    }
}
