pub mod commands;
pub mod directions;
pub mod position;
pub mod toy_robot;
pub mod toy_robot_error;

pub use commands::execute_command;
pub use toy_robot::ToyRobot;
pub use toy_robot_error::ToyRobotError;

#[cfg(test)]
mod tests {
    use crate::commands::execute_command;
    use crate::ToyRobot;
    use crate::ToyRobotError;
    #[test]
    fn should_display_location() -> Result<(), ToyRobotError> {
        let mut robot = ToyRobot::default();
        execute_command(&mut robot, "PLACE 0,0,NORTH")?;
        assert_eq!(format!("{}", robot), "0,0,NORTH\n");
        Ok(())
    }

    #[test]
    fn should_move() -> Result<(), ToyRobotError> {
        let mut robot = ToyRobot::default();
        execute_command(&mut robot, "PLACE 0,0,NORTH")?;
        execute_command(&mut robot, "MOVE")?;
        assert_eq!(format!("{}", robot), "0,1,NORTH\n");
        Ok(())
    }

    #[test]
    fn should_turn_left() -> Result<(), ToyRobotError> {
        let mut robot = ToyRobot::default();
        execute_command(&mut robot, "PLACE 0,0,NORTH")?;
        execute_command(&mut robot, "LEFT")?;
        assert_eq!(format!("{}", robot), "0,0,WEST\n");
        Ok(())
    }

    #[test]
    fn should_move_to_target() -> Result<(), ToyRobotError> {
        let mut robot = ToyRobot::default();
        execute_command(&mut robot, "PLACE 1,2,EAST")?;
        execute_command(&mut robot, "MOVE")?;
        execute_command(&mut robot, "MOVE")?;
        execute_command(&mut robot, "LEFT")?;
        execute_command(&mut robot, "MOVE")?;
        assert_eq!(format!("{}", robot), "3,3,NORTH\n");
        Ok(())
    }

    #[test]
    fn should_place() -> Result<(), ToyRobotError> {
        let mut robot = ToyRobot::default();
        execute_command(&mut robot, "PLACE 1,2,EAST")?;
        execute_command(&mut robot, "MOVE")?;
        execute_command(&mut robot, "LEFT")?;
        execute_command(&mut robot, "MOVE")?;
        execute_command(&mut robot, "PLACE 3,1")?;
        execute_command(&mut robot, "MOVE")?;
        assert_eq!(format!("{}", robot), "3,2,NORTH\n");
        Ok(())
    }
}
