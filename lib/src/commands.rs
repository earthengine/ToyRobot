use super::directions::Direction;
use crate::toy_robot_error::ToyRobotError;
use crate::ToyRobot;
use core::str::FromStr;
use regex::Regex;

enum Command {
    Place(i8, i8, Option<Direction>),
    Move,
    Left,
    Right,
    Report,
}

fn commands() -> Regex {
    Regex::new(concat!(
        r#"\s*(?P<place>PLACE\s+(?P<x>\d)\s*,\s*(?P<y>\d)\s*(,\s*(?P<direction>EAST|SOUTH|WEST|NORTH))?)\s*|"#,
        r#"\s*(?P<move>MOVE)\s*|"#,
        r#"\s*(?P<left>LEFT)\s*|"#,
        r#"\s*(?P<right>RIGHT)\s*|"#,
        r#"\s*(?P<report>REPORT)\s*"#,
    ))
    .unwrap()
}

impl Command {
    fn parse_command(command: &str) -> Result<Command, ToyRobotError> {
        let invalid_command = || ToyRobotError::InvalidCommand(command.to_owned());
        let captures = commands().captures(command).ok_or_else(invalid_command)?;
        if captures.name("place").is_some() {
            let x = i8::from_str(captures.name("x").unwrap().as_str()).unwrap();
            let y = i8::from_str(captures.name("y").unwrap().as_str()).unwrap();
            let direction = captures
                .name("direction")
                .map(|d| Direction::from_str(d.as_str()))
                .transpose()
                .unwrap();
            return Ok(Command::Place(x, y, direction));
        } else if captures.name("move").is_some() {
            return Ok(Command::Move);
        } else if captures.name("left").is_some() {
            return Ok(Command::Left);
        } else if captures.name("right").is_some() {
            return Ok(Command::Right);
        } else if captures.name("report").is_some() {
            return Ok(Command::Report);
        }

        Err(ToyRobotError::InvalidCommand(command.to_owned()))
    }

    fn execute(&self, robot: &mut ToyRobot) -> Result<String, ToyRobotError> {
        match self {
            Command::Place(x, y, direction) => robot.place_robot(*x, *y, *direction),
            Command::Move => robot.move_robot(),
            Command::Left => robot.robot_turn_left(),
            Command::Right => robot.robot_turn_right(),
            Command::Report => Ok(format!("Robot status: {}", robot)),
        }
    }
}

pub fn execute_command(robot: &mut ToyRobot, command: &str) -> Result<String, ToyRobotError> {
    let command = Command::parse_command(command)?;

    command.execute(robot)
}
