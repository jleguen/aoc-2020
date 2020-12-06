/**
 * Day 4 - Password Processing
 */
use std::fmt;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum Item {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}



#[derive(Debug)]
struct Passport {
    byr: Option<Item>,
    iyr: Option<Item>,
    eyr: Option<Item>,
    hgt: Option<Item>,
    hcl: Option<Item>,
    ecl: Option<Item>,
    pid: Option<Item>,
    cid: Option<Item>,
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::BirthYear(e) => write!(f, "byr:{}", e),
            Item::IssueYear(e) => write!(f, "iyr:{}", e),
            Item::ExpirationYear(e) => write!(f, "eyr:{}", e),
            Item::Height(e) => write!(f, "hgt:{}", e),
            Item::HairColor(e) => write!(f, "hcl:{}", e),
            Item::EyeColor(e) => write!(f, "ecl:{}", e),
            Item::PassportID(e) => write!(f, "pid:{}", e),
            Item::CountryID(e) => write!(f, "cid:{}", e),
        }
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_item(item: &Option<Item>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match item {
                Some(e) => write!(f, "{} ", e),
                None => write!(f, "_ "),
            }
        }
        write!(f, "Passport > ")?;
        fmt_item(&self.byr, f)?;
        fmt_item(&self.iyr, f)?;
        fmt_item(&self.eyr, f)?;
        fmt_item(&self.hgt, f)?;
        fmt_item(&self.hcl, f)?;
        fmt_item(&self.ecl, f)?;
        fmt_item(&self.pid, f)?;
        fmt_item(&self.cid, f)?;
        Ok(())
    }
}

impl Passport {
    /**
     * Valid only if all fields are present (cid is optional)
     */
    fn valid(&self) -> bool {
        fn valid_year(value: &str, min: u32, max: u32) -> bool {
            let num = value.parse::<u32>().unwrap();
            value.len() == 4 && num >= min && num <= max
        }
        fn valid_height(value: &str) -> bool {
            if value.ends_with("cm") {
                let height = value.trim_end_matches("cm").parse::<u32>().unwrap();
                height >= 150 && height <= 193
            } else if value.ends_with("in") {
                let height = value.trim_end_matches("in").parse::<u32>().unwrap();
                height >= 59 && height <= 76
            } else {
                false
            }
        }
        fn valid_hair_color(value: &str) -> bool {
            if value.len() == 7 && value.starts_with("#") {
                value.trim_start_matches("#")
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .count() == 6
            } else {
                false
            }
        }
        fn valid_eye_color(value: &str) -> bool {
            match value {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        }
        fn valid_id(value: &str) -> bool {
            value.len() == 9 && value.chars()
                .filter(|c| c.is_ascii_digit())
                .count() == 9
        }

        fn valid_item(item: &Option<Item>) -> bool {
            match item {
                None => false,
                Some(Item::BirthYear(e)) => valid_year(e, 1920, 2002),
                Some(Item::IssueYear(e)) => valid_year(e, 2010, 2020),
                Some(Item::ExpirationYear(e)) => valid_year(e, 2020, 2030),
                Some(Item::Height(e)) => valid_height(e),
                Some(Item::HairColor(e)) => valid_hair_color(e),
                Some(Item::EyeColor(e)) => valid_eye_color(e),
                Some(Item::PassportID(e)) => valid_id(e),
                Some(Item::CountryID(_)) => true
            }
        }
        valid_item(&self.byr) &&
            valid_item(&self.iyr) &&
            valid_item(&self.eyr) &&
            valid_item(&self.hgt) &&
            valid_item(&self.hcl) &&
            valid_item(&self.ecl) &&
            valid_item(&self.pid)
    }

    fn parse(input: &String) -> Passport {
        // str::split_whitespace
        let mut p = Passport{byr:None, iyr:None, eyr:None, hgt:None, hcl:None, ecl:None, pid:None, cid:None};
        println!("{}", input);
        for item in input.split_whitespace() {
            let components: Vec<&str> = item.split(':').collect();
            let key = components[0];
            let value = components[1];
            //println!("{} | {} {}", item, key, value);
            match key {
                "byr" => p.byr = Some(Item::BirthYear(String::from(value))),
                "iyr" => p.iyr = Some(Item::IssueYear(String::from(value))),
                "eyr" => p.eyr = Some(Item::ExpirationYear(String::from(value))),
                "hgt" => p.hgt = Some(Item::Height(String::from(value))),
                "hcl" => p.hcl = Some(Item::HairColor(String::from(value))),
                "ecl" => p.ecl = Some(Item::EyeColor(String::from(value))),
                "pid" => p.pid = Some(Item::PassportID(String::from(value))),
                "cid" => p.cid = Some(Item::CountryID(String::from(value))),
                _ => ()
            }
        }
        p
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn empty_line(input: &str) -> bool {
    input.trim().is_empty()
}

#[aoc_generator(day4)]
fn input_gen(input: &str) -> Vec<Passport> {
    let mut batch = Vec::new();
    let mut entry = String::new();
    for line in input.lines() {
        // found an empty line, try to generate a Passport
        if empty_line(line) {
            batch.push(Passport::parse(&entry));
            entry.clear();
        } else {
            entry.push_str(line);
            entry.push(' ');
        }
    }
    // Don't forget last entry
    batch.push(Passport::parse(&entry));

    batch
}


// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
/**
 * Count the number of valid passports
 */
#[aoc(day4, part1)]
fn part1(batch: &[Passport]) -> usize {
    let valid: Vec<&Passport> = batch.iter().filter(|p| p.valid()).collect();
    valid.len()
}


// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn check_input_gen() {
        let passports = input_gen(INPUT);
        assert_eq!(passports.len(), 4);
        for p in passports {
            println!("{}", p);
        }
    }
}
