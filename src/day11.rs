/**
 * Day 11 - Seating System
 */
use std::fmt;
use parse_display::{Display, FromStr};

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
#[derive(Clone, Debug, Display, FromStr, PartialEq)]
enum Position {
    #[display(".")]
    Floor,
    #[display("L")]
    Empty,
    #[display("#")]
    Occupied,
}

#[derive(Clone, Debug, PartialEq)]
struct Seats {
    layout: Vec<Vec<Position>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
impl Seats {
    fn new() -> Self {
        let layout = Vec::new();
        Seats {layout}
    }

    fn from_str(input: &str) -> Self {
        let mut seats = Seats::new();
        for line in input.lines() {
            seats.add_row(line);
        }

        seats
    }

    fn add_row(&mut self, line: &str) {
        let mut row = Vec::new();
        for position in line.chars() {
            let pos = match position {
                '.' => Position::Floor,
                'L' => Position::Empty,
                '#' => Position::Occupied,
                _ => unreachable!(),
            };
            row.push(pos);
        }
        self.layout.push(row);
    }

    fn size(&self) -> (i32, i32) {
        (self.layout.len() as i32, self.layout[0].len() as i32)
    }

    fn get(&self, x: i32, y: i32) -> &Position {
        if x < 0 || x >= self.size().0 ||
            y < 0 || y >= self.size().1 {
                return &Position::Floor;
            }
        &self.layout[x as usize][y as usize]
    }

    fn all_occupied(&self) -> u32 {
        let mut cnt = 0;
        for i in 0..self.size().0 {
            for j in 0..self.size().1 {
                if let Position::Occupied = self.get(i,j) {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    
    /** True if one seat is occupied in the given direction */
    fn direction_occupied(&self, x:i32, y:i32, dir: &Direction) -> bool {
        let mut i = x;
        let mut j = y;
        loop {
            match dir {
                Direction::Up => i -= 1,
                Direction::Down => i += 1,
                Direction::Left => j -= 1,
                Direction::Right =>j += 1,
                Direction::UpLeft => {i -=1; j-=1;},
                Direction::UpRight => {i -= 1; j += 1;},
                Direction::DownLeft => {i += 1; j -= 1;},
                Direction::DownRight => {i += 1; j += 1;},
            }
            if i < 0 || i > self.size().0 ||
                j < 0 || j > self.size().1 {
                    return false;
                }
            if let Position::Occupied = self.get(i,j) {
                return true;
            }
            if let Position::Empty = self.get(i,j) {
                return false;
            }
        }
    }

    /** Count occupied seats around x,y in all directions */
    fn count_visible_occupied(&self, x:i32, y:i32) -> i32 {
        let mut cnt: i32 = 0;
        for dir in &[Direction::Up, Direction::Down, 
                     Direction::Left, Direction::Right,
                     Direction::UpLeft, Direction::UpRight,
                     Direction::DownLeft, Direction::DownRight] {
            if let true = self.direction_occupied(x, y, dir) {
                cnt += 1;
            }
        }
        cnt
    }

    /** Count occupied seats around x,y */
    fn count_adjacent_occupied(&self, x:i32, y:i32) -> i32 {
        let mut cnt: i32 = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                if i == x && j == y { continue; }
                if let Position::Occupied = self.get(i, j) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn evolution_visible(&self, x: i32, y: i32, tolerance: i32) -> Position {
        match self.get(x, y) {
            Position::Floor => Position::Floor,
            Position::Empty => match self.count_visible_occupied(x,y) {
                0 => Position::Occupied,
                _ => Position::Empty,
            },
            Position::Occupied => { 
                if self.count_visible_occupied(x,y) > tolerance { Position::Empty } 
                else { Position::Occupied } 
            },
        }
    }

    /** Count occupied seats around x,y and compute evolution */
    fn evolution(&self, x: i32, y: i32, tolerance: i32) -> Position {
        match self.get(x, y) {
            Position::Floor => Position::Floor,
            Position::Empty => match self.count_adjacent_occupied(x,y) {
                0 => Position::Occupied,
                _ => Position::Empty,
            },
            Position::Occupied => { 
                if self.count_adjacent_occupied(x,y) > tolerance { Position::Empty } 
                else { Position::Occupied } 
            },
        }
    }

    fn print_occupied(&self) {
        for i in 0..self.size().0 {
            for j in 0..self.size().1 {
                if let Position::Floor = self.get(i,j) {
                    print!(".");
                } else {
                    print!("{}", self.count_adjacent_occupied(i,j));
                }
            }
            println!("");
        }
        println!("");
    }

    fn run_once_visible(&mut self) -> bool {
        let orig = self.clone();

        for x in 0..orig.size().0 {
            for y in 0..orig.size().1 {
                let evo = orig.evolution_visible(x, y, 4);
                //println!("{} => {}", self.get(x,y), evo);
                self.layout[x as usize][y as usize] = evo;
            }
        }

        *self != orig
    }

    fn run_once_adjacent(&mut self) -> bool {
        let orig = self.clone();

        for x in 0..orig.size().0 {
            for y in 0..orig.size().1 {
                let evo = orig.evolution(x, y, 3);
                //println!("{} => {}", self.get(x,y), evo);
                self.layout[x as usize][y as usize] = evo;
            }
        }

        *self != orig
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.layout.iter() {
            for seat in row.iter() {
                write!(f, "{}", seat)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day11, part1)]
fn part1(input: &str) -> u32 {
    let mut seats = Seats::from_str(input);
    println!("{}", seats);
    while seats.run_once_adjacent() {
        println!("{}", seats);
    }
    seats.all_occupied()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u32 {
    let mut seats = Seats::from_str(input);
    println!("{}", seats);
    while seats.run_once_visible() {
        println!("{}", seats);
    }
    seats.all_occupied()
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    static ITER1: &'static str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    static ITER2: &'static str = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    static ITER3: &'static str = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

    static ITER4: &'static str = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

    static ITER5: &'static str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    #[test]
    fn test_seats() {
        let mut seats = Seats::new();
        for line in INPUT.lines() {
            seats.add_row(line);
        }
        println!("INPUT");
        println!("{}", seats);
        seats.print_occupied();

        assert_eq!(true, seats.run_once_adjacent());
        println!("Iteration 1");
        println!("{}", seats);
        seats.print_occupied();
        assert_eq!(Seats::from_str(ITER1), seats);

        assert_eq!(true, seats.run_once_adjacent());
        println!("Iteration 2");
        println!("{}", seats);
        seats.print_occupied();
        assert_eq!(Seats::from_str(ITER2), seats);

        assert_eq!(true, seats.run_once_adjacent());
        println!("Iteration 3");
        println!("{}", seats);
        seats.print_occupied();
        assert_eq!(Seats::from_str(ITER3), seats);

        assert_eq!(true, seats.run_once_adjacent());
        println!("Iteration 4");
        println!("{}", seats);
        seats.print_occupied();
        assert_eq!(Seats::from_str(ITER4), seats);

        assert_eq!(true, seats.run_once_adjacent());
        println!("Iteration 5");
        println!("{}", seats);
        seats.print_occupied();
        assert_eq!(Seats::from_str(ITER5), seats);

        assert_eq!(false, seats.run_once_adjacent());
        //while(seats.run_once_adjacent()) { println!("{}", seats); }
        assert_eq!(37, seats.all_occupied());
    }

    #[test]
    fn test_part2() {
        let mut seats = Seats::from_str(INPUT);
        while(seats.run_once_visible()) { println!("{}", seats); }
        assert_eq!(26, seats.all_occupied());
    }

}
