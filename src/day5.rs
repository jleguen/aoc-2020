
/**
 * Day 5 - Binary Boarding
 */
use std::fmt;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
}

/* FBFBBFFRLR
 * -> FBFBBFF   = 44    -> F=0, B=1
 * -> RLR       = 5     -> L=0, R=1
 *
 */

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
impl Seat {
    fn parse(input: &str) -> Seat {
        let bin_row = input[0..7].replace("F", "0").replace("B", "1");
        let row = u32::from_str_radix(bin_row.as_str(), 2).unwrap();
        let bin_col = input[7..10].replace("L", "0").replace("R", "1");
        let col = u32::from_str_radix(bin_col.as_str(), 2).unwrap();
        //println!("{}: {} {}", input, bin_row, bin_col);
        Seat{row, col}
    }

    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
#[aoc_generator(day5)]
fn input_gen(input: &str) -> Vec<Seat> {
    let mut res = Vec::new();
    for line in input.lines() {
        res.push(Seat::parse(line));
    }
    res
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day5, part1)]
fn part1(boarding_list: &[Seat]) -> u32 {
    let mut max: u32 = 0;
    for seat in boarding_list.iter() {
        if seat.id() > max {
            max = seat.id()
        }
    }
    max
}

#[aoc(day5, part2)]
fn part2(boarding_list: &[Seat]) -> usize {
    let mut list: [bool; 1024] = [false; 1024];
    for seat in boarding_list.iter() {
        list[seat.id() as usize] = true;
    }
    let mut min: usize = 0;

    for (id, present) in list.iter().enumerate() {
        if min == 0 && *present == true {
            min = id;
        }
        if min != 0 && *present == false {
            return id;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id() {
        assert_eq!(Seat{row:70, col:7}.id(), 567);
        assert_eq!(Seat{row:14, col:7}.id(), 119);
        assert_eq!(Seat{row:102, col:4}.id(), 820);
    }

    fn check_parse(input: &str, row: u32, col: u32) {
        let s = Seat::parse(input);
        assert_eq!(s.row, row);
        assert_eq!(s.col, col);
    }
    #[test]
    fn sample() {
        check_parse("FBFBBFFRLR", 44, 5);
        check_parse("BFFFBBFRRR", 70, 7);
        check_parse("FFFBBBFRRR", 14, 7);
        check_parse("BBFFBBFRLL", 102, 4);
    }
}
