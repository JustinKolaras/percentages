use eval::eval;
use regex::Captures;
use regex::Regex;
use std::str::FromStr;

const VERIFY: &str = r"\(((\d+\.?\d*|\+|\-)+)\)/(\d+)";
const WHITESPACE_ONLY: &str = r"\A\s*\z";

#[derive(Debug)]
enum TypeMatchError {
    Space,
    Digit,
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

pub struct CalcResult {
    pub elements: String,
    pub percentage: String,
}

pub struct ErrResult {
    pub error: String,
    pub emphasis: Option<String>,
}

impl FromStr for CalcData {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let verify_parser: Regex = Regex::new(VERIFY).unwrap();
        let whitespace_parser: Regex = Regex::new(WHITESPACE_ONLY).unwrap();

        if whitespace_parser.is_match(s) {
            return Err(ConvertError::NotMatch(TypeMatchError::Form));
        }

        if !verify_parser.is_match(s) {
            if s.contains(' ') {
                return Err(ConvertError::NotMatch(TypeMatchError::Space));
            }
            return Err(ConvertError::NotMatch(TypeMatchError::Form));
        }

        let captures: Captures = verify_parser.captures(s.trim()).unwrap();

        // Has to be done separately from the "catch all" if statement on line 42.
        // There is freedom to unwrap here as we've already checked if the text matches the RegEx pattern.
        if captures
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap()
            < 1
        {
            return Err(ConvertError::NotMatch(TypeMatchError::Digit));
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

pub fn run(numeric: String) -> Result<CalcResult, ErrResult> {
    let parsed: CalcData = match CalcData::from_str(&numeric) {
        Ok(v) => v,
        Err(err) => {
            match err {
                ConvertError::NotMatch(TypeMatchError::Space) => return Err(
                    ErrResult {
                        error: "Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.".to_string(), 
                        emphasis: Some("no spaces permitted".to_string()), 
                    }),
                ConvertError::NotMatch(TypeMatchError::Digit) => return Err(
                    ErrResult {
                        error: "Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.".to_string(), 
                        emphasis: Some("is a positive integer".to_string()),
                    }),
                ConvertError::NotMatch(TypeMatchError::Form) => return Err(
                    ErrResult {
                        error: "Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.".to_string(), 
                        emphasis: Some("must be in the form of (...)/x".to_string()),
                    }),
            };
        }
    };

    if parsed.add_seq_elcount != (parsed.divider as usize) {
        return Err(ErrResult {
            error: format!(
                "Number of elements does not equal divider. Number elements: {} Divider: {}",
                &parsed.add_seq_elcount, &parsed.divider
            ),
            emphasis: None,
        });
    }

    let evaluation: f64 = eval(format!("({}) * 100", &numeric).as_str())
        .unwrap()
        .as_f64()
        .unwrap()
        .round();

    Ok(CalcResult {
        elements: parsed.add_seq_elcount.to_string(),
        percentage: evaluation.to_string(),
    })
}
