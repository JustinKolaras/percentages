use colored::*;
use evalexpr::*;
use regex::Captures;
use regex::Regex;
use std::str::FromStr;
use std::{io, process};

const VERIFY: &str = r"\(((\d+\.?\d*|\+|\-)+)\)/(\d+)";
const WHITESPACE_ONLY: &str = r"\A\s*\z";

#[derive(Debug)]
enum TypeMatchError {
    Space,
    InvalidDigit,
    Form,
}

#[derive(Debug)]
enum ConvertError {
    NotMatch(TypeMatchError),
}

struct CalcData {
    add_seq_elcount: usize,
    divider: u32,
}

impl FromStr for CalcData {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let verify_parser: Regex = Regex::new(VERIFY).unwrap();

        if !verify_parser.is_match(s) {
            if s.contains(' ') {
                return Err(ConvertError::NotMatch(TypeMatchError::Space));
            }
            return Err(ConvertError::NotMatch(TypeMatchError::Form));
        }

        let captures: Captures = verify_parser.captures(s.trim()).unwrap();

        // Has to be done separately.
        // There is freedom to unwrap here as we've already checked matches.
        if captures
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap()
            < 1
        {
            return Err(ConvertError::NotMatch(TypeMatchError::InvalidDigit));
        }

        let add_seq_raw: String = captures.get(1).unwrap().as_str().to_owned();
        let add_seq_elcount: usize = add_seq_raw.split(['+', '-']).count();
        let divider: u32 = s.split('/').collect::<Vec<&str>>()[1]
            .trim()
            .parse::<u32>()
            .unwrap();

        Ok(CalcData {
            add_seq_elcount,
            divider,
        })
    }
}

fn main() {
    println!("Numeric?");

    let mut numeric: String;

    // Name for clarity.
    'redo_input: loop {
        numeric = String::new();
        io::stdin().read_line(&mut numeric).unwrap();

        if numeric.to_lowercase().trim() == "exit" {
            process::exit(0);
        }

        if Regex::new(WHITESPACE_ONLY).unwrap().is_match(&numeric) {
            continue 'redo_input;
        }

        let parsed = match CalcData::from_str(&numeric) {
            Ok(v) => v,
            Err(err) => {
                match err {
                    ConvertError::NotMatch(TypeMatchError::Space) => println!("Invalid equation, try again.\nNote: {}; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.", "no spaces permitted".bold() ),
                    ConvertError::NotMatch(TypeMatchError::InvalidDigit) => println!("Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` {}.", "is a positive integer".bold()),
                    ConvertError::NotMatch(TypeMatchError::Form) => println!("Invalid equation, try again.\nNote: no spaces permitted; {}\nwhere `...` is an addition sequence and `x` is a positive integer.", "must be in the form of (...)/x".bold()),
                };
                continue 'redo_input;
            }
        };

        if parsed.add_seq_elcount != (parsed.divider as usize) {
            println!(
                "Number of elements does not equal divider.\nNumber elements: {}\nDivider: {}",
                &parsed.add_seq_elcount, &parsed.divider
            );
            continue 'redo_input;
        }

        let evaluation = eval(format!("({}) * 100", &numeric).as_str()).unwrap();

        println!(
            "Number elements: {}",
            parsed.add_seq_elcount.to_string().bold()
        );
        println!("Result: {:.4}%", evaluation.to_string().bold());
    }
}
