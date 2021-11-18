use super::{directions::Direction, position::Position};
use crate::toy_robot_error::ToyRobotError;
use core::fmt::{Display, Error as FmtError, Formatter};
use core::str::FromStr;
use regex::Regex;

pub enum ToyRobot {
    OnTable {
        position: Position,
        direction: Direction,
    },
    NotOnTable,
}

impl Display for ToyRobot {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match self {
            ToyRobot::OnTable {
                position,
                direction,
            } => write!(fmt, "{},{}", position, direction),
            ToyRobot::NotOnTable => write!(fmt, "Not on table"),
        }
    }
}

impl Default for ToyRobot {
    fn default() -> Self {
        ToyRobot::NotOnTable
    }
}

impl FromStr for ToyRobot {
    type Err = ToyRobotError;

    fn from_str(to_parse: &str) -> Result<Self, ToyRobotError> {
        let reg = Self::pattern_str();
        let reg = Regex::new(reg).unwrap();
        let captures = reg
            .captures(to_parse)
            .ok_or(ToyRobotError::InvalidRobot(to_parse.to_owned()))?;
        if captures.name("on_table").is_some() {
            let position = Position::from_str(captures.name("position").unwrap().as_str())?;
            let direction = Direction::from_str(captures.name("direction").unwrap().as_str())?;
            Ok(ToyRobot::OnTable {
                position,
                direction,
            })
        } else {
            Ok(ToyRobot::NotOnTable)
        }
    }
}

impl ToyRobot {
    pub const fn pattern_str() -> &'static str {
        concat!(
            r#"(?P<on_table>(?P<position>"#,
            r#"(?P<x>\d)\s*,\s*(?P<y>\d)"#, //Position::pattern_str()
            r#")\s*,\s*(?P<direction>"#,
            "EAST|SOUTH|WEST|NORTH", //Direction::pattern_str()
            "))|"
        )
    }

    pub fn new(x: i8, y: i8, direction: &str) -> Result<Self, ToyRobotError> {
        let dir = Direction::from_str(direction)?;
        Ok(ToyRobot::OnTable {
            position: Position::new(x, y)?,
            direction: dir,
        })
    }

    fn direction(&self) -> Option<Direction> {
        match self {
            ToyRobot::OnTable { direction, .. } => Some(*direction),
            _ => None,
        }
    }

    pub(crate) fn place_robot(
        &mut self,
        new_x: i8,
        new_y: i8,
        new_direction: Option<Direction>,
    ) -> Result<String, ToyRobotError> {
        match (
            Position::new(new_x, new_y),
            new_direction.or(self.direction()),
        ) {
            (Ok(pos), Some(dir)) => {
                *self = ToyRobot::OnTable {
                    position: pos,
                    direction: dir,
                };
                Ok(format!("Robot placed"))
            }
            (Ok(_), None) => Err(ToyRobotError::DirectionIndeterminate),
            (Err(pos_err), _) => Err(pos_err),
        }
    }

    pub(crate) fn move_robot(&mut self) -> Result<String, ToyRobotError> {
        match self {
            ToyRobot::OnTable {
                direction: Direction::East,
                position,
            } => position.move_east()?,
            ToyRobot::OnTable {
                direction: Direction::South,
                position,
            } => position.move_south()?,
            ToyRobot::OnTable {
                direction: Direction::West,
                position,
            } => position.move_west()?,
            ToyRobot::OnTable {
                direction: Direction::North,
                position,
            } => position.move_north()?,
            _ => return Err(ToyRobotError::NotReadyToMove),
        }
        Ok(format!(
            "Robot moved 1 step towards {}",
            self.direction().unwrap()
        ))
    }

    pub(crate) fn robot_turn_left(&mut self) -> Result<String, ToyRobotError> {
        match self {
            ToyRobot::OnTable { direction, .. } => Ok({
                *direction = direction.turn_left();
                format!("Rotot turned left")
            }),
            _ => Err(ToyRobotError::NotReadyToMove),
        }
    }

    pub(crate) fn robot_turn_right(&mut self) -> Result<String, ToyRobotError> {
        match self {
            ToyRobot::OnTable { direction, .. } => Ok({
                *direction = direction.turn_right();
                format!("Rotot turned right")
            }),
            _ => Err(ToyRobotError::NotReadyToMove),
        }
    }
}
