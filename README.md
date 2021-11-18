# Introduction

Toy Robot is a simple lib to maintain a robot's location and its position in a table.

The location of the robot is identified by two integer numbers, representing the size of the table. In the code the size is 6x6 and the numbers are ranged from 0 to 5.

The robot starts with a "Not on table" status, and when it was placed on the table, a direction must be given. After that, subsequent commands can place it to somewhere else, with or without specifying a new direction.

The robot can also receive commands to move forward or turn left/right.

# Build and run

This program is written in Rust, the best programming language ever.

To start, following the instruction in the following page:

https://rustup.rs/

Then run the following in a console:

```
git clone https://github.com/earthengine/ToyRobot
cd console
cargo run
```

# The design

Why using Rust? I have a similar project before that was written in Rust, that is a console program accepts several commands.

Althought Rust has been described as a hard language, I feel this is this the right choice. Rust is a strict language which means the compiler will do a lot of things to prevent mistakes.

### Simple Duties

Rust seperates the data and behavour, which pushes this to the extreme. It also encourage to seperate similar duties into traits.

In this demo I have created 5 data structures in total to capture the different aspects that the toy robot will need.

- `Position` is to capture the position of the robot on the table, it is a `struct` with two private fields.
- `Direction` is to capture the direction that the robot currently facing. it is an `enum` that have 4 variants.
- `ToyRobot` is a `struct` that is combination of `Position` and `Direction`, represents a physical robot.
- `ToyRobotError` is an `enum` to capture all possible errors needed to be handled.
- `Commands` is an `enum` which variants are the console commands the library supports.

## The engine

Thanks to Rust's crate system, we can easily access the beautiful crate `rustyline`, which makes console programming very easy.

# Thoughts and to do lists

The following is what should be there in the final solution but was missed out

- The Rust supports generate document from code comments, we can even include tests/examples in the document.
- More tests to cover the errors etc.
- Introduce a optional command line option to show the command feedback.
- Some error variants was not actually in use. They should be removed later.
