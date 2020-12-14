/**
 * Day 12 - Rain Risk
 */
use std::fmt;
use parse_display::{Display, FromStr};

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
#[derive(Debug, Display, FromStr)]
enum Instruction {
    #[display("N{0}")]
    North(i32),
    #[display("S{0}")]
    South(i32),
    #[display("E{0}")]
    East(i32),
    #[display("W{0}")]
    West(i32),
    #[display("L{0}")]
    Left(i32),
    #[display("R{0}")]
    Right(i32),
    #[display("F{0}")]
    Forward(i32),
}

#[derive(Debug, Display, FromStr)]
#[display("{}")]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Boat {
    north: i32,
    east: i32,
    dir: Direction,
}

struct Waypoint {
    north: i32,
    east: i32,
}

struct BoatWaypoint {
    north: i32,
    east: i32,
    waypoint: Waypoint,
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
impl Waypoint {
    fn turn_left(&mut self) {
        let tmp = self.east;
        self.east = -self.north;
        self.north = tmp;
    }

    fn turn_right(&mut self) {
        let tmp = self.east;
        self.east = self.north;
        self.north = -tmp;
    }
}

impl BoatWaypoint {
    fn new() -> Self {
        BoatWaypoint{north: 0, east: 0, waypoint: Waypoint{east:10, north:1}}
    }

    fn distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::North(value) => self.waypoint.north += value,
            Instruction::South(value) => self.waypoint.north -= value,
            Instruction::East(value) => self.waypoint.east += value,
            Instruction::West(value) => self.waypoint.east -= value,
            Instruction::Left(value) => {
                for _ in 0..value/90 {
                    self.waypoint.turn_left();
                }
            },
            Instruction::Right(value) => {
                for _ in 0..value/90 {
                    self.waypoint.turn_right();
                }
            },
            Instruction::Forward(value) => {
                for _ in 0..*value {
                    self.north += self.waypoint.north;
                    self.east += self.waypoint.east;
                }
            },
        }
    }
}

impl fmt::Display for BoatWaypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {} | Waypoint {}, {} | {}", self.east, self.north, self.waypoint.east, self.waypoint.north, self.distance())
    }
}

// ---------------------------------------------------------------------------
impl Boat {
    fn new() -> Self {
        Boat{north:0, east:0, dir:Direction::East}
    }

    fn distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }

    fn turn_left(&mut self) {
        match self.dir {
            Direction::North => self.dir = Direction::West,
            Direction::West => self.dir = Direction::South,
            Direction::South => self.dir = Direction::East,
            Direction::East => self.dir = Direction::North,
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Direction::North => self.dir = Direction::East,
            Direction::East => self.dir = Direction::South,
            Direction::South => self.dir = Direction::West,
            Direction::West => self.dir = Direction::North,
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Forward(value) => {
                match self.dir {
                    Direction::North => self.north += value,
                    Direction::South => self.north -= value,
                    Direction::East => self.east += value,
                    Direction::West => self.east -= value,
                }
            },
            Instruction::North(value) => { self.north += value },
            Instruction::South(value) => { self.north -= value },
            Instruction::East(value) => { self.east += value },
            Instruction::West(value) => { self.east -= value },
            Instruction::Left(value) => {
                for _ in 0..value/90 {
                    self.turn_left();
                }
            },
            Instruction::Right(value) => {
                for _ in 0..value/90 {
                    self.turn_right();
                }
            },
        }
    }
}

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {} facing {} | {}", self.east, self.north, self.dir, self.distance())
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn input_gen(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let program = input_gen(input);
    let mut boat = Boat::new();
    for inst in program.iter() {
        boat.execute(inst);
        println!("{} -> {}", inst, boat);
    }
    boat.distance()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let program = input_gen(input);
    let mut boat = BoatWaypoint::new();
    for inst in program.iter() {
        boat.execute(inst);
        println!("{} -> {}", inst, boat);
    }
    boat.distance()
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_sample() {
        let mut boat = Boat::new();
        let inst = input_gen(INPUT);
        println!("{:?}", inst);
        println!("{}", boat);

        for i in inst.iter() {
            boat.execute(i);
            println!("{} -> {}", i, boat);
        }
    }

    #[test]
    fn test_sample2() {
        part2(INPUT);
    }
}
