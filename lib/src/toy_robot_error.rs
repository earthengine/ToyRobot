use core::fmt::{Display, Error as FmtError, Formatter};
use core::num::ParseIntError;

#[derive(Debug)]
pub enum ToyRobotError {
    FormatError(FmtError),
    InvalidRobot(String),
    InvalidCommand(String),
    InvalidDirection(String),
    InvalidPosition(String),
    PlacingOutOfBoundry(i8, i8),
    MovingOutOfBoundry(i8, i8),
    DirectionIndeterminate,
    NotReadyToMove,
    NotInt(ParseIntError),
}

impl From<ParseIntError> for ToyRobotError {
    fn from(e: ParseIntError) -> Self {
        Self::NotInt(e)
    }
}

impl From<FmtError> for ToyRobotError {
    fn from(e: FmtError) -> Self {
        Self::FormatError(e)
    }
}

impl Display for ToyRobotError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match self {
            ToyRobotError::FormatError(_) => Ok(()),
            ToyRobotError::InvalidRobot(e) => write!(fmt, "Invalid robot: {}", e),
            ToyRobotError::InvalidCommand(e) => write!(fmt, "Invalid command: {}", e),
            ToyRobotError::InvalidDirection(e) => write!(fmt, "Invalid direction: {}", e),
            ToyRobotError::InvalidPosition(e) => write!(fmt, "Invalid position: {}", e),
            ToyRobotError::PlacingOutOfBoundry(x, y) => {
                write!(fmt, "Robot location out of bountry: {}, {}", *x, *y)
            }
            ToyRobotError::MovingOutOfBoundry(x, y) => {
                write!(fmt, "Attemp to move out of bountry: {}, {}", *x, *y)
            }
            ToyRobotError::DirectionIndeterminate => write!(
                fmt,
                "Trying to place without direction when the robot was not on table"
            ),
            ToyRobotError::NotReadyToMove => {
                write!(fmt, "Robot is not on table so it cannot move or turn")
            }
            ToyRobotError::NotInt(e) => write!(fmt, "Expect integer: {}", e),
        }
    }
}
