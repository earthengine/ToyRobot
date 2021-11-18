use rustyline::Editor;

fn print_introduction() {
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
}

fn main() {
    print_introduction();

    let mut rl = Editor::<()>::new();
    let mut robot = toy_robot::ToyRobot::default();

    while let Ok(line) = rl.readline("ToyRobot >") {
        match toy_robot::execute_command(&mut robot, &line) {
            // TODO: the specification does not show any command feedback unless it is a report command.
            // Maybe this should be adjustable.
            Ok(_result) => {
                if line == "REPORT" {
                    println!("{}", robot);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
