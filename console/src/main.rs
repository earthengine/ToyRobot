use rustyline::Editor;
use toy_robot::ToyRobotError;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut robot = toy_robot::ToyRobot::default();
    println!(
        r#"
Welcome to our toy robot console.

Here is the commands you may use:

PLACE x, y (, direction) - Place the robot to the board
MOVE
LEFT
RIGHT
REPORT

Use <ctrl-c> to exit.

- x and y must be integers between 0 and 5
- direction can be EAST, NORTH, SOUTH or WEST. If omitted the robot must be placed with a direction before.
- MOVE will move 1 step towards the current direction.
- LEFT and RIGHT will turn the current direction.
- REPORT will print the current location of the robot and its direction
        "#
    );
    while let Ok(line) = rl.readline("ToyRobot >") {
        match toy_robot::execute_command(&mut robot, &line) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => println!("{}", e),
        }
    }
}
