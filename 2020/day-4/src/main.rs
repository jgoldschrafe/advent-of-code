#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug, Clone)]
struct ValidationError;

#[derive(Debug, Clone)]
enum HeightUnit {
    CENTIMETERS,
    INCHES,
}

#[derive(Debug, Clone)]
struct Height {
    unit: HeightUnit,
    value: i32,
}

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn validate(&self) -> Result<(), ValidationError> {
        // Birth year must be between 1920 and 2002, inclusive
        match self.byr {
            Some(byr) => Self::validate_in_range(1920, 2002, byr)?,
            None => return Err(ValidationError),
        }

        // Issue year must be between 2010 and 2020, inclusive
        match self.iyr {
            Some(iyr) => Self::validate_in_range(2010, 2020, iyr)?,
            None => return Err(ValidationError),
        }

        // Expiration year must be between 2020 and 2030, inclusive
        match self.eyr {
            Some(eyr) => Self::validate_in_range(2020, 2030, eyr)?,
            None => return Err(ValidationError),
        }

        // Height must be within valid range
        match &self.hgt {
            Some(hgt) => match hgt.unit {
                HeightUnit::CENTIMETERS => Self::validate_in_range(150, 193, hgt.value)?,
                HeightUnit::INCHES => Self::validate_in_range(59, 76, hgt.value)?,
            },
            None => return Err(ValidationError),
        }

        // Hair color must be HTML-type color
        match &self.hcl {
            Some(hcl) => Self::validate_hex_color(&hcl)?,
            None => return Err(ValidationError),
        }

        // Eye color must belong to a list of acceptable values
        match &self.ecl {
            Some(ecl) => Self::validate_eye_color(&ecl)?,
            None => return Err(ValidationError),
        }

        match &self.pid {
            Some(pid) => Self::validate_pid(&pid)?,
            None => return Err(ValidationError),
        }

        Ok(())
    }

    fn validate_in_range(min: i32, max: i32, value: i32) -> Result<(), ValidationError> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }

    fn validate_hex_color(value: &str) -> Result<(), ValidationError> {
        lazy_static! {
            static ref HEX_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        if HEX_RE.is_match(value) {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }

    fn validate_eye_color(value: &str) -> Result<(), ValidationError> {
        lazy_static! {
            static ref EYE_COLORS: Vec<String> = vec![
                "amb".to_string(),
                "blu".to_string(),
                "brn".to_string(),
                "gry".to_string(),
                "grn".to_string(),
                "hzl".to_string(),
                "oth".to_string(),
            ];
        }

        if EYE_COLORS.iter().any(|i| value == i) {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }

    fn validate_pid(value: &str) -> Result<(), ValidationError> {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        if PID_RE.is_match(value) {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::new();

        for tok in input_str.split(&[' ', '\n'][..]) {
            let mut kv_iter = tok.split(":");
            let key = kv_iter.next().unwrap();
            let value = kv_iter.next().unwrap().to_string();
            match &key[..] {
                "byr" => passport.byr = Some(match value.parse() {
                    Ok(byr) => byr,
                    Err(_) => return Err(ParseError),
                }),
                "iyr" => passport.iyr = Some(match value.parse() {
                    Ok(iyr) => iyr,
                    Err(_) => return Err(ParseError),
                }),
                "eyr" => passport.eyr = Some(match value.parse() {
                    Ok(eyr) => eyr,
                    Err(_) => return Err(ParseError),
                }),
                "hgt" => {
                    passport.hgt = Some(Height{
                        unit: match &value[value.len() - 2..] {
                            "cm" => HeightUnit::CENTIMETERS,
                            "in" => HeightUnit::INCHES,
                            _ => return Err(ParseError),
                        },
                        value: match &value[..value.len() - 2].parse() {
                            Ok(measurement) => *measurement,
                            Err(_) => return Err(ParseError),
                        }
                    });
                },
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                "cid" => passport.cid = Some(value),
                _ => return Err(ParseError),
            }
        }

        Ok(passport)
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    let passports: Vec<Passport> = buf.split("\n\n")
        .filter_map(|passport_str| passport_str.parse().ok())
        .collect();

    let num_valid: usize = passports.iter()
        .filter_map(|p| p.validate().ok())
        .count();

    println!("{}", num_valid);
    Ok(())
}
