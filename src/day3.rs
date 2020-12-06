/**
 * Day 3 - Toboggan Trajectory
 */
use parse_display::{Display, FromStr};
use std::fmt;
use std::str;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

// The map is a rectangular array of unknown dimensions.
// It is stored as a Vector of rows.
// Each row is a Vector of locations, empty or with a tree
#[derive(Debug, Display, FromStr, PartialEq, Eq)]
enum Location {
    #[display(".")]
    Empty,
    #[display("#")]
    Tree,
}

#[derive(Debug)]
struct Row {
    data: Vec<Location>
}

#[derive(Debug)]
struct Area {
    map: Vec<Row>,
}

// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct OutOfMapError;
impl fmt::Display for OutOfMapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of Map")
    }
}



impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in self.data.iter() {
            write!(f, "{}", l)?;
        }
        Ok(())
    }
}

impl Location {
    fn from_char(input: char) -> Result<Self, &'static str> {
        let loc = match input {
            '#' => Location::Tree,
            '.' => Location::Empty,
            _ => return Err("unknown char")
        };
        Ok(loc)
    }
}

impl str::FromStr for Row {
    type Err = std::char::ParseCharError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for c in input.chars() {
            data.push(Location::from_char(c).unwrap());
        }
        Ok(Row{data})
    }
}

impl Row {
    pub fn get(&self, y: usize) -> &Location {
        let index = y % self.data.len();
        &self.data[index]
    }
}

impl str::FromStr for Area {
    type Err = std::char::ParseCharError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        for line in input.lines() {
            map.push(line.parse::<Row>().unwrap());
        }
        let area = Area{map};
        println!("New {} x {} map", area.size().0, area.size().1);
        Ok(area)
    }
}

impl Area {
    /** New empty map */
    pub fn new() -> Area {
        Area{map:Vec::new()}
    }

    /** Return (rows, columns )*/
    pub fn size(&self) -> (usize, usize) {
        match self.map.len() {
            0 => (0, 0),
            _ => (self.map.len(), self.map[0].data.len())
        }
    }

    /** Get location value (with horizontal rollover) */
    pub fn get(&self, x: usize, y: usize) -> Result<&Location, OutOfMapError> {
        if x >= self.map.len() { return Err(OutOfMapError); }
        Ok(self.map[x].get(y))
    }

    /** Count trees on a slope */
    pub fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut res: usize = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        loop {
            match self.get(x, y) {
                Ok(Location::Empty) => (),
                Ok(Location::Tree) => res += 1,
                Err(_e) => break res,
            };
            x += down;
            y += right;
        }
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
#[aoc_generator(day3)]
fn input_gen(input: &str) -> Area {
    input.parse::<Area>().unwrap()
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
/**
 * Return the number of trees on a slope
 */
#[aoc(day3, part1)]
fn part1(map: &Area) -> usize {
    map.count_trees(3, 1)
}

/**
 * Return the slope with minimum number of trees
 */
#[aoc(day3, part2)]
fn part2(map: &Area) -> usize {
    let mut total: usize = map.count_trees(1, 1);
    total *= map.count_trees(3, 1);
    total *= map.count_trees(5, 1);
    total *= map.count_trees(7, 1);
    total *= map.count_trees(1, 2);
    total
}



// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_location() {
        assert_eq!(".".parse(), Ok(Location::Empty));
        assert_eq!("#".parse(), Ok(Location::Tree));
        assert_eq!(Location::from_char('.'), Ok(Location::Empty));
        assert_eq!(Location::from_char('#'), Ok(Location::Tree));
    }

    #[test]
    fn check_row() {
        let row = "#.".parse::<Row>().unwrap();
        assert_eq!(format!("{}", row), "#.");
        assert_eq!(row.data[0], Location::Tree);
        assert_eq!(row.data[1], Location::Empty);
    }

    #[test]
    fn check_row_get() {
        let row = "#.#.".parse::<Row>().unwrap();
        assert_eq!(*row.get(0), Location::Tree);
        assert_eq!(*row.get(2), Location::Tree);
        assert_eq!(*row.get(4), Location::Tree);
    }

    #[test]
    fn check_map_parse() {
        let map = "#.".parse::<Area>().unwrap();
        assert_eq!(map.map[0].data[0], Location::Tree);
    }

    #[test]
    fn check_map_size() {
        let map = Area::new();
        assert_eq!(map.size(), (0,0));

        let map = "#.".parse::<Area>().unwrap();
        assert_eq!(map.size(), (1,2));

        let map = "#.\n..".parse::<Area>().unwrap();
        assert_eq!(map.size(), (2,2));
    }

    #[test]
    fn check_map_get() {
        let map = "#.\n..".parse::<Area>().unwrap();
        assert_eq!(*map.get(0,0).unwrap(), Location::Tree);
        assert_eq!(*map.get(1,10).unwrap(), Location::Empty);

        assert_eq!(map.get(2,0).unwrap_err(), OutOfMapError);
    }

    #[test]
    fn sample1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = input_gen(input);
        assert_eq!(map.count_trees(1, 1), 2);
        assert_eq!(map.count_trees(3, 1), 7);
        assert_eq!(map.count_trees(5, 1), 3);
        assert_eq!(map.count_trees(7, 1), 4);
        assert_eq!(map.count_trees(1, 2), 2);
    }
}
