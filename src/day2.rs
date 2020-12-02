/**
 * Day 2 - Password Philosophy
 */
//use error_chain::error_chain;
use std::str::FromStr;
use std::error::Error;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
/*
mod errors {
    error_chain!{
        foreign_links {
            ParseChar(::std::char::ParseCharError);
            ParseInt(::std::num::ParseIntError);
        }
    }
}
use errors::*;
*/

/**
 * Represents a password policy
 * The letter has to appear between min and max times.
 */
#[derive(Debug)]
pub struct Policy {
    min: usize,
    max: usize,
    letter: char
}

#[derive(Debug)]
pub struct PasswordEntry {
    policy: Policy,
    password: String
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<PasswordEntry> {
    input.lines().map(|l| { PasswordEntry::from_str(l).unwrap() }).collect()
}

// ---------------------------------------------------------------------------
// Policy checker
// ---------------------------------------------------------------------------
impl FromStr for Policy {

    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Policy, Box<dyn Error>> {
        let dash = input.find('-').unwrap();
        let space = input.find(' ').unwrap();
        let min = usize::from_str(&input[0..dash])?;
        let max = usize::from_str(&input[dash+1..space])?;
        let letter: char = char::from_str(&input[space+1..])?;
        Ok(Policy{min, max, letter})
    }
}

impl FromStr for PasswordEntry {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<PasswordEntry, Box<dyn Error>> {
        let colon = input.find(':').unwrap();
        let policy = Policy::from_str(&input[0..colon])?;
        let password = String::from(&input[colon+2..]);
        Ok(PasswordEntry{policy, password})
    }
}

impl PasswordEntry {
    // Validation for the first part
    pub fn valid_old(&self) -> bool {
        let count = self.password.chars().filter(|x| x == &self.policy.letter).count();
        self.policy.min <= count && count <= self.policy.max
    }

    // Validation for the second part
    pub fn valid_new(&self) -> bool {
        let first = self.password.chars().nth(self.policy.min-1).unwrap() == self.policy.letter;
        let second = self.password.chars().nth(self.policy.max-1).unwrap() == self.policy.letter;
        (first || second) && !(first && second)
    }
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day2, part1)]
pub fn part1(db: &[PasswordEntry]) -> usize {
    db.iter().filter(|p| p.valid_old()).count()
}

#[aoc(day2, part2)]
pub fn part2(db: &[PasswordEntry]) -> usize {
    db.iter().filter(|p| p.valid_new()).count()
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn check_policy_constructor(input: &str, min: usize, max: usize, letter: char) {
        let p = Policy::from_str(input).unwrap();
        assert_eq!(p.letter, letter);
        assert_eq!(p.min, min);
        assert_eq!(p.max, max);
    }

    #[test]
    fn check_policy() {
        check_policy_constructor("1-3 a", 1, 3, 'a');
        check_policy_constructor("10-30 z", 10, 30, 'z');
    }

    #[test]
    fn check_password_entry() {
        let p: PasswordEntry = PasswordEntry::from_str("1-3 a: abcde").unwrap();
        assert_eq!(p.policy.letter, 'a');
        assert_eq!(p.policy.min, 1);
        assert_eq!(p.policy.max, 3);
        assert_eq!(p.password, String::from("abcde"));
    }

    #[test]
    fn check_sample1() {
        assert_eq!(PasswordEntry::from_str("1-3 a: abcde").unwrap().valid_old(), true);
        assert_eq!(PasswordEntry::from_str("1-3 b: cdefg").unwrap().valid_old(), false);
        assert_eq!(PasswordEntry::from_str("2-9 c: ccccccccc").unwrap().valid_old(), true);
        
        let data = input_gen("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(part1(&data), 2);
    }

    #[test]
    fn check_sample2() {
        assert_eq!(PasswordEntry::from_str("1-3 a: abcde").unwrap().valid_new(), true);
        assert_eq!(PasswordEntry::from_str("1-3 b: cdefg").unwrap().valid_new(), false);
        assert_eq!(PasswordEntry::from_str("2-9 c: ccccccccc").unwrap().valid_new(), false);
        let data = input_gen("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(part2(&data), 1);
    }
}
