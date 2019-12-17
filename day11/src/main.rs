use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::read_to_string;

use num_enum::TryFromPrimitive;

use computer::{Computer, IOMode};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn step_in_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y = self.y - 1,
            Direction::Down => self.y = self.y + 1,
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
        }
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(i64)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(i64)]
enum TurnDirection {
    AntiClockwise = 0,
    Clockwise = 1,
}

impl TurnDirection {
    fn from_i64(input: i64) -> TurnDirection {
        match TurnDirection::try_from(input) {
            Ok(td) => td,
            Err(e) => panic!("Cannot get turn direction from input: {}", e),
        }
    }
}

impl Direction {
    fn turn(&self, dir: TurnDirection) -> Direction {
        let i = *self as i64;
        match dir {
            TurnDirection::AntiClockwise => Direction::from_i64((i - 1).rem_euclid(4)),
            TurnDirection::Clockwise => Direction::from_i64((i + 1).rem_euclid(4)),
        }
    }

    fn from_i64(input: i64) -> Direction {
        match Direction::try_from(input) {
            Ok(dir) => dir,
            Err(e) => panic!("Cannot get direction from input: {}", e),
        }
    }
}

#[derive(Debug)]
struct Robot {
    facing: Direction,
    position: Point,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            facing: Direction::Up,
            position: Point { x: 0, y: 0 },
        }
    }

    fn turn(&mut self, dir: TurnDirection) {
        self.facing = self.facing.turn(dir);
    }
    fn step(&mut self) {
        self.position.step_in_dir(self.facing);
    }
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(i64)]
enum Colour {
    Black = 0,
    White = 1,
}

impl Colour {
    fn from_i64(input: i64) -> Colour {
        match Colour::try_from(input) {
            Ok(dir) => dir,
            Err(e) => panic!("Cannot get colour from input: {}", e),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Colour::Black => '█',
            Colour::White => ' ',
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let panel = run(Colour::Black);
    println!("Part 1 Solution: Painted {} Panels", panel.len());
}

fn part2() {
    let mut panel = run(Colour::White);

    println!("Solution Part 2 :");
    let y_range = 0..=panel.keys().map(|a| a.y).max().unwrap();
    let x_range = 0..=panel.keys().map(|a| a.x).max().unwrap();

    y_range
        .map(|y| {
            x_range
                .clone()
                .map(|x| Point { x, y })
                .map(|p| panel.entry(p).or_insert(Colour::Black).to_char())
                .collect::<String>()
        })
        .for_each(|l| println!("{}", l));
}

fn run<'a>(initial: Colour) -> HashMap<Point, Colour> {
    let input = read_to_string("input").expect("failed to read input file");
    let mut c = &mut Computer::from_string(&input[..], IOMode::Buffer);
    c.enable_logger = false;
    c.break_on_output = true;
    let robot = &mut Robot::new();
    let mut panel: HashMap<Point, Colour> = HashMap::new();

    panel.insert(robot.position, initial);

    while step(c, robot, &mut panel) {}

    panel
}

fn step(c: &mut Computer, robot: &mut Robot, panel: &mut HashMap<Point, Colour>) -> bool {
    c.input_to_buffer(*panel.entry(robot.position).or_insert(Colour::Black) as i64);
    c.run();
    let is_running = c.is_running();
    if is_running {
        let colour_to_paint = Colour::from_i64(c.output_from_buffer());
        c.run();
        let direction_to_turn = TurnDirection::from_i64(c.output_from_buffer());
        panel.insert(robot.position, colour_to_paint);
        robot.turn(direction_to_turn);
        robot.step();
    }
    return is_running;
}
